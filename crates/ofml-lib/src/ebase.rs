//! EBase file format reader for OFML data files.
//!
//! EBase is a proprietary binary database format used by EasternGraphics
//! in their OFML (Open Furniture Modeling Language) data files.
//!
//! Based on reverse engineering of ebase.dll from pCon.DataClient.

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::path::Path;

/// Magic bytes for EBase files
const MAGIC: &[u8] = b"EBDBF\x00";
const HEADER_SIZE: usize = 0x34; // 52 bytes

/// Error types for EBase operations
#[derive(Debug)]
pub enum EBaseError {
    Io(io::Error),
    InvalidMagic,
    UnsupportedVersion(u16),
    InvalidOffset,
    InvalidTable(String),
    ParseError(String),
}

impl From<io::Error> for EBaseError {
    fn from(err: io::Error) -> Self {
        EBaseError::Io(err)
    }
}

impl std::fmt::Display for EBaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EBaseError::Io(e) => write!(f, "IO error: {}", e),
            EBaseError::InvalidMagic => write!(f, "Invalid EBase magic"),
            EBaseError::UnsupportedVersion(v) => write!(f, "Unsupported version: {}", v),
            EBaseError::InvalidOffset => write!(f, "Invalid offset"),
            EBaseError::InvalidTable(name) => write!(f, "Invalid table: {}", name),
            EBaseError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for EBaseError {}

/// Column data types in EBase tables
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum ColumnType {
    Int8Signed = 1,
    Int8Unsigned = 2,
    Int16Signed = 3,
    Int16Unsigned = 4,
    Int32Signed = 5,
    Int32Unsigned = 6,
    Float32 = 7,
    Float64 = 8,
    StringInline = 9,
    StringOffset = 10,
    Blob = 11,
}

impl ColumnType {
    fn from_u16(value: u16) -> Option<Self> {
        match value {
            1 => Some(ColumnType::Int8Signed),
            2 => Some(ColumnType::Int8Unsigned),
            3 => Some(ColumnType::Int16Signed),
            4 => Some(ColumnType::Int16Unsigned),
            5 => Some(ColumnType::Int32Signed),
            6 => Some(ColumnType::Int32Unsigned),
            7 => Some(ColumnType::Float32),
            8 => Some(ColumnType::Float64),
            9 => Some(ColumnType::StringInline),
            10 => Some(ColumnType::StringOffset),
            11 => Some(ColumnType::Blob),
            _ => None,
        }
    }

    fn size(&self) -> usize {
        match self {
            ColumnType::Int8Signed | ColumnType::Int8Unsigned => 1,
            ColumnType::Int16Signed | ColumnType::Int16Unsigned => 2,
            ColumnType::Int32Signed | ColumnType::Int32Unsigned => 4,
            ColumnType::Float32 => 4,
            ColumnType::Float64 => 8,
            ColumnType::StringInline => 0, // Variable
            ColumnType::StringOffset | ColumnType::Blob => 4,
        }
    }
}

/// A column definition in an EBase table
#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub type_id: u16,
    pub offset: u16,
    pub size: usize,
    pub flags: u16,
}

impl Column {
    pub fn column_type(&self) -> Option<ColumnType> {
        ColumnType::from_u16(self.type_id)
    }
}

/// An EBase table with columns and record info
#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub record_count: u32,
    pub record_size: u16,
    pub data_offset: u32,
}

/// A value from an EBase record
#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    UInt(u64),
    Float(f64),
    String(String),
    Blob(u32),
    Null,
}

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            Value::UInt(u) => Some(*u as i64),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            Value::Int(i) => Some(*i as f64),
            Value::UInt(u) => Some(*u as f64),
            _ => None,
        }
    }
}

/// A record (row) from an EBase table
pub type Record = HashMap<String, Value>;

/// EBase database reader
pub struct EBaseReader {
    reader: BufReader<File>,
    pub major_version: u16,
    pub minor_version: u16,
    pub tables: HashMap<String, Table>,
    string_pool_offset: u32,
    string_pool_size: u32,
    string_cache: HashMap<u32, String>,
}

impl EBaseReader {
    /// Open an EBase database file
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, EBaseError> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        // Read and validate magic
        let mut magic = [0u8; 6];
        reader.read_exact(&mut magic)?;
        if magic != MAGIC {
            return Err(EBaseError::InvalidMagic);
        }

        // Read header
        let mut header = [0u8; HEADER_SIZE - 6];
        reader.read_exact(&mut header)?;

        // Parse header (all big-endian)
        let major_version = u16::from_be_bytes([header[2], header[3]]);
        let minor_version = u16::from_be_bytes([header[4], header[5]]);

        if major_version != 1 {
            return Err(EBaseError::UnsupportedVersion(major_version));
        }

        let string_pool_offset =
            u32::from_be_bytes([header[14], header[15], header[16], header[17]]);
        let string_data_size = u32::from_be_bytes([header[34], header[35], header[36], header[37]]);
        let num_tables = u32::from_be_bytes([header[38], header[39], header[40], header[41]]);

        let mut db = EBaseReader {
            reader,
            major_version,
            minor_version,
            tables: HashMap::new(),
            string_pool_offset,
            string_pool_size: string_data_size,
            string_cache: HashMap::new(),
        };

        // Parse table directory (starts at 0x38)
        db.parse_table_directory(num_tables, 0x38)?;

        Ok(db)
    }

    /// Read a length-prefixed string from the string pool
    fn read_string_at(&mut self, offset: u32) -> Result<String, EBaseError> {
        if offset == 0 {
            return Ok(String::new());
        }

        // Check cache
        if let Some(s) = self.string_cache.get(&offset) {
            return Ok(s.clone());
        }

        // Validate offset is within string pool
        if self.string_pool_offset > 0 {
            let pool_end = self.string_pool_offset + self.string_pool_size;
            if offset < self.string_pool_offset || offset >= pool_end {
                return Ok(String::new());
            }
        }

        // Save position, read string, restore position
        let pos = self.reader.stream_position()?;
        self.reader.seek(SeekFrom::Start(offset as u64))?;

        // Read length prefix (2 bytes BE)
        let mut len_bytes = [0u8; 2];
        if self.reader.read_exact(&mut len_bytes).is_err() {
            self.reader.seek(SeekFrom::Start(pos))?;
            return Ok(String::new());
        }
        let str_len = u16::from_be_bytes(len_bytes) as usize;

        if str_len == 0 || str_len > 65535 || str_len > self.string_pool_size as usize {
            self.reader.seek(SeekFrom::Start(pos))?;
            return Ok(String::new());
        }

        // Read string data
        let mut data = vec![0u8; str_len];
        if self.reader.read_exact(&mut data).is_err() {
            self.reader.seek(SeekFrom::Start(pos))?;
            return Ok(String::new());
        }

        self.reader.seek(SeekFrom::Start(pos))?;

        // Decode as UTF-8 or Latin-1
        let result = String::from_utf8(data.clone())
            .unwrap_or_else(|_| data.iter().map(|&b| b as char).collect())
            .trim_end_matches('\0')
            .to_string();

        self.string_cache.insert(offset, result.clone());
        Ok(result)
    }

    /// Parse the table directory
    fn parse_table_directory(
        &mut self,
        num_tables: u32,
        dir_offset: u32,
    ) -> Result<(), EBaseError> {
        if num_tables == 0 || dir_offset == 0 {
            return Ok(());
        }

        for i in 0..num_tables {
            self.reader
                .seek(SeekFrom::Start((dir_offset + i * 8) as u64))?;

            let mut entry = [0u8; 8];
            self.reader.read_exact(&mut entry)?;

            let name_offset = u32::from_be_bytes([entry[0], entry[1], entry[2], entry[3]]);
            let table_def_offset = u32::from_be_bytes([entry[4], entry[5], entry[6], entry[7]]);

            let table_name = self.read_string_at(name_offset)?;
            if !table_name.is_empty() && table_def_offset > 0 {
                if let Ok(table) = self.parse_table_definition(&table_name, table_def_offset) {
                    self.tables.insert(table_name, table);
                }
            }
        }

        Ok(())
    }

    /// Parse a table definition
    fn parse_table_definition(&mut self, name: &str, offset: u32) -> Result<Table, EBaseError> {
        self.reader.seek(SeekFrom::Start(offset as u64))?;

        let mut header = [0u8; 36];
        self.reader.read_exact(&mut header)?;

        let record_count = u32::from_be_bytes([header[4], header[5], header[6], header[7]]);
        let column_count = u16::from_be_bytes([header[8], header[9]]);
        let record_size = u16::from_be_bytes([header[10], header[11]]);
        let column_def_offset =
            u32::from_be_bytes([header[16], header[17], header[18], header[19]]);
        let data_start = u32::from_be_bytes([header[20], header[21], header[22], header[23]]);

        // Parse columns
        let mut columns = Vec::new();
        if column_count > 0 && column_def_offset > 0 {
            self.reader
                .seek(SeekFrom::Start(column_def_offset as u64))?;

            for _ in 0..column_count {
                let mut col_data = [0u8; 32];
                self.reader.read_exact(&mut col_data)?;

                let col_name_offset =
                    u32::from_be_bytes([col_data[0], col_data[1], col_data[2], col_data[3]]);
                let col_type = u16::from_be_bytes([col_data[4], col_data[5]]);
                let col_flags = u16::from_be_bytes([col_data[6], col_data[7]]);
                let col_offset = u16::from_be_bytes([col_data[8], col_data[9]]);

                let col_name = self.read_string_at(col_name_offset)?;
                let col_size = ColumnType::from_u16(col_type)
                    .map(|t| t.size())
                    .unwrap_or(4);

                columns.push(Column {
                    name: col_name,
                    type_id: col_type,
                    offset: col_offset,
                    size: col_size,
                    flags: col_flags,
                });
            }
        }

        Ok(Table {
            name: name.to_string(),
            columns,
            record_count,
            record_size,
            data_offset: data_start,
        })
    }

    /// Read records from a table
    pub fn read_records(
        &mut self,
        table_name: &str,
        limit: Option<usize>,
    ) -> Result<Vec<Record>, EBaseError> {
        let table = self
            .tables
            .get(table_name)
            .ok_or_else(|| EBaseError::InvalidTable(table_name.to_string()))?
            .clone();

        if table.record_count == 0 || table.data_offset == 0 {
            return Ok(Vec::new());
        }

        let count = limit.map_or(table.record_count as usize, |l| {
            l.min(table.record_count as usize)
        });
        let mut records = Vec::with_capacity(count);

        for i in 0..count {
            let record_offset = table.data_offset as u64 + (i as u64 * table.record_size as u64);
            self.reader.seek(SeekFrom::Start(record_offset))?;

            let mut record_data = vec![0u8; table.record_size as usize];
            self.reader.read_exact(&mut record_data)?;

            let mut record = Record::new();
            for col in &table.columns {
                let value = self.parse_column_value(&record_data, col)?;
                record.insert(col.name.clone(), value);
            }
            records.push(record);
        }

        Ok(records)
    }

    /// Parse a column value from record data
    fn parse_column_value(
        &mut self,
        record_data: &[u8],
        col: &Column,
    ) -> Result<Value, EBaseError> {
        let offset = col.offset as usize;
        if offset >= record_data.len() {
            return Ok(Value::Null);
        }

        match col.type_id {
            1 => {
                // INT8_SIGNED
                Ok(Value::Int(record_data[offset] as i8 as i64))
            }
            2 => {
                // INT8_UNSIGNED
                Ok(Value::UInt(record_data[offset] as u64))
            }
            3 => {
                // INT16_SIGNED
                if offset + 2 > record_data.len() {
                    return Ok(Value::Null);
                }
                let val = i16::from_be_bytes([record_data[offset], record_data[offset + 1]]);
                Ok(Value::Int(val as i64))
            }
            4 => {
                // INT16_UNSIGNED
                if offset + 2 > record_data.len() {
                    return Ok(Value::Null);
                }
                let val = u16::from_be_bytes([record_data[offset], record_data[offset + 1]]);
                Ok(Value::UInt(val as u64))
            }
            5 => {
                // INT32_SIGNED
                if offset + 4 > record_data.len() {
                    return Ok(Value::Null);
                }
                let val = i32::from_be_bytes([
                    record_data[offset],
                    record_data[offset + 1],
                    record_data[offset + 2],
                    record_data[offset + 3],
                ]);
                Ok(Value::Int(val as i64))
            }
            6 => {
                // INT32_UNSIGNED
                if offset + 4 > record_data.len() {
                    return Ok(Value::Null);
                }
                let val = u32::from_be_bytes([
                    record_data[offset],
                    record_data[offset + 1],
                    record_data[offset + 2],
                    record_data[offset + 3],
                ]);
                Ok(Value::UInt(val as u64))
            }
            7 => {
                // FLOAT32
                if offset + 4 > record_data.len() {
                    return Ok(Value::Null);
                }
                let val = f32::from_be_bytes([
                    record_data[offset],
                    record_data[offset + 1],
                    record_data[offset + 2],
                    record_data[offset + 3],
                ]);
                Ok(Value::Float(val as f64))
            }
            8 => {
                // FLOAT64
                if offset + 8 > record_data.len() {
                    return Ok(Value::Null);
                }
                let val = f64::from_be_bytes([
                    record_data[offset],
                    record_data[offset + 1],
                    record_data[offset + 2],
                    record_data[offset + 3],
                    record_data[offset + 4],
                    record_data[offset + 5],
                    record_data[offset + 6],
                    record_data[offset + 7],
                ]);
                Ok(Value::Float(val))
            }
            9 => {
                // STRING_INLINE
                let end = record_data[offset..]
                    .iter()
                    .position(|&b| b == 0)
                    .map(|p| offset + p)
                    .unwrap_or(record_data.len());
                let data = &record_data[offset..end];
                let s = String::from_utf8(data.to_vec())
                    .unwrap_or_else(|_| data.iter().map(|&b| b as char).collect());
                Ok(Value::String(s))
            }
            10 => {
                // STRING_OFFSET
                if offset + 4 > record_data.len() {
                    return Ok(Value::Null);
                }
                let str_offset = u32::from_be_bytes([
                    record_data[offset],
                    record_data[offset + 1],
                    record_data[offset + 2],
                    record_data[offset + 3],
                ]);
                if str_offset == 0 {
                    Ok(Value::String(String::new()))
                } else {
                    Ok(Value::String(self.read_string_at(str_offset)?))
                }
            }
            11 => {
                // BLOB
                if offset + 4 > record_data.len() {
                    return Ok(Value::Null);
                }
                let val = u32::from_be_bytes([
                    record_data[offset],
                    record_data[offset + 1],
                    record_data[offset + 2],
                    record_data[offset + 3],
                ]);
                Ok(Value::Blob(val))
            }
            _ => Ok(Value::Null),
        }
    }

    /// Get table names
    pub fn table_names(&self) -> Vec<&str> {
        self.tables.keys().map(|s| s.as_str()).collect()
    }

    /// Get a table definition
    pub fn get_table(&self, name: &str) -> Option<&Table> {
        self.tables.get(name)
    }
}

/// ODB3D record - 3D object definition
#[derive(Debug, Clone)]
pub struct Odb3dRecord {
    pub odb_name: String,
    pub obj_name: String,
    pub visible: String,
    pub x_offs: String,
    pub y_offs: String,
    pub z_offs: String,
    pub x_rot: String,
    pub y_rot: String,
    pub z_rot: String,
    pub ctor: String,
    pub mat: String,
    pub attrib: String,
    pub link: String,
}

impl Odb3dRecord {
    /// Parse from a record HashMap
    pub fn from_record(record: &Record) -> Option<Self> {
        Some(Odb3dRecord {
            odb_name: record.get("odb_name")?.as_str()?.to_string(),
            obj_name: record.get("obj_name")?.as_str().unwrap_or("").to_string(),
            visible: record.get("visible")?.as_str().unwrap_or("").to_string(),
            x_offs: record.get("x_offs")?.as_str().unwrap_or("0").to_string(),
            y_offs: record.get("y_offs")?.as_str().unwrap_or("0").to_string(),
            z_offs: record.get("z_offs")?.as_str().unwrap_or("0").to_string(),
            x_rot: record.get("x_rot")?.as_str().unwrap_or("0").to_string(),
            y_rot: record.get("y_rot")?.as_str().unwrap_or("0").to_string(),
            z_rot: record.get("z_rot")?.as_str().unwrap_or("0").to_string(),
            ctor: record.get("ctor")?.as_str().unwrap_or("").to_string(),
            mat: record.get("mat")?.as_str().unwrap_or("").to_string(),
            attrib: record.get("attrib")?.as_str().unwrap_or("").to_string(),
            link: record.get("link")?.as_str().unwrap_or("").to_string(),
        })
    }

    /// Parse the constructor to extract geometry file name
    /// Format: "filename" scale_x scale_y scale_z imp
    /// or: "::package::path::filename" scale_x scale_y scale_z imp
    pub fn parse_ctor(&self) -> Option<(String, [f32; 3])> {
        let ctor = self.ctor.trim();
        if ctor.is_empty() || ctor == "top" {
            return None;
        }

        // Extract quoted filename
        if !ctor.starts_with('"') {
            return None;
        }

        let end_quote = ctor[1..].find('"')?;
        let mut filename = &ctor[1..=end_quote];

        // Strip package path prefix (e.g., "::gsx::ac::filename" -> "filename")
        if filename.starts_with("::") {
            if let Some(pos) = filename.rfind("::") {
                filename = &filename[pos + 2..];
            }
        }

        // Parse scale factors after filename
        let rest = &ctor[end_quote + 2..];
        let parts: Vec<&str> = rest.split_whitespace().collect();

        let scale = if parts.len() >= 3 {
            [
                parts[0].parse().unwrap_or(1.0),
                parts[1].parse().unwrap_or(1.0),
                parts[2].parse().unwrap_or(1.0),
            ]
        } else {
            [1.0, 1.0, 1.0]
        };

        Some((filename.to_string(), scale))
    }

    /// Parse offset as float (simple numeric values only)
    /// For expressions with variables, use `evaluate_offset()` instead.
    pub fn parse_offset(&self) -> [f32; 3] {
        [
            self.x_offs.parse().unwrap_or(0.0),
            self.y_offs.parse().unwrap_or(0.0),
            self.z_offs.parse().unwrap_or(0.0),
        ]
    }

    /// Parse rotation as float (simple numeric values only)
    /// For expressions with variables, use `evaluate_rotation()` instead.
    pub fn parse_rotation(&self) -> [f32; 3] {
        [
            self.x_rot.parse().unwrap_or(0.0),
            self.y_rot.parse().unwrap_or(0.0),
            self.z_rot.parse().unwrap_or(0.0),
        ]
    }

    /// Evaluate offset expressions with variable substitution.
    ///
    /// Offset fields can contain PostScript expressions like:
    /// - Simple numbers: `0.5`, `-0.3`
    /// - Variable references: `${M__BREITE:-1000} 1000 /`
    /// - Function calls: `f_offset`
    ///
    /// Returns the evaluated [x, y, z] offset in meters.
    pub fn evaluate_offset(&self, props: &std::collections::HashMap<String, f64>) -> [f32; 3] {
        use crate::ebase_expr::EbaseEvaluator;

        let eval_field = |field: &str| -> f32 {
            let trimmed = field.trim();
            if trimmed.is_empty() {
                return 0.0;
            }

            // Try simple float parse first (most common case)
            if let Ok(val) = trimmed.parse::<f32>() {
                return val;
            }

            // Try expression evaluation
            let mut evaluator = EbaseEvaluator::new();
            match evaluator.evaluate_to_number(trimmed, props) {
                Ok(val) => val as f32,
                Err(_) => 0.0, // Default to 0 on error
            }
        };

        [
            eval_field(&self.x_offs),
            eval_field(&self.y_offs),
            eval_field(&self.z_offs),
        ]
    }

    /// Evaluate rotation expressions with variable substitution.
    ///
    /// Rotation fields can contain PostScript expressions.
    /// Rotation is applied in order: X → Y → Z (each affects subsequent axes).
    /// Angles are in degrees, counter-clockwise positive.
    ///
    /// Returns the evaluated [x, y, z] rotation in degrees.
    pub fn evaluate_rotation(&self, props: &std::collections::HashMap<String, f64>) -> [f32; 3] {
        use crate::ebase_expr::EbaseEvaluator;

        let eval_field = |field: &str| -> f32 {
            let trimmed = field.trim();
            if trimmed.is_empty() {
                return 0.0;
            }

            // Try simple float parse first
            if let Ok(val) = trimmed.parse::<f32>() {
                return val;
            }

            // Try expression evaluation
            let mut evaluator = EbaseEvaluator::new();
            match evaluator.evaluate_to_number(trimmed, props) {
                Ok(val) => val as f32,
                Err(_) => 0.0,
            }
        };

        [
            eval_field(&self.x_rot),
            eval_field(&self.y_rot),
            eval_field(&self.z_rot),
        ]
    }
}

/// Read ODB3D records from an ebase file
pub fn read_odb3d(path: &Path) -> Result<Vec<Odb3dRecord>, EBaseError> {
    let mut reader = EBaseReader::open(path)?;

    if !reader.tables.contains_key("odb3d") {
        return Ok(Vec::new());
    }

    let records = reader.read_records("odb3d", None)?;
    let mut result = Vec::new();

    for record in &records {
        if let Some(odb_record) = Odb3dRecord::from_record(record) {
            // Skip empty records
            if !odb_record.odb_name.is_empty() || !odb_record.obj_name.is_empty() {
                result.push(odb_record);
            }
        }
    }

    Ok(result)
}

/// ODB2D record - 2D object definition for floor plan views
#[derive(Debug, Clone)]
pub struct Odb2dRecord {
    /// Object database name reference
    pub odb_name: String,
    /// Object instance name
    pub obj_name: String,
    /// Visibility expression
    pub visible: String,
    /// 2D primitive type (lines, polygon, rect, circle, arc, text, etc.)
    pub prim_type: String,
    /// X coordinates (comma-separated for multi-point primitives)
    pub x_coords: String,
    /// Y coordinates (comma-separated for multi-point primitives)
    pub y_coords: String,
    /// Width (for rect, ellipse)
    pub width: String,
    /// Height (for rect, ellipse)
    pub height: String,
    /// Radius (for circle, arc)
    pub radius: String,
    /// Start angle for arcs (degrees)
    pub start_angle: String,
    /// End/sweep angle for arcs (degrees)
    pub end_angle: String,
    /// Text content (for text primitives)
    pub text: String,
    /// Font size (for text primitives)
    pub font_size: String,
    /// Layer name for visibility control
    pub layer: String,
    /// Line color expression
    pub color: String,
    /// Fill color expression
    pub fill_color: String,
    /// Line width
    pub line_width: String,
    /// Line style (solid, dashed, dotted)
    pub line_style: String,
    /// Custom attributes
    pub attrib: String,
}

impl Odb2dRecord {
    /// Parse from a record HashMap
    pub fn from_record(record: &Record) -> Option<Self> {
        Some(Odb2dRecord {
            odb_name: record
                .get("odb_name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            obj_name: record
                .get("obj_name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            visible: record
                .get("visible")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            prim_type: record
                .get("prim_type")
                .and_then(|v| v.as_str())
                .or_else(|| record.get("type").and_then(|v| v.as_str()))
                .unwrap_or("")
                .to_string(),
            x_coords: record
                .get("x_coords")
                .and_then(|v| v.as_str())
                .or_else(|| record.get("x").and_then(|v| v.as_str()))
                .or_else(|| record.get("x1").and_then(|v| v.as_str()))
                .unwrap_or("")
                .to_string(),
            y_coords: record
                .get("y_coords")
                .and_then(|v| v.as_str())
                .or_else(|| record.get("y").and_then(|v| v.as_str()))
                .or_else(|| record.get("y1").and_then(|v| v.as_str()))
                .unwrap_or("")
                .to_string(),
            width: record
                .get("width")
                .and_then(|v| v.as_str())
                .or_else(|| record.get("w").and_then(|v| v.as_str()))
                .unwrap_or("")
                .to_string(),
            height: record
                .get("height")
                .and_then(|v| v.as_str())
                .or_else(|| record.get("h").and_then(|v| v.as_str()))
                .unwrap_or("")
                .to_string(),
            radius: record
                .get("radius")
                .and_then(|v| v.as_str())
                .or_else(|| record.get("r").and_then(|v| v.as_str()))
                .unwrap_or("")
                .to_string(),
            start_angle: record
                .get("start_angle")
                .and_then(|v| v.as_str())
                .or_else(|| record.get("angle1").and_then(|v| v.as_str()))
                .unwrap_or("")
                .to_string(),
            end_angle: record
                .get("end_angle")
                .and_then(|v| v.as_str())
                .or_else(|| record.get("angle2").and_then(|v| v.as_str()))
                .unwrap_or("")
                .to_string(),
            text: record
                .get("text")
                .and_then(|v| v.as_str())
                .or_else(|| record.get("label").and_then(|v| v.as_str()))
                .unwrap_or("")
                .to_string(),
            font_size: record
                .get("font_size")
                .and_then(|v| v.as_str())
                .or_else(|| record.get("fsize").and_then(|v| v.as_str()))
                .unwrap_or("10")
                .to_string(),
            layer: record
                .get("layer")
                .and_then(|v| v.as_str())
                .unwrap_or("default")
                .to_string(),
            color: record
                .get("color")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            fill_color: record
                .get("fill_color")
                .and_then(|v| v.as_str())
                .or_else(|| record.get("fill").and_then(|v| v.as_str()))
                .unwrap_or("")
                .to_string(),
            line_width: record
                .get("line_width")
                .and_then(|v| v.as_str())
                .or_else(|| record.get("lwidth").and_then(|v| v.as_str()))
                .unwrap_or("1")
                .to_string(),
            line_style: record
                .get("line_style")
                .and_then(|v| v.as_str())
                .or_else(|| record.get("lstyle").and_then(|v| v.as_str()))
                .unwrap_or("solid")
                .to_string(),
            attrib: record
                .get("attrib")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        })
    }

    /// Parse X coordinates as a vector of floats
    pub fn parse_x_coords(&self) -> Vec<f64> {
        self.parse_coords(&self.x_coords)
    }

    /// Parse Y coordinates as a vector of floats
    pub fn parse_y_coords(&self) -> Vec<f64> {
        self.parse_coords(&self.y_coords)
    }

    fn parse_coords(&self, s: &str) -> Vec<f64> {
        s.split([',', ' '])
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.trim().parse().ok())
            .collect()
    }

    /// Parse width as float
    pub fn parse_width(&self) -> f64 {
        self.width.trim().parse().unwrap_or(0.0)
    }

    /// Parse height as float
    pub fn parse_height(&self) -> f64 {
        self.height.trim().parse().unwrap_or(0.0)
    }

    /// Parse radius as float
    pub fn parse_radius(&self) -> f64 {
        self.radius.trim().parse().unwrap_or(0.0)
    }

    /// Parse start angle as float (degrees)
    pub fn parse_start_angle(&self) -> f64 {
        self.start_angle.trim().parse().unwrap_or(0.0)
    }

    /// Parse end angle as float (degrees)
    pub fn parse_end_angle(&self) -> f64 {
        self.end_angle.trim().parse().unwrap_or(360.0)
    }

    /// Parse font size as float
    pub fn parse_font_size(&self) -> f64 {
        self.font_size.trim().parse().unwrap_or(10.0)
    }

    /// Parse line width as float
    pub fn parse_line_width(&self) -> f32 {
        self.line_width.trim().parse().unwrap_or(1.0)
    }

    /// Parse color string "r g b" or "r g b a" to [f32; 4]
    pub fn parse_color(&self) -> [f32; 4] {
        Self::parse_color_string(&self.color)
    }

    /// Parse fill color string
    pub fn parse_fill_color(&self) -> Option<[f32; 4]> {
        if self.fill_color.is_empty() {
            None
        } else {
            Some(Self::parse_color_string(&self.fill_color))
        }
    }

    fn parse_color_string(s: &str) -> [f32; 4] {
        let parts: Vec<f32> = s
            .split_whitespace()
            .filter_map(|p| p.parse().ok())
            .collect();
        match parts.len() {
            0 => [0.0, 0.0, 0.0, 1.0],
            1 => [parts[0], parts[0], parts[0], 1.0],
            2 => [parts[0], parts[1], 0.0, 1.0],
            3 => [parts[0], parts[1], parts[2], 1.0],
            _ => [parts[0], parts[1], parts[2], parts[3]],
        }
    }

    /// Get line style enum value
    pub fn get_line_style(&self) -> &str {
        match self.line_style.to_lowercase().as_str() {
            "dashed" | "dash" => "dashed",
            "dotted" | "dot" => "dotted",
            "dashdot" | "dash-dot" => "dashdot",
            _ => "solid",
        }
    }
}

/// Read ODB2D records from an ebase file
pub fn read_odb2d(path: &Path) -> Result<Vec<Odb2dRecord>, EBaseError> {
    let mut reader = EBaseReader::open(path)?;

    if !reader.tables.contains_key("odb2d") {
        return Ok(Vec::new());
    }

    let records = reader.read_records("odb2d", None)?;
    let mut result = Vec::new();

    for record in &records {
        if let Some(odb_record) = Odb2dRecord::from_record(record) {
            // Skip completely empty records
            if !odb_record.odb_name.is_empty() || !odb_record.prim_type.is_empty() {
                result.push(odb_record);
            }
        }
    }

    Ok(result)
}

/// Function definition from funcs table
#[derive(Debug, Clone)]
pub struct FuncDef {
    pub name: String,
    pub body: String,
}

/// OCD (Object Control Data) record for product configuration
#[derive(Debug, Clone)]
pub struct OcdRecord {
    /// Record fields as key-value pairs
    pub fields: HashMap<String, String>,
}

impl OcdRecord {
    /// Create from a generic record
    pub fn from_record(record: &Record) -> Self {
        let fields = record
            .iter()
            .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
            .collect();
        Self { fields }
    }

    /// Get a field value
    pub fn get(&self, key: &str) -> Option<&str> {
        self.fields.get(key).map(|s| s.as_str())
    }
}

/// Read OCD (Object Control Data) records from an ebase file
///
/// OCD files contain configuration data for products including:
/// - @Category entries for product categorization
/// - @MatPackage entries for material packages
/// - @ShowAddAttPts entries for attachment point visibility
/// - Other control parameters
pub fn read_ocd(path: &Path, table_name: &str) -> Result<Vec<OcdRecord>, EBaseError> {
    let mut reader = EBaseReader::open(path)?;

    // Try common OCD table names
    let table_names = if table_name.is_empty() {
        vec!["ocd", "control", "data"]
    } else {
        vec![table_name]
    };

    for name in table_names {
        if reader.tables.contains_key(name) {
            let records = reader.read_records(name, None)?;
            return Ok(records
                .into_iter()
                .map(|r| OcdRecord::from_record(&r))
                .collect());
        }
    }

    // No OCD table found, return empty
    Ok(Vec::new())
}

/// Read function definitions from an ebase file
pub fn read_funcs(path: &Path) -> Result<Vec<FuncDef>, EBaseError> {
    let mut reader = EBaseReader::open(path)?;

    if !reader.tables.contains_key("funcs") {
        return Ok(Vec::new());
    }

    let records = reader.read_records("funcs", None)?;
    let mut result = Vec::new();

    for record in &records {
        let name = record
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let body = record
            .get("body")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        if !name.is_empty() {
            result.push(FuncDef { name, body });
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_type_size() {
        assert_eq!(ColumnType::Int8Signed.size(), 1);
        assert_eq!(ColumnType::Int16Signed.size(), 2);
        assert_eq!(ColumnType::Int32Signed.size(), 4);
        assert_eq!(ColumnType::Float64.size(), 8);
        assert_eq!(ColumnType::StringOffset.size(), 4);
    }

    #[test]
    fn test_parse_ctor() {
        let record = Odb3dRecord {
            odb_name: "test".to_string(),
            obj_name: "o1".to_string(),
            visible: "".to_string(),
            x_offs: "0".to_string(),
            y_offs: "0.5".to_string(),
            z_offs: "0".to_string(),
            x_rot: "0".to_string(),
            y_rot: "0".to_string(),
            z_rot: "0".to_string(),
            ctor: "\"sbucosmod3_t31_openunit40_glass\" 1 1 1 imp".to_string(),
            mat: "".to_string(),
            attrib: "".to_string(),
            link: "".to_string(),
        };

        let (filename, scale) = record.parse_ctor().unwrap();
        assert_eq!(filename, "sbucosmod3_t31_openunit40_glass");
        assert_eq!(scale, [1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_odb2d_record_coords() {
        let mut record_map = Record::new();
        record_map.insert("odb_name".to_string(), Value::String("test_2d".to_string()));
        record_map.insert(
            "prim_type".to_string(),
            Value::String("polygon".to_string()),
        );
        record_map.insert(
            "x_coords".to_string(),
            Value::String("0,100,100,0".to_string()),
        );
        record_map.insert(
            "y_coords".to_string(),
            Value::String("0,0,50,50".to_string()),
        );
        record_map.insert(
            "color".to_string(),
            Value::String("0.5 0.5 0.5".to_string()),
        );

        let record = Odb2dRecord::from_record(&record_map).unwrap();
        assert_eq!(record.odb_name, "test_2d");
        assert_eq!(record.prim_type, "polygon");

        let x = record.parse_x_coords();
        assert_eq!(x, vec![0.0, 100.0, 100.0, 0.0]);

        let y = record.parse_y_coords();
        assert_eq!(y, vec![0.0, 0.0, 50.0, 50.0]);

        let color = record.parse_color();
        assert!((color[0] - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_odb2d_record_rect() {
        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("rect".to_string()));
        record_map.insert("x".to_string(), Value::String("10".to_string()));
        record_map.insert("y".to_string(), Value::String("20".to_string()));
        record_map.insert("width".to_string(), Value::String("100".to_string()));
        record_map.insert("height".to_string(), Value::String("50".to_string()));

        let record = Odb2dRecord::from_record(&record_map).unwrap();
        assert_eq!(record.prim_type, "rect");
        assert_eq!(record.parse_x_coords(), vec![10.0]);
        assert_eq!(record.parse_y_coords(), vec![20.0]);
        assert_eq!(record.parse_width(), 100.0);
        assert_eq!(record.parse_height(), 50.0);
    }

    #[test]
    fn test_odb2d_record_circle() {
        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("circle".to_string()));
        record_map.insert("x".to_string(), Value::String("50".to_string()));
        record_map.insert("y".to_string(), Value::String("50".to_string()));
        record_map.insert("r".to_string(), Value::String("25".to_string()));

        let record = Odb2dRecord::from_record(&record_map).unwrap();
        assert_eq!(record.prim_type, "circle");
        assert_eq!(record.parse_radius(), 25.0);
    }

    #[test]
    fn test_odb2d_record_arc() {
        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("arc".to_string()));
        record_map.insert("x".to_string(), Value::String("50".to_string()));
        record_map.insert("y".to_string(), Value::String("50".to_string()));
        record_map.insert("r".to_string(), Value::String("25".to_string()));
        record_map.insert("angle1".to_string(), Value::String("0".to_string()));
        record_map.insert("angle2".to_string(), Value::String("90".to_string()));

        let record = Odb2dRecord::from_record(&record_map).unwrap();
        assert_eq!(record.prim_type, "arc");
        assert_eq!(record.parse_start_angle(), 0.0);
        assert_eq!(record.parse_end_angle(), 90.0);
    }

    #[test]
    fn test_odb2d_record_text() {
        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("text".to_string()));
        record_map.insert("x".to_string(), Value::String("10".to_string()));
        record_map.insert("y".to_string(), Value::String("20".to_string()));
        record_map.insert("text".to_string(), Value::String("Hello World".to_string()));
        record_map.insert("font_size".to_string(), Value::String("12".to_string()));

        let record = Odb2dRecord::from_record(&record_map).unwrap();
        assert_eq!(record.prim_type, "text");
        assert_eq!(record.text, "Hello World");
        assert_eq!(record.parse_font_size(), 12.0);
    }

    #[test]
    fn test_odb2d_record_line_style() {
        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("lines".to_string()));
        record_map.insert(
            "line_style".to_string(),
            Value::String("dashed".to_string()),
        );
        record_map.insert("line_width".to_string(), Value::String("2.5".to_string()));

        let record = Odb2dRecord::from_record(&record_map).unwrap();
        assert_eq!(record.get_line_style(), "dashed");
        assert!((record.parse_line_width() - 2.5).abs() < 0.001);
    }

    #[test]
    fn test_odb2d_color_parsing() {
        // RGB color
        let color = Odb2dRecord::parse_color_string("0.8 0.4 0.2");
        assert!((color[0] - 0.8).abs() < 0.001);
        assert!((color[1] - 0.4).abs() < 0.001);
        assert!((color[2] - 0.2).abs() < 0.001);
        assert!((color[3] - 1.0).abs() < 0.001); // Default alpha

        // RGBA color
        let color = Odb2dRecord::parse_color_string("0.5 0.5 0.5 0.7");
        assert!((color[3] - 0.7).abs() < 0.001);

        // Single value (grayscale)
        let color = Odb2dRecord::parse_color_string("0.5");
        assert!((color[0] - 0.5).abs() < 0.001);
        assert!((color[1] - 0.5).abs() < 0.001);

        // Empty string
        let color = Odb2dRecord::parse_color_string("");
        assert!((color[0] - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_odb2d_two_component_color() {
        // Two components (partial color)
        let color = Odb2dRecord::parse_color_string("0.5 0.3");
        assert!((color[0] - 0.5).abs() < 0.001);
        assert!((color[1] - 0.3).abs() < 0.001);
        assert!((color[2] - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_odb2d_fill_color() {
        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("rect".to_string()));
        record_map.insert("fill_color".to_string(), Value::String("0.8 0.2 0.1".to_string()));

        let record = Odb2dRecord::from_record(&record_map).unwrap();
        let fill_color = record.parse_fill_color().unwrap();
        assert!((fill_color[0] - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_odb2d_fill_color_empty() {
        let mut record_map = Record::new();
        record_map.insert("prim_type".to_string(), Value::String("rect".to_string()));

        let record = Odb2dRecord::from_record(&record_map).unwrap();
        assert!(record.parse_fill_color().is_none());
    }

    #[test]
    fn test_odb2d_line_style_variants() {
        // Test dotted
        let mut record_map = Record::new();
        record_map.insert("line_style".to_string(), Value::String("dotted".to_string()));
        let record = Odb2dRecord::from_record(&record_map).unwrap();
        assert_eq!(record.get_line_style(), "dotted");

        // Test dot (alias)
        let mut record_map = Record::new();
        record_map.insert("line_style".to_string(), Value::String("dot".to_string()));
        let record = Odb2dRecord::from_record(&record_map).unwrap();
        assert_eq!(record.get_line_style(), "dotted");

        // Test dashdot
        let mut record_map = Record::new();
        record_map.insert("line_style".to_string(), Value::String("dashdot".to_string()));
        let record = Odb2dRecord::from_record(&record_map).unwrap();
        assert_eq!(record.get_line_style(), "dashdot");

        // Test dash-dot (alias)
        let mut record_map = Record::new();
        record_map.insert("line_style".to_string(), Value::String("dash-dot".to_string()));
        let record = Odb2dRecord::from_record(&record_map).unwrap();
        assert_eq!(record.get_line_style(), "dashdot");

        // Test solid (default)
        let mut record_map = Record::new();
        record_map.insert("line_style".to_string(), Value::String("solid".to_string()));
        let record = Odb2dRecord::from_record(&record_map).unwrap();
        assert_eq!(record.get_line_style(), "solid");

        // Test unknown (defaults to solid)
        let mut record_map = Record::new();
        record_map.insert("line_style".to_string(), Value::String("unknown".to_string()));
        let record = Odb2dRecord::from_record(&record_map).unwrap();
        assert_eq!(record.get_line_style(), "solid");
    }

    #[test]
    fn test_odb3d_record_debug_clone() {
        let record = Odb3dRecord {
            odb_name: "test".to_string(),
            obj_name: "obj".to_string(),
            visible: "1".to_string(),
            x_offs: "0".to_string(),
            y_offs: "0".to_string(),
            z_offs: "0".to_string(),
            x_rot: "0".to_string(),
            y_rot: "0".to_string(),
            z_rot: "0".to_string(),
            ctor: "test.geo".to_string(),
            mat: "mat1".to_string(),
            attrib: "attr".to_string(),
            link: "link".to_string(),
        };

        let debug_str = format!("{:?}", record);
        assert!(debug_str.contains("test"));

        let cloned = record.clone();
        assert_eq!(cloned.odb_name, "test");
    }

    #[test]
    fn test_odb3d_record_parse_offset() {
        let record = Odb3dRecord {
            odb_name: "test".to_string(),
            obj_name: "obj".to_string(),
            visible: "".to_string(),
            x_offs: "10.5".to_string(),
            y_offs: "20.3".to_string(),
            z_offs: "30.1".to_string(),
            x_rot: "0".to_string(),
            y_rot: "0".to_string(),
            z_rot: "0".to_string(),
            ctor: "".to_string(),
            mat: "".to_string(),
            attrib: "".to_string(),
            link: "".to_string(),
        };

        let offset = record.parse_offset();
        assert!((offset[0] - 10.5).abs() < 0.001);
        assert!((offset[1] - 20.3).abs() < 0.001);
        assert!((offset[2] - 30.1).abs() < 0.001);
    }

    #[test]
    fn test_odb3d_record_parse_rotation() {
        let record = Odb3dRecord {
            odb_name: "test".to_string(),
            obj_name: "obj".to_string(),
            visible: "".to_string(),
            x_offs: "0".to_string(),
            y_offs: "0".to_string(),
            z_offs: "0".to_string(),
            x_rot: "45".to_string(),
            y_rot: "90".to_string(),
            z_rot: "180".to_string(),
            ctor: "".to_string(),
            mat: "".to_string(),
            attrib: "".to_string(),
            link: "".to_string(),
        };

        let rotation = record.parse_rotation();
        assert!((rotation[0] - 45.0).abs() < 0.001);
        assert!((rotation[1] - 90.0).abs() < 0.001);
        assert!((rotation[2] - 180.0).abs() < 0.001);
    }

    #[test]
    fn test_odb3d_record_visible_field() {
        // Test visible field values
        let record = Odb3dRecord {
            odb_name: "test".to_string(),
            obj_name: "obj".to_string(),
            visible: "".to_string(),
            x_offs: "0".to_string(),
            y_offs: "0".to_string(),
            z_offs: "0".to_string(),
            x_rot: "0".to_string(),
            y_rot: "0".to_string(),
            z_rot: "0".to_string(),
            ctor: "".to_string(),
            mat: "".to_string(),
            attrib: "".to_string(),
            link: "".to_string(),
        };
        assert_eq!(record.visible, "");

        // "1" is visible
        let mut record2 = record.clone();
        record2.visible = "1".to_string();
        assert_eq!(record2.visible, "1");

        // "0" is not visible
        record2.visible = "0".to_string();
        assert_eq!(record2.visible, "0");
    }

    #[test]
    fn test_odb3d_record_parse_ctor_none() {
        let record = Odb3dRecord {
            odb_name: "test".to_string(),
            obj_name: "obj".to_string(),
            visible: "".to_string(),
            x_offs: "0".to_string(),
            y_offs: "0".to_string(),
            z_offs: "0".to_string(),
            x_rot: "0".to_string(),
            y_rot: "0".to_string(),
            z_rot: "0".to_string(),
            ctor: "".to_string(),  // Empty ctor
            mat: "".to_string(),
            attrib: "".to_string(),
            link: "".to_string(),
        };

        assert!(record.parse_ctor().is_none());
    }

    #[test]
    fn test_odb2d_record_debug_clone() {
        let mut record_map = Record::new();
        record_map.insert("odb_name".to_string(), Value::String("test".to_string()));
        record_map.insert("prim_type".to_string(), Value::String("polygon".to_string()));

        let record = Odb2dRecord::from_record(&record_map).unwrap();
        let debug_str = format!("{:?}", record);
        assert!(debug_str.contains("test"));

        let cloned = record.clone();
        assert_eq!(cloned.odb_name, "test");
    }

    #[test]
    fn test_odb2d_record_from_record_none() {
        // Record without prim_type (but might have odb_name)
        let mut record_map = Record::new();
        record_map.insert("other_field".to_string(), Value::String("value".to_string()));

        let record = Odb2dRecord::from_record(&record_map);
        assert!(record.is_some());  // Should still create a record
    }

    #[test]
    fn test_value_as_i64() {
        assert_eq!(Value::Int(42).as_i64(), Some(42));
        assert_eq!(Value::UInt(10).as_i64(), Some(10));  // UInt converts to i64
        assert_eq!(Value::Float(3.14).as_i64(), None);  // Float doesn't convert
        assert_eq!(Value::String("123".to_string()).as_i64(), None);
    }

    #[test]
    fn test_value_as_f64() {
        assert_eq!(Value::Float(3.14).as_f64(), Some(3.14));
        assert_eq!(Value::Int(42).as_f64(), Some(42.0));
        assert_eq!(Value::UInt(5).as_f64(), Some(5.0));
        assert_eq!(Value::String("3.14".to_string()).as_f64(), None);
    }

    #[test]
    fn test_value_as_str() {
        assert_eq!(Value::String("test".to_string()).as_str(), Some("test"));
        assert_eq!(Value::Int(42).as_str(), None);
    }

    #[test]
    fn test_value_debug() {
        let val = Value::Int(42);
        let debug_str = format!("{:?}", val);
        assert!(debug_str.contains("Int"));
    }

    #[test]
    fn test_value_clone() {
        let val = Value::String("test".to_string());
        let cloned = val.clone();
        assert_eq!(val.as_str(), cloned.as_str());
    }

    #[test]
    fn test_column_type_debug() {
        let col_type = ColumnType::Int32Signed;
        let debug_str = format!("{:?}", col_type);
        assert!(debug_str.contains("Int32Signed"));
    }

    #[test]
    fn test_column_type_clone() {
        let col_type = ColumnType::Float64;
        let cloned = col_type.clone();
        assert_eq!(cloned.size(), 8);
    }

    #[test]
    fn test_column_type_all_sizes() {
        assert_eq!(ColumnType::Int8Unsigned.size(), 1);
        assert_eq!(ColumnType::Int16Unsigned.size(), 2);
        assert_eq!(ColumnType::Int32Unsigned.size(), 4);
        assert_eq!(ColumnType::Float32.size(), 4);
    }

    #[test]
    fn test_func_def_debug_clone() {
        let func = FuncDef {
            name: "testFunc".to_string(),
            body: "return 42".to_string(),
        };

        let debug_str = format!("{:?}", func);
        assert!(debug_str.contains("testFunc"));

        let cloned = func.clone();
        assert_eq!(cloned.name, "testFunc");
        assert_eq!(cloned.body, "return 42");
    }

    #[test]
    fn test_ocd_record_from_record() {
        let mut record_map = Record::new();
        record_map.insert("key1".to_string(), Value::String("value1".to_string()));
        record_map.insert("key2".to_string(), Value::Int(42));

        let ocd_record = OcdRecord::from_record(&record_map);
        assert_eq!(ocd_record.get("key1"), Some("value1"));
        // Int values get converted to empty string via as_str() returning None
        // The actual value depends on the implementation
        assert!(ocd_record.get("key2").is_some());
    }

    #[test]
    fn test_ocd_record_get_missing() {
        let record = OcdRecord {
            fields: HashMap::new(),
        };
        assert!(record.get("missing").is_none());
    }

    #[test]
    fn test_ocd_record_debug_clone() {
        let mut fields = HashMap::new();
        fields.insert("key".to_string(), "value".to_string());
        let record = OcdRecord { fields };

        let debug_str = format!("{:?}", record);
        assert!(debug_str.contains("OcdRecord"));

        let cloned = record.clone();
        assert_eq!(cloned.get("key"), Some("value"));
    }

    #[test]
    fn test_ebase_error_display() {
        let io_err = EBaseError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"));
        let display_str = format!("{}", io_err);
        assert!(display_str.contains("IO error"));  // Note: "IO error" not "I/O error"

        let parse_err = EBaseError::ParseError("invalid data".to_string());
        let display_str = format!("{}", parse_err);
        assert!(display_str.contains("invalid data") || display_str.contains("Parse"));

        let invalid_table = EBaseError::InvalidTable("missing".to_string());
        let display_str = format!("{}", invalid_table);
        assert!(display_str.contains("missing") || display_str.contains("Invalid table"));
    }

    #[test]
    fn test_ebase_error_debug() {
        let err = EBaseError::InvalidMagic;
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("InvalidMagic"));
    }

    #[test]
    fn test_ebase_error_unsupported_version() {
        let err = EBaseError::UnsupportedVersion(999);
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("999"));
    }

    #[test]
    fn test_ebase_error_invalid_offset() {
        let err = EBaseError::InvalidOffset;
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("InvalidOffset"));
    }

    #[test]
    fn test_ebase_error_display_all_variants() {
        // InvalidMagic
        let err = EBaseError::InvalidMagic;
        let s = format!("{}", err);
        assert!(s.contains("Invalid EBase magic"));

        // UnsupportedVersion
        let err = EBaseError::UnsupportedVersion(42);
        let s = format!("{}", err);
        assert!(s.contains("42"));

        // InvalidOffset
        let err = EBaseError::InvalidOffset;
        let s = format!("{}", err);
        assert!(s.contains("Invalid offset"));
    }

    #[test]
    fn test_column_type_from_u16_all_variants() {
        assert_eq!(ColumnType::from_u16(1), Some(ColumnType::Int8Signed));
        assert_eq!(ColumnType::from_u16(2), Some(ColumnType::Int8Unsigned));
        assert_eq!(ColumnType::from_u16(3), Some(ColumnType::Int16Signed));
        assert_eq!(ColumnType::from_u16(4), Some(ColumnType::Int16Unsigned));
        assert_eq!(ColumnType::from_u16(5), Some(ColumnType::Int32Signed));
        assert_eq!(ColumnType::from_u16(6), Some(ColumnType::Int32Unsigned));
        assert_eq!(ColumnType::from_u16(7), Some(ColumnType::Float32));
        assert_eq!(ColumnType::from_u16(8), Some(ColumnType::Float64));
        assert_eq!(ColumnType::from_u16(9), Some(ColumnType::StringInline));
        assert_eq!(ColumnType::from_u16(10), Some(ColumnType::StringOffset));
        assert_eq!(ColumnType::from_u16(11), Some(ColumnType::Blob));
        assert_eq!(ColumnType::from_u16(12), None);
        assert_eq!(ColumnType::from_u16(0), None);
        assert_eq!(ColumnType::from_u16(999), None);
    }

    #[test]
    fn test_column_type_size_all_variants() {
        // Test all column type sizes
        assert_eq!(ColumnType::Int8Signed.size(), 1);
        assert_eq!(ColumnType::Int8Unsigned.size(), 1);
        assert_eq!(ColumnType::Int16Signed.size(), 2);
        assert_eq!(ColumnType::Int16Unsigned.size(), 2);
        assert_eq!(ColumnType::Int32Signed.size(), 4);
        assert_eq!(ColumnType::Int32Unsigned.size(), 4);
        assert_eq!(ColumnType::Float32.size(), 4);
        assert_eq!(ColumnType::Float64.size(), 8);
        assert_eq!(ColumnType::StringInline.size(), 0);
        assert_eq!(ColumnType::StringOffset.size(), 4);
        assert_eq!(ColumnType::Blob.size(), 4);
    }

    #[test]
    fn test_column_type_debug_clone_copy() {
        let col_type = ColumnType::Float64;
        let debug_str = format!("{:?}", col_type);
        assert!(debug_str.contains("Float64"));

        let cloned = col_type.clone();
        assert_eq!(cloned, ColumnType::Float64);

        // Copy trait - use after copy
        let copied = col_type;
        assert_eq!(copied, col_type);
    }

    #[test]
    fn test_column_struct() {
        let col = Column {
            name: "test_col".to_string(),
            type_id: 5, // Int32Signed
            offset: 10,
            size: 4,
            flags: 0,
        };

        let debug_str = format!("{:?}", col);
        assert!(debug_str.contains("test_col"));

        assert_eq!(col.column_type(), Some(ColumnType::Int32Signed));

        // Invalid type_id
        let invalid_col = Column {
            name: "invalid".to_string(),
            type_id: 999,
            offset: 0,
            size: 0,
            flags: 0,
        };
        assert_eq!(invalid_col.column_type(), None);
    }

    #[test]
    fn test_table_struct() {
        let table = Table {
            name: "test_table".to_string(),
            columns: vec![
                Column {
                    name: "col1".to_string(),
                    type_id: 5,
                    offset: 0,
                    size: 4,
                    flags: 0,
                },
            ],
            record_count: 100,
            record_size: 32,
            data_offset: 1024,
        };

        let debug_str = format!("{:?}", table);
        assert!(debug_str.contains("test_table"));

        let cloned = table.clone();
        assert_eq!(cloned.name, "test_table");
        assert_eq!(cloned.record_count, 100);
    }

    #[test]
    fn test_value_pattern_matching() {
        let int_val = Value::Int(42);
        assert!(matches!(int_val, Value::Int(_)));
        assert!(!matches!(int_val, Value::Float(_)));
        assert!(!matches!(int_val, Value::String(_)));

        let float_val = Value::Float(3.14);
        assert!(!matches!(float_val, Value::Int(_)));
        assert!(matches!(float_val, Value::Float(_)));
        assert!(!matches!(float_val, Value::String(_)));

        let str_val = Value::String("hello".to_string());
        assert!(!matches!(str_val, Value::Int(_)));
        assert!(!matches!(str_val, Value::Float(_)));
        assert!(matches!(str_val, Value::String(_)));

        let blob_val = Value::Blob(123);
        assert!(!matches!(blob_val, Value::Int(_)));
        assert!(!matches!(blob_val, Value::Float(_)));
        assert!(!matches!(blob_val, Value::String(_)));
        assert!(matches!(blob_val, Value::Blob(_)));

        let null_val = Value::Null;
        assert!(matches!(null_val, Value::Null));

        let uint_val = Value::UInt(999);
        assert!(matches!(uint_val, Value::UInt(_)));
    }

    #[test]
    fn test_value_conversions() {
        let int_val = Value::Int(100);
        assert_eq!(int_val.as_i64(), Some(100));
        assert!(int_val.as_f64().is_some()); // Int can convert to f64
        assert!(int_val.as_str().is_none());

        let uint_val = Value::UInt(200);
        assert_eq!(uint_val.as_i64(), Some(200));
        assert!(uint_val.as_f64().is_some());

        let float_val = Value::Float(2.5);
        assert!(float_val.as_i64().is_none());
        assert_eq!(float_val.as_f64(), Some(2.5));
        assert!(float_val.as_str().is_none());

        let str_val = Value::String("test".to_string());
        assert!(str_val.as_i64().is_none());
        assert!(str_val.as_f64().is_none());
        assert_eq!(str_val.as_str(), Some("test"));

        let blob_val = Value::Blob(456);
        assert!(blob_val.as_i64().is_none());
        assert!(blob_val.as_f64().is_none());
        assert!(blob_val.as_str().is_none());

        let null_val = Value::Null;
        assert!(null_val.as_i64().is_none());
        assert!(null_val.as_f64().is_none());
        assert!(null_val.as_str().is_none());
    }

    #[test]
    fn test_value_debug_clone() {
        let int_val = Value::Int(123);
        let debug_str = format!("{:?}", int_val);
        assert!(debug_str.contains("123"));

        let cloned = int_val.clone();
        assert!(matches!(cloned, Value::Int(123)));

        let str_val = Value::String("hello".to_string());
        let str_cloned = str_val.clone();
        assert_eq!(str_cloned.as_str(), Some("hello"));
    }
}
