//! OFML Framework Classes - Implementation of Go* and Oi* classes.
//!
//! This module implements the OFML framework classes used for geometry
//! transformation and product configuration.
//!
//! ## Namespaces
//!
//! - `::ofml::go::*` - Geometry Operation classes (GoYLTrans, GoMirror, etc.)
//! - `::ofml::oi::*` - Object Instance classes (OiPart, OiBlock, etc.)
//! - `::ofml::xoi::*` - Extended Object Instance classes

use crate::errors::OfmlClassError;

/// OFML framework class identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OfmlClassType {
    // ::ofml::go namespace - Geometry Operations
    /// GoMetaType - Base class for configurable products
    GoMetaType,
    /// GoXLRTransYLRTrans - X/Y combined stretching
    GoXLRTransYLRTrans,
    /// GoYLTrans - Y-axis (height) stretching
    GoYLTrans,
    /// GoXLTrans - X-axis (width) stretching
    GoXLTrans,
    /// GoZLTrans - Z-axis (depth) stretching
    GoZLTrans,
    /// GoMirror - Geometry mirroring
    GoMirror,

    // ::ofml::oi namespace - Object Instances
    /// OiPart - Base part class
    OiPart,
    /// OiBlock - Axis-aligned cuboid
    OiBlock,
    /// OiCylinder - Cylindrical geometry
    OiCylinder,
    /// OiEllipsoid - Ellipsoid geometry
    OiEllipsoid,
    /// OiSphere - Spherical geometry
    OiSphere,
    /// OiPolygon - Planar convex polygon
    OiPolygon,
    /// OiImport - External geometry import
    OiImport,
    /// OiFrame - Frame/border geometry
    OiFrame,
    /// OiRotation - Rotational sweep geometry
    OiRotation,
    /// OiSweep - Extrusion/sweep geometry
    OiSweep,

    /// Custom/unknown class
    Custom(String),
}

impl OfmlClassType {
    /// Parse a class name string to OfmlClassType.
    ///
    /// # Arguments
    ///
    /// * `name` - Fully qualified class name (e.g., "::ofml::go::GoYLTrans")
    ///
    /// # Returns
    ///
    /// The corresponding OfmlClassType, or Custom for unknown classes.
    pub fn from_name(name: &str) -> Self {
        // Normalize: remove leading colons, lowercase compare
        let normalized = name.trim_start_matches(':');

        match normalized {
            // Go classes
            "ofml::go::GoMetaType" | "GoMetaType" => OfmlClassType::GoMetaType,
            "ofml::go::GoXLRTransYLRTrans" | "GoXLRTransYLRTrans" => {
                OfmlClassType::GoXLRTransYLRTrans
            }
            "ofml::go::GoYLTrans" | "GoYLTrans" => OfmlClassType::GoYLTrans,
            "ofml::go::GoXLTrans" | "GoXLTrans" => OfmlClassType::GoXLTrans,
            "ofml::go::GoZLTrans" | "GoZLTrans" => OfmlClassType::GoZLTrans,
            "ofml::go::GoMirror" | "GoMirror" => OfmlClassType::GoMirror,

            // Oi classes
            "ofml::oi::OiPart" | "OiPart" => OfmlClassType::OiPart,
            "ofml::oi::OiBlock" | "OiBlock" => OfmlClassType::OiBlock,
            "ofml::oi::OiCylinder" | "OiCylinder" => OfmlClassType::OiCylinder,
            "ofml::oi::OiEllipsoid" | "OiEllipsoid" => OfmlClassType::OiEllipsoid,
            "ofml::oi::OiSphere" | "OiSphere" => OfmlClassType::OiSphere,
            "ofml::oi::OiPolygon" | "OiPolygon" => OfmlClassType::OiPolygon,
            "ofml::oi::OiImport" | "OiImport" => OfmlClassType::OiImport,
            "ofml::oi::OiFrame" | "OiFrame" => OfmlClassType::OiFrame,
            "ofml::oi::OiRotation" | "OiRotation" => OfmlClassType::OiRotation,
            "ofml::oi::OiSweep" | "OiSweep" => OfmlClassType::OiSweep,

            _ => OfmlClassType::Custom(name.to_string()),
        }
    }

    /// Get the fully qualified class name.
    pub fn full_name(&self) -> String {
        match self {
            OfmlClassType::GoMetaType => "::ofml::go::GoMetaType".to_string(),
            OfmlClassType::GoXLRTransYLRTrans => "::ofml::go::GoXLRTransYLRTrans".to_string(),
            OfmlClassType::GoYLTrans => "::ofml::go::GoYLTrans".to_string(),
            OfmlClassType::GoXLTrans => "::ofml::go::GoXLTrans".to_string(),
            OfmlClassType::GoZLTrans => "::ofml::go::GoZLTrans".to_string(),
            OfmlClassType::GoMirror => "::ofml::go::GoMirror".to_string(),

            OfmlClassType::OiPart => "::ofml::oi::OiPart".to_string(),
            OfmlClassType::OiBlock => "::ofml::oi::OiBlock".to_string(),
            OfmlClassType::OiCylinder => "::ofml::oi::OiCylinder".to_string(),
            OfmlClassType::OiEllipsoid => "::ofml::oi::OiEllipsoid".to_string(),
            OfmlClassType::OiSphere => "::ofml::oi::OiSphere".to_string(),
            OfmlClassType::OiPolygon => "::ofml::oi::OiPolygon".to_string(),
            OfmlClassType::OiImport => "::ofml::oi::OiImport".to_string(),
            OfmlClassType::OiFrame => "::ofml::oi::OiFrame".to_string(),
            OfmlClassType::OiRotation => "::ofml::oi::OiRotation".to_string(),
            OfmlClassType::OiSweep => "::ofml::oi::OiSweep".to_string(),

            OfmlClassType::Custom(name) => name.clone(),
        }
    }

    /// Check if this is a Go (geometry operation) class.
    pub fn is_go_class(&self) -> bool {
        matches!(
            self,
            OfmlClassType::GoMetaType
                | OfmlClassType::GoXLRTransYLRTrans
                | OfmlClassType::GoYLTrans
                | OfmlClassType::GoXLTrans
                | OfmlClassType::GoZLTrans
                | OfmlClassType::GoMirror
        )
    }

    /// Check if this is an Oi (object instance) class.
    pub fn is_oi_class(&self) -> bool {
        matches!(
            self,
            OfmlClassType::OiPart
                | OfmlClassType::OiBlock
                | OfmlClassType::OiCylinder
                | OfmlClassType::OiEllipsoid
                | OfmlClassType::OiSphere
                | OfmlClassType::OiPolygon
                | OfmlClassType::OiImport
                | OfmlClassType::OiFrame
                | OfmlClassType::OiRotation
                | OfmlClassType::OiSweep
        )
    }
}

/// Parameters for GoYLTrans (Y-axis stretching).
///
/// Stretches geometry in the Y-axis direction.
#[derive(Debug, Clone)]
pub struct GoYLTransParams {
    /// Base height before stretching
    pub base_height: f64,
    /// Target height after stretching
    pub target_height: f64,
    /// Minimum Y value to begin stretching
    pub stretch_min: f64,
}

impl GoYLTransParams {
    /// Create from parameter slice.
    pub fn from_params(params: &[f64]) -> Result<Self, OfmlClassError> {
        if params.len() < 3 {
            return Err(OfmlClassError::InvalidParamCount {
                class: "GoYLTrans".to_string(),
                expected: 3,
                got: params.len(),
            });
        }
        Ok(Self {
            base_height: params[0],
            target_height: params[1],
            stretch_min: params[2],
        })
    }

    /// Apply transformation to a Y coordinate.
    pub fn transform_y(&self, y: f64) -> f64 {
        if y <= self.stretch_min {
            y
        } else {
            let scale = self.target_height / self.base_height;
            self.stretch_min + (y - self.stretch_min) * scale
        }
    }
}

/// Parameters for GoXLTrans (X-axis stretching).
#[derive(Debug, Clone)]
pub struct GoXLTransParams {
    /// Base width before stretching
    pub base_width: f64,
    /// Target width after stretching
    pub target_width: f64,
    /// Minimum X value to begin stretching
    pub stretch_min: f64,
}

impl GoXLTransParams {
    /// Create from parameter slice.
    pub fn from_params(params: &[f64]) -> Result<Self, OfmlClassError> {
        if params.len() < 3 {
            return Err(OfmlClassError::InvalidParamCount {
                class: "GoXLTrans".to_string(),
                expected: 3,
                got: params.len(),
            });
        }
        Ok(Self {
            base_width: params[0],
            target_width: params[1],
            stretch_min: params[2],
        })
    }

    /// Apply transformation to an X coordinate.
    pub fn transform_x(&self, x: f64) -> f64 {
        if x <= self.stretch_min {
            x
        } else {
            let scale = self.target_width / self.base_width;
            self.stretch_min + (x - self.stretch_min) * scale
        }
    }
}

/// Parameters for GoZLTrans (Z-axis stretching).
#[derive(Debug, Clone)]
pub struct GoZLTransParams {
    /// Base depth before stretching
    pub base_depth: f64,
    /// Target depth after stretching
    pub target_depth: f64,
    /// Minimum Z value to begin stretching
    pub stretch_min: f64,
}

impl GoZLTransParams {
    /// Create from parameter slice.
    pub fn from_params(params: &[f64]) -> Result<Self, OfmlClassError> {
        if params.len() < 3 {
            return Err(OfmlClassError::InvalidParamCount {
                class: "GoZLTrans".to_string(),
                expected: 3,
                got: params.len(),
            });
        }
        Ok(Self {
            base_depth: params[0],
            target_depth: params[1],
            stretch_min: params[2],
        })
    }

    /// Apply transformation to a Z coordinate.
    pub fn transform_z(&self, z: f64) -> f64 {
        if z <= self.stretch_min {
            z
        } else {
            let scale = self.target_depth / self.base_depth;
            self.stretch_min + (z - self.stretch_min) * scale
        }
    }
}

/// Parameters for GoXLRTransYLRTrans (X/Y combined stretching).
#[derive(Debug, Clone)]
pub struct GoXLRTransYLRTransParams {
    /// X-axis minimum stretch point
    pub x_min: f64,
    /// X-axis maximum stretch point
    pub x_max: f64,
    /// X-axis scale factor
    pub x_scale: f64,
    /// Y-axis minimum stretch point
    pub y_min: f64,
    /// Y-axis maximum stretch point
    pub y_max: f64,
    /// Y-axis scale factor
    pub y_scale: f64,
}

impl GoXLRTransYLRTransParams {
    /// Create from parameter slice.
    pub fn from_params(params: &[f64]) -> Result<Self, OfmlClassError> {
        if params.len() < 6 {
            return Err(OfmlClassError::InvalidParamCount {
                class: "GoXLRTransYLRTrans".to_string(),
                expected: 6,
                got: params.len(),
            });
        }
        Ok(Self {
            x_min: params[0],
            x_max: params[1],
            x_scale: params[2],
            y_min: params[3],
            y_max: params[4],
            y_scale: params[5],
        })
    }

    /// Apply transformation to X and Y coordinates.
    pub fn transform(&self, x: f64, y: f64) -> (f64, f64) {
        let new_x = if x < self.x_min {
            x
        } else if x > self.x_max {
            self.x_max + (x - self.x_max) * self.x_scale
        } else {
            self.x_min + (x - self.x_min) * self.x_scale
        };

        let new_y = if y < self.y_min {
            y
        } else if y > self.y_max {
            self.y_max + (y - self.y_max) * self.y_scale
        } else {
            self.y_min + (y - self.y_min) * self.y_scale
        };

        (new_x, new_y)
    }
}

/// Mirror axis for GoMirror.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MirrorAxis {
    /// Mirror across YZ plane (flip X)
    X,
    /// Mirror across XZ plane (flip Y)
    Y,
    /// Mirror across XY plane (flip Z)
    Z,
}

/// Parameters for GoMirror (geometry mirroring).
#[derive(Debug, Clone)]
pub struct GoMirrorParams {
    /// Mirror axis
    pub axis: MirrorAxis,
    /// Mirror plane offset
    pub offset: f64,
}

impl GoMirrorParams {
    /// Create from parameter slice.
    ///
    /// Parameters: [axis_index, offset]
    /// - axis_index: 0=X, 1=Y, 2=Z
    pub fn from_params(params: &[f64]) -> Result<Self, OfmlClassError> {
        if params.is_empty() {
            return Err(OfmlClassError::InvalidParamCount {
                class: "GoMirror".to_string(),
                expected: 1,
                got: 0,
            });
        }

        let axis = match params[0] as i32 {
            0 => MirrorAxis::X,
            1 => MirrorAxis::Y,
            2 => MirrorAxis::Z,
            other => {
                return Err(OfmlClassError::InvalidParamType {
                    class: "GoMirror".to_string(),
                    index: 0,
                    expected: "0, 1, or 2",
                    got: other.to_string(),
                })
            }
        };

        let offset = params.get(1).copied().unwrap_or(0.0);

        Ok(Self { axis, offset })
    }

    /// Apply mirror transformation to a coordinate.
    pub fn transform(&self, x: f64, y: f64, z: f64) -> (f64, f64, f64) {
        match self.axis {
            MirrorAxis::X => (2.0 * self.offset - x, y, z),
            MirrorAxis::Y => (x, 2.0 * self.offset - y, z),
            MirrorAxis::Z => (x, y, 2.0 * self.offset - z),
        }
    }
}

/// Result of OFML class instantiation - represents the transformation or geometry to create.
#[derive(Debug, Clone)]
pub enum OfmlClassInstance {
    /// Geometry transformation - apply to loaded geometry
    Transform(GeometryTransform),
    /// Primitive geometry - create directly
    Primitive(PrimitiveGeometry),
    /// No-op - class doesn't produce geometry
    None,
}

/// A geometry transformation to apply to loaded meshes.
#[derive(Debug, Clone)]
pub enum GeometryTransform {
    /// Y-axis stretching (GoYLTrans)
    YStretch(GoYLTransParams),
    /// X-axis stretching (GoXLTrans)
    XStretch(GoXLTransParams),
    /// Z-axis stretching (GoZLTrans)
    ZStretch(GoZLTransParams),
    /// X/Y combined stretching (GoXLRTransYLRTrans)
    XYStretch(GoXLRTransYLRTransParams),
    /// Mirroring (GoMirror)
    Mirror(GoMirrorParams),
    /// Scale uniformly
    Scale([f64; 3]),
}

/// Primitive geometry to create.
#[derive(Debug, Clone)]
pub enum PrimitiveGeometry {
    /// Block with dimensions [width, height, depth]
    Block { dimensions: [f64; 3] },
    /// Cylinder with radius and height
    Cylinder { radius: f64, height: f64 },
    /// Sphere with radius
    Sphere { radius: f64 },
    /// Ellipsoid with radii [rx, ry, rz]
    Ellipsoid { radii: [f64; 3] },
    /// Polygon with 2D vertices and thickness
    Polygon {
        vertices: Vec<[f64; 2]>,
        thickness: f64,
    },
}

/// OFML class registry for looking up and instantiating classes.
pub struct OfmlClassRegistry;

impl OfmlClassRegistry {
    /// Look up a class by name.
    pub fn lookup(name: &str) -> Option<OfmlClassType> {
        let class_type = OfmlClassType::from_name(name);
        match class_type {
            OfmlClassType::Custom(_) => None,
            _ => Some(class_type),
        }
    }

    /// Check if a class name is a known OFML class.
    pub fn is_known(name: &str) -> bool {
        Self::lookup(name).is_some()
    }

    /// Get the expected parameter count for a class.
    pub fn expected_params(class_type: &OfmlClassType) -> usize {
        match class_type {
            OfmlClassType::GoYLTrans => 3,
            OfmlClassType::GoXLTrans => 3,
            OfmlClassType::GoZLTrans => 3,
            OfmlClassType::GoXLRTransYLRTrans => 6,
            OfmlClassType::GoMirror => 1,    // minimum, can have 2
            OfmlClassType::OiBlock => 3,     // dimensions [x, y, z]
            OfmlClassType::OiCylinder => 2,  // radius, height
            OfmlClassType::OiSphere => 1,    // radius
            OfmlClassType::OiEllipsoid => 3, // rx, ry, rz
            _ => 0,
        }
    }

    /// Instantiate an OFML class with parameters.
    ///
    /// This is the main entry point for processing `clsref` results from EBASE expressions.
    ///
    /// # Arguments
    ///
    /// * `class_name` - Fully qualified class name (e.g., "::ofml::go::GoYLTrans")
    /// * `params` - Constructor parameters from the EBASE expression
    ///
    /// # Returns
    ///
    /// An `OfmlClassInstance` describing the transformation or geometry to create.
    ///
    /// # Example
    ///
    /// ```
    /// use ofml_lib::ofml_classes::{OfmlClassRegistry, OfmlClassInstance, GeometryTransform};
    ///
    /// let instance = OfmlClassRegistry::instantiate("::ofml::go::GoYLTrans", &[100.0, 200.0, 10.0]);
    /// match instance {
    ///     Ok(OfmlClassInstance::Transform(GeometryTransform::YStretch(params))) => {
    ///         assert_eq!(params.base_height, 100.0);
    ///         assert_eq!(params.target_height, 200.0);
    ///     }
    ///     _ => panic!("Expected YStretch transform"),
    /// }
    /// ```
    pub fn instantiate(
        class_name: &str,
        params: &[f64],
    ) -> Result<OfmlClassInstance, OfmlClassError> {
        let class_type = OfmlClassType::from_name(class_name);

        match class_type {
            // Geometry transformation classes
            OfmlClassType::GoYLTrans => {
                let trans = GoYLTransParams::from_params(params)?;
                Ok(OfmlClassInstance::Transform(GeometryTransform::YStretch(
                    trans,
                )))
            }
            OfmlClassType::GoXLTrans => {
                let trans = GoXLTransParams::from_params(params)?;
                Ok(OfmlClassInstance::Transform(GeometryTransform::XStretch(
                    trans,
                )))
            }
            OfmlClassType::GoZLTrans => {
                let trans = GoZLTransParams::from_params(params)?;
                Ok(OfmlClassInstance::Transform(GeometryTransform::ZStretch(
                    trans,
                )))
            }
            OfmlClassType::GoXLRTransYLRTrans => {
                let trans = GoXLRTransYLRTransParams::from_params(params)?;
                Ok(OfmlClassInstance::Transform(GeometryTransform::XYStretch(
                    trans,
                )))
            }
            OfmlClassType::GoMirror => {
                let trans = GoMirrorParams::from_params(params)?;
                Ok(OfmlClassInstance::Transform(GeometryTransform::Mirror(
                    trans,
                )))
            }
            OfmlClassType::GoMetaType => {
                // GoMetaType is a base class, doesn't produce direct geometry
                // The scale parameters are typically [width, depth, height]
                if params.len() >= 3 {
                    Ok(OfmlClassInstance::Transform(GeometryTransform::Scale([
                        params[0], params[1], params[2],
                    ])))
                } else {
                    Ok(OfmlClassInstance::None)
                }
            }

            // Primitive geometry classes
            OfmlClassType::OiBlock => {
                if params.len() >= 3 {
                    Ok(OfmlClassInstance::Primitive(PrimitiveGeometry::Block {
                        dimensions: [params[0], params[1], params[2]],
                    }))
                } else {
                    Err(OfmlClassError::InvalidParamCount {
                        class: "OiBlock".to_string(),
                        expected: 3,
                        got: params.len(),
                    })
                }
            }
            OfmlClassType::OiCylinder => {
                if params.len() >= 2 {
                    Ok(OfmlClassInstance::Primitive(PrimitiveGeometry::Cylinder {
                        radius: params[0],
                        height: params[1],
                    }))
                } else {
                    Err(OfmlClassError::InvalidParamCount {
                        class: "OiCylinder".to_string(),
                        expected: 2,
                        got: params.len(),
                    })
                }
            }
            OfmlClassType::OiSphere => {
                if !params.is_empty() {
                    Ok(OfmlClassInstance::Primitive(PrimitiveGeometry::Sphere {
                        radius: params[0],
                    }))
                } else {
                    Err(OfmlClassError::InvalidParamCount {
                        class: "OiSphere".to_string(),
                        expected: 1,
                        got: 0,
                    })
                }
            }
            OfmlClassType::OiEllipsoid => {
                if params.len() >= 3 {
                    Ok(OfmlClassInstance::Primitive(PrimitiveGeometry::Ellipsoid {
                        radii: [params[0], params[1], params[2]],
                    }))
                } else {
                    Err(OfmlClassError::InvalidParamCount {
                        class: "OiEllipsoid".to_string(),
                        expected: 3,
                        got: params.len(),
                    })
                }
            }

            // Classes that don't produce direct geometry
            OfmlClassType::OiPart
            | OfmlClassType::OiPolygon
            | OfmlClassType::OiImport
            | OfmlClassType::OiFrame
            | OfmlClassType::OiRotation
            | OfmlClassType::OiSweep => Ok(OfmlClassInstance::None),

            // Unknown/custom class
            OfmlClassType::Custom(name) => Err(OfmlClassError::UnknownClass(name)),
        }
    }
}

impl GeometryTransform {
    /// Apply this transformation to a vertex coordinate.
    ///
    /// Returns the transformed [x, y, z] coordinates.
    pub fn transform_vertex(&self, x: f64, y: f64, z: f64) -> [f64; 3] {
        match self {
            GeometryTransform::YStretch(params) => [x, params.transform_y(y), z],
            GeometryTransform::XStretch(params) => [params.transform_x(x), y, z],
            GeometryTransform::ZStretch(params) => [x, y, params.transform_z(z)],
            GeometryTransform::XYStretch(params) => {
                let (new_x, new_y) = params.transform(x, y);
                [new_x, new_y, z]
            }
            GeometryTransform::Mirror(params) => {
                let (new_x, new_y, new_z) = params.transform(x, y, z);
                [new_x, new_y, new_z]
            }
            GeometryTransform::Scale(scale) => [x * scale[0], y * scale[1], z * scale[2]],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ofml_class_type_from_name() {
        assert_eq!(
            OfmlClassType::from_name("::ofml::go::GoYLTrans"),
            OfmlClassType::GoYLTrans
        );
        assert_eq!(
            OfmlClassType::from_name("GoYLTrans"),
            OfmlClassType::GoYLTrans
        );
        assert_eq!(
            OfmlClassType::from_name("::ofml::oi::OiBlock"),
            OfmlClassType::OiBlock
        );
        assert_eq!(OfmlClassType::from_name("OiBlock"), OfmlClassType::OiBlock);

        // Unknown class
        assert!(matches!(
            OfmlClassType::from_name("UnknownClass"),
            OfmlClassType::Custom(_)
        ));
    }

    #[test]
    fn test_ofml_class_type_full_name() {
        assert_eq!(
            OfmlClassType::GoYLTrans.full_name(),
            "::ofml::go::GoYLTrans"
        );
        assert_eq!(OfmlClassType::OiBlock.full_name(), "::ofml::oi::OiBlock");
    }

    #[test]
    fn test_ofml_class_type_full_name_all_go_classes() {
        assert_eq!(
            OfmlClassType::GoMetaType.full_name(),
            "::ofml::go::GoMetaType"
        );
        assert_eq!(
            OfmlClassType::GoXLRTransYLRTrans.full_name(),
            "::ofml::go::GoXLRTransYLRTrans"
        );
        assert_eq!(
            OfmlClassType::GoXLTrans.full_name(),
            "::ofml::go::GoXLTrans"
        );
        assert_eq!(
            OfmlClassType::GoZLTrans.full_name(),
            "::ofml::go::GoZLTrans"
        );
        assert_eq!(
            OfmlClassType::GoMirror.full_name(),
            "::ofml::go::GoMirror"
        );
    }

    #[test]
    fn test_ofml_class_type_full_name_all_oi_classes() {
        assert_eq!(OfmlClassType::OiPart.full_name(), "::ofml::oi::OiPart");
        assert_eq!(
            OfmlClassType::OiCylinder.full_name(),
            "::ofml::oi::OiCylinder"
        );
        assert_eq!(
            OfmlClassType::OiEllipsoid.full_name(),
            "::ofml::oi::OiEllipsoid"
        );
        assert_eq!(OfmlClassType::OiSphere.full_name(), "::ofml::oi::OiSphere");
        assert_eq!(
            OfmlClassType::OiPolygon.full_name(),
            "::ofml::oi::OiPolygon"
        );
        assert_eq!(OfmlClassType::OiImport.full_name(), "::ofml::oi::OiImport");
        assert_eq!(OfmlClassType::OiFrame.full_name(), "::ofml::oi::OiFrame");
        assert_eq!(
            OfmlClassType::OiRotation.full_name(),
            "::ofml::oi::OiRotation"
        );
        assert_eq!(OfmlClassType::OiSweep.full_name(), "::ofml::oi::OiSweep");
    }

    #[test]
    fn test_ofml_class_type_full_name_custom() {
        let custom = OfmlClassType::Custom("::my::custom::Class".to_string());
        assert_eq!(custom.full_name(), "::my::custom::Class");
    }

    #[test]
    fn test_ofml_class_type_classification() {
        assert!(OfmlClassType::GoYLTrans.is_go_class());
        assert!(!OfmlClassType::GoYLTrans.is_oi_class());

        assert!(!OfmlClassType::OiBlock.is_go_class());
        assert!(OfmlClassType::OiBlock.is_oi_class());
    }

    #[test]
    fn test_go_yl_trans_params() {
        let params = GoYLTransParams::from_params(&[100.0, 200.0, 10.0]).unwrap();

        // Below stretch_min: unchanged
        assert_eq!(params.transform_y(5.0), 5.0);

        // At stretch_min: unchanged
        assert_eq!(params.transform_y(10.0), 10.0);

        // Above stretch_min: scaled
        let transformed = params.transform_y(60.0);
        // Original range: 60 - 10 = 50 (50% of 100)
        // Scaled: 10 + 50 * 2.0 = 110
        assert!((transformed - 110.0).abs() < 0.001);
    }

    #[test]
    fn test_go_mirror_params() {
        let params = GoMirrorParams::from_params(&[0.0, 50.0]).unwrap(); // Mirror X at 50

        let (x, y, z) = params.transform(30.0, 10.0, 5.0);
        // 2 * 50 - 30 = 70
        assert!((x - 70.0).abs() < 0.001);
        assert_eq!(y, 10.0);
        assert_eq!(z, 5.0);
    }

    #[test]
    fn test_go_xlr_trans_ylr_trans_params() {
        let params =
            GoXLRTransYLRTransParams::from_params(&[10.0, 90.0, 2.0, 10.0, 90.0, 2.0]).unwrap();

        // Point within stretch region
        let (x, y) = params.transform(50.0, 50.0);
        // x: 10 + (50 - 10) * 2 = 90
        assert!((x - 90.0).abs() < 0.001);
        assert!((y - 90.0).abs() < 0.001);
    }

    #[test]
    fn test_ofml_class_registry() {
        assert!(OfmlClassRegistry::is_known("::ofml::go::GoYLTrans"));
        assert!(OfmlClassRegistry::is_known("GoYLTrans"));
        assert!(!OfmlClassRegistry::is_known("UnknownClass"));

        assert_eq!(
            OfmlClassRegistry::lookup("GoYLTrans"),
            Some(OfmlClassType::GoYLTrans)
        );
        assert_eq!(OfmlClassRegistry::lookup("UnknownClass"), None);

        assert_eq!(
            OfmlClassRegistry::expected_params(&OfmlClassType::GoYLTrans),
            3
        );
        assert_eq!(
            OfmlClassRegistry::expected_params(&OfmlClassType::OiBlock),
            3
        );
    }

    #[test]
    fn test_invalid_param_count() {
        let result = GoYLTransParams::from_params(&[100.0, 200.0]);
        assert!(matches!(
            result,
            Err(OfmlClassError::InvalidParamCount { .. })
        ));
    }

    #[test]
    fn test_instantiate_go_yl_trans() {
        let result = OfmlClassRegistry::instantiate("::ofml::go::GoYLTrans", &[100.0, 200.0, 10.0]);
        match result {
            Ok(OfmlClassInstance::Transform(GeometryTransform::YStretch(params))) => {
                assert_eq!(params.base_height, 100.0);
                assert_eq!(params.target_height, 200.0);
                assert_eq!(params.stretch_min, 10.0);
            }
            _ => panic!("Expected YStretch transform"),
        }
    }

    #[test]
    fn test_instantiate_go_xl_trans() {
        let result = OfmlClassRegistry::instantiate("GoXLTrans", &[50.0, 100.0, 5.0]);
        match result {
            Ok(OfmlClassInstance::Transform(GeometryTransform::XStretch(params))) => {
                assert_eq!(params.base_width, 50.0);
                assert_eq!(params.target_width, 100.0);
                assert_eq!(params.stretch_min, 5.0);
            }
            _ => panic!("Expected XStretch transform"),
        }
    }

    #[test]
    fn test_instantiate_go_xlr_trans_ylr_trans() {
        let result = OfmlClassRegistry::instantiate(
            "::ofml::go::GoXLRTransYLRTrans",
            &[0.0, 100.0, 1.5, 0.0, 80.0, 1.2],
        );
        match result {
            Ok(OfmlClassInstance::Transform(GeometryTransform::XYStretch(params))) => {
                assert_eq!(params.x_min, 0.0);
                assert_eq!(params.x_max, 100.0);
                assert_eq!(params.x_scale, 1.5);
                assert_eq!(params.y_min, 0.0);
                assert_eq!(params.y_max, 80.0);
                assert_eq!(params.y_scale, 1.2);
            }
            _ => panic!("Expected XYStretch transform"),
        }
    }

    #[test]
    fn test_instantiate_go_mirror() {
        let result = OfmlClassRegistry::instantiate("GoMirror", &[1.0, 25.0]); // Mirror Y at 25
        match result {
            Ok(OfmlClassInstance::Transform(GeometryTransform::Mirror(params))) => {
                assert_eq!(params.axis, MirrorAxis::Y);
                assert_eq!(params.offset, 25.0);
            }
            _ => panic!("Expected Mirror transform"),
        }
    }

    #[test]
    fn test_instantiate_oi_block() {
        let result = OfmlClassRegistry::instantiate("OiBlock", &[1.6, 0.025, 0.8]);
        match result {
            Ok(OfmlClassInstance::Primitive(PrimitiveGeometry::Block { dimensions })) => {
                assert!((dimensions[0] - 1.6).abs() < 0.001);
                assert!((dimensions[1] - 0.025).abs() < 0.001);
                assert!((dimensions[2] - 0.8).abs() < 0.001);
            }
            _ => panic!("Expected Block primitive"),
        }
    }

    #[test]
    fn test_instantiate_oi_cylinder() {
        let result = OfmlClassRegistry::instantiate("OiCylinder", &[0.05, 0.74]);
        match result {
            Ok(OfmlClassInstance::Primitive(PrimitiveGeometry::Cylinder { radius, height })) => {
                assert!((radius - 0.05).abs() < 0.001);
                assert!((height - 0.74).abs() < 0.001);
            }
            _ => panic!("Expected Cylinder primitive"),
        }
    }

    #[test]
    fn test_instantiate_oi_sphere() {
        let result = OfmlClassRegistry::instantiate("OiSphere", &[0.5]);
        match result {
            Ok(OfmlClassInstance::Primitive(PrimitiveGeometry::Sphere { radius })) => {
                assert!((radius - 0.5).abs() < 0.001);
            }
            _ => panic!("Expected Sphere primitive"),
        }
    }

    #[test]
    fn test_instantiate_oi_ellipsoid() {
        let result = OfmlClassRegistry::instantiate("OiEllipsoid", &[0.3, 0.4, 0.5]);
        match result {
            Ok(OfmlClassInstance::Primitive(PrimitiveGeometry::Ellipsoid { radii })) => {
                assert!((radii[0] - 0.3).abs() < 0.001);
                assert!((radii[1] - 0.4).abs() < 0.001);
                assert!((radii[2] - 0.5).abs() < 0.001);
            }
            _ => panic!("Expected Ellipsoid primitive"),
        }
    }

    #[test]
    fn test_instantiate_unknown_class() {
        let result = OfmlClassRegistry::instantiate("::unknown::FakeClass", &[1.0, 2.0]);
        assert!(matches!(result, Err(OfmlClassError::UnknownClass(_))));
    }

    #[test]
    fn test_instantiate_oi_part_returns_none() {
        let result = OfmlClassRegistry::instantiate("OiPart", &[]);
        match result {
            Ok(OfmlClassInstance::None) => { /* Expected */ }
            _ => panic!("Expected None for OiPart"),
        }
    }

    #[test]
    fn test_instantiate_go_meta_type_with_scale() {
        let result = OfmlClassRegistry::instantiate("GoMetaType", &[1.6, 0.8, 0.74]);
        match result {
            Ok(OfmlClassInstance::Transform(GeometryTransform::Scale(scale))) => {
                assert!((scale[0] - 1.6).abs() < 0.001);
                assert!((scale[1] - 0.8).abs() < 0.001);
                assert!((scale[2] - 0.74).abs() < 0.001);
            }
            _ => panic!("Expected Scale transform"),
        }
    }

    #[test]
    fn test_geometry_transform_y_stretch() {
        let transform = GeometryTransform::YStretch(GoYLTransParams {
            base_height: 100.0,
            target_height: 200.0,
            stretch_min: 10.0,
        });

        // Below stretch min - unchanged
        let result = transform.transform_vertex(5.0, 5.0, 5.0);
        assert_eq!(result[0], 5.0);
        assert_eq!(result[1], 5.0);
        assert_eq!(result[2], 5.0);

        // Above stretch min - scaled
        let result = transform.transform_vertex(5.0, 60.0, 5.0);
        assert_eq!(result[0], 5.0);
        assert!((result[1] - 110.0).abs() < 0.001);
        assert_eq!(result[2], 5.0);
    }

    #[test]
    fn test_geometry_transform_scale() {
        let transform = GeometryTransform::Scale([2.0, 1.5, 0.5]);
        let result = transform.transform_vertex(10.0, 20.0, 30.0);
        assert!((result[0] - 20.0).abs() < 0.001);
        assert!((result[1] - 30.0).abs() < 0.001);
        assert!((result[2] - 15.0).abs() < 0.001);
    }

    #[test]
    fn test_geometry_transform_mirror_x() {
        let transform = GeometryTransform::Mirror(GoMirrorParams {
            axis: MirrorAxis::X,
            offset: 50.0,
        });
        let result = transform.transform_vertex(30.0, 10.0, 5.0);
        assert!((result[0] - 70.0).abs() < 0.001); // 2*50 - 30 = 70
        assert_eq!(result[1], 10.0);
        assert_eq!(result[2], 5.0);
    }

    #[test]
    fn test_instantiate_insufficient_params() {
        // OiBlock needs 3 params, only giving 2
        let result = OfmlClassRegistry::instantiate("OiBlock", &[1.0, 2.0]);
        assert!(matches!(
            result,
            Err(OfmlClassError::InvalidParamCount { .. })
        ));

        // OiCylinder needs 2 params, only giving 1
        let result = OfmlClassRegistry::instantiate("OiCylinder", &[1.0]);
        assert!(matches!(
            result,
            Err(OfmlClassError::InvalidParamCount { .. })
        ));
    }

    // ========== Additional Coverage Tests ==========

    #[test]
    fn test_go_xl_trans_params_insufficient() {
        let result = GoXLTransParams::from_params(&[1.0, 2.0]);
        assert!(matches!(
            result,
            Err(OfmlClassError::InvalidParamCount { .. })
        ));
    }

    #[test]
    fn test_go_xl_trans_transform() {
        let params = GoXLTransParams::from_params(&[100.0, 200.0, 10.0]).unwrap();
        // Below stretch_min: unchanged
        assert_eq!(params.transform_x(5.0), 5.0);
        // Above stretch_min: scaled
        let expected = 10.0 + (50.0 - 10.0) * 2.0; // = 90
        assert!((params.transform_x(50.0) - expected).abs() < 0.001);
    }

    #[test]
    fn test_go_zl_trans_params_insufficient() {
        let result = GoZLTransParams::from_params(&[1.0, 2.0]);
        assert!(matches!(
            result,
            Err(OfmlClassError::InvalidParamCount { .. })
        ));
    }

    #[test]
    fn test_go_zl_trans_transform() {
        let params = GoZLTransParams::from_params(&[100.0, 200.0, 10.0]).unwrap();
        // Below stretch_min: unchanged
        assert_eq!(params.transform_z(5.0), 5.0);
        // Above stretch_min: scaled
        let expected = 10.0 + (50.0 - 10.0) * 2.0; // = 90
        assert!((params.transform_z(50.0) - expected).abs() < 0.001);
    }

    #[test]
    fn test_go_xlr_trans_ylr_trans_params_insufficient() {
        let result = GoXLRTransYLRTransParams::from_params(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        assert!(matches!(
            result,
            Err(OfmlClassError::InvalidParamCount { .. })
        ));
    }

    #[test]
    fn test_go_xlr_trans_ylr_trans_transform_all_regions() {
        let params = GoXLRTransYLRTransParams::from_params(&[
            10.0, 50.0, 2.0, // x_min, x_max, x_scale
            20.0, 60.0, 1.5, // y_min, y_max, y_scale
        ])
        .unwrap();

        // X below min, Y below min
        let (x, y) = params.transform(5.0, 15.0);
        assert_eq!(x, 5.0);
        assert_eq!(y, 15.0);

        // X between min and max, Y between min and max
        let (x, y) = params.transform(30.0, 40.0);
        let expected_x = 10.0 + (30.0 - 10.0) * 2.0; // 10 + 40 = 50
        let expected_y = 20.0 + (40.0 - 20.0) * 1.5; // 20 + 30 = 50
        assert!((x - expected_x).abs() < 0.001);
        assert!((y - expected_y).abs() < 0.001);

        // X above max, Y above max
        let (x, y) = params.transform(70.0, 80.0);
        let expected_x = 50.0 + (70.0 - 50.0) * 2.0; // 50 + 40 = 90
        let expected_y = 60.0 + (80.0 - 60.0) * 1.5; // 60 + 30 = 90
        assert!((x - expected_x).abs() < 0.001);
        assert!((y - expected_y).abs() < 0.001);
    }

    #[test]
    fn test_go_mirror_params_empty() {
        let result = GoMirrorParams::from_params(&[]);
        assert!(matches!(
            result,
            Err(OfmlClassError::InvalidParamCount { .. })
        ));
    }

    #[test]
    fn test_go_mirror_params_invalid_axis() {
        let result = GoMirrorParams::from_params(&[99.0]);
        assert!(matches!(
            result,
            Err(OfmlClassError::InvalidParamType { .. })
        ));
    }

    #[test]
    fn test_go_mirror_y_axis() {
        let params = GoMirrorParams::from_params(&[1.0, 25.0]).unwrap();
        assert_eq!(params.axis, MirrorAxis::Y);
        let (x, y, z) = params.transform(10.0, 20.0, 30.0);
        assert_eq!(x, 10.0);
        assert!((y - 30.0).abs() < 0.001); // 2*25 - 20 = 30
        assert_eq!(z, 30.0);
    }

    #[test]
    fn test_go_mirror_z_axis() {
        let params = GoMirrorParams::from_params(&[2.0, 50.0]).unwrap();
        assert_eq!(params.axis, MirrorAxis::Z);
        let (x, y, z) = params.transform(10.0, 20.0, 30.0);
        assert_eq!(x, 10.0);
        assert_eq!(y, 20.0);
        assert!((z - 70.0).abs() < 0.001); // 2*50 - 30 = 70
    }

    #[test]
    fn test_geometry_transform_x_stretch() {
        let transform = GeometryTransform::XStretch(GoXLTransParams {
            base_width: 100.0,
            target_width: 200.0,
            stretch_min: 10.0,
        });

        // Below stretch min - unchanged
        let result = transform.transform_vertex(5.0, 5.0, 5.0);
        assert_eq!(result[0], 5.0);

        // Above stretch min - scaled
        let result = transform.transform_vertex(60.0, 5.0, 5.0);
        assert!((result[0] - 110.0).abs() < 0.001); // 10 + (60-10)*2 = 110
    }

    #[test]
    fn test_geometry_transform_z_stretch() {
        let transform = GeometryTransform::ZStretch(GoZLTransParams {
            base_depth: 100.0,
            target_depth: 200.0,
            stretch_min: 10.0,
        });

        // Below stretch min - unchanged
        let result = transform.transform_vertex(5.0, 5.0, 5.0);
        assert_eq!(result[2], 5.0);

        // Above stretch min - scaled
        let result = transform.transform_vertex(5.0, 5.0, 60.0);
        assert!((result[2] - 110.0).abs() < 0.001); // 10 + (60-10)*2 = 110
    }

    #[test]
    fn test_geometry_transform_xy_stretch() {
        let transform = GeometryTransform::XYStretch(GoXLRTransYLRTransParams {
            x_min: 10.0,
            x_max: 50.0,
            x_scale: 2.0,
            y_min: 10.0,
            y_max: 50.0,
            y_scale: 1.5,
        });

        let result = transform.transform_vertex(30.0, 30.0, 100.0);
        let expected_x = 10.0 + (30.0 - 10.0) * 2.0; // = 50
        let expected_y = 10.0 + (30.0 - 10.0) * 1.5; // = 40
        assert!((result[0] - expected_x).abs() < 0.001);
        assert!((result[1] - expected_y).abs() < 0.001);
        assert_eq!(result[2], 100.0); // Z unchanged
    }

    #[test]
    fn test_geometry_transform_mirror_y() {
        let transform = GeometryTransform::Mirror(GoMirrorParams {
            axis: MirrorAxis::Y,
            offset: 50.0,
        });
        let result = transform.transform_vertex(10.0, 30.0, 5.0);
        assert_eq!(result[0], 10.0);
        assert!((result[1] - 70.0).abs() < 0.001); // 2*50 - 30 = 70
        assert_eq!(result[2], 5.0);
    }

    #[test]
    fn test_geometry_transform_mirror_z() {
        let transform = GeometryTransform::Mirror(GoMirrorParams {
            axis: MirrorAxis::Z,
            offset: 50.0,
        });
        let result = transform.transform_vertex(10.0, 5.0, 30.0);
        assert_eq!(result[0], 10.0);
        assert_eq!(result[1], 5.0);
        assert!((result[2] - 70.0).abs() < 0.001); // 2*50 - 30 = 70
    }

    #[test]
    fn test_instantiate_oi_sphere_missing_params() {
        let result = OfmlClassRegistry::instantiate("OiSphere", &[]);
        assert!(matches!(
            result,
            Err(OfmlClassError::InvalidParamCount { .. })
        ));
    }

    #[test]
    fn test_instantiate_oi_ellipsoid_missing_params() {
        let result = OfmlClassRegistry::instantiate("OiEllipsoid", &[1.0, 2.0]);
        assert!(matches!(
            result,
            Err(OfmlClassError::InvalidParamCount { .. })
        ));
    }

    #[test]
    fn test_instantiate_classes_returning_none() {
        for class in &[
            "OiPolygon",
            "OiImport",
            "OiFrame",
            "OiRotation",
            "OiSweep",
        ] {
            let result = OfmlClassRegistry::instantiate(class, &[]);
            match result {
                Ok(OfmlClassInstance::None) => { /* Expected */ }
                _ => panic!("Expected None for {}", class),
            }
        }
    }

    #[test]
    fn test_ofml_class_type_debug_clone() {
        let class_type = OfmlClassType::GoYLTrans;
        let debug_str = format!("{:?}", class_type);
        assert!(debug_str.contains("GoYLTrans"));

        let cloned = class_type.clone();
        assert_eq!(cloned, OfmlClassType::GoYLTrans);
    }

    #[test]
    fn test_ofml_class_type_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(OfmlClassType::GoYLTrans);
        set.insert(OfmlClassType::OiBlock);
        set.insert(OfmlClassType::Custom("Test".to_string()));

        assert!(set.contains(&OfmlClassType::GoYLTrans));
        assert!(set.contains(&OfmlClassType::OiBlock));
        assert!(!set.contains(&OfmlClassType::OiCylinder));
    }

    #[test]
    fn test_mirror_axis_debug_clone() {
        let axis = MirrorAxis::X;
        let debug_str = format!("{:?}", axis);
        assert!(debug_str.contains("X"));

        let cloned = axis.clone();
        assert_eq!(cloned, MirrorAxis::X);
    }

    #[test]
    fn test_go_params_debug() {
        let y_params = GoYLTransParams {
            base_height: 100.0,
            target_height: 200.0,
            stretch_min: 10.0,
        };
        let debug_str = format!("{:?}", y_params);
        assert!(debug_str.contains("GoYLTransParams"));

        let x_params = GoXLTransParams {
            base_width: 100.0,
            target_width: 200.0,
            stretch_min: 10.0,
        };
        let debug_str = format!("{:?}", x_params);
        assert!(debug_str.contains("GoXLTransParams"));

        let z_params = GoZLTransParams {
            base_depth: 100.0,
            target_depth: 200.0,
            stretch_min: 10.0,
        };
        let debug_str = format!("{:?}", z_params);
        assert!(debug_str.contains("GoZLTransParams"));
    }

    #[test]
    fn test_go_params_clone() {
        let y_params = GoYLTransParams {
            base_height: 100.0,
            target_height: 200.0,
            stretch_min: 10.0,
        };
        let cloned = y_params.clone();
        assert_eq!(cloned.base_height, 100.0);

        let xy_params = GoXLRTransYLRTransParams {
            x_min: 10.0,
            x_max: 50.0,
            x_scale: 2.0,
            y_min: 20.0,
            y_max: 60.0,
            y_scale: 1.5,
        };
        let cloned = xy_params.clone();
        assert_eq!(cloned.x_min, 10.0);
        assert_eq!(cloned.y_scale, 1.5);
    }

    #[test]
    fn test_go_mirror_params_debug_clone() {
        let params = GoMirrorParams {
            axis: MirrorAxis::Y,
            offset: 25.0,
        };
        let debug_str = format!("{:?}", params);
        assert!(debug_str.contains("GoMirrorParams"));

        let cloned = params.clone();
        assert_eq!(cloned.axis, MirrorAxis::Y);
        assert_eq!(cloned.offset, 25.0);
    }

    #[test]
    fn test_ofml_class_instance_debug_clone() {
        let instance = OfmlClassInstance::None;
        let debug_str = format!("{:?}", instance);
        assert!(debug_str.contains("None"));

        let cloned = instance.clone();
        assert!(matches!(cloned, OfmlClassInstance::None));
    }

    #[test]
    fn test_geometry_transform_debug_clone() {
        let transform = GeometryTransform::Scale([1.0, 2.0, 3.0]);
        let debug_str = format!("{:?}", transform);
        assert!(debug_str.contains("Scale"));

        let cloned = transform.clone();
        if let GeometryTransform::Scale(scale) = cloned {
            assert_eq!(scale, [1.0, 2.0, 3.0]);
        }
    }

    #[test]
    fn test_primitive_geometry_debug_clone() {
        let prim = PrimitiveGeometry::Block {
            dimensions: [1.0, 2.0, 3.0],
        };
        let debug_str = format!("{:?}", prim);
        assert!(debug_str.contains("Block"));

        let cloned = prim.clone();
        if let PrimitiveGeometry::Block { dimensions } = cloned {
            assert_eq!(dimensions, [1.0, 2.0, 3.0]);
        }

        let sphere = PrimitiveGeometry::Sphere { radius: 5.0 };
        let cloned = sphere.clone();
        if let PrimitiveGeometry::Sphere { radius } = cloned {
            assert!((radius - 5.0).abs() < 0.001);
        }

        let cylinder = PrimitiveGeometry::Cylinder {
            radius: 2.0,
            height: 10.0,
        };
        let cloned = cylinder.clone();
        if let PrimitiveGeometry::Cylinder { radius, height } = cloned {
            assert!((radius - 2.0).abs() < 0.001);
            assert!((height - 10.0).abs() < 0.001);
        }

        let ellipsoid = PrimitiveGeometry::Ellipsoid {
            radii: [1.0, 2.0, 3.0],
        };
        let cloned = ellipsoid.clone();
        if let PrimitiveGeometry::Ellipsoid { radii } = cloned {
            assert_eq!(radii, [1.0, 2.0, 3.0]);
        }
    }

    #[test]
    fn test_expected_params() {
        // Test various class types have correct expected param counts
        assert_eq!(OfmlClassRegistry::expected_params(&OfmlClassType::GoYLTrans), 3);
        assert_eq!(OfmlClassRegistry::expected_params(&OfmlClassType::GoXLTrans), 3);
        assert_eq!(OfmlClassRegistry::expected_params(&OfmlClassType::GoZLTrans), 3);
        assert_eq!(OfmlClassRegistry::expected_params(&OfmlClassType::GoXLRTransYLRTrans), 6);
        assert_eq!(OfmlClassRegistry::expected_params(&OfmlClassType::GoMirror), 1);
        assert_eq!(OfmlClassRegistry::expected_params(&OfmlClassType::OiBlock), 3);
        assert_eq!(OfmlClassRegistry::expected_params(&OfmlClassType::OiCylinder), 2);
        assert_eq!(OfmlClassRegistry::expected_params(&OfmlClassType::OiSphere), 1);
        assert_eq!(OfmlClassRegistry::expected_params(&OfmlClassType::OiEllipsoid), 3);
        // Unknown/other types return 0
        assert_eq!(OfmlClassRegistry::expected_params(&OfmlClassType::GoMetaType), 0);
    }

    #[test]
    fn test_instantiate_go_z_l_trans() {
        // Test GoZLTrans instantiation
        let result = OfmlClassRegistry::instantiate("::ofml::go::GoZLTrans", &[100.0, 200.0, 10.0]);
        assert!(result.is_ok());
        match result.unwrap() {
            OfmlClassInstance::Transform(GeometryTransform::ZStretch(params)) => {
                assert_eq!(params.base_depth, 100.0);
                assert_eq!(params.target_depth, 200.0);
                assert_eq!(params.stretch_min, 10.0);
            }
            _ => panic!("Expected ZStretch transform"),
        }
    }

    #[test]
    fn test_instantiate_go_meta_type_insufficient_params() {
        // GoMetaType with less than 3 params returns None
        let result = OfmlClassRegistry::instantiate("::ofml::go::GoMetaType", &[1.0, 2.0]);
        assert!(result.is_ok());
        match result.unwrap() {
            OfmlClassInstance::None => {}
            _ => panic!("Expected None for GoMetaType with insufficient params"),
        }
    }
}
