//! OFML Interpreter - Expression evaluation and statement execution

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use crate::ast::*;
use crate::env::Environment;
use crate::scene::{AlignMode, Axis, SceneGraph, SceneNode};
use crate::value::*;

/// Interpreter result
pub type InterpResult<T> = Result<T, InterpError>;

/// Interpreter error
#[derive(Debug, Clone)]
pub struct InterpError {
    pub message: String,
    pub kind: ErrorKind,
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Runtime,
    TypeError,
    NameError,
    Return(Value),
    Break,
    Continue,
}

impl InterpError {
    pub fn runtime(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
            kind: ErrorKind::Runtime,
        }
    }
    pub fn type_error(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
            kind: ErrorKind::TypeError,
        }
    }
    pub fn name_error(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
            kind: ErrorKind::NameError,
        }
    }
}

impl std::fmt::Display for InterpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for InterpError {}

impl From<String> for InterpError {
    fn from(s: String) -> Self {
        Self::runtime(s)
    }
}

/// The OFML interpreter
pub struct Interpreter {
    /// Variable environment
    pub env: Environment,
    /// Registered classes by short name (for backwards compatibility)
    pub classes: HashMap<String, Rc<ClassValue>>,
    /// Registered classes by fully qualified name (e.g., "::vitra::basics::VitraOiBTGPlElement3")
    pub qualified_classes: HashMap<String, Rc<ClassValue>>,
    /// Current package context (e.g., "::vitra::basics")
    pub current_package: String,
    /// Current 'self' object for method calls
    pub current_self: Option<Rc<RefCell<ObjInstance>>>,
    /// Output buffer for debugging
    pub output: Vec<String>,
    /// Scene graph for 3D objects
    pub scene: SceneGraph,
    /// Mapping from object instances to scene nodes
    pub obj_to_node: HashMap<u64, Rc<RefCell<SceneNode>>>,
    /// Next object ID
    next_obj_id: u64,
    /// Current ALB path (for product database lookups)
    pub current_alb_path: Option<PathBuf>,
    /// Call depth counter to prevent stack overflow
    call_depth: usize,
    /// Total operation counter to prevent infinite execution
    operation_count: usize,
}

impl Interpreter {
    /// Create a new interpreter
    pub fn new() -> Self {
        let mut interp = Self {
            env: Environment::new(),
            classes: HashMap::new(),
            qualified_classes: HashMap::new(),
            current_package: String::new(),
            current_self: None,
            output: Vec::new(),
            scene: SceneGraph::new(),
            obj_to_node: HashMap::new(),
            next_obj_id: 1,
            current_alb_path: None,
            call_depth: 0,
            operation_count: 0,
        };
        interp.register_builtins();
        interp.register_ofml_classes();
        interp.register_xoi_classes();
        interp
    }

    /// Set the ALB path for geometry loading
    pub fn set_alb_path(&mut self, path: PathBuf) {
        self.scene.set_alb_path(&path);
    }

    /// Get the next object ID
    #[allow(dead_code)]
    fn next_obj_id(&mut self) -> u64 {
        let id = self.next_obj_id;
        self.next_obj_id += 1;
        id
    }

    /// Register native OFML classes
    fn register_ofml_classes(&mut self) {
        // Mathematical constants
        self.env
            .define_global("sPi", Value::Float(std::f64::consts::PI));
        self.env
            .define_global("sE", Value::Float(std::f64::consts::E));

        // Math functions
        self.env.define_global(
            "floor",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let val = args.first().and_then(|v| v.to_float()).unwrap_or(0.0);
                Ok(Value::Int(val.floor() as i64))
            })),
        );
        self.env.define_global(
            "ceil",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let val = args.first().and_then(|v| v.to_float()).unwrap_or(0.0);
                Ok(Value::Int(val.ceil() as i64))
            })),
        );
        self.env.define_global(
            "round",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let val = args.first().and_then(|v| v.to_float()).unwrap_or(0.0);
                Ok(Value::Int(val.round() as i64))
            })),
        );
        self.env.define_global(
            "abs",
            Value::NativeFunc(Rc::new(|_interp, args| match args.first() {
                Some(Value::Int(n)) => Ok(Value::Int(n.abs())),
                Some(Value::Float(f)) => Ok(Value::Float(f.abs())),
                _ => Ok(Value::Int(0)),
            })),
        );
        self.env.define_global(
            "sqrt",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let val = args.first().and_then(|v| v.to_float()).unwrap_or(0.0);
                Ok(Value::Float(val.sqrt()))
            })),
        );
        self.env.define_global(
            "pow",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let base = args.first().and_then(|v| v.to_float()).unwrap_or(0.0);
                let exp = args.get(1).and_then(|v| v.to_float()).unwrap_or(0.0);
                Ok(Value::Float(base.powf(exp)))
            })),
        );
        self.env.define_global(
            "sin",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let val = args.first().and_then(|v| v.to_float()).unwrap_or(0.0);
                Ok(Value::Float(val.sin()))
            })),
        );
        self.env.define_global(
            "cos",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let val = args.first().and_then(|v| v.to_float()).unwrap_or(0.0);
                Ok(Value::Float(val.cos()))
            })),
        );
        self.env.define_global(
            "atan2",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let y = args.first().and_then(|v| v.to_float()).unwrap_or(0.0);
                let x = args.get(1).and_then(|v| v.to_float()).unwrap_or(1.0);
                Ok(Value::Float(y.atan2(x)))
            })),
        );

        // Register OiObject base class
        let oi_object = self.create_native_class("OiObject", None);
        self.classes
            .insert("OiObject".to_string(), oi_object.clone());
        self.env
            .define_global("OiObject", Value::Class(oi_object.clone()));

        // Register OiPart class (extends OiObject)
        let oi_part = self.create_native_class("OiPart", Some(oi_object.clone()));
        self.classes.insert("OiPart".to_string(), oi_part.clone());
        self.env
            .define_global("OiPart", Value::Class(oi_part.clone()));

        // Register OiGeometry class (extends OiObject)
        let oi_geometry = self.create_native_class("OiGeometry", Some(oi_object.clone()));
        self.classes
            .insert("OiGeometry".to_string(), oi_geometry.clone());
        self.env
            .define_global("OiGeometry", Value::Class(oi_geometry.clone()));

        // Register OiBlock class (extends OiGeometry)
        let oi_block = self.create_native_class("OiBlock", Some(oi_geometry.clone()));
        self.classes.insert("OiBlock".to_string(), oi_block.clone());
        self.env
            .define_global("OiBlock", Value::Class(oi_block.clone()));

        // Register xxPart class (base for OiPart)
        let xx_part = self.create_native_class("xxPart", Some(oi_object.clone()));
        self.classes.insert("xxPart".to_string(), xx_part.clone());
        self.env
            .define_global("xxPart", Value::Class(xx_part.clone()));

        // Register Top class (base for OiGeometry in some hierarchies)
        let top = self.create_native_class("Top", Some(oi_object.clone()));
        self.classes.insert("Top".to_string(), top.clone());
        self.env.define_global("Top", Value::Class(top.clone()));

        // Register OiImport class (extends OiGeometry, loads 3DS/OFF files)
        let oi_import = self.create_native_class("OiImport", Some(oi_geometry.clone()));
        self.classes
            .insert("OiImport".to_string(), oi_import.clone());
        self.env
            .define_global("OiImport", Value::Class(oi_import.clone()));

        // Register OiCylinder class (extends OiGeometry)
        let oi_cylinder = self.create_native_class("OiCylinder", Some(oi_geometry.clone()));
        self.classes
            .insert("OiCylinder".to_string(), oi_cylinder.clone());
        self.env
            .define_global("OiCylinder", Value::Class(oi_cylinder.clone()));

        // Register OiSphere class (extends OiGeometry)
        let oi_sphere = self.create_native_class("OiSphere", Some(oi_geometry.clone()));
        self.classes
            .insert("OiSphere".to_string(), oi_sphere.clone());
        self.env
            .define_global("OiSphere", Value::Class(oi_sphere.clone()));

        // Register OiSurface class (extends OiGeometry) - for surface primitives
        let oi_surface = self.create_native_class("OiSurface", Some(oi_geometry.clone()));
        self.classes
            .insert("OiSurface".to_string(), oi_surface.clone());
        self.env
            .define_global("OiSurface", Value::Class(oi_surface.clone()));

        // Register OiEllipsoid class (extends OiGeometry)
        let oi_ellipsoid = self.create_native_class("OiEllipsoid", Some(oi_geometry.clone()));
        self.classes
            .insert("OiEllipsoid".to_string(), oi_ellipsoid.clone());
        self.env
            .define_global("OiEllipsoid", Value::Class(oi_ellipsoid.clone()));

        // Register OiPolygon class (extends OiGeometry)
        let oi_polygon = self.create_native_class("OiPolygon", Some(oi_geometry.clone()));
        self.classes
            .insert("OiPolygon".to_string(), oi_polygon.clone());
        self.env
            .define_global("OiPolygon", Value::Class(oi_polygon.clone()));

        // Register OiFrame class (extends OiGeometry)
        let oi_frame = self.create_native_class("OiFrame", Some(oi_geometry.clone()));
        self.classes.insert("OiFrame".to_string(), oi_frame.clone());
        self.env
            .define_global("OiFrame", Value::Class(oi_frame.clone()));

        // Register OiRotation class (extends OiGeometry)
        let oi_rotation = self.create_native_class("OiRotation", Some(oi_geometry.clone()));
        self.classes
            .insert("OiRotation".to_string(), oi_rotation.clone());
        self.env
            .define_global("OiRotation", Value::Class(oi_rotation.clone()));

        // Register OiSweep class (extends OiGeometry)
        let oi_sweep = self.create_native_class("OiSweep", Some(oi_geometry.clone()));
        self.classes.insert("OiSweep".to_string(), oi_sweep.clone());
        self.env
            .define_global("OiSweep", Value::Class(oi_sweep.clone()));

        // Register xOiPlGroup class (extends OiPart)
        let x_oi_plgroup = self.create_native_class("xOiPlGroup", Some(oi_part.clone()));
        self.classes
            .insert("xOiPlGroup".to_string(), x_oi_plgroup.clone());
        self.env
            .define_global("xOiPlGroup", Value::Class(x_oi_plgroup.clone()));

        // Register xOiJointPlGroup class (extends xOiPlGroup)
        let x_oi_joint_plgroup =
            self.create_native_class("xOiJointPlGroup", Some(x_oi_plgroup.clone()));
        self.classes
            .insert("xOiJointPlGroup".to_string(), x_oi_joint_plgroup.clone());
        self.env
            .define_global("xOiJointPlGroup", Value::Class(x_oi_joint_plgroup));

        // Register GoMetaType class (extends OiPart) - for configurator products
        let go_meta_type = self.create_native_class("GoMetaType", Some(oi_part.clone()));
        self.classes
            .insert("GoMetaType".to_string(), go_meta_type.clone());
        self.env
            .define_global("GoMetaType", Value::Class(go_meta_type));

        // oiExists - check if object exists by name
        self.env.define_global(
            "oiExists",
            Value::NativeFunc(Rc::new(|interp, args| {
                let name = args.first().map(|v| v.to_string_val()).unwrap_or_default();
                Ok(Value::Bool(interp.scene.exists(&name)))
            })),
        );

        // remove - remove object
        self.env.define_global(
            "remove",
            Value::NativeFunc(Rc::new(|interp, args| {
                if let Some(Value::Object(obj)) = args.first() {
                    let obj_id = obj.as_ptr() as u64;
                    if let Some(node) = interp.obj_to_node.remove(&obj_id) {
                        let node_id = node.borrow().id;
                        interp.scene.remove_node(node_id);
                    }
                }
                Ok(Value::Null)
            })),
        );

        // getName - get object name
        self.env.define_global(
            "getName",
            Value::NativeFunc(Rc::new(|interp, _args| {
                if let Some(ref instance) = interp.current_self {
                    Ok(Value::String(Rc::new(instance.borrow().name.clone())))
                } else {
                    Ok(Value::Null)
                }
            })),
        );

        // xOiDebugStartFunc / xOiDebugFinishFunc / xOiDebugPrint - debug stubs
        self.env.define_global(
            "xOiDebugStartFunc",
            Value::NativeFunc(Rc::new(|_interp, _args| Ok(Value::Null))),
        );
        self.env.define_global(
            "xOiDebugFinishFunc",
            Value::NativeFunc(Rc::new(|_interp, _args| Ok(Value::Null))),
        );
        self.env.define_global(
            "xOiDebugPrint",
            Value::NativeFunc(Rc::new(|_interp, _args| Ok(Value::Null))),
        );
        self.env.define_global(
            "oiDebugStartFunc",
            Value::NativeFunc(Rc::new(|_interp, _args| Ok(Value::Null))),
        );
        self.env.define_global(
            "oiDebugFinishFunc",
            Value::NativeFunc(Rc::new(|_interp, _args| Ok(Value::Null))),
        );
        self.env.define_global(
            "oiDebugPrint",
            Value::NativeFunc(Rc::new(|_interp, _args| Ok(Value::Null))),
        );
        self.env.define_global(
            "afError",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let msg = args.first().map(|v| v.to_string_val()).unwrap_or_default();
                eprintln!("afError: {}", msg);
                Ok(Value::Null)
            })),
        );

        // oiRegisterPackage - Register a package (stub, always returns 1/success)
        self.env.define_global(
            "oiRegisterPackage",
            Value::NativeFunc(Rc::new(|_interp, _args| Ok(Value::Int(1)))),
        );

        // xOiGetPackage - Get package name from object's type
        self.env.define_global(
            "xOiGetPackage",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let obj = args.first().cloned().unwrap_or(Value::Null);
                match obj {
                    Value::Object(instance) => {
                        let class = &instance.borrow().class;
                        Ok(Value::String(Rc::new(class.package.clone())))
                    }
                    Value::Class(class) => Ok(Value::String(Rc::new(class.package.clone()))),
                    _ => Ok(Value::Null),
                }
            })),
        );

        // isInTP - Check if object is in a planning group (stub, returns NULL for now)
        self.env.define_global(
            "isInTP",
            Value::NativeFunc(Rc::new(|_interp, _args| Ok(Value::Null))),
        );

        // xOiPackage2ProgID - Convert package name to program ID (e.g., "::vitra::basics" -> @vitra_basics)
        self.env.define_global(
            "xOiPackage2ProgID",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let pkg = match args.first() {
                    Some(Value::String(s)) => s.to_string(),
                    _ => return Ok(Value::Null),
                };
                // Extract manufacturer and series from package name like "::vitra::basics"
                let parts: Vec<&str> = pkg.trim_start_matches("::").split("::").collect();
                if parts.len() >= 2 {
                    let prog_id = format!("{}_{}", parts[0], parts[1]);
                    Ok(Value::Symbol(Rc::new(prog_id)))
                } else if parts.len() == 1 && !parts[0].is_empty() {
                    Ok(Value::Symbol(Rc::new(parts[0].to_string())))
                } else {
                    Ok(Value::Null)
                }
            })),
        );

        // xOiProgID2Manufacturer - Extract manufacturer from program ID (e.g., @vitra_basics -> "vitra")
        self.env.define_global(
            "xOiProgID2Manufacturer",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let pid = match args.first() {
                    Some(Value::Symbol(s)) => s.to_string(),
                    _ => return Ok(Value::String(Rc::new(String::new()))),
                };
                // Find underscore and return part before it
                if let Some(pos) = pid.find('_') {
                    Ok(Value::String(Rc::new(pid[..pos].to_string())))
                } else {
                    Ok(Value::String(Rc::new(pid)))
                }
            })),
        );

        // xOiProgID2Series - Extract series from program ID (e.g., @vitra_basics -> "basics")
        self.env.define_global(
            "xOiProgID2Series",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let pid = match args.first() {
                    Some(Value::Symbol(s)) => s.to_string(),
                    _ => return Ok(Value::String(Rc::new(String::new()))),
                };
                // Find underscore and return part after it
                if let Some(pos) = pid.find('_') {
                    Ok(Value::String(Rc::new(pid[pos + 1..].to_string())))
                } else {
                    Ok(Value::String(Rc::new(String::new())))
                }
            })),
        );

        // xOiCreateArticle / xOiCreateArticle2 - Create an article (stub, returns NULL)
        self.env.define_global(
            "xOiCreateArticle",
            Value::NativeFunc(Rc::new(|_interp, _args| Ok(Value::Null))),
        );
        self.env.define_global(
            "xOiCreateArticle2",
            Value::NativeFunc(Rc::new(|_interp, _args| Ok(Value::Null))),
        );

        // article2Class - Map article to class (stub, returns NULL)
        self.env.define_global(
            "article2Class",
            Value::NativeFunc(Rc::new(|_interp, _args| Ok(Value::Null))),
        );

        // Block(parent, @name, [width, height, depth]) - creates a block primitive
        // This is a NATIVE function called by OiBlock::initialize
        self.env.define_global(
            "Block",
            Value::NativeFunc(Rc::new(|interp, args| {
                // Args: parent_object, @name_symbol, [width, height, depth]
                let parent_obj = match args.first() {
                    Some(Value::Object(obj)) => Some(obj.clone()),
                    _ => None,
                };

                let name = match args.get(1) {
                    Some(Value::Symbol(s)) => s.to_string(),
                    Some(v) => v.to_string_val(),
                    None => "geo".to_string(),
                };

                let dims = match args.get(2) {
                    Some(Value::Array(arr)) => {
                        let arr = arr.borrow();
                        [
                            arr.first().and_then(|v| v.to_float()).unwrap_or(1.0) as f32,
                            arr.get(1).and_then(|v| v.to_float()).unwrap_or(1.0) as f32,
                            arr.get(2).and_then(|v| v.to_float()).unwrap_or(1.0) as f32,
                        ]
                    }
                    _ => [1.0, 1.0, 1.0],
                };

                // Get parent scene node if parent object exists
                let parent_node = parent_obj.as_ref().and_then(|p| {
                    let obj_id = p.as_ptr() as u64;
                    interp.obj_to_node.get(&obj_id).cloned()
                });

                // Create scene node with block geometry
                let node = interp.scene.create_block(name.clone(), dims, parent_node);

                // Create an object instance for the block
                let block_class = interp
                    .classes
                    .get("OiBlock")
                    .cloned()
                    .unwrap_or_else(|| interp.create_native_class("OiBlock", None));

                let instance = Rc::new(RefCell::new(ObjInstance {
                    class: block_class,
                    name: name.clone(),
                    ..Default::default()
                }));

                // Link to parent
                if let Some(ref parent) = parent_obj {
                    instance.borrow_mut().parent = Some(parent.clone());
                    parent.borrow_mut().children.push(instance.clone());
                    parent
                        .borrow_mut()
                        .fields
                        .insert(name.clone(), Value::Object(instance.clone()));
                }

                // Map instance to scene node
                let obj_id = instance.as_ptr() as u64;
                interp.obj_to_node.insert(obj_id, node);

                Ok(Value::Object(instance))
            })),
        );

        // OffPolygon(parent, @name, "filename") - loads OFF/3DS geometry from file
        // This is the core native for loading 3DS geometry from ALB archives
        self.env.define_global(
            "OffPolygon",
            Value::NativeFunc(Rc::new(|interp, args| {
                // Args: parent_object, @name_symbol, "filename"
                let parent_obj = match args.first() {
                    Some(Value::Object(obj)) => Some(obj.clone()),
                    _ => None,
                };

                let name = match args.get(1) {
                    Some(Value::Symbol(s)) => s.to_string(),
                    Some(v) => v.to_string_val(),
                    None => "geo".to_string(),
                };

                let filename = match args.get(2) {
                    Some(Value::String(s)) => s.to_string(),
                    Some(v) => v.to_string_val(),
                    None => return Ok(Value::Null),
                };

                // Get parent scene node
                let parent_node = parent_obj.as_ref().and_then(|p| {
                    let obj_id = p.as_ptr() as u64;
                    interp.obj_to_node.get(&obj_id).cloned()
                });

                // Create scene node with 3DS geometry
                let node = interp.scene.create_part(name.clone(), parent_node);

                // Load geometry from ALB if path is set
                if let Some(ref alb_path) = interp.scene.alb_path.clone() {
                    if let Err(e) =
                        interp
                            .scene
                            .load_3ds_from_alb(node.clone(), alb_path, &filename)
                    {
                        eprintln!("OffPolygon: Failed to load '{}': {}", filename, e);
                    }
                } else {
                    eprintln!("OffPolygon: No ALB path set, cannot load '{}'", filename);
                }

                // Create object instance
                let geo_class = interp
                    .classes
                    .get("OiGeometry")
                    .cloned()
                    .unwrap_or_else(|| interp.create_native_class("OiGeometry", None));

                let instance = Rc::new(RefCell::new(ObjInstance {
                    class: geo_class,
                    name: name.clone(),
                    ..Default::default()
                }));

                // Link to parent
                if let Some(ref parent) = parent_obj {
                    instance.borrow_mut().parent = Some(parent.clone());
                    parent.borrow_mut().children.push(instance.clone());
                    parent
                        .borrow_mut()
                        .fields
                        .insert(name.clone(), Value::Object(instance.clone()));
                }

                // Map instance to scene node
                let obj_id = instance.as_ptr() as u64;
                interp.obj_to_node.insert(obj_id, node);

                Ok(Value::Object(instance))
            })),
        );

        // Geo(parent, @name, "filename.3ds") - creates geometry from external 3DS file
        // Alias for OffPolygon for compatibility
        self.env.define_global(
            "Geo",
            Value::NativeFunc(Rc::new(|interp, args| {
                // Args: parent_object, @name_symbol, "pattern"
                let parent_obj = match args.first() {
                    Some(Value::Object(obj)) => Some(obj.clone()),
                    _ => None,
                };

                let name = match args.get(1) {
                    Some(Value::Symbol(s)) => s.to_string(),
                    Some(v) => v.to_string_val(),
                    None => "geo".to_string(),
                };

                let pattern = match args.get(2) {
                    Some(Value::String(s)) => s.to_string(),
                    Some(v) => v.to_string_val(),
                    None => return Ok(Value::Null),
                };

                // Get parent scene node
                let parent_node = parent_obj.as_ref().and_then(|p| {
                    let obj_id = p.as_ptr() as u64;
                    interp.obj_to_node.get(&obj_id).cloned()
                });

                // Create scene node (initially without geometry)
                let node = interp.scene.create_part(name.clone(), parent_node);

                // Try to load geometry from ALB
                if let Err(e) = interp.scene.load_geometry(node.clone(), &pattern) {
                    eprintln!("Geo: Failed to load '{}': {}", pattern, e);
                }

                // Create object instance
                let geo_class = interp
                    .classes
                    .get("OiGeometry")
                    .cloned()
                    .unwrap_or_else(|| interp.create_native_class("OiGeometry", None));

                let instance = Rc::new(RefCell::new(ObjInstance {
                    class: geo_class,
                    name: name.clone(),
                    ..Default::default()
                }));

                // Link to parent
                if let Some(ref parent) = parent_obj {
                    instance.borrow_mut().parent = Some(parent.clone());
                    parent.borrow_mut().children.push(instance.clone());
                    parent
                        .borrow_mut()
                        .fields
                        .insert(name.clone(), Value::Object(instance.clone()));
                }

                // Map instance to scene node
                let obj_id = instance.as_ptr() as u64;
                interp.obj_to_node.insert(obj_id, node);

                Ok(Value::Object(instance))
            })),
        );

        // acos, asin, atan - math functions
        self.env.define_global(
            "acos",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let val = args.first().and_then(|v| v.to_float()).unwrap_or(0.0);
                Ok(Value::Float(val.acos()))
            })),
        );
        self.env.define_global(
            "asin",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let val = args.first().and_then(|v| v.to_float()).unwrap_or(0.0);
                Ok(Value::Float(val.asin()))
            })),
        );
        self.env.define_global(
            "atan",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let val = args.first().and_then(|v| v.to_float()).unwrap_or(0.0);
                Ok(Value::Float(val.atan()))
            })),
        );
        self.env.define_global(
            "cos",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let val = args.first().and_then(|v| v.to_float()).unwrap_or(0.0);
                Ok(Value::Float(val.cos()))
            })),
        );
        self.env.define_global(
            "sin",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let val = args.first().and_then(|v| v.to_float()).unwrap_or(0.0);
                Ok(Value::Float(val.sin()))
            })),
        );
        self.env.define_global(
            "tan",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let val = args.first().and_then(|v| v.to_float()).unwrap_or(0.0);
                Ok(Value::Float(val.tan()))
            })),
        );
        self.env.define_global(
            "sqrt",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let val = args.first().and_then(|v| v.to_float()).unwrap_or(0.0);
                Ok(Value::Float(val.sqrt()))
            })),
        );
        self.env.define_global(
            "abs",
            Value::NativeFunc(Rc::new(|_interp, args| match args.first() {
                Some(Value::Int(n)) => Ok(Value::Int(n.abs())),
                Some(Value::Float(f)) => Ok(Value::Float(f.abs())),
                _ => Ok(Value::Float(0.0)),
            })),
        );

        // fabs - float absolute value (common in OFML)
        self.env.define_global(
            "fabs",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let val = args.first().and_then(|v| v.to_float()).unwrap_or(0.0);
                Ok(Value::Float(val.abs()))
            })),
        );

        // Mod - modulo function returning [quotient, remainder]
        self.env.define_global(
            "Mod",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let a = args.first().and_then(|v| v.to_float()).unwrap_or(0.0);
                let b = args.get(1).and_then(|v| v.to_float()).unwrap_or(1.0);
                let quotient = (a / b).floor();
                let remainder = a - (quotient * b);
                Ok(Value::Array(Rc::new(RefCell::new(vec![
                    Value::Float(quotient),
                    Value::Float(remainder),
                ]))))
            })),
        );

        // getPDManager - returns a property definition manager object
        // The PDManager provides access to product databases
        self.env.define_global(
            "getPDManager",
            Value::NativeFunc(Rc::new(|interp, _args| {
                // Create a PDManager object that can be used to access product databases
                let pdm_class = interp
                    .classes
                    .get("PDManager")
                    .cloned()
                    .unwrap_or_else(|| interp.create_native_class("PDManager", None));

                let instance = Rc::new(RefCell::new(ObjInstance {
                    class: pdm_class,
                    name: "PDManager".to_string(),
                    ..Default::default()
                }));

                Ok(Value::Object(instance))
            })),
        );

        // Symbol - create a symbol from a string
        self.env.define_global(
            "Symbol",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let s = args.first().map(|v| v.to_string_val()).unwrap_or_default();
                Ok(Value::Symbol(Rc::new(s)))
            })),
        );

        // setInsertMode - sets the insertion mode for planning groups (xOiPlGroup)
        // Valid modes: 0 (default/free), 1 (intermediate), 2 (inner element)
        // Note: This is NOT a native OiObject method, but used by xOiPlGroup/xOiPlanning
        self.env.define_global(
            "setInsertMode",
            Value::NativeFunc(Rc::new(|interp, args| {
                let mode = args.first().and_then(|v| v.to_int()).unwrap_or(0);
                // Validate mode is 0, 1, or 2 (as per xOiPlGroup::setInsertMode)
                let valid_mode = if (0..=2).contains(&mode) { mode } else { 0 };
                if let Some(ref instance) = interp.current_self {
                    instance
                        .borrow_mut()
                        .properties
                        .insert("mXoiInsertMode".to_string(), Value::Int(valid_mode));
                }
                Ok(Value::Null)
            })),
        );

        // getInsertMode - gets the insertion mode (defaults to 0 if NULL)
        self.env.define_global(
            "getInsertMode",
            Value::NativeFunc(Rc::new(|interp, _args| {
                if let Some(ref instance) = interp.current_self {
                    if let Some(mode) = instance.borrow().properties.get("mXoiInsertMode") {
                        if !matches!(mode, Value::Null) {
                            return Ok(mode.clone());
                        }
                    }
                }
                Ok(Value::Int(0)) // Default mode per xOiPlGroup::getInsertMode
            })),
        );

        // setResolution - sets object space resolution for parametric primitives (OFML spec 4.2.5)
        // Float value in range 0.0 (min detail) to 1.0 (max detail), initial: 0.1
        // Returns self for method chaining
        self.env.define_global(
            "setResolution",
            Value::NativeFunc(Rc::new(|interp, args| {
                let resolution = args.first().and_then(|v| v.to_float()).unwrap_or(0.1);
                // Clamp to valid range 0.0 to 1.0
                let clamped = resolution.clamp(0.0, 1.0);
                if let Some(ref instance) = interp.current_self {
                    instance
                        .borrow_mut()
                        .properties
                        .insert("_resolution".to_string(), Value::Float(clamped));
                    // Return self for method chaining (per OFML spec)
                    return Ok(Value::Object(instance.clone()));
                }
                Ok(Value::Null)
            })),
        );

        // getResolution - gets the object space resolution (Float)
        // Returns inherited resolution or 0.1 (initial default per OFML spec)
        self.env.define_global(
            "getResolution",
            Value::NativeFunc(Rc::new(|interp, _args| {
                if let Some(ref instance) = interp.current_self {
                    if let Some(res) = instance.borrow().properties.get("_resolution") {
                        return Ok(res.clone());
                    }
                }
                Ok(Value::Float(0.1)) // Initial resolution per OFML spec
            })),
        );

        // disableCD - disables collision detection (no-op for interpreter)
        self.env.define_global(
            "disableCD",
            Value::NativeFunc(Rc::new(|_interp, _args| {
                // Collision detection is not implemented
                Ok(Value::Null)
            })),
        );

        // enableCD - enables collision detection (no-op for interpreter)
        self.env.define_global(
            "enableCD",
            Value::NativeFunc(Rc::new(|_interp, _args| {
                // Collision detection is not implemented
                Ok(Value::Null)
            })),
        );

        // xOiObjInList - Check if object is in list (returns 0 or 1)
        self.env.define_global(
            "xOiObjInList",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let obj = args.first().cloned().unwrap_or(Value::Null);
                let list = args.get(1).cloned().unwrap_or(Value::Null);

                if let Value::Array(arr) = list {
                    for item in arr.borrow().iter() {
                        if item.equals(&obj) {
                            return Ok(Value::Int(1));
                        }
                    }
                }
                Ok(Value::Int(0))
            })),
        );

        // xOiParseString2List - Parse string into array by separator
        self.env.define_global(
            "xOiParseString2List",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let str_val = args.first().cloned().unwrap_or(Value::Null);
                let sep = args.get(1).cloned().unwrap_or(Value::Null);

                if matches!(str_val, Value::Null) {
                    return Ok(Value::Null);
                }

                let s = str_val.to_string_val();
                let separator = match &sep {
                    Value::String(sep_str) => sep_str.to_string(),
                    Value::Int(ch) => char::from_u32(*ch as u32)
                        .map(|c| c.to_string())
                        .unwrap_or(",".to_string()),
                    _ => ",".to_string(),
                };

                let parts: Vec<Value> = s
                    .split(&separator)
                    .map(|p| Value::String(Rc::new(p.to_string())))
                    .collect();

                Ok(Value::Array(Rc::new(RefCell::new(parts))))
            })),
        );

        // xOiList2Hash - Convert list of [key, value] pairs to hash
        self.env.define_global(
            "xOiList2Hash",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let list = args.first().cloned().unwrap_or(Value::Null);

                if matches!(list, Value::Null) {
                    return Ok(Value::Null);
                }

                let mut hash = HashMap::new();

                if let Value::Array(arr) = list {
                    for entry in arr.borrow().iter() {
                        if let Value::Array(pair) = entry {
                            let pair = pair.borrow();
                            if pair.len() >= 2 {
                                let key = pair[0].to_string_val();
                                let value = pair[1].clone();
                                hash.insert(key, value);
                            }
                        }
                    }
                }

                Ok(Value::Hash(Rc::new(RefCell::new(hash))))
            })),
        );

        // isNullOrEmpty - Check if value is null or empty
        self.env.define_global(
            "isNullOrEmpty",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let val = args.first().cloned().unwrap_or(Value::Null);
                let is_empty = match &val {
                    Value::Null => true,
                    Value::String(s) => s.is_empty(),
                    Value::Array(arr) => arr.borrow().is_empty(),
                    Value::Hash(h) => h.borrow().is_empty(),
                    _ => false,
                };
                Ok(Value::Bool(is_empty))
            })),
        );

        // new2DObj - Create a 2D object (stub - returns null for now)
        self.env.define_global(
            "new2DObj",
            Value::NativeFunc(Rc::new(|_interp, _args| {
                // 2D objects not implemented
                Ok(Value::Null)
            })),
        );

        // getMTChildren - Get metatype children (stub)
        self.env.define_global(
            "getMTChildren",
            Value::NativeFunc(Rc::new(|_interp, _args| {
                // Return empty array
                Ok(Value::Array(Rc::new(RefCell::new(Vec::new()))))
            })),
        );

        // initializeFrame - Initialize frame (stub)
        self.env.define_global(
            "initializeFrame",
            Value::NativeFunc(Rc::new(|_interp, _args| Ok(Value::Null))),
        );

        // getPropKeys - Get property keys (alias for getPropertyKeys)
        self.env.define_global(
            "getPropKeys",
            Value::NativeFunc(Rc::new(|interp, _args| {
                if let Some(ref instance) = interp.current_self {
                    let keys: Vec<Value> = instance
                        .borrow()
                        .properties
                        .keys()
                        .map(|k| Value::Symbol(Rc::new(k.clone())))
                        .collect();
                    return Ok(Value::Array(Rc::new(RefCell::new(keys))));
                }
                Ok(Value::Array(Rc::new(RefCell::new(Vec::new()))))
            })),
        );

        // oiGetPropKeys - Native function to get property keys
        self.env.define_global(
            "oiGetPropKeys",
            Value::NativeFunc(Rc::new(|_interp, args| {
                if let Some(Value::Object(instance)) = args.first() {
                    let keys: Vec<Value> = instance
                        .borrow()
                        .properties
                        .keys()
                        .map(|k| Value::Symbol(Rc::new(k.clone())))
                        .collect();
                    return Ok(Value::Array(Rc::new(RefCell::new(keys))));
                }
                Ok(Value::Array(Rc::new(RefCell::new(Vec::new()))))
            })),
        );

        // xOiDebugStartFunc / xOiDebugStartFunc2 - Debug logging stubs
        self.env.define_global(
            "xOiDebugStartFunc",
            Value::NativeFunc(Rc::new(|_interp, _args| Ok(Value::Null))),
        );
        self.env.define_global(
            "xOiDebugStartFunc2",
            Value::NativeFunc(Rc::new(|_interp, _args| Ok(Value::Null))),
        );
        self.env.define_global(
            "xOiDebugFinishFunc",
            Value::NativeFunc(Rc::new(|_interp, _args| Ok(Value::Null))),
        );

        // xOiCreateArticle - Create article stub
        self.env.define_global(
            "xOiCreateArticle",
            Value::NativeFunc(Rc::new(|_interp, _args| Ok(Value::Null))),
        );

        // oo_createArticle - Create article variant
        self.env.define_global(
            "oo_createArticle",
            Value::NativeFunc(Rc::new(|_interp, _args| Ok(Value::Null))),
        );

        // getLanguage - Get current language (returns "en")
        self.env.define_global(
            "getLanguage",
            Value::NativeFunc(Rc::new(|_interp, _args| {
                Ok(Value::String(Rc::new("en".to_string())))
            })),
        );

        // typeOf - Get type of value as string
        self.env.define_global(
            "typeOf",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let val = args.first().cloned().unwrap_or(Value::Null);
                let type_name = match val {
                    Value::Null => "Null",
                    Value::Int(_) => "Int",
                    Value::Float(_) => "Float",
                    Value::Bool(_) => "Bool",
                    Value::String(_) => "String",
                    Value::Symbol(_) => "Symbol",
                    Value::Array(_) => "Vector",
                    Value::Hash(_) => "Hash",
                    Value::Object(_) => "Object",
                    Value::Class(_) => "Class",
                    Value::Func(_) => "Function",
                    Value::NativeFunc(_) => "Function",
                    Value::Vec3(_) => "Vector",
                };
                Ok(Value::String(Rc::new(type_name.to_string())))
            })),
        );

        // lastObj - Return null (used in some contexts to track last created object)
        self.env.define_global(
            "lastObj",
            Value::NativeFunc(Rc::new(|_interp, _args| Ok(Value::Null))),
        );
    }

    /// Create a native OFML class
    fn create_native_class(&self, name: &str, parent: Option<Rc<ClassValue>>) -> Rc<ClassValue> {
        Rc::new(ClassValue {
            name: name.to_string(),
            package: String::new(), // Native classes have no package
            parent,
            methods: HashMap::new(),
            rules: HashMap::new(),
            static_vars: HashMap::new(),
            decl: ClassDecl {
                modifiers: vec![],
                name: name.to_string(),
                parent: None,
                members: vec![],
                span: crate::ast::Span::default(),
            },
        })
    }

    /// Register built-in functions and classes
    fn register_builtins(&mut self) {
        // Hash constructor
        self.env.define_global(
            "Hash",
            Value::NativeFunc(Rc::new(|_interp, _args| {
                Ok(Value::Hash(Rc::new(RefCell::new(HashMap::new()))))
            })),
        );

        // String constructor
        self.env.define_global(
            "String",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let s = args.first().map(|v| v.to_string_val()).unwrap_or_default();
                Ok(Value::String(Rc::new(s)))
            })),
        );

        // Float constructor
        self.env.define_global(
            "Float",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let f = args.first().and_then(|v| v.to_float()).unwrap_or(0.0);
                Ok(Value::Float(f))
            })),
        );

        // Int constructor
        self.env.define_global(
            "Int",
            Value::NativeFunc(Rc::new(|_interp, args| {
                let i = args.first().and_then(|v| v.to_int()).unwrap_or(0);
                Ok(Value::Int(i))
            })),
        );

        // print function (for debugging)
        self.env.define_global(
            "print",
            Value::NativeFunc(Rc::new(|interp, args| {
                let msg = args
                    .iter()
                    .map(|v| v.to_string_val())
                    .collect::<Vec<_>>()
                    .join(" ");
                interp.output.push(msg.clone());
                println!("{}", msg);
                Ok(Value::Null)
            })),
        );
    }

    /// Execute a translation unit
    pub fn execute(&mut self, unit: &TranslationUnit) -> InterpResult<Value> {
        // Set current package context from translation unit
        let prev_package = std::mem::replace(
            &mut self.current_package,
            unit.package
                .as_ref()
                .map(|p| p.to_string())
                .unwrap_or_default(),
        );

        let mut result = Value::Null;

        for stmt in &unit.statements {
            result = self.execute_stmt(stmt)?;
        }

        // Restore previous package context
        self.current_package = prev_package;

        Ok(result)
    }

    /// Execute a statement
    pub fn execute_stmt(&mut self, stmt: &Stmt) -> InterpResult<Value> {
        match stmt {
            Stmt::Var(decl) => self.execute_var_decl(decl),
            Stmt::VarList(decls) => {
                for decl in decls {
                    self.execute_var_decl(decl)?;
                }
                Ok(Value::Null)
            }
            Stmt::Class(decl) => self.execute_class_decl(decl),
            Stmt::Func(decl) => self.execute_func_decl(decl),
            Stmt::Expr(expr) => self.evaluate(expr),
            Stmt::Block(block) => self.execute_block(block),
            Stmt::If(if_stmt) => self.execute_if(if_stmt),
            Stmt::While(while_stmt) => self.execute_while(while_stmt),
            Stmt::DoWhile(do_while) => self.execute_do_while(do_while),
            Stmt::For(for_stmt) => self.execute_for(for_stmt),
            Stmt::Foreach(foreach) => self.execute_foreach(foreach),
            Stmt::Return(expr) => self.execute_return(expr.as_ref()),
            Stmt::Break(_) => Err(InterpError {
                message: "break".into(),
                kind: ErrorKind::Break,
            }),
            Stmt::Continue(_) => Err(InterpError {
                message: "continue".into(),
                kind: ErrorKind::Continue,
            }),
            Stmt::Throw(expr) => {
                let val = self.evaluate(expr)?;
                Err(InterpError::runtime(format!("Exception: {}", val)))
            }
            Stmt::Try(try_stmt) => self.execute_try(try_stmt),
            Stmt::Switch(switch) => self.execute_switch(switch),
            Stmt::Empty => Ok(Value::Null),
        }
    }

    /// Execute variable declaration
    fn execute_var_decl(&mut self, decl: &VarDecl) -> InterpResult<Value> {
        let value = if let Some(ref init) = decl.initializer {
            self.evaluate(init)?
        } else {
            Value::Null
        };
        self.env.define(&decl.name, value);
        Ok(Value::Null)
    }

    /// Execute class declaration
    fn execute_class_decl(&mut self, decl: &ClassDecl) -> InterpResult<Value> {
        // Resolve parent class - try qualified name first, then short name
        let parent = if let Some(ref parent_name) = decl.parent {
            // Build fully qualified parent name if it has package parts
            let fq_parent = parent_name.to_string();

            // Try fully qualified lookup first
            if let Some(p) = self.qualified_classes.get(&fq_parent) {
                Some(p.clone())
            } else {
                // Fall back to short name lookup
                let name = parent_name.parts.last().unwrap_or(&parent_name.parts[0]);
                self.classes.get(name).cloned()
            }
        } else {
            None
        };

        let mut methods = HashMap::new();
        let mut rules = HashMap::new();
        let mut static_vars = HashMap::new();

        // Process class members
        for member in &decl.members {
            match member {
                ClassMember::Func(func) => {
                    let func_val = Rc::new(FuncValue {
                        name: func.name.clone(),
                        params: func.params.clone(),
                        body: func.body.clone(),
                        class: None, // Will be set later
                        is_static: func.modifiers.contains(&Modifier::Static),
                    });
                    methods.insert(func.name.clone(), func_val);
                }
                ClassMember::Rule(rule) => {
                    let func_val = Rc::new(FuncValue {
                        name: rule.name.clone(),
                        params: rule.params.clone(),
                        body: Some(rule.body.clone()),
                        class: None,
                        is_static: false,
                    });
                    rules.insert(rule.name.clone(), func_val);
                }
                ClassMember::Var(var) => {
                    if var.modifiers.contains(&Modifier::Static) {
                        let value = if let Some(ref init) = var.initializer {
                            self.evaluate(init)?
                        } else {
                            Value::Null
                        };
                        static_vars.insert(var.name.clone(), value);
                    }
                }
                ClassMember::Expr(_) => {
                    // Class-level expressions are evaluated when creating instances
                }
            }
        }

        let class = Rc::new(ClassValue {
            name: decl.name.clone(),
            package: self.current_package.clone(),
            parent,
            methods,
            rules,
            static_vars,
            decl: decl.clone(),
        });

        // Register by short name (may be overwritten by later classes with same name)
        self.classes.insert(decl.name.clone(), class.clone());

        // Register by fully qualified name (unique, never overwritten)
        let qualified_name = class.qualified_name();
        self.qualified_classes.insert(qualified_name, class.clone());

        self.env.define_global(&decl.name, Value::Class(class));

        Ok(Value::Null)
    }

    /// Execute function declaration
    fn execute_func_decl(&mut self, decl: &FuncDecl) -> InterpResult<Value> {
        let func = Rc::new(FuncValue {
            name: decl.name.clone(),
            params: decl.params.clone(),
            body: decl.body.clone(),
            class: None,
            is_static: true,
        });
        self.env.define(&decl.name, Value::Func(func));
        Ok(Value::Null)
    }

    /// Execute block
    fn execute_block(&mut self, block: &Block) -> InterpResult<Value> {
        self.env.push_scope();
        let mut result = Value::Null;

        for stmt in &block.stmts {
            match self.execute_stmt(stmt) {
                Ok(val) => result = val,
                Err(e) => {
                    self.env.pop_scope();
                    return Err(e);
                }
            }
        }

        self.env.pop_scope();
        Ok(result)
    }

    /// Execute if statement
    fn execute_if(&mut self, if_stmt: &IfStmt) -> InterpResult<Value> {
        let cond = self.evaluate(&if_stmt.condition)?;
        if cond.is_truthy() {
            self.execute_stmt(&if_stmt.then_branch)
        } else if let Some(ref else_branch) = if_stmt.else_branch {
            self.execute_stmt(else_branch)
        } else {
            Ok(Value::Null)
        }
    }

    /// Execute while loop
    fn execute_while(&mut self, while_stmt: &WhileStmt) -> InterpResult<Value> {
        const MAX_LOOP_ITERATIONS: usize = 100_000;
        let mut result = Value::Null;
        let mut iterations = 0;
        loop {
            iterations += 1;
            if iterations > MAX_LOOP_ITERATIONS {
                // Break out of loop gracefully instead of erroring
                // This allows constructors to continue even if a loop gets stuck
                break;
            }
            let cond = self.evaluate(&while_stmt.condition)?;
            if !cond.is_truthy() {
                break;
            }
            match self.execute_stmt(&while_stmt.body) {
                Ok(val) => result = val,
                Err(e) if matches!(e.kind, ErrorKind::Break) => break,
                Err(e) if matches!(e.kind, ErrorKind::Continue) => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(result)
    }

    /// Execute do-while loop
    fn execute_do_while(&mut self, do_while: &DoWhileStmt) -> InterpResult<Value> {
        const MAX_LOOP_ITERATIONS: usize = 100_000;
        let mut result = Value::Null;
        let mut iterations = 0;
        loop {
            iterations += 1;
            if iterations > MAX_LOOP_ITERATIONS {
                // Break out of loop gracefully instead of erroring
                break;
            }
            match self.execute_stmt(&do_while.body) {
                Ok(val) => result = val,
                Err(e) if matches!(e.kind, ErrorKind::Break) => break,
                Err(e) if matches!(e.kind, ErrorKind::Continue) => {}
                Err(e) => return Err(e),
            }
            let cond = self.evaluate(&do_while.condition)?;
            if !cond.is_truthy() {
                break;
            }
        }
        Ok(result)
    }

    /// Execute for loop
    fn execute_for(&mut self, for_stmt: &ForStmt) -> InterpResult<Value> {
        const MAX_LOOP_ITERATIONS: usize = 100_000;
        self.env.push_scope();

        // Init
        if let Some(ref init) = for_stmt.init {
            self.execute_stmt(init)?;
        }

        let mut result = Value::Null;
        let mut iterations = 0;
        loop {
            iterations += 1;
            if iterations > MAX_LOOP_ITERATIONS {
                // Break out of loop gracefully instead of erroring
                break;
            }

            // Condition
            if let Some(ref cond) = for_stmt.condition {
                let cond_val = self.evaluate(cond)?;
                if !cond_val.is_truthy() {
                    break;
                }
            }

            // Body
            match self.execute_stmt(&for_stmt.body) {
                Ok(val) => result = val,
                Err(e) if matches!(e.kind, ErrorKind::Break) => break,
                Err(e) if matches!(e.kind, ErrorKind::Continue) => {}
                Err(e) => {
                    self.env.pop_scope();
                    return Err(e);
                }
            }

            // Update
            if let Some(ref update) = for_stmt.update {
                self.evaluate(update)?;
            }
        }

        self.env.pop_scope();
        Ok(result)
    }

    /// Execute foreach loop
    fn execute_foreach(&mut self, foreach: &ForeachStmt) -> InterpResult<Value> {
        let iterable = self.evaluate(&foreach.iterable)?;
        let items = match &iterable {
            Value::Array(arr) => arr.borrow().clone(),
            Value::Hash(hash) => hash
                .borrow()
                .keys()
                .map(|k| Value::String(Rc::new(k.clone())))
                .collect(),
            Value::Null => Vec::new(), // Iterating over Null is empty iteration
            Value::String(s) => {
                // Iterate over characters in string
                s.chars().map(|c| Value::Int(c as i64)).collect()
            }
            _ => return Err(InterpError::type_error("Cannot iterate over this type")),
        };

        self.env.push_scope();
        let mut result = Value::Null;

        for item in items {
            self.env.define(&foreach.var_name, item);
            match self.execute_stmt(&foreach.body) {
                Ok(val) => result = val,
                Err(e) if matches!(e.kind, ErrorKind::Break) => break,
                Err(e) if matches!(e.kind, ErrorKind::Continue) => continue,
                Err(e) => {
                    self.env.pop_scope();
                    return Err(e);
                }
            }
        }

        self.env.pop_scope();
        Ok(result)
    }

    /// Execute return statement
    fn execute_return(&mut self, expr: Option<&Expr>) -> InterpResult<Value> {
        let value = if let Some(e) = expr {
            self.evaluate(e)?
        } else {
            Value::Null
        };
        Err(InterpError {
            message: "return".into(),
            kind: ErrorKind::Return(value),
        })
    }

    /// Execute try-catch
    fn execute_try(&mut self, try_stmt: &TryStmt) -> InterpResult<Value> {
        match self.execute_block(&try_stmt.try_block) {
            Ok(val) => {
                if let Some(ref finally) = try_stmt.finally_block {
                    self.execute_block(finally)?;
                }
                Ok(val)
            }
            Err(e)
                if matches!(
                    e.kind,
                    ErrorKind::Runtime | ErrorKind::TypeError | ErrorKind::NameError
                ) =>
            {
                if let Some(ref catch_block) = try_stmt.catch_block {
                    self.env.push_scope();
                    if let Some(ref var) = try_stmt.catch_var {
                        self.env
                            .define(var, Value::String(Rc::new(e.message.clone())));
                    }
                    let result = self.execute_block(catch_block);
                    self.env.pop_scope();

                    if let Some(ref finally) = try_stmt.finally_block {
                        self.execute_block(finally)?;
                    }
                    result
                } else {
                    if let Some(ref finally) = try_stmt.finally_block {
                        self.execute_block(finally)?;
                    }
                    Err(e)
                }
            }
            Err(e) => {
                if let Some(ref finally) = try_stmt.finally_block {
                    self.execute_block(finally)?;
                }
                Err(e)
            }
        }
    }

    /// Execute switch statement
    fn execute_switch(&mut self, switch: &SwitchStmt) -> InterpResult<Value> {
        let switch_val = self.evaluate(&switch.expr)?;
        let mut result = Value::Null;
        let mut matched = false;
        let mut default_index = None;

        // Find matching case
        for (i, case) in switch.cases.iter().enumerate() {
            if case.value.is_none() {
                default_index = Some(i);
                continue;
            }
            if let Some(ref case_expr) = case.value {
                let case_val = self.evaluate(case_expr)?;
                if switch_val.equals(&case_val) {
                    matched = true;
                    // Execute this and all following cases (fall-through)
                    for case in &switch.cases[i..] {
                        for stmt in &case.stmts {
                            match self.execute_stmt(stmt) {
                                Ok(val) => result = val,
                                Err(e) if matches!(e.kind, ErrorKind::Break) => return Ok(result),
                                Err(e) => return Err(e),
                            }
                        }
                    }
                    break;
                }
            }
        }

        // Execute default if no match
        if !matched {
            if let Some(idx) = default_index {
                for case in &switch.cases[idx..] {
                    for stmt in &case.stmts {
                        match self.execute_stmt(stmt) {
                            Ok(val) => result = val,
                            Err(e) if matches!(e.kind, ErrorKind::Break) => return Ok(result),
                            Err(e) => return Err(e),
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    /// Evaluate an expression
    pub fn evaluate(&mut self, expr: &Expr) -> InterpResult<Value> {
        const MAX_OPERATIONS: usize = 1_000_000;
        self.operation_count += 1;
        if self.operation_count > MAX_OPERATIONS {
            return Ok(Value::Null); // Graceful degradation
        }
        match expr {
            Expr::Int(n) => Ok(Value::Int(*n)),
            Expr::Float(f) => Ok(Value::Float(*f)),
            Expr::String(s) => Ok(Value::String(Rc::new(s.clone()))),
            Expr::Symbol(s) => Ok(Value::Symbol(Rc::new(s.clone()))),
            Expr::Null => Ok(Value::Null),
            Expr::SelfRef => self.get_self(),
            Expr::SuperRef => self.get_self(), // Simplified for now
            Expr::Ident(name) => self.get_variable(name),
            Expr::QualifiedName(qn) => self.get_qualified_name(qn),
            Expr::Array(elements) => self.eval_array(elements),
            Expr::List(elements) => self.eval_array(elements), // Treat same as array
            Expr::Binary(bin) => self.eval_binary(bin),
            Expr::Unary(unary) => self.eval_unary(unary),
            Expr::Conditional(cond) => self.eval_conditional(cond),
            Expr::Assign(assign) => self.eval_assign(assign),
            Expr::Call(call) => self.eval_call(call),
            Expr::Index(idx) => self.eval_index(idx),
            Expr::Range(range) => self.eval_range(range),
            Expr::Member(member) => self.eval_member(member),
            Expr::Instanceof(inst) => self.eval_instanceof(inst),
            Expr::Paren(inner) => self.evaluate(inner),
        }
    }

    fn get_self(&self) -> InterpResult<Value> {
        self.current_self
            .clone()
            .map(Value::Object)
            .ok_or_else(|| InterpError::runtime("'self' used outside of method"))
    }

    fn get_variable(&self, name: &str) -> InterpResult<Value> {
        // First try environment (local vars, globals)
        if let Some(val) = self.env.get(name) {
            return Ok(val);
        }

        // Then try current object's fields (OFML allows accessing fields without self.)
        if let Some(ref instance) = self.current_self {
            if let Some(val) = instance.borrow().fields.get(name) {
                return Ok(val.clone());
            }
        }

        Err(InterpError::name_error(format!(
            "Undefined variable: {}",
            name
        )))
    }

    fn get_qualified_name(&self, qn: &QualifiedName) -> InterpResult<Value> {
        // Try to find the class or value
        let name = qn.parts.last().unwrap();
        self.env
            .get(name)
            .or_else(|| self.classes.get(name).map(|c| Value::Class(c.clone())))
            .ok_or_else(|| InterpError::name_error(format!("Undefined: {}", qn)))
    }

    fn eval_array(&mut self, elements: &[Expr]) -> InterpResult<Value> {
        let values: Result<Vec<_>, _> = elements.iter().map(|e| self.evaluate(e)).collect();
        Ok(Value::Array(Rc::new(RefCell::new(values?))))
    }

    fn eval_binary(&mut self, bin: &BinaryExpr) -> InterpResult<Value> {
        let left = self.evaluate(&bin.left)?;
        let right = self.evaluate(&bin.right)?;

        match bin.op {
            BinaryOp::Add => self.binary_add(&left, &right),
            BinaryOp::Sub => self.binary_sub(&left, &right),
            BinaryOp::Mul => self.binary_mul(&left, &right),
            BinaryOp::Div => self.binary_div(&left, &right),
            BinaryOp::Mod => self.binary_mod(&left, &right),
            BinaryOp::Eq => Ok(Value::Bool(left.equals(&right))),
            BinaryOp::Ne => Ok(Value::Bool(!left.equals(&right))),
            BinaryOp::Lt => self.binary_cmp(&left, &right, |a, b| a < b),
            BinaryOp::Le => self.binary_cmp(&left, &right, |a, b| a <= b),
            BinaryOp::Gt => self.binary_cmp(&left, &right, |a, b| a > b),
            BinaryOp::Ge => self.binary_cmp(&left, &right, |a, b| a >= b),
            BinaryOp::And => Ok(Value::Bool(left.is_truthy() && right.is_truthy())),
            BinaryOp::Or => Ok(Value::Bool(left.is_truthy() || right.is_truthy())),
            BinaryOp::BitAnd => self.binary_bitop(&left, &right, |a, b| a & b),
            BinaryOp::BitOr => self.binary_bitop(&left, &right, |a, b| a | b),
            BinaryOp::BitXor => self.binary_bitop(&left, &right, |a, b| a ^ b),
            BinaryOp::Shl => self.binary_bitop(&left, &right, |a, b| a << b),
            BinaryOp::Shr => self.binary_bitop(&left, &right, |a, b| a >> b),
            BinaryOp::Ushr => self.binary_bitop(&left, &right, |a, b| ((a as u64) >> b) as i64),
            BinaryOp::Min => self.binary_minmax(&left, &right, true),
            BinaryOp::Max => self.binary_minmax(&left, &right, false),
            BinaryOp::PatternMatch => Ok(Value::Bool(false)), // Simplified
        }
    }

    fn binary_add(&self, left: &Value, right: &Value) -> InterpResult<Value> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + *b as f64)),
            // Null treated as 0 in addition
            (Value::Null, Value::Int(b)) => Ok(Value::Int(*b)),
            (Value::Null, Value::Float(b)) => Ok(Value::Float(*b)),
            (Value::Int(a), Value::Null) => Ok(Value::Int(*a)),
            (Value::Float(a), Value::Null) => Ok(Value::Float(*a)),
            (Value::Null, Value::Null) => Ok(Value::Int(0)),
            (Value::String(a), Value::String(b)) => {
                Ok(Value::String(Rc::new(format!("{}{}", a, b))))
            }
            (Value::String(a), b) => Ok(Value::String(Rc::new(format!(
                "{}{}",
                a,
                b.to_string_val()
            )))),
            (a, Value::String(b)) => Ok(Value::String(Rc::new(format!(
                "{}{}",
                a.to_string_val(),
                b
            )))),
            _ => Err(InterpError::type_error(format!(
                "Cannot add {} and {}",
                left.type_name(),
                right.type_name()
            ))),
        }
    }

    fn binary_sub(&self, left: &Value, right: &Value) -> InterpResult<Value> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - *b as f64)),
            // Null treated as 0 in subtraction
            (Value::Null, Value::Int(b)) => Ok(Value::Int(-b)),
            (Value::Null, Value::Float(b)) => Ok(Value::Float(-b)),
            (Value::Int(a), Value::Null) => Ok(Value::Int(*a)),
            (Value::Float(a), Value::Null) => Ok(Value::Float(*a)),
            (Value::Null, Value::Null) => Ok(Value::Int(0)),
            _ => Err(InterpError::type_error("Cannot subtract these types")),
        }
    }

    fn binary_mul(&self, left: &Value, right: &Value) -> InterpResult<Value> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * *b as f64)),
            // Null treated as 0 in multiplication (x * 0 = 0)
            (Value::Null, _) | (_, Value::Null) => Ok(Value::Int(0)),
            _ => Err(InterpError::type_error("Cannot multiply these types")),
        }
    }

    fn binary_div(&self, left: &Value, right: &Value) -> InterpResult<Value> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    return Err(InterpError::runtime("Division by zero"));
                }
                Ok(Value::Int(a / b))
            }
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 / b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a / *b as f64)),
            // Null division - treat Null as 0
            (Value::Null, _) => Ok(Value::Float(0.0)),
            (Value::Int(a), Value::Null) => Ok(Value::Float(*a as f64)), // x / 0 treated as x
            (Value::Float(a), Value::Null) => Ok(Value::Float(*a)),
            _ => Err(InterpError::type_error("Cannot divide these types")),
        }
    }

    fn binary_mod(&self, left: &Value, right: &Value) -> InterpResult<Value> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    return Err(InterpError::runtime("Modulo by zero"));
                }
                Ok(Value::Int(a % b))
            }
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a % b)),
            _ => Err(InterpError::type_error("Cannot modulo these types")),
        }
    }

    fn binary_cmp<F>(&self, left: &Value, right: &Value, cmp: F) -> InterpResult<Value>
    where
        F: Fn(f64, f64) -> bool,
    {
        // Null comparisons: Null compared to anything is false
        if matches!(left, Value::Null) || matches!(right, Value::Null) {
            return Ok(Value::Bool(false));
        }
        let a = left
            .to_float()
            .ok_or_else(|| InterpError::type_error("Cannot compare"))?;
        let b = right
            .to_float()
            .ok_or_else(|| InterpError::type_error("Cannot compare"))?;
        Ok(Value::Bool(cmp(a, b)))
    }

    fn binary_bitop<F>(&self, left: &Value, right: &Value, op: F) -> InterpResult<Value>
    where
        F: Fn(i64, i64) -> i64,
    {
        let a = left
            .to_int()
            .ok_or_else(|| InterpError::type_error("Bitwise requires integers"))?;
        let b = right
            .to_int()
            .ok_or_else(|| InterpError::type_error("Bitwise requires integers"))?;
        Ok(Value::Int(op(a, b)))
    }

    fn binary_minmax(&self, left: &Value, right: &Value, is_min: bool) -> InterpResult<Value> {
        // Handle Null in min/max - Null is treated as the identity element
        let a = match left {
            Value::Null => return Ok(right.clone()), // min/max with Null returns the other value
            v => v
                .to_float()
                .ok_or_else(|| InterpError::type_error("Cannot compare"))?,
        };
        let b = match right {
            Value::Null => return Ok(left.clone()),
            v => v
                .to_float()
                .ok_or_else(|| InterpError::type_error("Cannot compare"))?,
        };
        let result = if is_min { a.min(b) } else { a.max(b) };
        // Preserve integer type if both are integers
        if matches!(left, Value::Int(_)) && matches!(right, Value::Int(_)) {
            Ok(Value::Int(result as i64))
        } else {
            Ok(Value::Float(result))
        }
    }

    fn eval_unary(&mut self, unary: &UnaryExpr) -> InterpResult<Value> {
        let val = self.evaluate(&unary.operand)?;
        match unary.op {
            UnaryOp::Neg => match val {
                Value::Int(n) => Ok(Value::Int(-n)),
                Value::Float(f) => Ok(Value::Float(-f)),
                Value::Null => Ok(Value::Int(0)), // Negating Null returns 0
                _ => Err(InterpError::type_error("Cannot negate")),
            },
            UnaryOp::Pos => Ok(val),
            UnaryOp::Not => Ok(Value::Bool(!val.is_truthy())),
            UnaryOp::BitNot => {
                let n = val
                    .to_int()
                    .ok_or_else(|| InterpError::type_error("Bitwise requires integer"))?;
                Ok(Value::Int(!n))
            }
            UnaryOp::Test => Ok(Value::Bool(val.is_truthy())),
            UnaryOp::Resolve => {
                // $ operator resolves symbol to variable
                match val {
                    Value::Symbol(s) => self.get_variable(&s),
                    _ => Err(InterpError::type_error("$ requires symbol")),
                }
            }
            UnaryOp::PreInc | UnaryOp::PostInc => {
                // Simplified - doesn't handle the pre/post difference correctly
                let n = val
                    .to_int()
                    .ok_or_else(|| InterpError::type_error("Cannot increment"))?;
                Ok(Value::Int(n + 1))
            }
            UnaryOp::PreDec | UnaryOp::PostDec => {
                let n = val
                    .to_int()
                    .ok_or_else(|| InterpError::type_error("Cannot decrement"))?;
                Ok(Value::Int(n - 1))
            }
        }
    }

    fn eval_conditional(&mut self, cond: &ConditionalExpr) -> InterpResult<Value> {
        let test = self.evaluate(&cond.condition)?;
        if test.is_truthy() {
            self.evaluate(&cond.then_expr)
        } else {
            self.evaluate(&cond.else_expr)
        }
    }

    fn eval_assign(&mut self, assign: &AssignExpr) -> InterpResult<Value> {
        let value = self.evaluate(&assign.value)?;

        // Handle compound assignment
        let final_value = match assign.op {
            AssignOp::Assign => value,
            _ => {
                let current = self.evaluate(&assign.target)?;
                match assign.op {
                    AssignOp::AddAssign => self.binary_add(&current, &value)?,
                    AssignOp::SubAssign => self.binary_sub(&current, &value)?,
                    AssignOp::MulAssign => self.binary_mul(&current, &value)?,
                    AssignOp::DivAssign => self.binary_div(&current, &value)?,
                    AssignOp::ModAssign => self.binary_mod(&current, &value)?,
                    _ => value,
                }
            }
        };

        // Assign to target
        match &*assign.target {
            Expr::Ident(name) => {
                self.env.set_or_define(name, final_value.clone());
            }
            Expr::Member(member) => {
                let obj = self.evaluate(&member.object)?;
                self.set_member(&obj, &member.member, final_value.clone())?;
            }
            Expr::Index(idx) => {
                let obj = self.evaluate(&idx.object)?;
                let index = self.evaluate(&idx.index)?;
                self.set_index(&obj, &index, final_value.clone())?;
            }
            _ => return Err(InterpError::runtime("Invalid assignment target")),
        }

        Ok(final_value)
    }

    fn eval_call(&mut self, call: &CallExpr) -> InterpResult<Value> {
        // Evaluate arguments
        let args: Result<Vec<_>, _> = call.args.iter().map(|a| self.evaluate(a)).collect();
        let args = args?;

        // Handle method calls (obj.method())
        if let Expr::Member(member) = &*call.callee {
            let obj = self.evaluate(&member.object)?;
            return self.call_method(&obj, &member.member, args);
        }

        // Handle qualified name calls (Class::method())
        if let Expr::QualifiedName(qn) = &*call.callee {
            if qn.parts.len() >= 2 {
                let short_class_name = &qn.parts[qn.parts.len() - 2];
                let method_name = qn.parts.last().unwrap();

                // Build fully qualified class name (everything except the method)
                let class_parts: Vec<&str> = qn.parts[..qn.parts.len() - 1]
                    .iter()
                    .map(|s| s.as_str())
                    .collect();
                let fq_class_name = if qn.absolute {
                    format!("::{}", class_parts.join("::"))
                } else {
                    class_parts.join("::")
                };

                // Try fully qualified lookup first, then fall back to short name
                let class = self
                    .qualified_classes
                    .get(&fq_class_name)
                    .cloned()
                    .or_else(|| self.classes.get(short_class_name).cloned());

                if let Some(class) = class {
                    if let Some(method) = class.methods.get(method_name) {
                        return self.call_function(method.clone(), args);
                    }
                    // Check parent classes for the method using lazy resolution
                    let mut current = Some(class.clone());
                    while let Some(c) = current {
                        if let Some(method) = c.methods.get(method_name) {
                            return self.call_function(method.clone(), args);
                        }
                        // Lazy parent resolution
                        current = if c.parent.is_some() {
                            c.parent.clone()
                        } else if let Some(ref parent_name) = c.decl.parent {
                            let parent_short =
                                parent_name.parts.last().unwrap_or(&parent_name.parts[0]);
                            self.classes.get(parent_short).cloned()
                        } else {
                            None
                        };
                    }
                    // If method not found but class exists, return Null gracefully
                    // This handles Parent::initialize() when parent has no initialize
                    return Ok(Value::Null);
                } else {
                    // Class not found - for Parent::initialize() style calls, return Null
                    // This handles cases where parent class isn't loaded (missing dependencies)
                    if method_name == "initialize" || method_name.starts_with("init") {
                        return Ok(Value::Null);
                    }
                }
            }
        }

        // Check if this is a simple identifier call and we're inside a method
        // If so, try to call it as a method on self first
        if let Expr::Ident(name) = &*call.callee {
            if let Some(ref instance) = self.current_self {
                // First check if it's a method in the class hierarchy
                let class = instance.borrow().class.clone();
                if let Some(method) = class.methods.get(name) {
                    return self.call_function(method.clone(), args);
                }
                // Also check parent classes using lazy resolution
                let mut current = Some(class.clone());
                while let Some(c) = current {
                    if let Some(method) = c.methods.get(name) {
                        return self.call_function(method.clone(), args);
                    }
                    // Lazy parent resolution
                    current = if c.parent.is_some() {
                        c.parent.clone()
                    } else if let Some(ref parent_name) = c.decl.parent {
                        let parent_short =
                            parent_name.parts.last().unwrap_or(&parent_name.parts[0]);
                        self.classes.get(parent_short).cloned()
                    } else {
                        None
                    };
                }
                // Then try as a built-in method on self (setPosition, rotate, etc.)
                let obj = Value::Object(instance.clone());
                if self.is_builtin_method(name) {
                    return self.call_method(&obj, name, args);
                }
            }
        }

        // Regular function call
        let callee = self.evaluate(&call.callee)?;
        match callee {
            Value::Func(func) => self.call_function(func, args),
            Value::NativeFunc(func) => func(self, args).map_err(InterpError::from),
            Value::Class(class) => self.instantiate_class(class, args),
            // Graceful degradation: calling Null returns Null
            Value::Null => Ok(Value::Null),
            _ => Err(InterpError::type_error(format!(
                "Cannot call {}",
                callee.type_name()
            ))),
        }
    }

    fn call_function(&mut self, func: Rc<FuncValue>, args: Vec<Value>) -> InterpResult<Value> {
        const MAX_CALL_DEPTH: usize = 500;
        const MAX_OPERATIONS: usize = 1_000_000;

        self.call_depth += 1;
        self.operation_count += 1;

        // Check call depth
        if self.call_depth > MAX_CALL_DEPTH {
            self.call_depth -= 1;
            return Err(InterpError::runtime(format!(
                "Maximum call depth ({}) exceeded - possible infinite recursion in function '{}'",
                MAX_CALL_DEPTH, func.name
            )));
        }

        // Check total operations - gracefully break on limit
        if self.operation_count > MAX_OPERATIONS {
            self.call_depth -= 1;
            // Return Null instead of error for graceful degradation
            return Ok(Value::Null);
        }

        let body = func.body.as_ref().ok_or_else(|| {
            self.call_depth -= 1;
            InterpError::runtime("Cannot call native function")
        })?;

        self.env.push_scope();

        // Bind parameters
        for (i, param) in func.params.iter().enumerate() {
            let value = args.get(i).cloned().unwrap_or(Value::Null);
            self.env.define(param, value);
        }

        // Execute body
        let result = match self.execute_block(body) {
            Ok(val) => val,
            Err(e) if matches!(e.kind, ErrorKind::Return(ref _v)) => {
                if let ErrorKind::Return(v) = e.kind {
                    v
                } else {
                    Value::Null
                }
            }
            Err(e) => {
                self.env.pop_scope();
                self.call_depth -= 1;
                return Err(e);
            }
        };

        self.env.pop_scope();
        self.call_depth -= 1;
        Ok(result)
    }

    pub fn call_method(
        &mut self,
        obj: &Value,
        method: &str,
        args: Vec<Value>,
    ) -> InterpResult<Value> {
        // Handle built-in methods
        match obj {
            Value::Array(arr) => return self.call_array_method(arr.clone(), method, args),
            Value::Hash(hash) => return self.call_hash_method(hash.clone(), method, args),
            Value::Object(instance) => {
                // Handle object methods
                let instance = instance.clone();

                // Check for built-in geometry methods
                match method {
                    "setPosition" => {
                        if let Some(pos) = args.first() {
                            instance
                                .borrow_mut()
                                .set_position(pos)
                                .map_err(InterpError::from)?;
                            // Also update scene node if exists
                            let obj_id = instance.as_ptr() as u64;
                            if let Some(node) = self.obj_to_node.get(&obj_id) {
                                let pos_arr = match pos {
                                    Value::Array(arr) => {
                                        let arr = arr.borrow();
                                        [
                                            arr.first().and_then(|v| v.to_float()).unwrap_or(0.0)
                                                as f32,
                                            arr.get(1).and_then(|v| v.to_float()).unwrap_or(0.0)
                                                as f32,
                                            arr.get(2).and_then(|v| v.to_float()).unwrap_or(0.0)
                                                as f32,
                                        ]
                                    }
                                    _ => [0.0, 0.0, 0.0],
                                };
                                node.borrow_mut().set_position(pos_arr);
                            }
                        }
                        return Ok(Value::Null);
                    }
                    "getPosition" => {
                        return Ok(instance.borrow().get_position());
                    }
                    "setAlignment" => {
                        // setAlignment([@X, @Y, @Z]) - set alignment based on OFML modes
                        // @I = min bound (default), @C = center, @A = max bound
                        if let Some(Value::Array(arr)) = args.first() {
                            let arr = arr.borrow();
                            let parse_align = |v: Option<&Value>| -> AlignMode {
                                match v {
                                    Some(Value::Symbol(s)) => match s.as_str() {
                                        "I" => AlignMode::Min,
                                        "C" => AlignMode::Center,
                                        "A" => AlignMode::Max,
                                        _ => AlignMode::Min,
                                    },
                                    _ => AlignMode::Min,
                                }
                            };
                            let align_x = parse_align(arr.first());
                            let align_y = parse_align(arr.get(1));
                            let align_z = parse_align(arr.get(2));

                            // Update scene node
                            let obj_id = instance.as_ptr() as u64;
                            if let Some(node) = self.obj_to_node.get(&obj_id) {
                                node.borrow_mut().set_alignment(align_x, align_y, align_z);
                            }
                        }
                        return Ok(Value::Null);
                    }
                    "setFootAlignment" => {
                        // setFootAlignment() - set alignment based on ground contact point
                        // Useful for L-shaped or irregular geometry where bounding box center
                        // doesn't represent the actual anchor/contact point
                        let obj_id = instance.as_ptr() as u64;
                        if let Some(node) = self.obj_to_node.get(&obj_id) {
                            node.borrow_mut().set_foot_alignment();
                        }
                        return Ok(Value::Null);
                    }
                    "rotate" => {
                        // rotate(@axis, angle) - rotate around axis
                        if let (Some(Value::Symbol(axis)), Some(angle)) =
                            (args.first(), args.get(1))
                        {
                            let angle_f = angle.to_float().unwrap_or(0.0);
                            let axis_enum = match axis.as_str() {
                                "NX" => Axis::X,
                                "NY" => Axis::Y,
                                "NZ" => Axis::Z,
                                _ => Axis::Y,
                            };
                            // Update ObjInstance rotation
                            let mut inst = instance.borrow_mut();
                            match axis_enum {
                                Axis::X => inst.rotation[0] += angle_f,
                                Axis::Y => inst.rotation[1] += angle_f,
                                Axis::Z => inst.rotation[2] += angle_f,
                            }
                            drop(inst);
                            // Update scene node
                            let obj_id = instance.as_ptr() as u64;
                            if let Some(node) = self.obj_to_node.get(&obj_id) {
                                node.borrow_mut().rotate(axis_enum, angle_f as f32);
                            }
                        }
                        return Ok(Value::Null);
                    }
                    "getRotation" => {
                        // getRotation(@axis) - get rotation around axis
                        if let Some(Value::Symbol(axis)) = args.first() {
                            let inst = instance.borrow();
                            let angle = match axis.as_str() {
                                "NX" => inst.rotation[0],
                                "NY" => inst.rotation[1],
                                "NZ" => inst.rotation[2],
                                _ => 0.0,
                            };
                            return Ok(Value::Float(angle));
                        }
                        return Ok(Value::Float(0.0));
                    }
                    "getLocalBounds" => {
                        // Returns [[minX, minY, minZ], [maxX, maxY, maxZ]]
                        let obj_id = instance.as_ptr() as u64;
                        if let Some(node) = self.obj_to_node.get(&obj_id) {
                            let bounds = node.borrow().get_local_bounds();
                            let min = Value::Array(Rc::new(RefCell::new(vec![
                                Value::Float(bounds[0][0] as f64),
                                Value::Float(bounds[0][1] as f64),
                                Value::Float(bounds[0][2] as f64),
                            ])));
                            let max = Value::Array(Rc::new(RefCell::new(vec![
                                Value::Float(bounds[1][0] as f64),
                                Value::Float(bounds[1][1] as f64),
                                Value::Float(bounds[1][2] as f64),
                            ])));
                            return Ok(Value::Array(Rc::new(RefCell::new(vec![min, max]))));
                        }
                        // Default empty bounds
                        let zero = Value::Array(Rc::new(RefCell::new(vec![
                            Value::Float(0.0),
                            Value::Float(0.0),
                            Value::Float(0.0),
                        ])));
                        return Ok(Value::Array(Rc::new(RefCell::new(vec![
                            zero.clone(),
                            zero,
                        ]))));
                    }
                    "setMaterial" => {
                        // setMaterial(materialName) - set material on scene node
                        if let Some(mat) = args.first() {
                            let mat_name = mat.to_string_val();
                            instance.borrow_mut().properties.insert(
                                "material".to_string(),
                                Value::String(Rc::new(mat_name.clone())),
                            );
                            let obj_id = instance.as_ptr() as u64;
                            if let Some(node) = self.obj_to_node.get(&obj_id) {
                                node.borrow_mut().material = mat_name;
                            }
                        }
                        return Ok(Value::Null);
                    }
                    "notSelectable" => {
                        let obj_id = instance.as_ptr() as u64;
                        if let Some(node) = self.obj_to_node.get(&obj_id) {
                            node.borrow_mut().selectable = false;
                        }
                        return Ok(Value::Null);
                    }
                    "setCutable" => {
                        // Store as property
                        if let Some(val) = args.first() {
                            instance
                                .borrow_mut()
                                .properties
                                .insert("cutable".to_string(), val.clone());
                        }
                        return Ok(Value::Null);
                    }
                    "setRtAxis" | "setTrAxis" => {
                        // Store axis properties
                        if let Some(val) = args.first() {
                            instance
                                .borrow_mut()
                                .properties
                                .insert(method.to_string(), val.clone());
                        }
                        return Ok(Value::Null);
                    }
                    "set2DName" => {
                        if let Some(val) = args.first() {
                            instance
                                .borrow_mut()
                                .properties
                                .insert("2DName".to_string(), val.clone());
                        }
                        return Ok(Value::Null);
                    }
                    "getName" => {
                        return Ok(Value::String(Rc::new(instance.borrow().name.clone())));
                    }
                    "getElements" => {
                        // Return child objects
                        let children: Vec<Value> = instance
                            .borrow()
                            .children
                            .iter()
                            .map(|c| Value::Object(c.clone()))
                            .collect();
                        return Ok(Value::Array(Rc::new(RefCell::new(children))));
                    }
                    "getPropValue" => {
                        if let Some(Value::Symbol(prop)) = args.first() {
                            return Ok(instance.borrow().get_prop_value(prop));
                        }
                        return Ok(Value::Null);
                    }
                    "setPropValue" => {
                        if let (Some(Value::Symbol(prop)), Some(val)) = (args.first(), args.get(1))
                        {
                            instance.borrow_mut().set_prop_value(prop, val.clone());
                        }
                        return Ok(Value::Null);
                    }
                    "hasProperty" => {
                        if let Some(Value::Symbol(prop)) = args.first() {
                            return Ok(Value::Bool(
                                instance.borrow().properties.contains_key(prop.as_str()),
                            ));
                        }
                        return Ok(Value::Bool(false));
                    }
                    "hasMember" => {
                        // Check if object has a method or field
                        if let Some(Value::Symbol(name)) = args.first() {
                            let inst = instance.borrow();
                            let has_field = inst.fields.contains_key(name.as_str());
                            let has_method = inst.class.methods.contains_key(name.as_str());
                            return Ok(Value::Bool(has_field || has_method));
                        }
                        return Ok(Value::Bool(false));
                    }
                    "isA" => {
                        if let Some(arg) = args.first() {
                            let class_name = match arg {
                                Value::String(s) => s.to_string(),
                                Value::Symbol(s) => s.to_string(),
                                Value::Class(c) => c.name.clone(),
                                _ => return Ok(Value::Bool(false)),
                            };
                            return Ok(Value::Bool(instance.borrow().is_a(&class_name)));
                        }
                        return Ok(Value::Bool(false));
                    }
                    "getFather" => {
                        let parent = instance.borrow().parent.clone();
                        return Ok(parent.map(Value::Object).unwrap_or(Value::Null));
                    }
                    "getChildren" => {
                        let children: Vec<Value> = instance
                            .borrow()
                            .children
                            .iter()
                            .map(|c| Value::Object(c.clone()))
                            .collect();
                        return Ok(Value::Array(Rc::new(RefCell::new(children))));
                    }
                    "getClass" => {
                        // Return the class name of this object
                        return Ok(Value::String(Rc::new(instance.borrow().class.name.clone())));
                    }
                    "getProductDB" => {
                        // PDManager.getProductDB(@product_name) - returns a ProductDB object
                        // This is used to access product database for property lookups
                        if instance.borrow().class.name == "PDManager" {
                            let pdb_class = self
                                .classes
                                .get("ProductDB")
                                .cloned()
                                .unwrap_or_else(|| self.create_native_class("ProductDB", None));

                            let product_name =
                                args.first().map(|v| v.to_string_val()).unwrap_or_default();

                            let pdb_instance = Rc::new(RefCell::new(ObjInstance {
                                class: pdb_class,
                                name: format!("ProductDB:{}", product_name),
                                ..Default::default()
                            }));
                            // Store the product name as a field
                            pdb_instance.borrow_mut().fields.insert(
                                "product".to_string(),
                                Value::String(Rc::new(product_name)),
                            );
                            // Store the ALB path if available
                            if let Some(alb_path) = &self.current_alb_path {
                                pdb_instance.borrow_mut().fields.insert(
                                    "dataRootDir".to_string(),
                                    Value::String(Rc::new(alb_path.display().to_string())),
                                );
                            }

                            return Ok(Value::Object(pdb_instance));
                        }
                        return Ok(Value::Null);
                    }
                    "getDataRootDir" => {
                        // ProductDB.getDataRootDir() - returns the data root directory path
                        if instance.borrow().class.name == "ProductDB" {
                            let dir = instance
                                .borrow()
                                .fields
                                .get("dataRootDir")
                                .cloned()
                                .unwrap_or_else(|| Value::String(Rc::new(String::new())));
                            return Ok(dir);
                        }
                        return Ok(Value::String(Rc::new(String::new())));
                    }
                    "setState" => {
                        // Set object state flags
                        if let Some(val) = args.first().and_then(|v| v.to_int()) {
                            instance
                                .borrow_mut()
                                .properties
                                .insert("_state".to_string(), Value::Int(val));
                        }
                        return Ok(Value::Null);
                    }
                    "getState" => {
                        // Get object state flags
                        let state = instance
                            .borrow()
                            .properties
                            .get("_state")
                            .and_then(|v| v.to_int())
                            .unwrap_or(0);
                        return Ok(Value::Int(state));
                    }
                    "setDimensions" => {
                        // Set block dimensions - update scene node
                        if let Some(Value::Array(dims)) = args.first() {
                            let dims = dims.borrow();
                            let w = dims.first().and_then(|v| v.to_float()).unwrap_or(1.0) as f32;
                            let h = dims.get(1).and_then(|v| v.to_float()).unwrap_or(1.0) as f32;
                            let d = dims.get(2).and_then(|v| v.to_float()).unwrap_or(1.0) as f32;
                            let obj_id = instance.as_ptr() as u64;
                            if let Some(node) = self.obj_to_node.get(&obj_id) {
                                node.borrow_mut().geometry = crate::scene::Geometry::Block {
                                    width: w,
                                    height: h,
                                    depth: d,
                                };
                            }
                        }
                        return Ok(Value::Null);
                    }
                    "getDimensions" => {
                        // Get block dimensions from scene node
                        let obj_id = instance.as_ptr() as u64;
                        if let Some(node) = self.obj_to_node.get(&obj_id) {
                            if let crate::scene::Geometry::Block {
                                width,
                                height,
                                depth,
                            } = node.borrow().geometry
                            {
                                return Ok(Value::Array(Rc::new(RefCell::new(vec![
                                    Value::Float(width as f64),
                                    Value::Float(height as f64),
                                    Value::Float(depth as f64),
                                ]))));
                            }
                        }
                        return Ok(Value::Array(Rc::new(RefCell::new(vec![
                            Value::Float(0.0),
                            Value::Float(0.0),
                            Value::Float(0.0),
                        ]))));
                    }
                    "isCat" => {
                        // Category check - stub, always returns false for now
                        return Ok(Value::Bool(false));
                    }
                    "article2Class" => {
                        // article2Class(pArticle) - returns the class name that models an article
                        // Per OFML spec: "If a global product data manager instance is registered,
                        // the query to this instance is delegated"
                        // For now, return NULL as the reference implementation does
                        // A real implementation would look up the article in the product database
                        return Ok(Value::Null);
                    }
                    "setupProperty" => {
                        // setupProperty(@propName, [label, min, max, group, choices], sortOrder)
                        if let (
                            Some(Value::Symbol(prop_name)),
                            Some(Value::Array(config)),
                            sort_order,
                        ) = (args.first(), args.get(1), args.get(2))
                        {
                            let config = config.borrow();
                            let mut prop_def = PropertyDef::default();
                            prop_def.name = prop_name.to_string();

                            // Parse config array: [label, min, max, group, choices]
                            if let Some(label) = config.first() {
                                prop_def.description = label.to_string_val();
                            }
                            // min and max are often NULL in OFML, skip for now
                            if let Some(group) = config.get(3) {
                                prop_def.group = group.to_int().unwrap_or(0) as i32;
                            }
                            // choices are stored as a string like "ch @opt1 @opt2"
                            if let Some(choices) = config.get(4) {
                                let choices_str = choices.to_string_val();
                                if choices_str.starts_with("ch ") {
                                    let parts: Vec<Value> = choices_str[3..]
                                        .split_whitespace()
                                        .filter(|s| s.starts_with('@'))
                                        .map(|s| Value::Symbol(Rc::new(s[1..].to_string())))
                                        .collect();
                                    prop_def.choices = parts;
                                }
                            }
                            if let Some(sort) = sort_order {
                                prop_def.sort_order = sort.to_int().unwrap_or(0) as i32;
                            }
                            prop_def.state = 3; // editable by default

                            let mut inst = instance.borrow_mut();
                            inst.prop_defs.insert(prop_name.to_string(), prop_def);
                            // Initialize property value to NULL if not set
                            if !inst.properties.contains_key(prop_name.as_str()) {
                                inst.properties.insert(prop_name.to_string(), Value::Null);
                            }
                        }
                        return Ok(Value::Null);
                    }
                    "removeProperty" => {
                        if let Some(Value::Symbol(prop_name)) = args.first() {
                            let mut inst = instance.borrow_mut();
                            inst.properties.remove(prop_name.as_str());
                            inst.prop_defs.remove(prop_name.as_str());
                            inst.prop_states.remove(prop_name.as_str());
                        }
                        return Ok(Value::Null);
                    }
                    "setPropState" => {
                        // setPropState(@propName, state) - 0=hidden, 1=readonly, 3=editable
                        if let (Some(Value::Symbol(prop_name)), Some(state)) =
                            (args.first(), args.get(1))
                        {
                            let state_val = state.to_int().unwrap_or(3) as i32;
                            instance
                                .borrow_mut()
                                .prop_states
                                .insert(prop_name.to_string(), state_val);
                        }
                        return Ok(Value::Null);
                    }
                    "getPropState" => {
                        if let Some(Value::Symbol(prop_name)) = args.first() {
                            let state = instance
                                .borrow()
                                .prop_states
                                .get(prop_name.as_str())
                                .copied()
                                .unwrap_or(3); // default to editable
                            return Ok(Value::Int(state as i64));
                        }
                        return Ok(Value::Int(3));
                    }
                    "setPropState2" => {
                        // setPropState2(@propName, state) - same as setPropState
                        if let (Some(Value::Symbol(prop_name)), Some(state)) =
                            (args.first(), args.get(1))
                        {
                            let state_val = state.to_int().unwrap_or(3) as i32;
                            instance
                                .borrow_mut()
                                .prop_states
                                .insert(prop_name.to_string(), state_val);
                        }
                        return Ok(Value::Null);
                    }
                    "getPropState2" => {
                        // getPropState2(@propName) - same as getPropState
                        if let Some(Value::Symbol(prop_name)) = args.first() {
                            let state = instance
                                .borrow()
                                .prop_states
                                .get(prop_name.as_str())
                                .copied()
                                .unwrap_or(3);
                            return Ok(Value::Int(state as i64));
                        }
                        return Ok(Value::Int(3));
                    }
                    "getPropertyKeys" => {
                        let keys: Vec<Value> = instance
                            .borrow()
                            .properties
                            .keys()
                            .map(|k| Value::Symbol(Rc::new(k.clone())))
                            .collect();
                        return Ok(Value::Array(Rc::new(RefCell::new(keys))));
                    }
                    "isCurrentPropsChanged" => {
                        // Check if properties have changed since last rule evaluation
                        // For now, return false (no changes)
                        return Ok(Value::Bool(false));
                    }
                    "setProgram" | "checkPropChanges" | "invalidatePicture" | "setArticleSpec"
                    | "getScene" | "getPlanning" | "getInfo" | "_beAnElement"
                    | "_unBeAnElement" | "beAnElement" | "unBeAnElement" | "hierSelectable"
                    | "notHierSelectable" | "setObjState" | "getObjState" | "disableCD"
                    | "enableCD" | "getWorldRotation" | "getWorldPosition" | "getLanguage"
                    | "setLanguage" | "hide" | "show" | "isHidden" | "getProgram"
                    | "setGeometry" | "getGeometry" => {
                        // Stub methods that are called but don't need implementation for geometry
                        return Ok(Value::Null);
                    }
                    "geo" => {
                        // Return self as geometry reference
                        return Ok(Value::Object(instance.clone()));
                    }
                    "setInsertMode" => {
                        // xOiPlGroup insert mode: 0 (default), 1 (intermediate), 2 (inner element)
                        let mode = args.first().and_then(|v| v.to_int()).unwrap_or(0);
                        // Validate mode is 0, 1, or 2 (as per xOiPlGroup::setInsertMode)
                        let valid_mode = if (0..=2).contains(&mode) { mode } else { 0 };
                        instance
                            .borrow_mut()
                            .properties
                            .insert("mXoiInsertMode".to_string(), Value::Int(valid_mode));
                        return Ok(Value::Null);
                    }
                    "getInsertMode" => {
                        // Returns mode or 0 if NULL (per xOiPlGroup::getInsertMode)
                        if let Some(mode) = instance.borrow().properties.get("mXoiInsertMode") {
                            if !matches!(mode, Value::Null) {
                                return Ok(mode.clone());
                            }
                        }
                        return Ok(Value::Int(0));
                    }
                    "setResolution" => {
                        // OFML spec 4.2.5: Float 0.0-1.0, returns self for chaining
                        let res = args.first().and_then(|v| v.to_float()).unwrap_or(0.1);
                        let clamped = res.clamp(0.0, 1.0);
                        instance
                            .borrow_mut()
                            .properties
                            .insert("_resolution".to_string(), Value::Float(clamped));
                        return Ok(Value::Object(instance.clone())); // Return self
                    }
                    "getResolution" => {
                        // OFML spec 4.2.5: Returns Float, initial 0.1
                        if let Some(res) = instance.borrow().properties.get("_resolution") {
                            return Ok(res.clone());
                        }
                        return Ok(Value::Float(0.1));
                    }
                    _ => {}
                }

                // Look up method in class
                let class = instance.borrow().class.clone();
                if let Some(func) = class.methods.get(method) {
                    let old_self = self.current_self.take();
                    self.current_self = Some(instance.clone());
                    let result = self.call_function(func.clone(), args);
                    self.current_self = old_self;
                    return result;
                }

                return Err(InterpError::name_error(format!(
                    "Unknown method: {}",
                    method
                )));
            }
            Value::Null => {
                // Calling any method on Null returns Null (null-safe pattern)
                return Ok(Value::Null);
            }
            Value::String(s) => {
                // Handle string methods
                return self.call_string_method(s.clone(), method, args);
            }
            Value::Int(_) | Value::Float(_) | Value::Bool(_) | Value::Symbol(_) => {
                // Calling method on primitive returns Null (graceful degradation)
                return Ok(Value::Null);
            }
            _ => {}
        }

        Err(InterpError::type_error(format!(
            "Cannot call method on {}",
            obj.type_name()
        )))
    }

    fn call_array_method(
        &mut self,
        arr: Rc<RefCell<Vec<Value>>>,
        method: &str,
        args: Vec<Value>,
    ) -> InterpResult<Value> {
        match method {
            "size" | "length" => Ok(Value::Int(arr.borrow().len() as i64)),
            "empty" => Ok(Value::Bool(arr.borrow().is_empty())),
            "pushBack" | "push" => {
                if let Some(val) = args.first() {
                    arr.borrow_mut().push(val.clone());
                }
                Ok(Value::Null)
            }
            "popBack" | "pop" => Ok(arr.borrow_mut().pop().unwrap_or(Value::Null)),
            "popFront" => {
                let mut arr = arr.borrow_mut();
                if arr.is_empty() {
                    Ok(Value::Null)
                } else {
                    Ok(arr.remove(0))
                }
            }
            "pushFront" => {
                if let Some(val) = args.first() {
                    arr.borrow_mut().insert(0, val.clone());
                }
                Ok(Value::Null)
            }
            "clear" => {
                arr.borrow_mut().clear();
                Ok(Value::Null)
            }
            "find" => {
                // Find element in array, returns index or -1
                if let Some(val) = args.first() {
                    let arr = arr.borrow();
                    for (i, v) in arr.iter().enumerate() {
                        if v.equals(val) {
                            return Ok(Value::Int(i as i64));
                        }
                    }
                }
                Ok(Value::Int(-1))
            }
            "get" => {
                if let Some(idx) = args.first().and_then(|v| v.to_int()) {
                    Ok(arr
                        .borrow()
                        .get(idx as usize)
                        .cloned()
                        .unwrap_or(Value::Null))
                } else {
                    Ok(Value::Null)
                }
            }
            "insert" | "insertAt" => {
                if let (Some(idx), Some(val)) = (args.first().and_then(|v| v.to_int()), args.get(1))
                {
                    let mut arr = arr.borrow_mut();
                    let idx = idx as usize;
                    if idx <= arr.len() {
                        arr.insert(idx, val.clone());
                    }
                }
                Ok(Value::Null)
            }
            "removeAt" | "erase" => {
                if let Some(idx) = args.first().and_then(|v| v.to_int()) {
                    let mut arr = arr.borrow_mut();
                    let idx = idx as usize;
                    if idx < arr.len() {
                        arr.remove(idx);
                    }
                }
                Ok(Value::Null)
            }
            _ => Err(InterpError::name_error(format!(
                "Unknown array method: {}",
                method
            ))),
        }
    }

    fn call_hash_method(
        &mut self,
        hash: Rc<RefCell<HashMap<String, Value>>>,
        method: &str,
        args: Vec<Value>,
    ) -> InterpResult<Value> {
        match method {
            "size" | "length" => Ok(Value::Int(hash.borrow().len() as i64)),
            "empty" => Ok(Value::Bool(hash.borrow().is_empty())),
            "hasKey" | "contains" => {
                if let Some(key) = args.first() {
                    let key_str = key.to_string_val();
                    Ok(Value::Bool(hash.borrow().contains_key(&key_str)))
                } else {
                    Ok(Value::Bool(false))
                }
            }
            "keys" => {
                let keys: Vec<Value> = hash
                    .borrow()
                    .keys()
                    .map(|k| Value::String(Rc::new(k.clone())))
                    .collect();
                Ok(Value::Array(Rc::new(RefCell::new(keys))))
            }
            "clear" => {
                hash.borrow_mut().clear();
                Ok(Value::Null)
            }
            _ => Err(InterpError::name_error(format!(
                "Unknown hash method: {}",
                method
            ))),
        }
    }

    fn call_string_method(
        &mut self,
        s: Rc<String>,
        method: &str,
        args: Vec<Value>,
    ) -> InterpResult<Value> {
        match method {
            "size" | "length" => Ok(Value::Int(s.len() as i64)),
            "empty" => Ok(Value::Bool(s.is_empty())),
            "find" => {
                // find(substring, [startPos]) - returns position or -1 if not found
                let search = args.first().map(|v| v.to_string_val()).unwrap_or_default();
                let start_pos = args.get(1).and_then(|v| v.to_int()).unwrap_or(0) as usize;

                if start_pos >= s.len() {
                    return Ok(Value::Int(-1));
                }

                let haystack = &s[start_pos..];
                match haystack.find(&search) {
                    Some(pos) => Ok(Value::Int((pos + start_pos) as i64)),
                    None => Ok(Value::Int(-1)),
                }
            }
            "substr" => {
                // substr(start, [length]) - extract substring
                let start = args.first().and_then(|v| v.to_int()).unwrap_or(0) as usize;
                let len = args.get(1).and_then(|v| v.to_int());

                if start >= s.len() {
                    return Ok(Value::String(Rc::new(String::new())));
                }

                let result = match len {
                    Some(l) if l > 0 => {
                        let end = (start + l as usize).min(s.len());
                        s.chars().skip(start).take(end - start).collect()
                    }
                    _ => s.chars().skip(start).collect(),
                };
                Ok(Value::String(Rc::new(result)))
            }
            "substring" => {
                // substring(start, end) - extract substring from start to end
                let start = args.first().and_then(|v| v.to_int()).unwrap_or(0) as usize;
                let end = args
                    .get(1)
                    .and_then(|v| v.to_int())
                    .map(|e| e as usize)
                    .unwrap_or(s.len());

                let start = start.min(s.len());
                let end = end.min(s.len());
                let result: String = s
                    .chars()
                    .skip(start)
                    .take(end.saturating_sub(start))
                    .collect();
                Ok(Value::String(Rc::new(result)))
            }
            "toUpper" | "upper" => Ok(Value::String(Rc::new(s.to_uppercase()))),
            "toLower" | "lower" => Ok(Value::String(Rc::new(s.to_lowercase()))),
            "trim" => Ok(Value::String(Rc::new(s.trim().to_string()))),
            "startsWith" => {
                let prefix = args.first().map(|v| v.to_string_val()).unwrap_or_default();
                Ok(Value::Bool(s.starts_with(&prefix)))
            }
            "endsWith" => {
                let suffix = args.first().map(|v| v.to_string_val()).unwrap_or_default();
                Ok(Value::Bool(s.ends_with(&suffix)))
            }
            "replace" => {
                // replace(old, new) - replace first occurrence
                let old = args.first().map(|v| v.to_string_val()).unwrap_or_default();
                let new = args.get(1).map(|v| v.to_string_val()).unwrap_or_default();
                Ok(Value::String(Rc::new(s.replacen(&old, &new, 1))))
            }
            "replaceAll" => {
                // replaceAll(old, new) - replace all occurrences
                let old = args.first().map(|v| v.to_string_val()).unwrap_or_default();
                let new = args.get(1).map(|v| v.to_string_val()).unwrap_or_default();
                Ok(Value::String(Rc::new(s.replace(&old, &new))))
            }
            "split" => {
                // split(separator) - split string into array
                let sep = args
                    .first()
                    .map(|v| v.to_string_val())
                    .unwrap_or_else(|| " ".to_string());
                let parts: Vec<Value> = s
                    .split(&sep)
                    .map(|part| Value::String(Rc::new(part.to_string())))
                    .collect();
                Ok(Value::Array(Rc::new(RefCell::new(parts))))
            }
            "charAt" => {
                // charAt(index) - get character at index
                let idx = args.first().and_then(|v| v.to_int()).unwrap_or(0) as usize;
                match s.chars().nth(idx) {
                    Some(c) => Ok(Value::String(Rc::new(c.to_string()))),
                    None => Ok(Value::String(Rc::new(String::new()))),
                }
            }
            "contains" => {
                let search = args.first().map(|v| v.to_string_val()).unwrap_or_default();
                Ok(Value::Bool(s.contains(&search)))
            }
            "indexOf" => {
                // indexOf is alias for find
                let search = args.first().map(|v| v.to_string_val()).unwrap_or_default();
                let start_pos = args.get(1).and_then(|v| v.to_int()).unwrap_or(0) as usize;

                if start_pos >= s.len() {
                    return Ok(Value::Int(-1));
                }

                let haystack = &s[start_pos..];
                match haystack.find(&search) {
                    Some(pos) => Ok(Value::Int((pos + start_pos) as i64)),
                    None => Ok(Value::Int(-1)),
                }
            }
            _ => {
                // Unknown string method - return Null instead of error for graceful degradation
                Ok(Value::Null)
            }
        }
    }

    /// Public method to instantiate a class with no arguments
    pub fn instantiate_class_public(&mut self, class: Rc<ClassValue>) -> InterpResult<Value> {
        self.instantiate_class(class, vec![])
    }

    fn instantiate_class(
        &mut self,
        class: Rc<ClassValue>,
        args: Vec<Value>,
    ) -> InterpResult<Value> {
        // Check for OFML constructor pattern: ClassName(parent, @name, ...)
        // This creates a child object on the parent with the given name
        let (parent_obj, obj_name, remaining_args) =
            self.parse_ofml_constructor_args(&class.name, &args);

        let instance = Rc::new(RefCell::new(ObjInstance {
            class: class.clone(),
            name: obj_name.clone(),
            ..Default::default()
        }));

        // Set up parent-child relationship
        if let Some(ref parent) = parent_obj {
            instance.borrow_mut().parent = Some(parent.clone());
            parent.borrow_mut().children.push(instance.clone());
            // Register child by name on parent's fields
            parent
                .borrow_mut()
                .fields
                .insert(obj_name.clone(), Value::Object(instance.clone()));
        }

        // Create scene node for geometry classes
        let is_geometry_class = self.is_geometry_class(&class.name);
        if is_geometry_class {
            let parent_node = parent_obj.as_ref().and_then(|p| {
                let obj_id = p.as_ptr() as u64;
                self.obj_to_node.get(&obj_id).cloned()
            });

            // Special handling for OiBlock: create block geometry with dimensions
            let node = if class.name == "OiBlock"
                || self.class_inherits_from(&class.name, "OiBlock")
            {
                // OiBlock(parent, @name, [width, height, depth])
                let dims = match remaining_args.first() {
                    Some(Value::Array(arr)) => {
                        let arr = arr.borrow();
                        [
                            arr.first().and_then(|v| v.to_float()).unwrap_or(1.0) as f32,
                            arr.get(1).and_then(|v| v.to_float()).unwrap_or(1.0) as f32,
                            arr.get(2).and_then(|v| v.to_float()).unwrap_or(1.0) as f32,
                        ]
                    }
                    _ => [1.0, 1.0, 1.0],
                };
                self.scene.create_block(obj_name.clone(), dims, parent_node)
            } else if class.name == "OiCylinder"
                || self.class_inherits_from(&class.name, "OiCylinder")
            {
                // OiCylinder(parent, @name, [radius, height])
                let (radius, height) = match remaining_args.first() {
                    Some(Value::Array(arr)) => {
                        let arr = arr.borrow();
                        (
                            arr.first().and_then(|v| v.to_float()).unwrap_or(0.5) as f32,
                            arr.get(1).and_then(|v| v.to_float()).unwrap_or(1.0) as f32,
                        )
                    }
                    _ => (0.5, 1.0),
                };
                self.scene
                    .create_cylinder(obj_name.clone(), radius, height, parent_node)
            } else if class.name == "OiSphere" || self.class_inherits_from(&class.name, "OiSphere")
            {
                // OiSphere(parent, @name, radius)
                let radius = remaining_args
                    .first()
                    .and_then(|v| v.to_float())
                    .unwrap_or(0.5) as f32;
                self.scene
                    .create_sphere(obj_name.clone(), radius, parent_node)
            } else {
                // Default: create a container node
                self.scene.create_part(obj_name.clone(), parent_node)
            };

            // Map object instance to scene node
            let obj_id = instance.as_ptr() as u64;
            self.obj_to_node.insert(obj_id, node);
        }

        // Initialize instance variables from class declaration AND parent classes
        // Collect inheritance chain from root to current (root first)
        // Use lazy parent resolution - look up parents by name from decl.parent
        let mut class_chain = Vec::new();
        let mut current_class = Some(class.clone());
        while let Some(c) = current_class {
            class_chain.push(c.clone());
            // Try to resolve parent from class.parent first, then fall back to decl.parent
            current_class = if c.parent.is_some() {
                c.parent.clone()
            } else if let Some(ref parent_name) = c.decl.parent {
                // Lazy resolution: look up parent by name
                let parent_short = parent_name.parts.last().unwrap_or(&parent_name.parts[0]);
                self.classes.get(parent_short).cloned()
            } else {
                None
            };
        }
        class_chain.reverse(); // Now ordered from root to derived

        // Initialize variables from each class in the chain (root first, so derived can override)
        for cls in &class_chain {
            for member in &cls.decl.members {
                match member {
                    ClassMember::Var(var) => {
                        // Set self context for evaluating initializers
                        let old_self = self.current_self.take();
                        self.current_self = Some(instance.clone());
                        let value = if let Some(ref init) = var.initializer {
                            self.evaluate(init)?
                        } else {
                            Value::Null
                        };
                        self.current_self = old_self;
                        instance.borrow_mut().fields.insert(var.name.clone(), value);
                    }
                    ClassMember::Expr(expr) => {
                        // Execute class-level expressions with self set
                        let old_self = self.current_self.take();
                        self.current_self = Some(instance.clone());
                        self.evaluate(expr)?;
                        self.current_self = old_self;
                    }
                    _ => {}
                }
            }
        }

        // Call initialize if it exists
        if class.methods.contains_key("initialize") {
            let old_self = self.current_self.take();
            self.current_self = Some(instance.clone());
            // Pass the full original args to initialize (OFML pattern)
            self.call_method(&Value::Object(instance.clone()), "initialize", args)?;
            self.current_self = old_self;
        }

        Ok(Value::Object(instance))
    }

    /// Parse OFML constructor pattern: ClassName(parent, @name, ...)
    /// Returns (parent_object, object_name, remaining_args)
    fn parse_ofml_constructor_args(
        &self,
        class_name: &str,
        args: &[Value],
    ) -> (Option<Rc<RefCell<ObjInstance>>>, String, Vec<Value>) {
        // Check if this looks like OFML pattern: first arg is object, second is symbol
        if args.len() >= 2 {
            if let (Value::Object(parent), Value::Symbol(name)) = (&args[0], &args[1]) {
                let remaining = if args.len() > 2 {
                    args[2..].to_vec()
                } else {
                    vec![]
                };
                return (Some(parent.clone()), name.to_string(), remaining);
            }
        }

        // Not OFML pattern, generate a name
        let name = format!("{}_{}", class_name.to_lowercase(), self.next_obj_id);
        (None, name, args.to_vec())
    }

    /// Check if a class is a geometry class that should create scene nodes
    fn is_geometry_class(&self, class_name: &str) -> bool {
        matches!(
            class_name,
            "OiObject" | "OiPart" | "OiBlock" | "OiCylinder" | "OiSphere" |
            // Also handle user-defined classes that extend these
            "aWKPart" | "aWKPlate2" | "aWKContainer" | "xOiPlGroup"
        ) || self.class_inherits_from(class_name, "OiPart")
            || self.class_inherits_from(class_name, "OiObject")
    }

    /// Check if a class inherits from a base class
    fn class_inherits_from(&self, class_name: &str, base_name: &str) -> bool {
        if let Some(class) = self.classes.get(class_name) {
            // Use lazy parent resolution
            let mut current = Some(class.clone());
            while let Some(c) = current {
                if c.name == base_name {
                    return true;
                }
                current = if c.parent.is_some() {
                    c.parent.clone()
                } else if let Some(ref parent_name) = c.decl.parent {
                    let parent_short = parent_name.parts.last().unwrap_or(&parent_name.parts[0]);
                    self.classes.get(parent_short).cloned()
                } else {
                    None
                };
            }
        }
        false
    }

    /// Check if a method name is a built-in object method
    fn is_builtin_method(&self, name: &str) -> bool {
        matches!(
            name,
            "setPosition"
                | "getPosition"
                | "setAlignment"
                | "rotate"
                | "getRotation"
                | "getLocalBounds"
                | "setMaterial"
                | "notSelectable"
                | "setCutable"
                | "setRtAxis"
                | "setTrAxis"
                | "set2DName"
                | "getName"
                | "getElements"
                | "getPropValue"
                | "setPropValue"
                | "hasProperty"
                | "hasMember"
                | "isA"
                | "getFather"
                | "getChildren"
                | "setProgram"
                | "checkPropChanges"
                | "invalidatePicture"
                | "setArticleSpec"
                | "removeProperty"
                | "setPropState"
                | "setPropState2"
                | "getPropState2"
                | "getPropertyKeys"
                | "getScene"
                | "getPlanning"
                | "getInfo"
                | "setupProperty"
                | "getPropState"
                | "isCurrentPropsChanged"
                | "setFootAlignment"
                | "getProductDB"
                | "getDataRootDir"
                | "getClass"
                | "setState"
                | "getState"
                | "setDimensions"
                | "getDimensions"
                | "isCat"
                | "article2Class"
        )
    }

    fn set_member(&mut self, obj: &Value, member: &str, value: Value) -> InterpResult<()> {
        match obj {
            Value::Object(instance) => {
                instance
                    .borrow_mut()
                    .fields
                    .insert(member.to_string(), value);
                Ok(())
            }
            _ => Err(InterpError::type_error("Cannot set member on this type")),
        }
    }

    fn set_index(&mut self, obj: &Value, index: &Value, value: Value) -> InterpResult<()> {
        match obj {
            Value::Array(arr) => {
                let idx = index
                    .to_int()
                    .ok_or_else(|| InterpError::type_error("Array index must be integer"))?;
                let idx = idx as usize;
                let mut arr = arr.borrow_mut();
                if idx < arr.len() {
                    arr[idx] = value;
                } else {
                    // Extend array if needed
                    while arr.len() <= idx {
                        arr.push(Value::Null);
                    }
                    arr[idx] = value;
                }
                Ok(())
            }
            Value::Hash(hash) => {
                let key = match index {
                    Value::Symbol(s) => s.to_string(),
                    Value::String(s) => s.to_string(),
                    _ => index.to_string_val(),
                };
                hash.borrow_mut().insert(key, value);
                Ok(())
            }
            Value::Null => {
                // Assigning to Null index is a no-op (silently ignore)
                Ok(())
            }
            Value::Object(instance) => {
                // Object indexing - use property assignment
                let key = match index {
                    Value::Symbol(s) => s.to_string(),
                    Value::String(s) => s.to_string(),
                    _ => index.to_string_val(),
                };
                instance.borrow_mut().properties.insert(key, value);
                Ok(())
            }
            _ => Err(InterpError::type_error("Cannot index this type")),
        }
    }

    fn eval_index(&mut self, idx: &IndexExpr) -> InterpResult<Value> {
        let obj = self.evaluate(&idx.object)?;
        let index = self.evaluate(&idx.index)?;

        match obj {
            Value::Array(arr) => {
                let i = index
                    .to_int()
                    .ok_or_else(|| InterpError::type_error("Array index must be integer"))?;
                Ok(arr.borrow().get(i as usize).cloned().unwrap_or(Value::Null))
            }
            Value::Hash(hash) => {
                let key = match index {
                    Value::Symbol(s) => s.to_string(),
                    Value::String(s) => s.to_string(),
                    _ => index.to_string_val(),
                };
                Ok(hash.borrow().get(&key).cloned().unwrap_or(Value::Null))
            }
            Value::String(s) => {
                let i = index
                    .to_int()
                    .ok_or_else(|| InterpError::type_error("String index must be integer"))?;
                // Return single character as a string (OFML behavior)
                Ok(s.chars()
                    .nth(i as usize)
                    .map(|c| Value::String(Rc::new(c.to_string())))
                    .unwrap_or(Value::Null))
            }
            Value::Null => {
                // Indexing Null returns Null (common pattern in dynamic languages)
                Ok(Value::Null)
            }
            Value::Object(instance) => {
                // Object indexing - use property access
                let key = match index {
                    Value::Symbol(s) => s.to_string(),
                    Value::String(s) => s.to_string(),
                    _ => index.to_string_val(),
                };
                Ok(instance
                    .borrow()
                    .properties
                    .get(&key)
                    .cloned()
                    .unwrap_or(Value::Null))
            }
            Value::NativeFunc(_) | Value::Func(_) => {
                // Indexing a function returns Null (common for undefined variables used as functions)
                Ok(Value::Null)
            }
            Value::Int(_) | Value::Float(_) | Value::Bool(_) | Value::Symbol(_) => {
                // Indexing a primitive type returns Null (graceful degradation)
                Ok(Value::Null)
            }
            other => Err(InterpError::type_error(&format!(
                "Cannot index type {:?}",
                std::mem::discriminant(&other)
            ))),
        }
    }

    fn eval_range(&mut self, range: &RangeExpr) -> InterpResult<Value> {
        let obj = self.evaluate(&range.object)?;
        let start = range.start.as_ref().map(|e| self.evaluate(e)).transpose()?;
        let end = range.end.as_ref().map(|e| self.evaluate(e)).transpose()?;

        match obj {
            Value::Array(arr) => {
                let arr = arr.borrow();
                let start_idx = start.and_then(|v| v.to_int()).unwrap_or(0) as usize;
                let end_idx = end.and_then(|v| v.to_int()).unwrap_or(arr.len() as i64) as usize;
                let slice: Vec<Value> = arr
                    .get(start_idx..end_idx)
                    .map(|s| s.to_vec())
                    .unwrap_or_default();
                Ok(Value::Array(Rc::new(RefCell::new(slice))))
            }
            Value::String(s) => {
                let start_idx = start.and_then(|v| v.to_int()).unwrap_or(0) as usize;
                let end_idx = end.and_then(|v| v.to_int()).unwrap_or(s.len() as i64) as usize;
                let slice: String = s
                    .chars()
                    .skip(start_idx)
                    .take(end_idx - start_idx)
                    .collect();
                Ok(Value::String(Rc::new(slice)))
            }
            _ => Err(InterpError::type_error("Cannot slice this type")),
        }
    }

    fn eval_member(&mut self, member: &MemberExpr) -> InterpResult<Value> {
        let obj = self.evaluate(&member.object)?;

        match obj {
            Value::Object(instance) => {
                // Check fields first
                if let Some(val) = instance.borrow().fields.get(&member.member) {
                    return Ok(val.clone());
                }
                // Check properties
                if let Some(val) = instance.borrow().properties.get(&member.member) {
                    return Ok(val.clone());
                }
                // Handle special built-in members
                match member.member.as_str() {
                    "geo" => {
                        // Return self as geometry reference
                        return Ok(Value::Object(instance.clone()));
                    }
                    "parent" => {
                        // Return parent object if any
                        if let Some(ref p) = instance.borrow().parent {
                            return Ok(Value::Object(p.clone()));
                        }
                        return Ok(Value::Null);
                    }
                    _ => {}
                }
                // Return method reference for later call
                let class = instance.borrow().class.clone();
                if let Some(func) = class.methods.get(&member.member) {
                    return Ok(Value::Func(func.clone()));
                }
                // For unknown members, return Null instead of error (lenient)
                Ok(Value::Null)
            }
            Value::Class(class) => {
                // Static member access
                if let Some(val) = class.static_vars.get(&member.member) {
                    return Ok(val.clone());
                }
                if let Some(func) = class.methods.get(&member.member) {
                    return Ok(Value::Func(func.clone()));
                }
                Err(InterpError::name_error(format!(
                    "Unknown static member: {}",
                    member.member
                )))
            }
            Value::Null => {
                // Accessing member on Null returns Null (null-safe pattern)
                Ok(Value::Null)
            }
            _ => Err(InterpError::type_error(format!(
                "Cannot access member on {}",
                obj.type_name()
            ))),
        }
    }

    fn eval_instanceof(&mut self, inst: &InstanceofExpr) -> InterpResult<Value> {
        let obj = self.evaluate(&inst.expr)?;
        let type_val = self.evaluate(&inst.type_name)?;

        match (&obj, &type_val) {
            (Value::Object(instance), Value::Class(class)) => {
                Ok(Value::Bool(instance.borrow().is_a(&class.name)))
            }
            _ => Ok(Value::Bool(false)),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    #[test]
    fn test_class_package_tracking() {
        let source = r#"
            package ::vitra::basics;

            public class VitraOiBTGPlElement3 : xOiBTGPlElement3 {
                public func initialize(pFa, pNa) {
                    // Parent call
                }
            }
        "#;

        let mut parser = Parser::new(source).unwrap();
        let ast = parser.parse().unwrap();

        let mut interp = Interpreter::new();
        interp.execute(&ast).unwrap();

        // Check class is registered with correct package
        let class = interp.classes.get("VitraOiBTGPlElement3").unwrap();
        assert_eq!(class.package, "::vitra::basics");
        assert_eq!(class.name, "VitraOiBTGPlElement3");
    }

    #[test]
    fn test_class_qualified_name() {
        let source = r#"
            package ::vitra::basics;
            public class TestClass : xOiBTGPlElement3 {}
        "#;

        let mut parser = Parser::new(source).unwrap();
        let ast = parser.parse().unwrap();

        let mut interp = Interpreter::new();
        interp.execute(&ast).unwrap();

        let class = interp.classes.get("TestClass").unwrap();
        assert_eq!(class.qualified_name(), "::vitra::basics::TestClass");
    }

    #[test]
    fn test_qualified_class_lookup() {
        let source = r#"
            package ::vitra::basics;
            public class UniqueClass : xOiBTGPlElement3 {}
        "#;

        let mut parser = Parser::new(source).unwrap();
        let ast = parser.parse().unwrap();

        let mut interp = Interpreter::new();
        interp.execute(&ast).unwrap();

        // Verify class is registered in qualified_classes map
        let fq_name = "::vitra::basics::UniqueClass";
        assert!(interp.qualified_classes.contains_key(fq_name));

        let class = interp.qualified_classes.get(fq_name).unwrap();
        assert_eq!(class.name, "UniqueClass");
        assert_eq!(class.package, "::vitra::basics");
    }

    #[test]
    fn test_multiple_classes_same_short_name_different_packages() {
        // First package
        let source1 = r#"
            package ::vitra::basics;
            public class SharedName : xOiBTGPlElement3 {
                public func getValue() { return 1; }
            }
        "#;

        // Second package with same class name
        let source2 = r#"
            package ::vitra::workit;
            public class SharedName : xOiBTGPlElement3 {
                public func getValue() { return 2; }
            }
        "#;

        let mut parser1 = Parser::new(source1).unwrap();
        let ast1 = parser1.parse().unwrap();

        let mut parser2 = Parser::new(source2).unwrap();
        let ast2 = parser2.parse().unwrap();

        let mut interp = Interpreter::new();
        interp.execute(&ast1).unwrap();
        interp.execute(&ast2).unwrap();

        // Both should exist in qualified_classes with different keys
        assert!(interp
            .qualified_classes
            .contains_key("::vitra::basics::SharedName"));
        assert!(interp
            .qualified_classes
            .contains_key("::vitra::workit::SharedName"));

        // short name lookup returns last registered (workit)
        let short_class = interp.classes.get("SharedName").unwrap();
        assert_eq!(short_class.package, "::vitra::workit");

        // Qualified lookups return correct classes
        let basics_class = interp
            .qualified_classes
            .get("::vitra::basics::SharedName")
            .unwrap();
        assert_eq!(basics_class.package, "::vitra::basics");

        let workit_class = interp
            .qualified_classes
            .get("::vitra::workit::SharedName")
            .unwrap();
        assert_eq!(workit_class.package, "::vitra::workit");
    }

    #[test]
    fn test_parent_class_inheritance_chain() {
        let source = r#"
            package ::vitra::basics;

            public class VitraOiBTGPlElement3 : xOiBTGPlElement3 {
                public func initialize(pFa, pNa) {}
            }

            public class ViOiBTGPlElement3 : VitraOiBTGPlElement3 {
                public func initialize(pFa, pNa) {
                    // Would call ::vitra::basics::VitraOiBTGPlElement3::initialize(pFa, pNa);
                }
            }
        "#;

        let mut parser = Parser::new(source).unwrap();
        let ast = parser.parse().unwrap();

        let mut interp = Interpreter::new();
        interp.execute(&ast).unwrap();

        // Verify inheritance chain
        let vi_class = interp.classes.get("ViOiBTGPlElement3").unwrap();
        assert!(vi_class.parent.is_some());

        let parent = vi_class.parent.as_ref().unwrap();
        assert_eq!(parent.name, "VitraOiBTGPlElement3");
        assert_eq!(parent.package, "::vitra::basics");
    }

    #[test]
    fn test_native_classes_have_empty_package() {
        let interp = Interpreter::new();

        // Native classes should have empty package
        let oi_object = interp.classes.get("OiObject").unwrap();
        assert_eq!(oi_object.package, "");

        let x_oi_btg = interp.classes.get("xOiBTGPlElement3").unwrap();
        assert_eq!(x_oi_btg.package, "");
    }
}
