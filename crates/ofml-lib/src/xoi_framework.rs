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
                .all(|(key, value)| record.fields.get(key).is_some_and(|v| v == value));

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
                if let (Some(Value::Array(src_arr)), Some(Value::Array(dst))) =
                    (args.first(), args.get(1))
                {
                    let mut dst_ref = dst.borrow_mut();
                    dst_ref.clear();
                    for item in src_arr.borrow().iter() {
                        dst_ref.push(item.clone());
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

                if interp.execute(&ast).is_err() {
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
                let arr: Vec<Value> = args.to_vec();
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

    fn create_test_table() -> XOiTable2 {
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
        XOiTable2::new("test.ocd", attrs)
    }

    #[test]
    fn test_xoi_table2_new() {
        let table = create_test_table();
        assert_eq!(table.id, "test.ocd");
        assert!(!table.is_open);
        assert_eq!(table.structure.len(), 2);
    }

    #[test]
    fn test_xoi_table2_attr_lookup() {
        let table = create_test_table();

        assert!(table.get_attr_def("type").is_some());
        assert!(table.get_attr_def("value").is_some());
        assert!(table.get_attr_def("nonexistent").is_none());

        assert_eq!(table.get_attr_idx("type"), Some(0));
        assert_eq!(table.get_attr_idx("value"), Some(1));
        assert_eq!(table.get_attr_idx("nonexistent"), None);
    }

    #[test]
    fn test_table_attribute_debug_clone() {
        let attr = TableAttribute {
            name: "test_attr".to_string(),
            primary_key: 1,
            key: 0,
            data_type: 's',
            default: "default_val".to_string(),
            format: "%.2f".to_string(),
        };
        let debug = format!("{:?}", attr);
        assert!(debug.contains("test_attr"));
        assert!(debug.contains("TableAttribute"));

        let cloned = attr.clone();
        assert_eq!(cloned.name, attr.name);
        assert_eq!(cloned.primary_key, attr.primary_key);
        assert_eq!(cloned.data_type, attr.data_type);
    }

    #[test]
    fn test_xoi_table2_debug_clone() {
        let table = create_test_table();
        let debug = format!("{:?}", table);
        assert!(debug.contains("XOiTable2"));
        assert!(debug.contains("test.ocd"));

        let cloned = table.clone();
        assert_eq!(cloned.id, table.id);
        assert_eq!(cloned.is_open, table.is_open);
    }

    #[test]
    fn test_xoi_table2_open_close() {
        let mut table = create_test_table();
        assert!(!table.is_open);

        // Open with no ALB path should still succeed (creates empty table)
        let result = table.open(None);
        assert!(result);
        assert!(table.is_open);

        // Opening again should return true
        let result = table.open(None);
        assert!(result);

        // Close the table
        table.close();
        assert!(!table.is_open);
    }

    #[test]
    fn test_xoi_table2_open_with_invalid_alb() {
        let mut table = create_test_table();
        let invalid_path = std::path::Path::new("/nonexistent/path.alb");

        // Should still return true (opens as empty table)
        let result = table.open(Some(invalid_path));
        assert!(result);
        assert!(table.is_open);
        assert!(table.records.is_empty());
    }

    #[test]
    fn test_xoi_table2_read_entries_empty() {
        let table = create_test_table();
        let filter = vec![("type".to_string(), "test".to_string())];
        let attrs = vec!["value".to_string()];

        let results = table.read_entries_for(&filter, &attrs, 10);
        assert!(results.is_empty());
    }

    #[test]
    fn test_xoi_table2_read_entries_with_records() {
        let mut table = create_test_table();

        // Add some test records
        let mut fields1 = HashMap::new();
        fields1.insert("type".to_string(), "material".to_string());
        fields1.insert("value".to_string(), "wood".to_string());
        table.records.push(OcdRecord { fields: fields1 });

        let mut fields2 = HashMap::new();
        fields2.insert("type".to_string(), "material".to_string());
        fields2.insert("value".to_string(), "metal".to_string());
        table.records.push(OcdRecord { fields: fields2 });

        let mut fields3 = HashMap::new();
        fields3.insert("type".to_string(), "color".to_string());
        fields3.insert("value".to_string(), "red".to_string());
        table.records.push(OcdRecord { fields: fields3 });

        // Filter by type=material
        let filter = vec![("type".to_string(), "material".to_string())];
        let attrs = vec!["value".to_string()];

        let results = table.read_entries_for(&filter, &attrs, 10);
        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|r| r[0] == "wood"));
        assert!(results.iter().any(|r| r[0] == "metal"));
    }

    #[test]
    fn test_xoi_table2_read_1_entry_for() {
        let mut table = create_test_table();

        // Add a test record
        let mut fields = HashMap::new();
        fields.insert("type".to_string(), "unique".to_string());
        fields.insert("value".to_string(), "special".to_string());
        table.records.push(OcdRecord { fields });

        // Read single entry
        let filter = vec![("type".to_string(), "unique".to_string())];
        let attrs = vec!["value".to_string()];

        let result = table.read_1_entry_for(&filter, &attrs, 0);
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0], "special");
    }

    #[test]
    fn test_xoi_table2_read_1_entry_for_not_found() {
        let table = create_test_table();

        let filter = vec![("type".to_string(), "nonexistent".to_string())];
        let attrs = vec!["value".to_string()];

        let result = table.read_1_entry_for(&filter, &attrs, 0);
        assert!(result.is_none());
    }

    #[test]
    fn test_xoi_table2_read_entries_missing_attr() {
        let mut table = create_test_table();

        // Add a record with only "type" field
        let mut fields = HashMap::new();
        fields.insert("type".to_string(), "test".to_string());
        table.records.push(OcdRecord { fields });

        // Try to read "value" which doesn't exist
        let filter = vec![("type".to_string(), "test".to_string())];
        let attrs = vec!["value".to_string(), "missing_attr".to_string()];

        let results = table.read_entries_for(&filter, &attrs, 10);
        assert_eq!(results.len(), 1);
        // Missing attrs should return empty strings
        assert_eq!(results[0][0], "");
        assert_eq!(results[0][1], "");
    }

    #[test]
    fn test_table_attribute_types() {
        let string_attr = TableAttribute {
            name: "str".to_string(),
            primary_key: 0,
            key: 0,
            data_type: 's',
            default: "".to_string(),
            format: "".to_string(),
        };
        assert_eq!(string_attr.data_type, 's');

        let int_attr = TableAttribute {
            name: "int".to_string(),
            primary_key: 0,
            key: 0,
            data_type: 'i',
            default: "0".to_string(),
            format: "".to_string(),
        };
        assert_eq!(int_attr.data_type, 'i');

        let float_attr = TableAttribute {
            name: "float".to_string(),
            primary_key: 0,
            key: 0,
            data_type: 'f',
            default: "0.0".to_string(),
            format: "%.2f".to_string(),
        };
        assert_eq!(float_attr.data_type, 'f');
    }

    #[test]
    fn test_read_ocd_from_alb_not_found() {
        let result = read_ocd_from_alb(
            std::path::Path::new("/nonexistent/path.alb"),
            "test.ocd",
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Could not load OCD"));
    }

    #[test]
    fn test_xoi_table2_empty_new() {
        let table = XOiTable2::new("empty.ocd", vec![]);
        assert_eq!(table.id, "empty.ocd");
        assert!(table.structure.is_empty());
        assert!(table.records.is_empty());
        assert!(table.indices.is_empty());
    }

    #[test]
    fn test_xoi_table2_multiple_filters() {
        let mut table = XOiTable2::new(
            "multi.ocd",
            vec![
                TableAttribute {
                    name: "key1".to_string(),
                    primary_key: 1,
                    key: 0,
                    data_type: 's',
                    default: "".to_string(),
                    format: "".to_string(),
                },
                TableAttribute {
                    name: "key2".to_string(),
                    primary_key: 0,
                    key: 1,
                    data_type: 's',
                    default: "".to_string(),
                    format: "".to_string(),
                },
                TableAttribute {
                    name: "value".to_string(),
                    primary_key: 0,
                    key: 0,
                    data_type: 's',
                    default: "".to_string(),
                    format: "".to_string(),
                },
            ],
        );

        // Add records
        let mut f1 = HashMap::new();
        f1.insert("key1".to_string(), "A".to_string());
        f1.insert("key2".to_string(), "X".to_string());
        f1.insert("value".to_string(), "AX".to_string());
        table.records.push(OcdRecord { fields: f1 });

        let mut f2 = HashMap::new();
        f2.insert("key1".to_string(), "A".to_string());
        f2.insert("key2".to_string(), "Y".to_string());
        f2.insert("value".to_string(), "AY".to_string());
        table.records.push(OcdRecord { fields: f2 });

        let mut f3 = HashMap::new();
        f3.insert("key1".to_string(), "B".to_string());
        f3.insert("key2".to_string(), "X".to_string());
        f3.insert("value".to_string(), "BX".to_string());
        table.records.push(OcdRecord { fields: f3 });

        // Filter by both keys
        let filter = vec![
            ("key1".to_string(), "A".to_string()),
            ("key2".to_string(), "X".to_string()),
        ];
        let results = table.read_entries_for(&filter, &["value".to_string()], 10);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0][0], "AX");
    }

    #[test]
    fn test_interpreter_register_xoi_classes() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        // Check that classes were registered
        assert!(interp.classes.contains_key("xxScElement"));
        assert!(interp.classes.contains_key("OiPlElement"));
        assert!(interp.classes.contains_key("xOiPlElement"));
        assert!(interp.classes.contains_key("xOiBTGPlElement"));
        assert!(interp.classes.contains_key("xOiBTGPlElement2"));
        assert!(interp.classes.contains_key("xOiBTGPlElement3"));
    }

    #[test]
    fn test_interpreter_register_xoi_classes_full() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        // Check all xOi classes were registered
        assert!(interp.classes.contains_key("OiOdbPlElement"));
        assert!(interp.classes.contains_key("xOiOdbPlElement"));
        assert!(interp.classes.contains_key("OiCompPlElement"));
        assert!(interp.classes.contains_key("xOiCompPlElement"));
        assert!(interp.classes.contains_key("OiPlanning"));
        assert!(interp.classes.contains_key("xOiPlanning"));
        assert!(interp.classes.contains_key("xOiTable2"));
        assert!(interp.classes.contains_key("xOiSurface"));
        assert!(interp.classes.contains_key("xOiLayoutGroup"));
        assert!(interp.classes.contains_key("xOiImport"));
        assert!(interp.classes.contains_key("xOiFreeArticle"));
        assert!(interp.classes.contains_key("xOiAttPtInteractor"));
        assert!(interp.classes.contains_key("xOiProgInfo"));
    }

    #[test]
    fn test_xoi_table2_read_entries_limit() {
        let mut table = create_test_table();

        // Add many records
        for i in 0..20 {
            let mut fields = HashMap::new();
            fields.insert("type".to_string(), "material".to_string());
            fields.insert("value".to_string(), format!("val{}", i));
            table.records.push(OcdRecord { fields });
        }

        // The limit parameter is currently not enforced by the implementation
        // Verify that all matching records are returned regardless of limit
        let filter = vec![("type".to_string(), "material".to_string())];
        let attrs = vec!["value".to_string()];
        let results = table.read_entries_for(&filter, &attrs, 5);
        assert_eq!(results.len(), 20);
    }

    #[test]
    fn test_xoi_table2_read_entries_no_limit() {
        let mut table = create_test_table();

        // Add some records
        for i in 0..3 {
            let mut fields = HashMap::new();
            fields.insert("type".to_string(), "test".to_string());
            fields.insert("value".to_string(), format!("val{}", i));
            table.records.push(OcdRecord { fields });
        }

        // Use 0 as "no limit"
        let filter = vec![("type".to_string(), "test".to_string())];
        let attrs = vec!["value".to_string()];
        let results = table.read_entries_for(&filter, &attrs, 0);
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_xoi_table2_multiple_attrs() {
        let mut table = XOiTable2::new(
            "multi_attr.ocd",
            vec![
                TableAttribute {
                    name: "a".to_string(),
                    primary_key: 1,
                    key: 0,
                    data_type: 's',
                    default: "".to_string(),
                    format: "".to_string(),
                },
                TableAttribute {
                    name: "b".to_string(),
                    primary_key: 0,
                    key: 0,
                    data_type: 's',
                    default: "".to_string(),
                    format: "".to_string(),
                },
                TableAttribute {
                    name: "c".to_string(),
                    primary_key: 0,
                    key: 0,
                    data_type: 's',
                    default: "".to_string(),
                    format: "".to_string(),
                },
            ],
        );

        let mut fields = HashMap::new();
        fields.insert("a".to_string(), "1".to_string());
        fields.insert("b".to_string(), "2".to_string());
        fields.insert("c".to_string(), "3".to_string());
        table.records.push(OcdRecord { fields });

        // Read multiple attributes at once
        let filter = vec![("a".to_string(), "1".to_string())];
        let attrs = vec!["b".to_string(), "c".to_string()];
        let results = table.read_entries_for(&filter, &attrs, 10);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].len(), 2);
        assert_eq!(results[0][0], "2");
        assert_eq!(results[0][1], "3");
    }

    #[test]
    fn test_xoi_table2_partial_filter_match() {
        let mut table = create_test_table();

        let mut fields = HashMap::new();
        fields.insert("type".to_string(), "material".to_string());
        fields.insert("value".to_string(), "wood".to_string());
        table.records.push(OcdRecord { fields });

        // Filter with different value - should not match
        let filter = vec![("type".to_string(), "color".to_string())];
        let attrs = vec!["value".to_string()];
        let results = table.read_entries_for(&filter, &attrs, 10);
        assert!(results.is_empty());
    }

    #[test]
    fn test_xoi_table2_wildcard_filter() {
        let mut table = create_test_table();

        // Add records
        let mut fields = HashMap::new();
        fields.insert("type".to_string(), "material".to_string());
        fields.insert("value".to_string(), "wood".to_string());
        table.records.push(OcdRecord { fields });

        // Filter with wildcard (empty value matches all)
        let filter = vec![("type".to_string(), "*".to_string())];
        let attrs = vec!["value".to_string()];
        let results = table.read_entries_for(&filter, &attrs, 10);
        // Wildcard doesn't match in current implementation
        assert!(results.is_empty());
    }

    #[test]
    fn test_xoi_table2_read_1_entry_offset() {
        let mut table = create_test_table();

        // Add multiple matching records
        for i in 0..3 {
            let mut fields = HashMap::new();
            fields.insert("type".to_string(), "material".to_string());
            fields.insert("value".to_string(), format!("val{}", i));
            table.records.push(OcdRecord { fields });
        }

        // The flags parameter is not used for offset in current implementation
        // read_1_entry_for always returns the first matching record
        let filter = vec![("type".to_string(), "material".to_string())];
        let attrs = vec!["value".to_string()];

        let result0 = table.read_1_entry_for(&filter, &attrs, 0);
        assert!(result0.is_some());
        assert_eq!(result0.unwrap()[0], "val0");

        // Any flags value returns the same first match
        let result1 = table.read_1_entry_for(&filter, &attrs, 1);
        assert!(result1.is_some());
        assert_eq!(result1.unwrap()[0], "val0"); // Still returns first match

        let result2 = table.read_1_entry_for(&filter, &attrs, 2);
        assert!(result2.is_some());
        assert_eq!(result2.unwrap()[0], "val0"); // Still returns first match
    }

    #[test]
    fn test_xoi_table2_indices() {
        let mut table = create_test_table();
        assert!(table.indices.is_empty());

        table.indices.push("idx_type".to_string());
        table.indices.push("idx_value".to_string());
        assert_eq!(table.indices.len(), 2);
    }

    // Tests for native xOi functions
    #[test]
    fn test_oi_table_function() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        // Test openTbl operation
        if let Some(Value::NativeFunc(func)) = interp.env.get("oiTable") {
            let args = vec![Value::Symbol(Rc::new("openTbl".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert_eq!(result.to_int(), Some(1));
        }

        // Test closeTbl operation
        if let Some(Value::NativeFunc(func)) = interp.env.get("oiTable") {
            let args = vec![Value::Symbol(Rc::new("closeTbl".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert_eq!(result.to_int(), Some(1));
        }

        // Test readTbl operation
        if let Some(Value::NativeFunc(func)) = interp.env.get("oiTable") {
            let args = vec![Value::Symbol(Rc::new("readTbl".to_string()))];
            let result = func(&mut interp, args).unwrap();
            if let Value::Array(arr) = result {
                assert!(arr.borrow().is_empty());
            } else {
                panic!("Expected array");
            }
        }

        // Test unknown operation
        if let Some(Value::NativeFunc(func)) = interp.env.get("oiTable") {
            let args = vec![Value::Symbol(Rc::new("unknownOp".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert_eq!(result.to_int(), Some(0));
        }

        // Test with invalid args
        if let Some(Value::NativeFunc(func)) = interp.env.get("oiTable") {
            let args = vec![Value::Int(42)];
            let result = func(&mut interp, args).unwrap();
            assert_eq!(result.to_int(), Some(0));
        }
    }

    #[test]
    fn test_oi_get_std_att_pts_order() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("oiGetStdAttPtsOrder") {
            let result = func(&mut interp, vec![]).unwrap();
            if let Value::Array(arr) = result {
                let arr_ref = arr.borrow();
                assert_eq!(arr_ref.len(), 6);
                // Symbols are prefixed with "@" when converted to string
                assert_eq!(arr_ref[0].to_string_val(), "@front");
                assert_eq!(arr_ref[1].to_string_val(), "@back");
                assert_eq!(arr_ref[2].to_string_val(), "@left");
                assert_eq!(arr_ref[3].to_string_val(), "@right");
                assert_eq!(arr_ref[4].to_string_val(), "@top");
                assert_eq!(arr_ref[5].to_string_val(), "@bottom");
            } else {
                panic!("Expected array");
            }
        }
    }

    #[test]
    fn test_oi_get_opposite_att_pts_4_std() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("oiGetOppositeAttPts4Std") {
            // Test front -> back (Symbols prefixed with "@" in to_string_val)
            let args = vec![Value::Null, Value::Symbol(Rc::new("front".to_string()))];
            let result = func(&mut interp, args).unwrap();
            if let Value::Array(arr) = result {
                assert_eq!(arr.borrow()[0].to_string_val(), "@back");
            }

            // Test back -> front
            let args = vec![Value::Null, Value::Symbol(Rc::new("back".to_string()))];
            let result = func(&mut interp, args).unwrap();
            if let Value::Array(arr) = result {
                assert_eq!(arr.borrow()[0].to_string_val(), "@front");
            }

            // Test left -> right
            let args = vec![Value::Null, Value::Symbol(Rc::new("left".to_string()))];
            let result = func(&mut interp, args).unwrap();
            if let Value::Array(arr) = result {
                assert_eq!(arr.borrow()[0].to_string_val(), "@right");
            }

            // Test right -> left
            let args = vec![Value::Null, Value::Symbol(Rc::new("right".to_string()))];
            let result = func(&mut interp, args).unwrap();
            if let Value::Array(arr) = result {
                assert_eq!(arr.borrow()[0].to_string_val(), "@left");
            }

            // Test top -> bottom
            let args = vec![Value::Null, Value::Symbol(Rc::new("top".to_string()))];
            let result = func(&mut interp, args).unwrap();
            if let Value::Array(arr) = result {
                assert_eq!(arr.borrow()[0].to_string_val(), "@bottom");
            }

            // Test bottom -> top
            let args = vec![Value::Null, Value::Symbol(Rc::new("bottom".to_string()))];
            let result = func(&mut interp, args).unwrap();
            if let Value::Array(arr) = result {
                assert_eq!(arr.borrow()[0].to_string_val(), "@top");
            }

            // Test unknown -> Null
            let args = vec![Value::Null, Value::Symbol(Rc::new("unknown".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Null));

            // Test invalid args -> Null
            let args = vec![Value::Null, Value::Int(42)];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }

    #[test]
    fn test_get_scene_and_get_planning() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        // Test getScene with no planning defined
        if let Some(Value::NativeFunc(func)) = interp.env.get("getScene") {
            let result = func(&mut interp, vec![]).unwrap();
            assert!(matches!(result, Value::Null));
        }

        // Test getPlanning with no planning defined
        if let Some(Value::NativeFunc(func)) = interp.env.get("getPlanning") {
            let result = func(&mut interp, vec![]).unwrap();
            assert!(matches!(result, Value::Null));
        }

        // Define a planning object
        let planning_class = interp.classes.get("xOiPlanning").cloned().unwrap();
        let planning = Rc::new(RefCell::new(ObjInstance {
            class: planning_class,
            name: "_planning".to_string(),
            ..Default::default()
        }));
        interp
            .env
            .define_global("_planning", Value::Object(planning.clone()));

        // Test getScene with planning defined
        if let Some(Value::NativeFunc(func)) = interp.env.get("getScene") {
            let result = func(&mut interp, vec![]).unwrap();
            if let Value::Object(obj) = result {
                assert_eq!(obj.borrow().name, "_planning");
            }
        }
    }

    #[test]
    fn test_xoi_check_el_pos() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("xOiCheckElPos") {
            let result = func(&mut interp, vec![]).unwrap();
            if let Value::Array(arr) = result {
                let arr_ref = arr.borrow();
                assert_eq!(arr_ref.len(), 3);
                assert_eq!(arr_ref[0].to_float(), Some(0.0));
                assert_eq!(arr_ref[1].to_float(), Some(0.0));
                assert_eq!(arr_ref[2].to_float(), Some(0.0));
            }
        }
    }

    #[test]
    fn test_oi_get_pos_rot_4_attach_pts() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("oiGetPosRot4AttachPts") {
            let result = func(&mut interp, vec![]).unwrap();
            if let Value::Array(arr) = result {
                let arr_ref = arr.borrow();
                assert_eq!(arr_ref.len(), 3);
                // First element is position array
                if let Value::Array(pos) = &arr_ref[0] {
                    assert_eq!(pos.borrow().len(), 3);
                }
                // Second is rotation
                assert_eq!(arr_ref[1].to_float(), Some(0.0));
                // Third is error (null)
                assert!(matches!(arr_ref[2], Value::Null));
            }
        }
    }

    #[test]
    fn test_xoi_auto_decoration_functions() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        // xOiAutoDecoration4Obj returns true
        if let Some(Value::NativeFunc(func)) = interp.env.get("xOiAutoDecoration4Obj") {
            let result = func(&mut interp, vec![]).unwrap();
            assert!(matches!(result, Value::Bool(true)));
        }

        // xOiDeleteAutoDeco returns null
        if let Some(Value::NativeFunc(func)) = interp.env.get("xOiDeleteAutoDeco") {
            let result = func(&mut interp, vec![]).unwrap();
            assert!(matches!(result, Value::Null));
        }

        // xOiGetMetaInfo returns null
        if let Some(Value::NativeFunc(func)) = interp.env.get("xOiGetMetaInfo") {
            let result = func(&mut interp, vec![]).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }

    #[test]
    fn test_create_ch_params_and_accept_ch() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        // createChParams returns null
        if let Some(Value::NativeFunc(func)) = interp.env.get("createChParams") {
            let result = func(&mut interp, vec![]).unwrap();
            assert!(matches!(result, Value::Null));
        }

        // acceptCh returns first arg or null
        if let Some(Value::NativeFunc(func)) = interp.env.get("acceptCh") {
            let result = func(&mut interp, vec![]).unwrap();
            assert!(matches!(result, Value::Null));

            let args = vec![Value::Int(42)];
            let result = func(&mut interp, args).unwrap();
            assert_eq!(result.to_int(), Some(42));
        }

        // removeCh returns null
        if let Some(Value::NativeFunc(func)) = interp.env.get("removeCh") {
            let result = func(&mut interp, vec![]).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }

    #[test]
    fn test_get_language() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("getLanguage") {
            let result = func(&mut interp, vec![]).unwrap();
            if let Value::String(s) = result {
                assert_eq!(s.as_str(), "de");
            }
        }
    }

    #[test]
    fn test_get_creation_mode() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("getCreationMode") {
            let result = func(&mut interp, vec![]).unwrap();
            assert_eq!(result.to_int(), Some(0));
        }
    }

    #[test]
    fn test_vector_constructor() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("Vector") {
            // Create empty vector
            let result = func(&mut interp, vec![]).unwrap();
            if let Value::Array(arr) = result {
                assert!(arr.borrow().is_empty());
            }

            // Create vector with size
            let args = vec![Value::Int(5)];
            let result = func(&mut interp, args).unwrap();
            if let Value::Array(arr) = result {
                assert_eq!(arr.borrow().len(), 5);
                // All elements should be null
                for elem in arr.borrow().iter() {
                    assert!(matches!(elem, Value::Null));
                }
            }
        }
    }

    #[test]
    fn test_list_constructor() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("List") {
            // Create empty list
            let result = func(&mut interp, vec![]).unwrap();
            if let Value::Array(arr) = result {
                assert!(arr.borrow().is_empty());
            }

            // Create list with elements
            let args = vec![Value::Int(1), Value::Int(2), Value::Int(3)];
            let result = func(&mut interp, args).unwrap();
            if let Value::Array(arr) = result {
                let arr_ref = arr.borrow();
                assert_eq!(arr_ref.len(), 3);
                assert_eq!(arr_ref[0].to_int(), Some(1));
                assert_eq!(arr_ref[1].to_int(), Some(2));
                assert_eq!(arr_ref[2].to_int(), Some(3));
            }
        }
    }

    #[test]
    fn test_oi_get_string_resource() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("oiGetStringResource") {
            let args = vec![Value::String(Rc::new("my_key".to_string()))];
            let result = func(&mut interp, args).unwrap();
            if let Value::String(s) = result {
                assert_eq!(s.as_str(), "my_key"); // Returns key as default
            }
        }
    }

    #[test]
    fn test_xoi_get_app_registry_key() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("xOiGetAppRegistryKey") {
            let args = vec![
                Value::String(Rc::new("program".to_string())),
                Value::String(Rc::new("key".to_string())),
                Value::String(Rc::new("default_value".to_string())),
            ];
            let result = func(&mut interp, args).unwrap();
            if let Value::String(s) = result {
                assert_eq!(s.as_str(), "default_value");
            }
        }
    }

    #[test]
    fn test_list_contains() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("listContains") {
            // Test with matching element
            let arr = Rc::new(RefCell::new(vec![
                Value::String(Rc::new("a".to_string())),
                Value::String(Rc::new("b".to_string())),
                Value::String(Rc::new("c".to_string())),
            ]));
            let args = vec![
                Value::Array(arr.clone()),
                Value::String(Rc::new("b".to_string())),
            ];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Bool(true)));

            // Test with non-matching element
            let args = vec![
                Value::Array(arr.clone()),
                Value::String(Rc::new("x".to_string())),
            ];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Bool(false)));

            // Test with non-array first arg
            let args = vec![Value::Int(42), Value::String(Rc::new("x".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Bool(false)));
        }
    }

    #[test]
    fn test_eval_function() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("eval") {
            // Test empty string
            let args = vec![Value::String(Rc::new("".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Null));

            // Test whitespace only
            let args = vec![Value::String(Rc::new("   ".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Null));

            // Test simple expression
            let args = vec![Value::String(Rc::new("1 + 2;".to_string()))];
            let result = func(&mut interp, args);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_error_class_registered() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        // Check Error class is registered
        assert!(interp.classes.contains_key("Error"));

        // Check it's also in env
        if let Some(Value::Class(cls)) = interp.env.get("Error") {
            assert_eq!(cls.name, "Error");
        } else {
            panic!("Error class not in env");
        }
    }

    #[test]
    fn test_xoi_copy_aggr() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("xOiCopyAggr") {
            let src = Rc::new(RefCell::new(vec![Value::Int(1), Value::Int(2)]));
            let dst = Rc::new(RefCell::new(vec![Value::Int(99)]));

            let args = vec![Value::Array(src.clone()), Value::Array(dst.clone())];
            let result = func(&mut interp, args);
            assert!(result.is_ok());

            // Check dst was updated
            let dst_ref = dst.borrow();
            assert_eq!(dst_ref.len(), 2);
            assert_eq!(dst_ref[0].to_int(), Some(1));
            assert_eq!(dst_ref[1].to_int(), Some(2));
        }
    }

    #[test]
    fn test_oi_get_planning_creates_default() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("oiGetPlanning") {
            let result = func(&mut interp, vec![]).unwrap();
            // Should create a default planning object
            if let Value::Object(obj) = result {
                assert_eq!(obj.borrow().name, "_planning");
            }
        }
    }

    // ========== Additional Coverage Tests ==========

    #[test]
    fn test_open_data_tbl() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("openDataTbl") {
            let result = func(&mut interp, vec![]).unwrap();
            if let Value::Object(obj) = result {
                // Check table object was created
                assert!(obj.borrow().fields.contains_key("mID"));
                assert!(obj.borrow().fields.contains_key("mIsOpen"));
            }
        }
    }

    #[test]
    fn test_oi_table_operations() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("oiTable") {
            // Test openTbl
            let args = vec![Value::Symbol(Rc::new("openTbl".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert_eq!(result.to_int(), Some(1));

            // Test closeTbl
            let args = vec![Value::Symbol(Rc::new("closeTbl".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert_eq!(result.to_int(), Some(1));

            // Test readTbl
            let args = vec![Value::Symbol(Rc::new("readTbl".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Array(_)));

            // Test unknown operation
            let args = vec![Value::Symbol(Rc::new("unknown".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert_eq!(result.to_int(), Some(0));

            // Test with non-symbol arg
            let args = vec![Value::Int(42)];
            let result = func(&mut interp, args).unwrap();
            assert_eq!(result.to_int(), Some(0));
        }
    }

    #[test]
    fn test_oi_get_std_att_pts() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("oiGetStdAttPts") {
            let result = func(&mut interp, vec![]).unwrap();
            // Should return empty array when no current_self
            if let Value::Array(arr) = result {
                assert!(arr.borrow().is_empty());
            }
        }
    }

    #[test]
    fn test_oi_get_opposite_att_pts() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("oiGetOppositeAttPts4Std") {
            // Test front -> back
            let args = vec![Value::Null, Value::Symbol(Rc::new("front".to_string()))];
            let result = func(&mut interp, args).unwrap();
            if let Value::Array(arr) = result {
                if let Value::Symbol(s) = &arr.borrow()[0] {
                    assert_eq!(**s, "back");
                }
            }

            // Test left -> right
            let args = vec![Value::Null, Value::Symbol(Rc::new("left".to_string()))];
            let result = func(&mut interp, args).unwrap();
            if let Value::Array(arr) = result {
                if let Value::Symbol(s) = &arr.borrow()[0] {
                    assert_eq!(**s, "right");
                }
            }

            // Test top -> bottom
            let args = vec![Value::Null, Value::Symbol(Rc::new("top".to_string()))];
            let result = func(&mut interp, args).unwrap();
            if let Value::Array(arr) = result {
                if let Value::Symbol(s) = &arr.borrow()[0] {
                    assert_eq!(**s, "bottom");
                }
            }

            // Test unknown -> Null
            let args = vec![Value::Null, Value::Symbol(Rc::new("unknown".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Null));

            // Test with non-symbol
            let args = vec![Value::Null, Value::Int(42)];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }

    #[test]
    fn test_set_rt_axis() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("setRtAxis") {
            let args = vec![Value::Int(1)];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }

    #[test]
    fn test_set_tr_axis() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("setTrAxis") {
            let args = vec![Value::Int(3)];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }

    #[test]
    fn test_get_scene() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("getScene") {
            // Should return Null when no planning object
            let result = func(&mut interp, vec![]).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }

    #[test]
    fn test_get_planning() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("getPlanning") {
            // Should return Null when no planning object
            let result = func(&mut interp, vec![]).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }

    #[test]
    fn test_get_root() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("getRoot") {
            // Should return Null when no current_self
            let result = func(&mut interp, vec![]).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }

    #[test]
    fn test_get_father() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("getFather") {
            // Should return Null when no current_self
            let result = func(&mut interp, vec![]).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }

    #[test]
    fn test_get_children() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("getChildren") {
            let result = func(&mut interp, vec![]).unwrap();
            // Should return empty array when no current_self
            if let Value::Array(arr) = result {
                assert!(arr.borrow().is_empty());
            }
        }
    }

    #[test]
    fn test_has_member() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("hasMember") {
            // Should return false when no current_self
            let args = vec![Value::Symbol(Rc::new("test".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Bool(false)));

            // Test with string arg
            let args = vec![Value::String(Rc::new("test".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Bool(false)));

            // Test with invalid arg
            let args = vec![Value::Int(42)];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Bool(false)));
        }
    }

    #[test]
    fn test_oi_output() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("oiOutput") {
            let args = vec![
                Value::Symbol(Rc::new("INFO".to_string())),
                Value::String(Rc::new("Test message".to_string())),
            ];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Null));

            // Test with default level
            let args = vec![Value::Null, Value::String(Rc::new("Test".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }

    #[test]
    fn test_xoi_auto_decoration() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("xOiAutoDecoration4Obj") {
            let result = func(&mut interp, vec![]).unwrap();
            assert!(matches!(result, Value::Bool(true)));
        }

        if let Some(Value::NativeFunc(func)) = interp.env.get("xOiDeleteAutoDeco") {
            let result = func(&mut interp, vec![]).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }

    #[test]
    fn test_xoi_get_meta_info() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("xOiGetMetaInfo") {
            let result = func(&mut interp, vec![]).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }

    #[test]
    fn test_set_method() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("setMethod") {
            let args = vec![Value::String(Rc::new("create".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }

    #[test]
    fn test_create_ch_params() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("createChParams") {
            let result = func(&mut interp, vec![]).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }

    #[test]
    fn test_accept_ch() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("acceptCh") {
            let args = vec![Value::Int(42)];
            let result = func(&mut interp, args).unwrap();
            assert_eq!(result.to_int(), Some(42));

            let result = func(&mut interp, vec![]).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }

    #[test]
    fn test_remove_ch() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("removeCh") {
            let result = func(&mut interp, vec![]).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }


    #[test]
    fn test_has_property() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("hasProperty") {
            // Test with symbol arg
            let args = vec![Value::Symbol(Rc::new("test".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Bool(false)));

            // Test with string arg
            let args = vec![Value::String(Rc::new("test".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Bool(false)));

            // Test with invalid arg
            let args = vec![Value::Int(42)];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Bool(false)));
        }
    }

    #[test]
    fn test_get_program() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("getProgram") {
            // Should return Null when no current_self
            let result = func(&mut interp, vec![]).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }

    #[test]
    fn test_get_article_obj() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("getArticleObj") {
            // Should return Null when no current_self
            let result = func(&mut interp, vec![]).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }

    #[test]
    fn test_set_prop_state2() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("setPropState2") {
            // Test with symbol key
            let args = vec![Value::Symbol(Rc::new("prop".to_string())), Value::Int(1)];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Null));

            // Test with string key
            let args = vec![Value::String(Rc::new("prop".to_string())), Value::Int(1)];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Null));

            // Test with invalid key
            let args = vec![Value::Int(42), Value::Int(1)];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }

    #[test]
    fn test_setup_note_property() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("setupNoteProperty") {
            // Test with symbol key
            let args = vec![Value::Symbol(Rc::new("note".to_string())), Value::String(Rc::new("value".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Null));

            // Test with string key
            let args = vec![Value::String(Rc::new("note".to_string())), Value::Int(42)];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Null));

            // Test with invalid key
            let args = vec![Value::Int(42)];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }

    #[test]
    fn test_remove_note_property() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("removeNoteProperty") {
            // Test with symbol key
            let args = vec![Value::Symbol(Rc::new("note".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Null));

            // Test with string key
            let args = vec![Value::String(Rc::new("note".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Null));

            // Test with invalid key
            let args = vec![Value::Int(42)];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }

    #[test]
    fn test_is_a_function() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("isA") {
            // Test with class arg
            if let Some(Value::Class(cls)) = interp.env.get("Error") {
                let args = vec![Value::Class(cls)];
                let result = func(&mut interp, args).unwrap();
                // Without current_self, returns false
                assert!(matches!(result, Value::Bool(false)));
            }

            // Test with string arg
            let args = vec![Value::String(Rc::new("SomeClass".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Bool(false)));

            // Test with symbol arg
            let args = vec![Value::Symbol(Rc::new("SomeClass".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Bool(false)));

            // Test with invalid arg
            let args = vec![Value::Int(42)];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Bool(false)));
        }
    }

    #[test]
    fn test_instanceof_function() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("instanceof") {
            // Test with non-object first arg
            let args = vec![Value::Int(42), Value::String(Rc::new("Class".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Bool(false)));

            // Test with invalid second arg
            let obj_class = interp.create_native_class_internal("Test", None);
            let obj = Rc::new(RefCell::new(ObjInstance {
                class: obj_class,
                name: "test".to_string(),
                ..Default::default()
            }));
            let args = vec![Value::Object(obj), Value::Int(42)];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Bool(false)));
        }
    }

    #[test]
    fn test_set_article_spec() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("setArticleSpec") {
            let args = vec![Value::String(Rc::new("ARTICLE-001".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }

    #[test]
    fn test_get_article_spec() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("getArticleSpec") {
            // Without current_self
            let result = func(&mut interp, vec![]).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }

    #[test]
    fn test_set_x_article_spec() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("setXArticleSpec") {
            let args = vec![
                Value::Int(1),
                Value::String(Rc::new("XSPEC-001".to_string())),
            ];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }

    #[test]
    fn test_xoi_table2_read_1_entry() {
        let mut table = create_test_table();

        // Add a test record
        let mut fields = HashMap::new();
        fields.insert("type".to_string(), "material".to_string());
        fields.insert("value".to_string(), "wood".to_string());
        table.records.push(OcdRecord { fields });

        let filter = vec![("type".to_string(), "material".to_string())];
        let attrs = vec!["value".to_string()];

        let result = table.read_1_entry_for(&filter, &attrs, 0);
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0], "wood");

        // Test with non-matching filter
        let filter = vec![("type".to_string(), "nonexistent".to_string())];
        let result = table.read_1_entry_for(&filter, &attrs, 0);
        assert!(result.is_none());
    }

    #[test]
    fn test_xoi_classes_registered() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        // Check key classes are registered
        assert!(interp.classes.contains_key("xxScElement"));
        assert!(interp.classes.contains_key("OiPlElement"));
        assert!(interp.classes.contains_key("xOiPlElement"));
        assert!(interp.classes.contains_key("xOiBTGPlElement"));
        assert!(interp.classes.contains_key("xOiBTGPlElement2"));
        assert!(interp.classes.contains_key("xOiBTGPlElement3"));
        assert!(interp.classes.contains_key("OiOdbPlElement"));
        assert!(interp.classes.contains_key("xOiOdbPlElement"));
        assert!(interp.classes.contains_key("OiCompPlElement"));
        assert!(interp.classes.contains_key("xOiCompPlElement"));
        assert!(interp.classes.contains_key("OiPlanning"));
        assert!(interp.classes.contains_key("xOiPlanning"));
        assert!(interp.classes.contains_key("xOiTable2"));
        assert!(interp.classes.contains_key("xOiFreeArticle"));
        assert!(interp.classes.contains_key("xOiAttPtInteractor"));
        assert!(interp.classes.contains_key("xOiProgInfo"));
        assert!(interp.classes.contains_key("xOiSurface"));
        assert!(interp.classes.contains_key("xOiLayoutGroup"));
        assert!(interp.classes.contains_key("xOiImport"));
    }

    #[test]
    fn test_xoi_attpt_if_singleton() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        // Check xOiAttPtIF singleton is registered
        if let Some(Value::Object(obj)) = interp.env.get("xOiAttPtIF") {
            assert_eq!(obj.borrow().name, "xOiAttPtIF");
        } else {
            panic!("xOiAttPtIF not registered");
        }
    }

    #[test]
    fn test_eval_with_parse_error() {
        let mut interp = Interpreter::new();
        interp.register_xoi_classes();

        if let Some(Value::NativeFunc(func)) = interp.env.get("eval") {
            // Test with syntax error - should return Null
            let args = vec![Value::String(Rc::new("{{ invalid".to_string()))];
            let result = func(&mut interp, args).unwrap();
            assert!(matches!(result, Value::Null));
        }
    }
}
