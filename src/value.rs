//! Runtime values for the OFML interpreter

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::ast::ClassDecl;

/// Runtime value types
#[derive(Clone)]
pub enum Value {
    /// Null value
    Null,
    /// Integer
    Int(i64),
    /// Floating point
    Float(f64),
    /// String
    String(Rc<String>),
    /// Symbol (interned string)
    Symbol(Rc<String>),
    /// Boolean (used internally, OFML uses Int for bools)
    Bool(bool),
    /// Array/Vector
    Array(Rc<RefCell<Vec<Value>>>),
    /// Hash map
    Hash(Rc<RefCell<HashMap<String, Value>>>),
    /// Object instance
    Object(Rc<RefCell<ObjInstance>>),
    /// Function reference
    Func(Rc<FuncValue>),
    /// Native function
    NativeFunc(Rc<NativeFn>),
    /// Class reference
    Class(Rc<ClassValue>),
    /// 3D Vector (convenience type for geometry)
    Vec3([f64; 3]),
}

/// Property definition for OFML property system
#[derive(Clone, Debug)]
pub struct PropertyDef {
    /// Property name (symbol)
    pub name: String,
    /// Property type info
    pub type_info: String,
    /// Property description/label
    pub description: String,
    /// Sort order
    pub sort_order: i32,
    /// Property group
    pub group: i32,
    /// Available choices (for enum properties)
    pub choices: Vec<Value>,
    /// Property state (0=hidden, 1=readonly, 3=editable)
    pub state: i32,
    /// Default value
    pub default_value: Value,
}

impl Default for PropertyDef {
    fn default() -> Self {
        Self {
            name: String::new(),
            type_info: String::new(),
            description: String::new(),
            sort_order: 0,
            group: 0,
            choices: Vec::new(),
            state: 3, // editable by default
            default_value: Value::Null,
        }
    }
}

/// Object instance
#[derive(Clone)]
pub struct ObjInstance {
    /// Class this object is an instance of
    pub class: Rc<ClassValue>,
    /// Instance variables
    pub fields: HashMap<String, Value>,
    /// Property values (OFML property system)
    pub properties: HashMap<String, Value>,
    /// Property definitions
    pub prop_defs: HashMap<String, PropertyDef>,
    /// Property states (0=hidden, 1=readonly, 3=editable)
    pub prop_states: HashMap<String, i32>,
    /// Parent object (for hierarchy)
    pub parent: Option<Rc<RefCell<ObjInstance>>>,
    /// Child objects
    pub children: Vec<Rc<RefCell<ObjInstance>>>,
    /// Object name
    pub name: String,
    /// Position in 3D space
    pub position: [f64; 3],
    /// Rotation (axis angles in radians)
    pub rotation: [f64; 3],
    /// Scale factor
    pub scale: f64,
    /// Material name
    pub material: Option<String>,
}

/// Function value
#[derive(Clone)]
pub struct FuncValue {
    pub name: String,
    pub params: Vec<String>,
    pub body: Option<crate::ast::Block>,
    pub class: Option<Rc<ClassValue>>,
    pub is_static: bool,
}

/// Class value
#[derive(Clone)]
pub struct ClassValue {
    pub name: String,
    /// Package this class belongs to (e.g., "::vitra::basics")
    pub package: String,
    pub parent: Option<Rc<ClassValue>>,
    pub methods: HashMap<String, Rc<FuncValue>>,
    pub rules: HashMap<String, Rc<FuncValue>>,
    pub static_vars: HashMap<String, Value>,
    pub decl: ClassDecl,
}

impl ClassValue {
    /// Get fully qualified class name (e.g., "::vitra::basics::VitraOiBTGPlElement3")
    pub fn qualified_name(&self) -> String {
        if self.package.is_empty() {
            self.name.clone()
        } else {
            format!("{}::{}", self.package, self.name)
        }
    }
}

/// Native function signature
pub type NativeFn =
    dyn Fn(&mut crate::interpreter::Interpreter, Vec<Value>) -> Result<Value, String>;

impl Value {
    /// Check if value is truthy (for conditionals)
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Bool(b) => *b,
            Value::Int(n) => *n != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(arr) => !arr.borrow().is_empty(),
            _ => true,
        }
    }

    /// Convert to integer
    pub fn to_int(&self) -> Option<i64> {
        match self {
            Value::Int(n) => Some(*n),
            Value::Float(f) => Some(*f as i64),
            Value::String(s) => s.parse().ok(),
            Value::Bool(b) => Some(if *b { 1 } else { 0 }),
            _ => None,
        }
    }

    /// Convert to float
    pub fn to_float(&self) -> Option<f64> {
        match self {
            Value::Int(n) => Some(*n as f64),
            Value::Float(f) => Some(*f),
            Value::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// Convert to string
    pub fn to_string_val(&self) -> String {
        match self {
            Value::Null => "NULL".to_string(),
            Value::Int(n) => n.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => s.to_string(),
            Value::Symbol(s) => format!("@{}", s),
            Value::Bool(b) => if *b { "1" } else { "0" }.to_string(),
            Value::Array(arr) => {
                let items: Vec<String> = arr.borrow().iter().map(|v| v.to_string_val()).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Hash(_) => "[Hash]".to_string(),
            Value::Object(obj) => format!("[Object:{}]", obj.borrow().name),
            Value::Func(f) => format!("[Func:{}]", f.name),
            Value::NativeFunc(_) => "[NativeFunc]".to_string(),
            Value::Class(c) => format!("[Class:{}]", c.name),
            Value::Vec3(v) => format!("[{}, {}, {}]", v[0], v[1], v[2]),
        }
    }

    /// Get type name
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Null => "Null",
            Value::Int(_) => "Int",
            Value::Float(_) => "Float",
            Value::String(_) => "String",
            Value::Symbol(_) => "Symbol",
            Value::Bool(_) => "Bool",
            Value::Array(_) => "Array",
            Value::Hash(_) => "Hash",
            Value::Object(_) => "Object",
            Value::Func(_) => "Func",
            Value::NativeFunc(_) => "NativeFunc",
            Value::Class(_) => "Class",
            Value::Vec3(_) => "Vec3",
        }
    }

    /// Check equality
    pub fn equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) => true,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Int(a), Value::Float(b)) => (*a as f64) == *b,
            (Value::Float(a), Value::Int(b)) => *a == (*b as f64),
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Symbol(a), Value::Symbol(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Vec3(a), Value::Vec3(b)) => a == b,
            _ => false,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_val())
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_val())
    }
}

impl Default for ObjInstance {
    fn default() -> Self {
        Self {
            class: Rc::new(ClassValue {
                name: "Object".to_string(),
                package: String::new(),
                parent: None,
                methods: HashMap::new(),
                rules: HashMap::new(),
                static_vars: HashMap::new(),
                decl: ClassDecl {
                    modifiers: vec![],
                    name: "Object".to_string(),
                    parent: None,
                    members: vec![],
                    span: crate::ast::Span::default(),
                },
            }),
            fields: HashMap::new(),
            properties: HashMap::new(),
            prop_defs: HashMap::new(),
            prop_states: HashMap::new(),
            parent: None,
            children: Vec::new(),
            name: String::new(),
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: 1.0,
            material: None,
        }
    }
}

impl ObjInstance {
    /// Check if this object is an instance of a class (or its parent)
    pub fn is_a(&self, class_name: &str) -> bool {
        let mut current = Some(self.class.clone());
        while let Some(cls) = current {
            if cls.name == class_name {
                return true;
            }
            current = cls.parent.clone();
        }
        false
    }

    /// Get property value
    pub fn get_prop_value(&self, prop: &str) -> Value {
        self.properties.get(prop).cloned().unwrap_or(Value::Null)
    }

    /// Set property value
    pub fn set_prop_value(&mut self, prop: &str, value: Value) {
        self.properties.insert(prop.to_string(), value);
    }

    /// Get position as Value
    pub fn get_position(&self) -> Value {
        Value::Array(Rc::new(RefCell::new(vec![
            Value::Float(self.position[0]),
            Value::Float(self.position[1]),
            Value::Float(self.position[2]),
        ])))
    }

    /// Set position from Value
    pub fn set_position(&mut self, pos: &Value) -> Result<(), String> {
        match pos {
            Value::Array(arr) => {
                let arr = arr.borrow();
                if arr.len() >= 3 {
                    self.position[0] = arr[0].to_float().unwrap_or(0.0);
                    self.position[1] = arr[1].to_float().unwrap_or(0.0);
                    self.position[2] = arr[2].to_float().unwrap_or(0.0);
                    Ok(())
                } else {
                    Err("Position array must have 3 elements".to_string())
                }
            }
            Value::Vec3(v) => {
                self.position = *v;
                Ok(())
            }
            _ => Err("Position must be an array or Vec3".to_string()),
        }
    }
}
