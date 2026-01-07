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

#[cfg(test)]
mod tests {
    use super::*;

    // ============== Value tests ==============

    #[test]
    fn test_value_null() {
        let v = Value::Null;
        assert!(!v.is_truthy());
        assert_eq!(v.to_int(), None);
        assert_eq!(v.to_float(), None);
        assert_eq!(v.to_string_val(), "NULL");
        assert_eq!(v.type_name(), "Null");
    }

    #[test]
    fn test_value_int() {
        let v = Value::Int(42);
        assert!(v.is_truthy());
        assert_eq!(v.to_int(), Some(42));
        assert_eq!(v.to_float(), Some(42.0));
        assert_eq!(v.to_string_val(), "42");
        assert_eq!(v.type_name(), "Int");

        // Zero is falsy
        let zero = Value::Int(0);
        assert!(!zero.is_truthy());
    }

    #[test]
    fn test_value_float() {
        let v = Value::Float(3.14);
        assert!(v.is_truthy());
        assert_eq!(v.to_int(), Some(3));
        assert_eq!(v.to_float(), Some(3.14));
        assert_eq!(v.type_name(), "Float");

        // Zero float is falsy
        let zero = Value::Float(0.0);
        assert!(!zero.is_truthy());
    }

    #[test]
    fn test_value_string() {
        let v = Value::String(Rc::new("hello".to_string()));
        assert!(v.is_truthy());
        assert_eq!(v.to_string_val(), "hello");
        assert_eq!(v.type_name(), "String");

        // Empty string is falsy
        let empty = Value::String(Rc::new(String::new()));
        assert!(!empty.is_truthy());

        // Numeric string conversion
        let num_str = Value::String(Rc::new("123".to_string()));
        assert_eq!(num_str.to_int(), Some(123));
        assert_eq!(num_str.to_float(), Some(123.0));

        // Non-numeric string
        let non_num = Value::String(Rc::new("abc".to_string()));
        assert_eq!(non_num.to_int(), None);
        assert_eq!(non_num.to_float(), None);
    }

    #[test]
    fn test_value_symbol() {
        let v = Value::Symbol(Rc::new("mysymbol".to_string()));
        assert!(v.is_truthy());
        assert_eq!(v.to_string_val(), "@mysymbol");
        assert_eq!(v.type_name(), "Symbol");
    }

    #[test]
    fn test_value_bool() {
        let t = Value::Bool(true);
        assert!(t.is_truthy());
        assert_eq!(t.to_int(), Some(1));
        assert_eq!(t.to_string_val(), "1");
        assert_eq!(t.type_name(), "Bool");

        let f = Value::Bool(false);
        assert!(!f.is_truthy());
        assert_eq!(f.to_int(), Some(0));
        assert_eq!(f.to_string_val(), "0");
    }

    #[test]
    fn test_value_array() {
        let arr = Value::Array(Rc::new(RefCell::new(vec![
            Value::Int(1),
            Value::Int(2),
            Value::Int(3),
        ])));
        assert!(arr.is_truthy());
        assert_eq!(arr.to_string_val(), "[1, 2, 3]");
        assert_eq!(arr.type_name(), "Array");

        // Empty array is falsy
        let empty = Value::Array(Rc::new(RefCell::new(vec![])));
        assert!(!empty.is_truthy());
    }

    #[test]
    fn test_value_hash() {
        let hash = Value::Hash(Rc::new(RefCell::new(HashMap::new())));
        assert!(hash.is_truthy());
        assert_eq!(hash.to_string_val(), "[Hash]");
        assert_eq!(hash.type_name(), "Hash");
    }

    #[test]
    fn test_value_vec3() {
        let v = Value::Vec3([1.0, 2.0, 3.0]);
        assert!(v.is_truthy());
        assert_eq!(v.to_string_val(), "[1, 2, 3]");
        assert_eq!(v.type_name(), "Vec3");
    }

    #[test]
    fn test_value_equals() {
        // Null equality
        assert!(Value::Null.equals(&Value::Null));
        assert!(!Value::Null.equals(&Value::Int(0)));

        // Int equality
        assert!(Value::Int(42).equals(&Value::Int(42)));
        assert!(!Value::Int(42).equals(&Value::Int(43)));

        // Float equality
        assert!(Value::Float(3.14).equals(&Value::Float(3.14)));

        // Cross-type numeric equality
        assert!(Value::Int(42).equals(&Value::Float(42.0)));
        assert!(Value::Float(42.0).equals(&Value::Int(42)));

        // String equality
        let s1 = Value::String(Rc::new("test".to_string()));
        let s2 = Value::String(Rc::new("test".to_string()));
        let s3 = Value::String(Rc::new("other".to_string()));
        assert!(s1.equals(&s2));
        assert!(!s1.equals(&s3));

        // Symbol equality
        let sym1 = Value::Symbol(Rc::new("sym".to_string()));
        let sym2 = Value::Symbol(Rc::new("sym".to_string()));
        assert!(sym1.equals(&sym2));

        // Bool equality
        assert!(Value::Bool(true).equals(&Value::Bool(true)));
        assert!(!Value::Bool(true).equals(&Value::Bool(false)));

        // Vec3 equality
        assert!(Value::Vec3([1.0, 2.0, 3.0]).equals(&Value::Vec3([1.0, 2.0, 3.0])));
        assert!(!Value::Vec3([1.0, 2.0, 3.0]).equals(&Value::Vec3([1.0, 2.0, 4.0])));

        // Different types don't equal
        assert!(!Value::Int(1).equals(&Value::String(Rc::new("1".to_string()))));
    }

    #[test]
    fn test_value_debug_display() {
        let v = Value::Int(42);
        assert_eq!(format!("{:?}", v), "42");
        assert_eq!(format!("{}", v), "42");
    }

    #[test]
    fn test_value_clone() {
        let v1 = Value::Int(42);
        let v2 = v1.clone();
        assert!(v1.equals(&v2));

        let arr = Value::Array(Rc::new(RefCell::new(vec![Value::Int(1)])));
        let arr2 = arr.clone();
        assert_eq!(arr.type_name(), arr2.type_name());
    }

    // ============== PropertyDef tests ==============

    #[test]
    fn test_property_def_default() {
        let pd = PropertyDef::default();
        assert_eq!(pd.name, "");
        assert_eq!(pd.type_info, "");
        assert_eq!(pd.description, "");
        assert_eq!(pd.sort_order, 0);
        assert_eq!(pd.group, 0);
        assert!(pd.choices.is_empty());
        assert_eq!(pd.state, 3); // editable
        assert!(matches!(pd.default_value, Value::Null));
    }

    #[test]
    fn test_property_def_clone_debug() {
        let pd = PropertyDef {
            name: "color".to_string(),
            type_info: "string".to_string(),
            description: "Color selection".to_string(),
            sort_order: 1,
            group: 0,
            choices: vec![Value::String(Rc::new("red".to_string()))],
            state: 3,
            default_value: Value::String(Rc::new("blue".to_string())),
        };

        let pd2 = pd.clone();
        assert_eq!(pd2.name, "color");

        let debug = format!("{:?}", pd);
        assert!(debug.contains("color"));
    }

    // ============== ClassValue tests ==============

    #[test]
    fn test_class_value_qualified_name() {
        let cv = ClassValue {
            name: "MyClass".to_string(),
            package: "::vitra::basics".to_string(),
            parent: None,
            methods: HashMap::new(),
            rules: HashMap::new(),
            static_vars: HashMap::new(),
            decl: ClassDecl {
                modifiers: vec![],
                name: "MyClass".to_string(),
                parent: None,
                members: vec![],
                span: crate::ast::Span::default(),
            },
        };

        assert_eq!(cv.qualified_name(), "::vitra::basics::MyClass");

        // Empty package
        let cv2 = ClassValue {
            name: "Standalone".to_string(),
            package: String::new(),
            parent: None,
            methods: HashMap::new(),
            rules: HashMap::new(),
            static_vars: HashMap::new(),
            decl: ClassDecl {
                modifiers: vec![],
                name: "Standalone".to_string(),
                parent: None,
                members: vec![],
                span: crate::ast::Span::default(),
            },
        };

        assert_eq!(cv2.qualified_name(), "Standalone");
    }

    // ============== ObjInstance tests ==============

    #[test]
    fn test_obj_instance_default() {
        let obj = ObjInstance::default();
        assert_eq!(obj.class.name, "Object");
        assert!(obj.fields.is_empty());
        assert!(obj.properties.is_empty());
        assert_eq!(obj.name, "");
        assert_eq!(obj.position, [0.0, 0.0, 0.0]);
        assert_eq!(obj.rotation, [0.0, 0.0, 0.0]);
        assert_eq!(obj.scale, 1.0);
        assert!(obj.material.is_none());
    }

    #[test]
    fn test_obj_instance_is_a() {
        let obj = ObjInstance::default();
        assert!(obj.is_a("Object"));
        assert!(!obj.is_a("SomethingElse"));
    }

    #[test]
    fn test_obj_instance_is_a_with_parent() {
        use crate::ast::QualifiedName;

        let parent_class = Rc::new(ClassValue {
            name: "Parent".to_string(),
            package: String::new(),
            parent: None,
            methods: HashMap::new(),
            rules: HashMap::new(),
            static_vars: HashMap::new(),
            decl: ClassDecl {
                modifiers: vec![],
                name: "Parent".to_string(),
                parent: None,
                members: vec![],
                span: crate::ast::Span::default(),
            },
        });

        let child_class = Rc::new(ClassValue {
            name: "Child".to_string(),
            package: String::new(),
            parent: Some(parent_class),
            methods: HashMap::new(),
            rules: HashMap::new(),
            static_vars: HashMap::new(),
            decl: ClassDecl {
                modifiers: vec![],
                name: "Child".to_string(),
                parent: Some(QualifiedName {
                    absolute: false,
                    parts: vec!["Parent".to_string()],
                    span: crate::ast::Span::default(),
                }),
                members: vec![],
                span: crate::ast::Span::default(),
            },
        });

        let mut obj = ObjInstance::default();
        obj.class = child_class;

        assert!(obj.is_a("Child"));
        assert!(obj.is_a("Parent"));
        assert!(!obj.is_a("Grandparent"));
    }

    #[test]
    fn test_obj_instance_properties() {
        let mut obj = ObjInstance::default();

        // Get non-existent property returns Null
        assert!(matches!(obj.get_prop_value("color"), Value::Null));

        // Set and get property
        obj.set_prop_value("color", Value::String(Rc::new("red".to_string())));
        let color = obj.get_prop_value("color");
        assert_eq!(color.to_string_val(), "red");

        // Overwrite property
        obj.set_prop_value("color", Value::String(Rc::new("blue".to_string())));
        let color = obj.get_prop_value("color");
        assert_eq!(color.to_string_val(), "blue");
    }

    #[test]
    fn test_obj_instance_get_position() {
        let mut obj = ObjInstance::default();
        obj.position = [1.0, 2.0, 3.0];

        let pos = obj.get_position();
        if let Value::Array(arr) = pos {
            let arr = arr.borrow();
            assert_eq!(arr.len(), 3);
            assert_eq!(arr[0].to_float(), Some(1.0));
            assert_eq!(arr[1].to_float(), Some(2.0));
            assert_eq!(arr[2].to_float(), Some(3.0));
        } else {
            panic!("Expected Array");
        }
    }

    #[test]
    fn test_obj_instance_set_position_array() {
        let mut obj = ObjInstance::default();

        let pos = Value::Array(Rc::new(RefCell::new(vec![
            Value::Float(10.0),
            Value::Float(20.0),
            Value::Float(30.0),
        ])));

        assert!(obj.set_position(&pos).is_ok());
        assert_eq!(obj.position, [10.0, 20.0, 30.0]);
    }

    #[test]
    fn test_obj_instance_set_position_vec3() {
        let mut obj = ObjInstance::default();

        let pos = Value::Vec3([5.0, 6.0, 7.0]);
        assert!(obj.set_position(&pos).is_ok());
        assert_eq!(obj.position, [5.0, 6.0, 7.0]);
    }

    #[test]
    fn test_obj_instance_set_position_invalid() {
        let mut obj = ObjInstance::default();

        // Wrong type
        let err = obj.set_position(&Value::Int(1));
        assert!(err.is_err());

        // Array too short
        let short = Value::Array(Rc::new(RefCell::new(vec![Value::Float(1.0)])));
        let err = obj.set_position(&short);
        assert!(err.is_err());
    }

    // ============== FuncValue tests ==============

    #[test]
    fn test_func_value_clone() {
        let fv = FuncValue {
            name: "test_func".to_string(),
            params: vec!["a".to_string(), "b".to_string()],
            body: None,
            class: None,
            is_static: false,
        };

        let fv2 = fv.clone();
        assert_eq!(fv2.name, "test_func");
        assert_eq!(fv2.params.len(), 2);
        assert!(!fv2.is_static);
    }

    // ============== Value type conversion tests ==============

    #[test]
    fn test_value_object_display() {
        let obj = ObjInstance {
            name: "TestObj".to_string(),
            ..Default::default()
        };
        let v = Value::Object(Rc::new(RefCell::new(obj)));
        assert_eq!(v.to_string_val(), "[Object:TestObj]");
        assert_eq!(v.type_name(), "Object");
    }

    #[test]
    fn test_value_func_display() {
        let func = FuncValue {
            name: "myFunc".to_string(),
            params: vec![],
            body: None,
            class: None,
            is_static: false,
        };
        let v = Value::Func(Rc::new(func));
        assert_eq!(v.to_string_val(), "[Func:myFunc]");
        assert_eq!(v.type_name(), "Func");
    }

    #[test]
    fn test_value_class_display() {
        let class = ClassValue {
            name: "TestClass".to_string(),
            package: String::new(),
            parent: None,
            methods: HashMap::new(),
            rules: HashMap::new(),
            static_vars: HashMap::new(),
            decl: ClassDecl {
                modifiers: vec![],
                name: "TestClass".to_string(),
                parent: None,
                members: vec![],
                span: crate::ast::Span::default(),
            },
        };
        let v = Value::Class(Rc::new(class));
        assert_eq!(v.to_string_val(), "[Class:TestClass]");
        assert_eq!(v.type_name(), "Class");
    }

    #[test]
    fn test_value_float_to_string_val() {
        let v = Value::Float(3.14);
        assert_eq!(v.to_string_val(), "3.14");

        let whole = Value::Float(100.0);
        assert_eq!(whole.to_string_val(), "100");
    }

    #[test]
    fn test_value_native_func_display() {
        // NativeFunc wraps an Rc<NativeFn> which is a Fn trait object
        // Create a simple native function
        let native_fn: Rc<NativeFn> = Rc::new(|_interp, _args| Ok(Value::Null));
        let v = Value::NativeFunc(native_fn);
        assert_eq!(v.to_string_val(), "[NativeFunc]");
        assert_eq!(v.type_name(), "NativeFunc");
    }
}
