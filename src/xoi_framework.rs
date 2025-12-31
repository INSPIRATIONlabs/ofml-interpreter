//! xOi Framework Implementation
//!
//! This module implements the xOi (Extended Object Instance) framework classes
//! used in OFML for advanced product configuration and database access.
//!
//! ## Class Hierarchy
//!
//! ```text
//! xxPart (native)
//!    └── OiPart
//!         └── OiPlElement
//!              └── xOiPlElement
//!                   └── xOiBTGPlElement
//!                        └── xOiBTGPlElement2
//!                             └── xOiBTGPlElement3 (most manufacturer products inherit from this)
//! ```
//!
//! ## Key Components
//!
//! - `xOiTable2` - Database access for reading product configuration data
//! - `xOiPlanning` - Scene/planning management
//! - `oiTable()` - Native function for EBASE table operations
//! - Attachment point system (stdAttPts, addAttPts)

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ebase::OcdRecord;
use crate::interpreter::Interpreter;
use crate::value::{ClassValue, ObjInstance, Value};

/// xOiTable2 - Database table wrapper for reading product configuration data.
///
/// This class provides access to OFML control data tables (OCD) which contain
/// product-specific configuration like material packages, categories, properties, etc.
#[derive(Debug, Clone)]
pub struct XOiTable2 {
    /// Table identifier (usually derived from article/program)
    pub id: String,
    /// Table structure definition
    pub structure: Vec<TableAttribute>,
    /// Whether the table is currently open
    pub is_open: bool,
    /// Cached table records
    pub records: Vec<OcdRecord>,
    /// Index definitions for fast lookup
    pub indices: Vec<String>,
}

/// Table attribute definition
#[derive(Debug, Clone)]
pub struct TableAttribute {
    /// Attribute name
    pub name: String,
    /// Primary key flag (0 or 1)
    pub primary_key: i32,
    /// Secondary key flag
    pub key: i32,
    /// Data type (@s=string, @i=int, @f=float)
    pub data_type: char,
    /// Default value
    pub default: String,
    /// Format string
    pub format: String,
}

impl XOiTable2 {
    /// Create a new table wrapper
    pub fn new(id: &str, structure: Vec<TableAttribute>) -> Self {
        Self {
            id: id.to_string(),
            structure,
            is_open: false,
            records: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Open the table and load records
    pub fn open(&mut self, alb_path: Option<&std::path::Path>) -> bool {
        if self.is_open {
            return true;
        }

        // Try to load OCD data from ALB
        if let Some(path) = alb_path {
            // The table ID is typically like "manufacturer.article.ocd"
            // We need to find the corresponding OCD file
            let ocd_name = format!("{}.ocd", self.id.replace('.', "/"));

            // Try to read OCD from ALB
            if let Ok(records) = read_ocd_from_alb(path, &ocd_name) {
                self.records = records;
                self.is_open = true;
                return true;
            }
        }

        // If no ALB path or loading failed, create empty table
        self.is_open = true;
        true
    }

    /// Close the table
    pub fn close(&mut self) {
        self.is_open = false;
    }

    /// Get attribute definition by name
    pub fn get_attr_def(&self, name: &str) -> Option<&TableAttribute> {
        self.structure.iter().find(|attr| attr.name == name)
    }

    /// Get attribute index by name
    pub fn get_attr_idx(&self, name: &str) -> Option<usize> {
        self.structure.iter().position(|attr| attr.name == name)
    }

    /// Read entries matching a filter
    pub fn read_entries_for(
        &self,
        filter: &[(String, String)],
        attrs: &[String],
        _limit: usize,
    ) -> Vec<Vec<String>> {
        let mut results = Vec::new();

        for record in &self.records {
            // Check if record matches filter
            let matches = filter
                .iter()
                .all(|(key, value)| record.fields.get(key).map_or(false, |v| v == value));

            if matches {
                // Extract requested attributes
                let row: Vec<String> = attrs
                    .iter()
                    .map(|attr| record.fields.get(attr).cloned().unwrap_or_default())
                    .collect();
                results.push(row);
            }
        }

        results
    }

    /// Read a single entry matching a filter
    pub fn read_1_entry_for(
        &self,
        filter: &[(String, String)],
        attrs: &[String],
        _flags: i32,
    ) -> Option<Vec<String>> {
        self.read_entries_for(filter, attrs, 1).into_iter().next()
    }
}

/// Try to read OCD file from ALB archive
fn read_ocd_from_alb(alb_path: &std::path::Path, ocd_name: &str) -> Result<Vec<OcdRecord>, String> {
    // Try to open the ALB and extract OCD
    if let Ok(records) = crate::ebase::read_ocd(alb_path, ocd_name) {
        return Ok(records);
    }

    Err(format!("Could not load OCD: {}", ocd_name))
}

/// xOi class hierarchy registration for the interpreter
impl Interpreter {
    /// Register xOi framework classes
    pub fn register_xoi_classes(&mut self) {
        // Get base classes
        let _oi_part = self
            .classes
            .get("OiPart")
            .cloned()
            .unwrap_or_else(|| self.create_native_class_internal("OiPart", None));
        let oi_object = self
            .classes
            .get("OiObject")
            .cloned()
            .unwrap_or_else(|| self.create_native_class_internal("OiObject", None));

        // Register xxScElement (base for scene elements)
        let xx_sc_element =
            self.create_native_class_internal("xxScElement", Some(oi_object.clone()));
        self.classes
            .insert("xxScElement".to_string(), xx_sc_element.clone());
        self.env
            .define_global("xxScElement", Value::Class(xx_sc_element.clone()));

        // Register OiPlElement (extends xxScElement for planning elements)
        let oi_pl_element =
            self.create_native_class_internal("OiPlElement", Some(xx_sc_element.clone()));
        self.classes
            .insert("OiPlElement".to_string(), oi_pl_element.clone());
        self.env
            .define_global("OiPlElement", Value::Class(oi_pl_element.clone()));

        // Register xOiPlElement (extends OiPlElement with attachment points)
        let x_oi_pl_element =
            self.create_native_class_internal("xOiPlElement", Some(oi_pl_element.clone()));
        self.classes
            .insert("xOiPlElement".to_string(), x_oi_pl_element.clone());
        self.env
            .define_global("xOiPlElement", Value::Class(x_oi_pl_element.clone()));

        // Register xOiBTGPlElement (extends xOiPlElement, "BTG" = Business Transaction Group)
        let x_oi_btg_pl_element =
            self.create_native_class_internal("xOiBTGPlElement", Some(x_oi_pl_element.clone()));
        self.classes
            .insert("xOiBTGPlElement".to_string(), x_oi_btg_pl_element.clone());
        self.env
            .define_global("xOiBTGPlElement", Value::Class(x_oi_btg_pl_element.clone()));

        // Register xOiBTGPlElement2 (extends xOiBTGPlElement)
        let x_oi_btg_pl_element2 = self
            .create_native_class_internal("xOiBTGPlElement2", Some(x_oi_btg_pl_element.clone()));
        self.classes
            .insert("xOiBTGPlElement2".to_string(), x_oi_btg_pl_element2.clone());
        self.env.define_global(
            "xOiBTGPlElement2",
            Value::Class(x_oi_btg_pl_element2.clone()),
        );

        // Register xOiBTGPlElement3 (main product element class most manufacturers use)
        let x_oi_btg_pl_element3 = self
            .create_native_class_internal("xOiBTGPlElement3", Some(x_oi_btg_pl_element2.clone()));
        self.classes
            .insert("xOiBTGPlElement3".to_string(), x_oi_btg_pl_element3.clone());
        self.env.define_global(
            "xOiBTGPlElement3",
            Value::Class(x_oi_btg_pl_element3.clone()),
        );

        // Register OiOdbPlElement (ODB = OFML Database - database-backed planning element)
        let oi_odb_pl_element =
            self.create_native_class_internal("OiOdbPlElement", Some(x_oi_btg_pl_element3.clone()));
        self.classes
            .insert("OiOdbPlElement".to_string(), oi_odb_pl_element.clone());
        self.env
            .define_global("OiOdbPlElement", Value::Class(oi_odb_pl_element.clone()));

        // Register xOiOdbPlElement (extended ODB element)
        let x_oi_odb_pl_element =
            self.create_native_class_internal("xOiOdbPlElement", Some(oi_odb_pl_element.clone()));
        self.classes
            .insert("xOiOdbPlElement".to_string(), x_oi_odb_pl_element.clone());
        self.env
            .define_global("xOiOdbPlElement", Value::Class(x_oi_odb_pl_element.clone()));

        // Register OiCompPlElement (composite planning element)
        let oi_comp_pl_element = self
            .create_native_class_internal("OiCompPlElement", Some(x_oi_btg_pl_element3.clone()));
        self.classes
            .insert("OiCompPlElement".to_string(), oi_comp_pl_element.clone());
        self.env
            .define_global("OiCompPlElement", Value::Class(oi_comp_pl_element.clone()));

        // Register xOiCompPlElement (extended composite element)
        let x_oi_comp_pl_element =
            self.create_native_class_internal("xOiCompPlElement", Some(oi_comp_pl_element.clone()));
        self.classes
            .insert("xOiCompPlElement".to_string(), x_oi_comp_pl_element.clone());
        self.env.define_global(
            "xOiCompPlElement",
            Value::Class(x_oi_comp_pl_element.clone()),
        );

        // Register OiPlanning (scene management)
        let oi_planning = self.create_native_class_internal("OiPlanning", Some(oi_object.clone()));
        self.classes
            .insert("OiPlanning".to_string(), oi_planning.clone());
        self.env
            .define_global("OiPlanning", Value::Class(oi_planning.clone()));

        // Register xOiPlanning (extended planning)
        let x_oi_planning =
            self.create_native_class_internal("xOiPlanning", Some(oi_planning.clone()));
        self.classes
            .insert("xOiPlanning".to_string(), x_oi_planning.clone());
        self.env
            .define_global("xOiPlanning", Value::Class(x_oi_planning.clone()));

        // Register xOiTable2 (database access)
        let x_oi_table2 = self.create_native_class_internal("xOiTable2", None);
        self.classes
            .insert("xOiTable2".to_string(), x_oi_table2.clone());
        self.env
            .define_global("xOiTable2", Value::Class(x_oi_table2.clone()));

        // Register xOiFreeArticle (free-standing articles)
        let x_oi_free_article =
            self.create_native_class_internal("xOiFreeArticle", Some(x_oi_btg_pl_element3.clone()));
        self.classes
            .insert("xOiFreeArticle".to_string(), x_oi_free_article.clone());
        self.env
            .define_global("xOiFreeArticle", Value::Class(x_oi_free_article.clone()));

        // Register xOiAttPtInteractor (attachment point interactor)
        let x_oi_attpt_interactor =
            self.create_native_class_internal("xOiAttPtInteractor", Some(oi_object.clone()));
        self.classes.insert(
            "xOiAttPtInteractor".to_string(),
            x_oi_attpt_interactor.clone(),
        );
        self.env.define_global(
            "xOiAttPtInteractor",
            Value::Class(x_oi_attpt_interactor.clone()),
        );

        // Register xOiProgInfo (program information - scene-level configuration)
        let x_oi_prog_info =
            self.create_native_class_internal("xOiProgInfo", Some(oi_object.clone()));
        self.classes
            .insert("xOiProgInfo".to_string(), x_oi_prog_info.clone());
        self.env
            .define_global("xOiProgInfo", Value::Class(x_oi_prog_info.clone()));

        // Register xOiSurface (surface/polygon element)
        let x_oi_surface = self.create_native_class_internal("xOiSurface", Some(oi_object.clone()));
        self.classes
            .insert("xOiSurface".to_string(), x_oi_surface.clone());
        self.env
            .define_global("xOiSurface", Value::Class(x_oi_surface.clone()));

        // Register xOiLayoutGroup (layout group for arranging elements)
        // Uses xOiBTGPlElement3 as parent since xOiPlGroup is registered in interpreter
        let x_oi_layout_group =
            self.create_native_class_internal("xOiLayoutGroup", Some(x_oi_btg_pl_element3.clone()));
        self.classes
            .insert("xOiLayoutGroup".to_string(), x_oi_layout_group.clone());
        self.env
            .define_global("xOiLayoutGroup", Value::Class(x_oi_layout_group.clone()));

        // Register xOiImport (3D model import element)
        let x_oi_import =
            self.create_native_class_internal("xOiImport", Some(x_oi_btg_pl_element3.clone()));
        self.classes
            .insert("xOiImport".to_string(), x_oi_import.clone());
        self.env
            .define_global("xOiImport", Value::Class(x_oi_import.clone()));

        // Register xOiAttPtIF (attachment point interface singleton)
        self.register_xoi_attpt_if();

        // Register helper functions
        self.register_xoi_functions();
    }

    /// Register xOiAttPtIF singleton object
    fn register_xoi_attpt_if(&mut self) {
        let attpt_if_class = self.create_native_class_internal("xOiAttPtIF_Class", None);
        let instance = Rc::new(RefCell::new(ObjInstance {
            class: attpt_if_class,
            name: "xOiAttPtIF".to_string(),
            ..Default::default()
        }));
        self.env
            .define_global("xOiAttPtIF", Value::Object(instance));
    }

    /// Register xOi helper functions
    fn register_xoi_functions(&mut self) {
        // openDataTbl - opens a control data table for the current article
        self.env.define_global(
            "openDataTbl",
            Value::NativeFunc(Rc::new(|interp, _args| {
                // Get current object's program and article to construct table ID
                let table_id = if let Some(ref instance) = interp.current_self {
                    let inst = instance.borrow();
                    // Try to get article from properties
                    let article = inst
                        .properties
                        .get("article")
                        .map(|v| v.to_string_val())
                        .unwrap_or_else(|| "default".to_string());
                    format!("ocd.{}", article)
                } else {
                    "ocd.default".to_string()
                };

                // Create xOiTable2 instance
                let table_class = interp
                    .classes
                    .get("xOiTable2")
                    .cloned()
                    .unwrap_or_else(|| interp.create_native_class_internal("xOiTable2", None));

                let instance = Rc::new(RefCell::new(ObjInstance {
                    class: table_class,
                    name: table_id.clone(),
                    ..Default::default()
                }));

                // Store table ID
                instance
                    .borrow_mut()
                    .fields
                    .insert("mID".to_string(), Value::String(Rc::new(table_id)));
                instance
                    .borrow_mut()
                    .fields
                    .insert("mIsOpen".to_string(), Value::Bool(true));

                Ok(Value::Object(instance))
            })),
        );

        // oiTable - low-level table operations
        self.env.define_global(
            "oiTable",
            Value::NativeFunc(Rc::new(|_interp, args| {
                // oiTable(@openTbl, @([tableId, structure])) -> 0/1
                // oiTable(@readTbl, @([filter, attrs])) -> results
                let op = match args.first() {
                    Some(Value::Symbol(s)) => s.to_string(),
                    _ => return Ok(Value::Int(0)),
                };

                match op.as_str() {
                    "openTbl" => {
                        // Always succeed for now - actual EBASE loading happens elsewhere
                        Ok(Value::Int(1))
                    }
                    "closeTbl" => Ok(Value::Int(1)),
                    "readTbl" => {
                        // Return empty results for now
                        Ok(Value::Array(Rc::new(RefCell::new(Vec::new()))))
                    }
                    _ => Ok(Value::Int(0)),
                }
            })),
        );

        // oiGetStdAttPts - get standard attachment points
        self.env.define_global(
            "oiGetStdAttPts",
            Value::NativeFunc(Rc::new(|interp, _args| {
                // Return attachment points from current object if available
                if let Some(ref instance) = interp.current_self {
                    let inst = instance.borrow();
                    if let Some(Value::Array(attpts)) = inst.fields.get("mStdAttPts") {
                        return Ok(Value::Array(attpts.clone()));
                    }
                }
                // Return empty list
                Ok(Value::Array(Rc::new(RefCell::new(Vec::new()))))
            })),
        );

        // oiGetStdAttPtsOrder - get attachment points in priority order
        self.env.define_global(
            "oiGetStdAttPtsOrder",
            Value::NativeFunc(Rc::new(|_interp, _args| {
                // Return default attachment point order
                Ok(Value::Array(Rc::new(RefCell::new(vec![
                    Value::Symbol(Rc::new("front".to_string())),
                    Value::Symbol(Rc::new("back".to_string())),
                    Value::Symbol(Rc::new("left".to_string())),
                    Value::Symbol(Rc::new("right".to_string())),
                    Value::Symbol(Rc::new("top".to_string())),
                    Value::Symbol(Rc::new("bottom".to_string())),
                ]))))
            })),
        );

        // oiGetOppositeAttPts4Std - get opposite attachment points
        self.env.define_global(
            "oiGetOppositeAttPts4Std",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let key = match args.get(1) {
                    Some(Value::Symbol(s)) => s.to_string(),
                    _ => return Ok(Value::Null),
                };

                // Return opposite attachment point
                let opposite = match key.as_str() {
                    "front" => "back",
                    "back" => "front",
                    "left" => "right",
                    "right" => "left",
                    "top" => "bottom",
                    "bottom" => "top",
                    _ => return Ok(Value::Null),
                };

                Ok(Value::Array(Rc::new(RefCell::new(vec![Value::Symbol(
                    Rc::new(opposite.to_string()),
                )]))))
            })),
        );

        // setRtAxis - set rotation axis
        self.env.define_global(
            "setRtAxis",
            Value::NativeFunc(Rc::new(|interp, args| {
                let axis = args.first().and_then(|v| v.to_int()).unwrap_or(2);
                if let Some(ref instance) = interp.current_self {
                    instance
                        .borrow_mut()
                        .fields
                        .insert("mRtAxis".to_string(), Value::Int(axis));
                }
                Ok(Value::Null)
            })),
        );

        // setTrAxis - set translation axis
        self.env.define_global(
            "setTrAxis",
            Value::NativeFunc(Rc::new(|interp, args| {
                let axis = args.first().and_then(|v| v.to_int()).unwrap_or(5);
                if let Some(ref instance) = interp.current_self {
                    instance
                        .borrow_mut()
                        .fields
                        .insert("mTrAxis".to_string(), Value::Int(axis));
                }
                Ok(Value::Null)
            })),
        );

        // getScene / getPlanning - get the planning object
        self.env.define_global(
            "getScene",
            Value::NativeFunc(Rc::new(|interp, _args| {
                // Return the current planning object if it exists
                if let Some(planning) = interp.env.get("_planning") {
                    return Ok(planning);
                }
                Ok(Value::Null)
            })),
        );

        self.env.define_global(
            "getPlanning",
            Value::NativeFunc(Rc::new(|interp, _args| {
                if let Some(planning) = interp.env.get("_planning") {
                    return Ok(planning);
                }
                Ok(Value::Null)
            })),
        );

        // getRoot - get root object
        self.env.define_global(
            "getRoot",
            Value::NativeFunc(Rc::new(|interp, _args| {
                if let Some(ref instance) = interp.current_self {
                    let mut current = instance.clone();
                    loop {
                        let parent = current.borrow().parent.clone();
                        match parent {
                            Some(p) => current = p,
                            None => break,
                        }
                    }
                    return Ok(Value::Object(current));
                }
                Ok(Value::Null)
            })),
        );

        // getFather - get parent object
        self.env.define_global(
            "getFather",
            Value::NativeFunc(Rc::new(|interp, _args| {
                if let Some(ref instance) = interp.current_self {
                    if let Some(parent) = &instance.borrow().parent {
                        return Ok(Value::Object(parent.clone()));
                    }
                }
                Ok(Value::Null)
            })),
        );

        // getChildren - get child objects
        self.env.define_global(
            "getChildren",
            Value::NativeFunc(Rc::new(|interp, _args| {
                if let Some(ref instance) = interp.current_self {
                    let children: Vec<Value> = instance
                        .borrow()
                        .children
                        .iter()
                        .map(|c| Value::Object(c.clone()))
                        .collect();
                    return Ok(Value::Array(Rc::new(RefCell::new(children))));
                }
                Ok(Value::Array(Rc::new(RefCell::new(Vec::new()))))
            })),
        );

        // hasMember - check if object has a member
        self.env.define_global(
            "hasMember",
            Value::NativeFunc(Rc::new(|interp, args| {
                let member = match args.first() {
                    Some(Value::Symbol(s)) => s.to_string(),
                    Some(Value::String(s)) => s.to_string(),
                    _ => return Ok(Value::Bool(false)),
                };

                if let Some(ref instance) = interp.current_self {
                    let inst = instance.borrow();
                    let has = inst.fields.contains_key(&member)
                        || inst.properties.contains_key(&member)
                        || inst.class.methods.contains_key(&member);
                    return Ok(Value::Bool(has));
                }
                Ok(Value::Bool(false))
            })),
        );

        // xOiCopyAggr - copy aggregate structures
        self.env.define_global(
            "xOiCopyAggr",
            Value::NativeFunc(Rc::new(|_interp, args| {
                // Copy source to destination
                if let (Some(src), Some(Value::Array(dst))) = (args.first(), args.get(1)) {
                    if let Value::Array(src_arr) = src {
                        let mut dst_ref = dst.borrow_mut();
                        dst_ref.clear();
                        for item in src_arr.borrow().iter() {
                            dst_ref.push(item.clone());
                        }
                    }
                }
                Ok(Value::Null)
            })),
        );

        // oiOutput - output messages
        self.env.define_global(
            "oiOutput",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let level = match args.first() {
                    Some(Value::Symbol(s)) => s.to_string(),
                    _ => "INFO".to_string(),
                };
                let msg = args.get(1).map(|v| v.to_string_val()).unwrap_or_default();
                eprintln!("[{}] {}", level, msg);
                Ok(Value::Null)
            })),
        );

        // xOiCheckElPos - check element position
        self.env.define_global(
            "xOiCheckElPos",
            Value::NativeFunc(Rc::new(|_interp, _args| {
                // Return current position (allow movement)
                Ok(Value::Array(Rc::new(RefCell::new(vec![
                    Value::Float(0.0),
                    Value::Float(0.0),
                    Value::Float(0.0),
                ]))))
            })),
        );

        // oiGetPosRot4AttachPts - get position and rotation for attachment points
        self.env.define_global(
            "oiGetPosRot4AttachPts",
            Value::NativeFunc(Rc::new(|_interp, _args| {
                // Return [position, rotation, error]
                Ok(Value::Array(Rc::new(RefCell::new(vec![
                    Value::Array(Rc::new(RefCell::new(vec![
                        Value::Float(0.0),
                        Value::Float(0.0),
                        Value::Float(0.0),
                    ]))),
                    Value::Float(0.0),
                    Value::Null,
                ]))))
            })),
        );

        // xOiAutoDecoration4Obj - auto decoration
        self.env.define_global(
            "xOiAutoDecoration4Obj",
            Value::NativeFunc(Rc::new(|_interp, _args| Ok(Value::Bool(true)))),
        );

        // xOiDeleteAutoDeco - delete auto decoration
        self.env.define_global(
            "xOiDeleteAutoDeco",
            Value::NativeFunc(Rc::new(|_interp, _args| Ok(Value::Null))),
        );

        // xOiGetMetaInfo - get meta information
        self.env.define_global(
            "xOiGetMetaInfo",
            Value::NativeFunc(Rc::new(|_interp, _args| Ok(Value::Null))),
        );

        // setMethod - set creation method string
        self.env.define_global(
            "setMethod",
            Value::NativeFunc(Rc::new(|interp, args| {
                let method = args.first().map(|v| v.to_string_val()).unwrap_or_default();
                if let Some(ref instance) = interp.current_self {
                    instance
                        .borrow_mut()
                        .fields
                        .insert("mMethod".to_string(), Value::String(Rc::new(method)));
                }
                Ok(Value::Null)
            })),
        );

        // createChParams - create child parameters
        self.env.define_global(
            "createChParams",
            Value::NativeFunc(Rc::new(|_interp, _args| Ok(Value::Null))),
        );

        // acceptCh - accept child
        self.env.define_global(
            "acceptCh",
            Value::NativeFunc(Rc::new(|_interp, args| {
                // Return the position
                Ok(args.first().cloned().unwrap_or(Value::Null))
            })),
        );

        // removeCh - remove child
        self.env.define_global(
            "removeCh",
            Value::NativeFunc(Rc::new(|_interp, _args| Ok(Value::Null))),
        );

        // eval - evaluate expression string
        self.env.define_global(
            "eval",
            Value::NativeFunc(Rc::new(|interp, args| {
                let code = args.first().map(|v| v.to_string_val()).unwrap_or_default();

                // Skip empty code
                if code.trim().is_empty() {
                    return Ok(Value::Null);
                }

                // Parse and evaluate the expression - return Null on errors for graceful degradation
                let parser_result = crate::parser::Parser::new(&code);
                let mut parser = match parser_result {
                    Ok(p) => p,
                    Err(_) => return Ok(Value::Null),
                };

                let ast = match parser.parse() {
                    Ok(a) => a,
                    Err(_) => return Ok(Value::Null),
                };

                if let Err(_) = interp.execute(&ast) {
                    return Ok(Value::Null);
                }

                // Return the last evaluated value or null
                Ok(Value::Null)
            })),
        );

        // Error class for try/catch
        let error_class = self.create_native_class_internal("Error", None);
        self.classes
            .insert("Error".to_string(), error_class.clone());
        self.env.define_global("Error", Value::Class(error_class));

        // Vector class (alias for Array in OFML)
        self.env.define_global(
            "Vector",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let size = args.first().and_then(|v| v.to_int()).unwrap_or(0) as usize;
                let arr = vec![Value::Null; size];
                Ok(Value::Array(Rc::new(RefCell::new(arr))))
            })),
        );

        // List class (alias for Array)
        self.env.define_global(
            "List",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let arr: Vec<Value> = args.iter().cloned().collect();
                Ok(Value::Array(Rc::new(RefCell::new(arr))))
            })),
        );

        // Additional helper functions for manufacturer CLS files

        // oiGetPlanning - get the current planning object
        self.env.define_global(
            "oiGetPlanning",
            Value::NativeFunc(Rc::new(|interp, _args| {
                if let Some(planning) = interp.env.get("_planning") {
                    return Ok(planning);
                }
                // Create a default planning object
                let planning_class = interp
                    .classes
                    .get("xOiPlanning")
                    .cloned()
                    .unwrap_or_else(|| interp.create_native_class_internal("xOiPlanning", None));
                let instance = Rc::new(RefCell::new(ObjInstance {
                    class: planning_class,
                    name: "_planning".to_string(),
                    ..Default::default()
                }));
                Ok(Value::Object(instance))
            })),
        );

        // oiGetStringResource - get localized string resource
        self.env.define_global(
            "oiGetStringResource",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let key = args.first().map(|v| v.to_string_val()).unwrap_or_default();
                let _lang = args
                    .get(1)
                    .map(|v| v.to_string_val())
                    .unwrap_or_else(|| "de".to_string());
                // Return the key as default (real implementation would look up translations)
                Ok(Value::String(Rc::new(key)))
            })),
        );

        // xOiGetAppRegistryKey - get application registry value
        self.env.define_global(
            "xOiGetAppRegistryKey",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let _program = args.first().map(|v| v.to_string_val()).unwrap_or_default();
                let _key = args.get(1).map(|v| v.to_string_val()).unwrap_or_default();
                let default = args.get(2).map(|v| v.to_string_val()).unwrap_or_default();
                // Return default value
                Ok(Value::String(Rc::new(default)))
            })),
        );

        // hasProperty - check if object has a property
        self.env.define_global(
            "hasProperty",
            Value::NativeFunc(Rc::new(|interp, args| {
                let key = match args.first() {
                    Some(Value::Symbol(s)) => s.to_string(),
                    Some(Value::String(s)) => s.to_string(),
                    _ => return Ok(Value::Bool(false)),
                };

                if let Some(ref instance) = interp.current_self {
                    let inst = instance.borrow();
                    let has =
                        inst.properties.contains_key(&key) || inst.prop_defs.contains_key(&key);
                    return Ok(Value::Bool(has));
                }
                Ok(Value::Bool(false))
            })),
        );

        // getLanguage - get current language
        self.env.define_global(
            "getLanguage",
            Value::NativeFunc(Rc::new(|_interp, _args| {
                Ok(Value::String(Rc::new("de".to_string())))
            })),
        );

        // getProgram - get current program/manufacturer ID
        self.env.define_global(
            "getProgram",
            Value::NativeFunc(Rc::new(|interp, _args| {
                if let Some(ref instance) = interp.current_self {
                    if let Some(prog) = instance.borrow().fields.get("mOiPID") {
                        return Ok(prog.clone());
                    }
                }
                Ok(Value::Null)
            })),
        );

        // getArticleObj - get the article object
        self.env.define_global(
            "getArticleObj",
            Value::NativeFunc(Rc::new(|interp, _args| {
                if let Some(ref instance) = interp.current_self {
                    if let Some(art) = instance.borrow().fields.get("mArticleObj") {
                        return Ok(art.clone());
                    }
                    // Return self as article object if no separate article
                    return Ok(Value::Object(instance.clone()));
                }
                Ok(Value::Null)
            })),
        );

        // setPropState2 - set property state (visibility, enabled, etc.)
        self.env.define_global(
            "setPropState2",
            Value::NativeFunc(Rc::new(|interp, args| {
                let key = match args.first() {
                    Some(Value::Symbol(s)) => s.to_string(),
                    Some(Value::String(s)) => s.to_string(),
                    _ => return Ok(Value::Null),
                };
                let state = args.get(1).and_then(|v| v.to_int()).unwrap_or(0) as i32;

                if let Some(ref instance) = interp.current_self {
                    instance.borrow_mut().prop_states.insert(key, state);
                }
                Ok(Value::Null)
            })),
        );

        // setupNoteProperty - setup a note/comment property
        self.env.define_global(
            "setupNoteProperty",
            Value::NativeFunc(Rc::new(|interp, args| {
                let key = match args.first() {
                    Some(Value::Symbol(s)) => s.to_string(),
                    Some(Value::String(s)) => s.to_string(),
                    _ => return Ok(Value::Null),
                };
                let value = args.get(1).cloned().unwrap_or(Value::Null);

                if let Some(ref instance) = interp.current_self {
                    instance.borrow_mut().properties.insert(key, value);
                }
                Ok(Value::Null)
            })),
        );

        // removeNoteProperty - remove a note property
        self.env.define_global(
            "removeNoteProperty",
            Value::NativeFunc(Rc::new(|interp, args| {
                let key = match args.first() {
                    Some(Value::Symbol(s)) => s.to_string(),
                    Some(Value::String(s)) => s.to_string(),
                    _ => return Ok(Value::Null),
                };

                if let Some(ref instance) = interp.current_self {
                    instance.borrow_mut().properties.remove(&key);
                }
                Ok(Value::Null)
            })),
        );

        // listContains - check if list contains an element
        self.env.define_global(
            "listContains",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let list = match args.first() {
                    Some(Value::Array(arr)) => arr.borrow().clone(),
                    _ => return Ok(Value::Bool(false)),
                };
                let item = args.get(1).cloned().unwrap_or(Value::Null);

                for elem in list {
                    if elem.to_string_val() == item.to_string_val() {
                        return Ok(Value::Bool(true));
                    }
                }
                Ok(Value::Bool(false))
            })),
        );

        // isA - check if object is instance of class
        self.env.define_global(
            "isA",
            Value::NativeFunc(Rc::new(|interp, args| {
                let class_name = match args.first() {
                    Some(Value::Class(c)) => c.name.clone(),
                    Some(Value::String(s)) => s.to_string(),
                    Some(Value::Symbol(s)) => s.to_string(),
                    _ => return Ok(Value::Bool(false)),
                };

                if let Some(ref instance) = interp.current_self {
                    let inst = instance.borrow();
                    // Check class hierarchy
                    let mut current_class = Some(inst.class.clone());
                    while let Some(cls) = current_class {
                        if cls.name == class_name {
                            return Ok(Value::Bool(true));
                        }
                        current_class = cls.parent.clone();
                    }
                }
                Ok(Value::Bool(false))
            })),
        );

        // instanceof operator support
        self.env.define_global(
            "instanceof",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let obj = match args.first() {
                    Some(Value::Object(o)) => o.clone(),
                    _ => return Ok(Value::Bool(false)),
                };
                let class_name = match args.get(1) {
                    Some(Value::Class(c)) => c.name.clone(),
                    Some(Value::String(s)) => s.to_string(),
                    Some(Value::Symbol(s)) => s.to_string(),
                    _ => return Ok(Value::Bool(false)),
                };

                let inst = obj.borrow();
                let mut current_class = Some(inst.class.clone());
                while let Some(cls) = current_class {
                    if cls.name == class_name {
                        return Ok(Value::Bool(true));
                    }
                    current_class = cls.parent.clone();
                }
                Ok(Value::Bool(false))
            })),
        );

        // setArticleSpec - set article specification
        self.env.define_global(
            "setArticleSpec",
            Value::NativeFunc(Rc::new(|interp, args| {
                let article = args.first().map(|v| v.to_string_val()).unwrap_or_default();
                if let Some(ref instance) = interp.current_self {
                    instance
                        .borrow_mut()
                        .fields
                        .insert("mArticleSpec".to_string(), Value::String(Rc::new(article)));
                }
                Ok(Value::Null)
            })),
        );

        // getArticleSpec - get article specification
        self.env.define_global(
            "getArticleSpec",
            Value::NativeFunc(Rc::new(|interp, _args| {
                if let Some(ref instance) = interp.current_self {
                    if let Some(spec) = instance.borrow().fields.get("mArticleSpec") {
                        return Ok(spec.clone());
                    }
                }
                Ok(Value::Null)
            })),
        );

        // getCreationMode - get object creation mode
        self.env.define_global(
            "getCreationMode",
            Value::NativeFunc(Rc::new(|_interp, _args| {
                Ok(Value::Int(0)) // Default creation mode
            })),
        );

        // setXArticleSpec - set extended article specification
        self.env.define_global(
            "setXArticleSpec",
            Value::NativeFunc(Rc::new(|interp, args| {
                let mode = args.first().cloned().unwrap_or(Value::Null);
                let spec = args.get(1).map(|v| v.to_string_val()).unwrap_or_default();
                if let Some(ref instance) = interp.current_self {
                    instance
                        .borrow_mut()
                        .fields
                        .insert("mXArticleSpec".to_string(), Value::String(Rc::new(spec)));
                    instance
                        .borrow_mut()
                        .fields
                        .insert("mXArticleMode".to_string(), mode);
                }
                Ok(Value::Null)
            })),
        );
    }

    /// Create a native class (internal version for xoi_framework)
    fn create_native_class_internal(
        &self,
        name: &str,
        parent: Option<Rc<ClassValue>>,
    ) -> Rc<ClassValue> {
        Rc::new(ClassValue {
            name: name.to_string(),
            package: String::new(), // Native classes have no package
            parent,
            methods: HashMap::new(),
            rules: HashMap::new(),
            static_vars: HashMap::new(),
            decl: crate::ast::ClassDecl {
                modifiers: vec![],
                name: name.to_string(),
                parent: None,
                members: vec![],
                span: crate::ast::Span::default(),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xoi_table2_new() {
        let attrs = vec![
            TableAttribute {
                name: "type".to_string(),
                primary_key: 1,
                key: 0,
                data_type: 's',
                default: "".to_string(),
                format: "0".to_string(),
            },
            TableAttribute {
                name: "value".to_string(),
                primary_key: 0,
                key: 0,
                data_type: 's',
                default: "".to_string(),
                format: "0".to_string(),
            },
        ];

        let table = XOiTable2::new("test.ocd", attrs);
        assert_eq!(table.id, "test.ocd");
        assert!(!table.is_open);
        assert_eq!(table.structure.len(), 2);
    }

    #[test]
    fn test_xoi_table2_attr_lookup() {
        let attrs = vec![
            TableAttribute {
                name: "type".to_string(),
                primary_key: 1,
                key: 0,
                data_type: 's',
                default: "".to_string(),
                format: "0".to_string(),
            },
            TableAttribute {
                name: "value".to_string(),
                primary_key: 0,
                key: 0,
                data_type: 's',
                default: "".to_string(),
                format: "0".to_string(),
            },
        ];

        let table = XOiTable2::new("test.ocd", attrs);

        assert!(table.get_attr_def("type").is_some());
        assert!(table.get_attr_def("value").is_some());
        assert!(table.get_attr_def("nonexistent").is_none());

        assert_eq!(table.get_attr_idx("type"), Some(0));
        assert_eq!(table.get_attr_idx("value"), Some(1));
        assert_eq!(table.get_attr_idx("nonexistent"), None);
    }
}
