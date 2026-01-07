//! Material System - MAT file parsing and material management.
//!
//! This module implements material handling for OFML products including:
//! - MAT file parsing
//! - Texture loading and projection
//! - Material resolution from expressions
//! - GLB material export

use std::collections::HashMap;

use crate::errors::MaterialError;

/// Texture projection modes from OFML spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextureProjection {
    /// Planar projection from X axis
    #[default]
    ProjectX,
    /// Planar projection from Y axis
    ProjectY,
    /// Planar projection from Z axis
    ProjectZ,
    /// Cylindrical projection
    Cylindrical,
    /// Spherical projection
    Spherical,
    /// Conical projection
    Conical,
    /// Circle projection
    Circle,
}

impl TextureProjection {
    /// Parse from OFML projection mode string.
    pub fn parse(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "prjx" | "project_x" | "x" => TextureProjection::ProjectX,
            "prjy" | "project_y" | "y" => TextureProjection::ProjectY,
            "prjz" | "project_z" | "z" => TextureProjection::ProjectZ,
            "cyl" | "cylindrical" => TextureProjection::Cylindrical,
            "sph" | "spherical" => TextureProjection::Spherical,
            "cone" | "conical" => TextureProjection::Conical,
            "circ" | "circle" => TextureProjection::Circle,
            _ => TextureProjection::ProjectZ,
        }
    }

    /// Get the OFML mode string.
    pub fn to_ofml_str(&self) -> &'static str {
        match self {
            TextureProjection::ProjectX => "prjx",
            TextureProjection::ProjectY => "prjy",
            TextureProjection::ProjectZ => "prjz",
            TextureProjection::Cylindrical => "cyl",
            TextureProjection::Spherical => "sph",
            TextureProjection::Conical => "cone",
            TextureProjection::Circle => "circ",
        }
    }
}

/// Texture definition including projection parameters.
#[derive(Debug, Clone)]
pub struct TextureDef {
    /// Texture filename
    pub filename: String,
    /// Projection mode
    pub projection: TextureProjection,
    /// U scale factor
    pub u_scale: f32,
    /// V scale factor
    pub v_scale: f32,
    /// U offset
    pub u_offset: f32,
    /// V offset
    pub v_offset: f32,
    /// Rotation angle (radians)
    pub rotation: f32,
}

impl TextureDef {
    /// Create a new texture definition with defaults.
    pub fn new(filename: impl Into<String>) -> Self {
        Self {
            filename: filename.into(),
            projection: TextureProjection::default(),
            u_scale: 1.0,
            v_scale: 1.0,
            u_offset: 0.0,
            v_offset: 0.0,
            rotation: 0.0,
        }
    }

    /// Set the projection mode.
    pub fn with_projection(mut self, projection: TextureProjection) -> Self {
        self.projection = projection;
        self
    }

    /// Set UV scale.
    pub fn with_scale(mut self, u_scale: f32, v_scale: f32) -> Self {
        self.u_scale = u_scale;
        self.v_scale = v_scale;
        self
    }

    /// Set UV offset.
    pub fn with_offset(mut self, u_offset: f32, v_offset: f32) -> Self {
        self.u_offset = u_offset;
        self.v_offset = v_offset;
        self
    }

    /// Set rotation.
    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }
}

/// Full material definition from MAT file or EBASE.
#[derive(Debug, Clone)]
pub struct MaterialDef {
    /// Material name/identifier
    pub name: String,
    /// Ambient color [R, G, B, A]
    pub ambient: [f32; 4],
    /// Diffuse color [R, G, B, A]
    pub diffuse: [f32; 4],
    /// Specular color [R, G, B, A]
    pub specular: [f32; 4],
    /// Shininess exponent (0-1000)
    pub shininess: f32,
    /// Transparency (0.0 = opaque, 1.0 = fully transparent)
    pub transparency: f32,
    /// Texture file reference
    pub texture: Option<TextureDef>,
}

impl MaterialDef {
    /// Create a new material with default grey color.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ambient: [0.2, 0.2, 0.2, 1.0],
            diffuse: [0.6, 0.6, 0.6, 1.0],
            specular: [0.3, 0.3, 0.3, 1.0],
            shininess: 32.0,
            transparency: 0.0,
            texture: None,
        }
    }

    /// Create a material from color name.
    pub fn from_color_name(name: &str) -> Self {
        let (diffuse, ambient, specular) = Self::color_from_name(name);
        Self {
            name: name.to_string(),
            ambient,
            diffuse,
            specular,
            shininess: 32.0,
            transparency: 0.0,
            texture: None,
        }
    }

    /// Get color components from a color name.
    fn color_from_name(name: &str) -> ([f32; 4], [f32; 4], [f32; 4]) {
        let lower = name.to_lowercase();

        // Extract color from material name patterns
        let diffuse = if lower.contains("white") {
            [0.9, 0.9, 0.9, 1.0]
        } else if lower.contains("black") {
            [0.1, 0.1, 0.1, 1.0]
        } else if lower.contains("red") {
            [0.8, 0.2, 0.2, 1.0]
        } else if lower.contains("green") {
            [0.2, 0.8, 0.2, 1.0]
        } else if lower.contains("blue") {
            [0.2, 0.2, 0.8, 1.0]
        } else if lower.contains("grey") || lower.contains("gray") {
            [0.5, 0.5, 0.5, 1.0]
        } else if lower.contains("wood") || lower.contains("oak") || lower.contains("walnut") {
            [0.6, 0.4, 0.2, 1.0]
        } else if lower.contains("metal") || lower.contains("steel") || lower.contains("chrome") {
            [0.7, 0.7, 0.75, 1.0]
        } else if lower.contains("silver") {
            [0.75, 0.75, 0.75, 1.0]
        } else if lower.contains("gold") {
            [0.8, 0.7, 0.2, 1.0]
        } else {
            // Default grey
            [0.5, 0.5, 0.5, 1.0]
        };

        let ambient = [diffuse[0] * 0.3, diffuse[1] * 0.3, diffuse[2] * 0.3, 1.0];

        let specular = if lower.contains("metal") || lower.contains("chrome") {
            [0.8, 0.8, 0.8, 1.0]
        } else {
            [0.3, 0.3, 0.3, 1.0]
        };

        (diffuse, ambient, specular)
    }

    /// Set the diffuse color.
    pub fn with_diffuse(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.diffuse = [r, g, b, a];
        self
    }

    /// Set the ambient color.
    pub fn with_ambient(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.ambient = [r, g, b, a];
        self
    }

    /// Set the specular color.
    pub fn with_specular(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.specular = [r, g, b, a];
        self
    }

    /// Set the shininess.
    pub fn with_shininess(mut self, shininess: f32) -> Self {
        self.shininess = shininess;
        self
    }

    /// Set the transparency.
    pub fn with_transparency(mut self, transparency: f32) -> Self {
        self.transparency = transparency;
        self
    }

    /// Set the texture.
    pub fn with_texture(mut self, texture: TextureDef) -> Self {
        self.texture = Some(texture);
        self
    }

    /// Check if material has transparency.
    pub fn is_transparent(&self) -> bool {
        self.transparency > 0.0 || self.diffuse[3] < 1.0
    }

    /// Parse a color from "r g b" or "r g b a" format.
    pub fn parse_color(value: &str) -> Result<[f32; 4], MaterialError> {
        let parts: Vec<&str> = value.split_whitespace().collect();
        if parts.len() < 3 {
            return Err(MaterialError::InvalidProperty {
                property: "color".to_string(),
                message: format!("Invalid color value: {}", value),
            });
        }

        let r: f32 = parts[0]
            .parse()
            .map_err(|_| MaterialError::InvalidProperty {
                property: "color".to_string(),
                message: format!("Invalid red component: {}", parts[0]),
            })?;
        let g: f32 = parts[1]
            .parse()
            .map_err(|_| MaterialError::InvalidProperty {
                property: "color".to_string(),
                message: format!("Invalid green component: {}", parts[1]),
            })?;
        let b: f32 = parts[2]
            .parse()
            .map_err(|_| MaterialError::InvalidProperty {
                property: "color".to_string(),
                message: format!("Invalid blue component: {}", parts[2]),
            })?;
        let a: f32 = parts.get(3).and_then(|s| s.parse().ok()).unwrap_or(1.0);

        Ok([r, g, b, a])
    }
}

impl Default for MaterialDef {
    fn default() -> Self {
        Self::new("default")
    }
}

/// MAT file parser.
pub struct MatParser;

impl MatParser {
    /// Parse a MAT file content.
    ///
    /// # Arguments
    ///
    /// * `content` - The MAT file content as a string
    /// * `name` - The material name to use
    ///
    /// # Returns
    ///
    /// The parsed MaterialDef.
    pub fn parse(content: &str, name: &str) -> Result<MaterialDef, MaterialError> {
        let mut material = MaterialDef::new(name);

        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();

            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
                continue;
            }

            // Parse key-value pairs
            let parts: Vec<&str> = line.splitn(2, char::is_whitespace).collect();
            if parts.len() < 2 {
                continue;
            }

            let key = parts[0].to_lowercase();
            let value = parts[1].trim();

            match key.as_str() {
                "amb" | "ambient" => {
                    material.ambient = Self::parse_color(value, line_num)?;
                }
                "dif" | "diffuse" => {
                    material.diffuse = Self::parse_color(value, line_num)?;
                }
                "spe" | "specular" => {
                    material.specular = Self::parse_color(value, line_num)?;
                }
                "shi" | "shininess" => {
                    material.shininess = value.parse().map_err(|_| MaterialError::ParseError {
                        line: line_num + 1,
                        message: format!("Invalid shininess value: {}", value),
                    })?;
                }
                "tra" | "transparency" => {
                    material.transparency =
                        value.parse().map_err(|_| MaterialError::ParseError {
                            line: line_num + 1,
                            message: format!("Invalid transparency value: {}", value),
                        })?;
                }
                "tex" | "texture" => {
                    material.texture = Some(TextureDef::new(value));
                }
                "prj" | "projection" => {
                    if let Some(ref mut tex) = material.texture {
                        tex.projection = TextureProjection::parse(value);
                    }
                }
                _ => {
                    // Ignore unknown keys
                }
            }
        }

        Ok(material)
    }

    /// Parse inline material definition ($ syntax).
    ///
    /// Format: `$ amb r g b; dif r g b; spe r g b; shi s; tra t`
    pub fn parse_inline(inline: &str, name: &str) -> Result<MaterialDef, MaterialError> {
        let mut material = MaterialDef::new(name);

        // Remove leading $ if present
        let content = inline.trim_start_matches('$').trim();

        // Split by semicolon
        for part in content.split(';') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            let tokens: Vec<&str> = part.split_whitespace().collect();
            if tokens.is_empty() {
                continue;
            }

            match tokens[0].to_lowercase().as_str() {
                "amb" | "ambient" if tokens.len() >= 4 => {
                    material.ambient = Self::parse_color_tokens(&tokens[1..4])?;
                }
                "dif" | "diffuse" if tokens.len() >= 4 => {
                    material.diffuse = Self::parse_color_tokens(&tokens[1..4])?;
                }
                "spe" | "specular" if tokens.len() >= 4 => {
                    material.specular = Self::parse_color_tokens(&tokens[1..4])?;
                }
                "shi" | "shininess" if tokens.len() >= 2 => {
                    material.shininess = tokens[1].parse().unwrap_or(32.0);
                }
                "tra" | "transparency" if tokens.len() >= 2 => {
                    material.transparency = tokens[1].parse().unwrap_or(0.0);
                }
                _ => {}
            }
        }

        Ok(material)
    }

    /// Parse a color from "r g b" or "r g b a" format.
    fn parse_color(value: &str, line: usize) -> Result<[f32; 4], MaterialError> {
        let parts: Vec<&str> = value.split_whitespace().collect();
        if parts.len() < 3 {
            return Err(MaterialError::ParseError {
                line: line + 1,
                message: format!("Invalid color value: {}", value),
            });
        }

        let r: f32 = parts[0].parse().map_err(|_| MaterialError::ParseError {
            line: line + 1,
            message: format!("Invalid red component: {}", parts[0]),
        })?;
        let g: f32 = parts[1].parse().map_err(|_| MaterialError::ParseError {
            line: line + 1,
            message: format!("Invalid green component: {}", parts[1]),
        })?;
        let b: f32 = parts[2].parse().map_err(|_| MaterialError::ParseError {
            line: line + 1,
            message: format!("Invalid blue component: {}", parts[2]),
        })?;
        let a: f32 = parts.get(3).and_then(|s| s.parse().ok()).unwrap_or(1.0);

        Ok([r, g, b, a])
    }

    /// Parse color from token slice.
    fn parse_color_tokens(tokens: &[&str]) -> Result<[f32; 4], MaterialError> {
        if tokens.len() < 3 {
            return Err(MaterialError::InvalidProperty {
                property: "color".to_string(),
                message: "Need at least 3 color components".to_string(),
            });
        }

        let r: f32 = tokens[0].parse().unwrap_or(0.5);
        let g: f32 = tokens[1].parse().unwrap_or(0.5);
        let b: f32 = tokens[2].parse().unwrap_or(0.5);
        let a: f32 = tokens.get(3).and_then(|s| s.parse().ok()).unwrap_or(1.0);

        Ok([r, g, b, a])
    }
}

/// Material cache and resolver.
pub struct MaterialResolver {
    /// Cached material definitions
    materials: HashMap<String, MaterialDef>,
    /// Fallback material
    fallback: MaterialDef,
}

impl MaterialResolver {
    /// Create a new material resolver.
    pub fn new() -> Self {
        Self {
            materials: HashMap::new(),
            fallback: MaterialDef::new("fallback").with_diffuse(0.8, 0.0, 0.8, 1.0), // Magenta for missing materials
        }
    }

    /// Register a material.
    pub fn register(&mut self, material: MaterialDef) {
        self.materials.insert(material.name.clone(), material);
    }

    /// Resolve a material by name or expression.
    pub fn resolve(&self, name: &str) -> &MaterialDef {
        // Direct lookup
        if let Some(mat) = self.materials.get(name) {
            return mat;
        }

        // Try without namespace prefix
        let short_name = name.split("::").last().unwrap_or(name);
        if let Some(mat) = self.materials.get(short_name) {
            return mat;
        }

        // Return fallback
        &self.fallback
    }

    /// Resolve and get owned material (creates from color name if not found).
    pub fn resolve_or_create(&self, name: &str) -> MaterialDef {
        if let Some(mat) = self.materials.get(name) {
            return mat.clone();
        }

        // Try without namespace prefix
        let short_name = name.split("::").last().unwrap_or(name);
        if let Some(mat) = self.materials.get(short_name) {
            return mat.clone();
        }

        // Create from color name
        MaterialDef::from_color_name(name)
    }

    /// Check if material exists.
    pub fn contains(&self, name: &str) -> bool {
        self.materials.contains_key(name)
    }

    /// Get all material names.
    pub fn names(&self) -> Vec<&String> {
        self.materials.keys().collect()
    }

    /// Set the fallback material.
    pub fn set_fallback(&mut self, material: MaterialDef) {
        self.fallback = material;
    }
}

/// EBASE material table entry.
///
/// EBASE mat tables store material definitions in a tab-separated format:
/// `name\tamb\tdif\tspe\tshi\ttra\ttex\tprj`
#[derive(Debug, Clone)]
pub struct EbaseMaterialEntry {
    /// Material name/key
    pub name: String,
    /// Ambient color string (e.g., "0.2 0.2 0.2")
    pub ambient: Option<String>,
    /// Diffuse color string (e.g., "0.6 0.6 0.6")
    pub diffuse: Option<String>,
    /// Specular color string (e.g., "0.3 0.3 0.3")
    pub specular: Option<String>,
    /// Shininess value
    pub shininess: Option<f32>,
    /// Transparency value
    pub transparency: Option<f32>,
    /// Texture filename
    pub texture: Option<String>,
    /// Texture projection mode
    pub projection: Option<String>,
}

impl EbaseMaterialEntry {
    /// Convert to a MaterialDef.
    pub fn to_material_def(&self) -> MaterialDef {
        let mut mat = MaterialDef::new(&self.name);

        if let Some(ref amb) = self.ambient {
            if let Ok(color) = MaterialDef::parse_color(amb) {
                mat.ambient = color;
            }
        }

        if let Some(ref dif) = self.diffuse {
            if let Ok(color) = MaterialDef::parse_color(dif) {
                mat.diffuse = color;
            }
        }

        if let Some(ref spe) = self.specular {
            if let Ok(color) = MaterialDef::parse_color(spe) {
                mat.specular = color;
            }
        }

        if let Some(shi) = self.shininess {
            mat.shininess = shi;
        }

        if let Some(tra) = self.transparency {
            mat.transparency = tra;
        }

        if let Some(ref tex) = self.texture {
            let mut tex_def = TextureDef::new(tex);
            if let Some(ref prj) = self.projection {
                tex_def.projection = TextureProjection::parse(prj);
            }
            mat.texture = Some(tex_def);
        }

        mat
    }
}

/// Parse EBASE mat table data.
///
/// EBASE mat tables use a simple line-based format:
/// - Lines starting with `#` are comments
/// - Empty lines are skipped
/// - Format: `name <tab> property=value <tab> property=value ...`
///
/// Properties can include:
/// - `amb=r g b` or `ambient=r g b` - Ambient color
/// - `dif=r g b` or `diffuse=r g b` - Diffuse color
/// - `spe=r g b` or `specular=r g b` - Specular color
/// - `shi=value` or `shininess=value` - Shininess
/// - `tra=value` or `transparency=value` - Transparency
/// - `tex=filename` or `texture=filename` - Texture file
/// - `prj=mode` or `projection=mode` - Texture projection
pub fn parse_ebase_mat_table(data: &str) -> Vec<EbaseMaterialEntry> {
    let mut entries = Vec::new();

    for line in data.lines() {
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Split by tabs or multiple spaces
        let parts: Vec<&str> = line
            .split(['\t', ';'])
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        if parts.is_empty() {
            continue;
        }

        let name = parts[0].to_string();
        let mut entry = EbaseMaterialEntry {
            name,
            ambient: None,
            diffuse: None,
            specular: None,
            shininess: None,
            transparency: None,
            texture: None,
            projection: None,
        };

        // Parse remaining parts as key=value pairs
        for part in &parts[1..] {
            if let Some((key, value)) = part.split_once('=') {
                let key = key.trim().to_lowercase();
                let value = value.trim();

                match key.as_str() {
                    "amb" | "ambient" => entry.ambient = Some(value.to_string()),
                    "dif" | "diffuse" => entry.diffuse = Some(value.to_string()),
                    "spe" | "specular" => entry.specular = Some(value.to_string()),
                    "shi" | "shininess" => entry.shininess = value.parse().ok(),
                    "tra" | "transparency" => entry.transparency = value.parse().ok(),
                    "tex" | "texture" => entry.texture = Some(value.to_string()),
                    "prj" | "projection" => entry.projection = Some(value.to_string()),
                    _ => {}
                }
            }
        }

        entries.push(entry);
    }

    entries
}

/// Load materials from EBASE mat table data into a resolver.
pub fn load_ebase_materials(data: &str, resolver: &mut MaterialResolver) {
    for entry in parse_ebase_mat_table(data) {
        let mat = entry.to_material_def();
        resolver.register(mat);
    }
}

impl Default for MaterialResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_projection() {
        assert_eq!(
            TextureProjection::parse("prjx"),
            TextureProjection::ProjectX
        );
        assert_eq!(
            TextureProjection::parse("prjy"),
            TextureProjection::ProjectY
        );
        assert_eq!(
            TextureProjection::parse("prjz"),
            TextureProjection::ProjectZ
        );
        assert_eq!(
            TextureProjection::parse("cyl"),
            TextureProjection::Cylindrical
        );
        assert_eq!(
            TextureProjection::parse("sph"),
            TextureProjection::Spherical
        );

        assert_eq!(TextureProjection::ProjectX.to_ofml_str(), "prjx");
    }

    #[test]
    fn test_texture_def() {
        let tex = TextureDef::new("wood.png")
            .with_projection(TextureProjection::ProjectY)
            .with_scale(2.0, 2.0)
            .with_offset(0.5, 0.5);

        assert_eq!(tex.filename, "wood.png");
        assert_eq!(tex.projection, TextureProjection::ProjectY);
        assert_eq!(tex.u_scale, 2.0);
        assert_eq!(tex.v_scale, 2.0);
    }

    #[test]
    fn test_material_def() {
        let mat = MaterialDef::new("test_material")
            .with_diffuse(0.8, 0.2, 0.2, 1.0)
            .with_shininess(64.0)
            .with_transparency(0.5);

        assert_eq!(mat.name, "test_material");
        assert_eq!(mat.diffuse[0], 0.8);
        assert_eq!(mat.shininess, 64.0);
        assert_eq!(mat.transparency, 0.5);
        assert!(mat.is_transparent());
    }

    #[test]
    fn test_material_from_color_name() {
        let mat = MaterialDef::from_color_name("::test::material::grey");
        assert!(mat.diffuse[0] > 0.4 && mat.diffuse[0] < 0.6);

        let mat = MaterialDef::from_color_name("wood_oak");
        assert!(mat.diffuse[0] > mat.diffuse[2]); // More red/brown than blue
    }

    #[test]
    fn test_mat_parser() {
        let content = r#"
            # Comment
            amb 0.2 0.2 0.2
            dif 0.8 0.4 0.2
            spe 0.3 0.3 0.3
            shi 32
            tra 0.0
        "#;

        let mat = MatParser::parse(content, "test").unwrap();
        assert_eq!(mat.ambient, [0.2, 0.2, 0.2, 1.0]);
        assert_eq!(mat.diffuse, [0.8, 0.4, 0.2, 1.0]);
        assert_eq!(mat.shininess, 32.0);
    }

    #[test]
    fn test_mat_parser_inline() {
        let inline = "$ amb 0.2 0.2 0.2; dif 0.8 0.4 0.2; shi 64";

        let mat = MatParser::parse_inline(inline, "inline_test").unwrap();
        assert_eq!(mat.ambient, [0.2, 0.2, 0.2, 1.0]);
        assert_eq!(mat.diffuse, [0.8, 0.4, 0.2, 1.0]);
        assert_eq!(mat.shininess, 64.0);
    }

    #[test]
    fn test_material_resolver() {
        let mut resolver = MaterialResolver::new();

        let mat = MaterialDef::new("test_material").with_diffuse(1.0, 0.0, 0.0, 1.0);
        resolver.register(mat);

        let resolved = resolver.resolve("test_material");
        assert_eq!(resolved.diffuse[0], 1.0);

        // Fallback for unknown
        let unknown = resolver.resolve("unknown_material");
        assert_eq!(unknown.name, "fallback");
    }

    #[test]
    fn test_material_resolver_or_create() {
        let resolver = MaterialResolver::new();

        // Creates from color name when not registered
        let mat = resolver.resolve_or_create("wood_panel");
        assert!(mat.diffuse[0] > mat.diffuse[2]); // Wood-like color
    }

    #[test]
    fn test_material_def_parse_color() {
        // Basic RGB color
        let color = MaterialDef::parse_color("0.8 0.4 0.2").unwrap();
        assert!((color[0] - 0.8).abs() < 0.001);
        assert!((color[1] - 0.4).abs() < 0.001);
        assert!((color[2] - 0.2).abs() < 0.001);
        assert!((color[3] - 1.0).abs() < 0.001); // Default alpha

        // RGBA color
        let color = MaterialDef::parse_color("0.5 0.5 0.5 0.7").unwrap();
        assert!((color[3] - 0.7).abs() < 0.001);

        // Invalid color - too few components
        assert!(MaterialDef::parse_color("0.8 0.4").is_err());
    }

    #[test]
    fn test_parse_ebase_mat_table_basic() {
        let data = r#"
# Comment line
wood_oak	dif=0.6 0.4 0.2	amb=0.2 0.1 0.05	shi=32
metal_chrome	dif=0.8 0.8 0.85	spe=0.9 0.9 0.9	shi=128
"#;

        let entries = parse_ebase_mat_table(data);
        assert_eq!(entries.len(), 2);

        // Check first entry
        assert_eq!(entries[0].name, "wood_oak");
        assert_eq!(entries[0].diffuse, Some("0.6 0.4 0.2".to_string()));
        assert_eq!(entries[0].ambient, Some("0.2 0.1 0.05".to_string()));
        assert_eq!(entries[0].shininess, Some(32.0));

        // Check second entry
        assert_eq!(entries[1].name, "metal_chrome");
        assert_eq!(entries[1].specular, Some("0.9 0.9 0.9".to_string()));
        assert_eq!(entries[1].shininess, Some(128.0));
    }

    #[test]
    fn test_parse_ebase_mat_table_with_texture() {
        let data = "textured_wood	tex=wood.png	prj=prjz	dif=0.5 0.5 0.5";

        let entries = parse_ebase_mat_table(data);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].texture, Some("wood.png".to_string()));
        assert_eq!(entries[0].projection, Some("prjz".to_string()));
    }

    #[test]
    fn test_parse_ebase_mat_table_semicolon_separator() {
        let data = "glass_material;dif=0.9 0.9 0.95;tra=0.5;shi=256";

        let entries = parse_ebase_mat_table(data);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "glass_material");
        assert_eq!(entries[0].transparency, Some(0.5));
    }

    #[test]
    fn test_ebase_material_entry_to_material_def() {
        let entry = EbaseMaterialEntry {
            name: "test_material".to_string(),
            ambient: Some("0.2 0.2 0.2".to_string()),
            diffuse: Some("0.8 0.4 0.2".to_string()),
            specular: Some("0.3 0.3 0.3".to_string()),
            shininess: Some(64.0),
            transparency: Some(0.1),
            texture: Some("wood.png".to_string()),
            projection: Some("prjy".to_string()),
        };

        let mat = entry.to_material_def();
        assert_eq!(mat.name, "test_material");
        assert!((mat.ambient[0] - 0.2).abs() < 0.001);
        assert!((mat.diffuse[0] - 0.8).abs() < 0.001);
        assert!((mat.specular[0] - 0.3).abs() < 0.001);
        assert_eq!(mat.shininess, 64.0);
        assert_eq!(mat.transparency, 0.1);
        assert!(mat.texture.is_some());
        let tex = mat.texture.unwrap();
        assert_eq!(tex.filename, "wood.png");
        assert_eq!(tex.projection, TextureProjection::ProjectY);
    }

    #[test]
    fn test_load_ebase_materials() {
        let data = r#"
mat_red	dif=0.9 0.1 0.1
mat_blue	dif=0.1 0.1 0.9
"#;

        let mut resolver = MaterialResolver::new();
        load_ebase_materials(data, &mut resolver);

        assert!(resolver.contains("mat_red"));
        assert!(resolver.contains("mat_blue"));

        let red = resolver.resolve("mat_red");
        assert!(red.diffuse[0] > red.diffuse[2]); // More red than blue

        let blue = resolver.resolve("mat_blue");
        assert!(blue.diffuse[2] > blue.diffuse[0]); // More blue than red
    }

    #[test]
    fn test_parse_ebase_mat_table_empty_lines() {
        let data = r#"

# Header comment

material1	dif=0.5 0.5 0.5

# Another comment

material2	dif=0.3 0.3 0.3

"#;

        let entries = parse_ebase_mat_table(data);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].name, "material1");
        assert_eq!(entries[1].name, "material2");
    }

    #[test]
    fn test_ebase_material_entry_minimal() {
        // Entry with only name, no properties
        let entry = EbaseMaterialEntry {
            name: "minimal_mat".to_string(),
            ambient: None,
            diffuse: None,
            specular: None,
            shininess: None,
            transparency: None,
            texture: None,
            projection: None,
        };

        let mat = entry.to_material_def();
        assert_eq!(mat.name, "minimal_mat");
        // Should have default values
        assert_eq!(mat.shininess, 32.0);
        assert_eq!(mat.transparency, 0.0);
    }

    #[test]
    fn test_texture_projection_default() {
        let default: TextureProjection = Default::default();
        assert_eq!(default, TextureProjection::ProjectX);
    }

    #[test]
    fn test_texture_projection_parse_aliases() {
        // Test all aliases
        assert_eq!(TextureProjection::parse("project_x"), TextureProjection::ProjectX);
        assert_eq!(TextureProjection::parse("x"), TextureProjection::ProjectX);
        assert_eq!(TextureProjection::parse("project_y"), TextureProjection::ProjectY);
        assert_eq!(TextureProjection::parse("y"), TextureProjection::ProjectY);
        assert_eq!(TextureProjection::parse("project_z"), TextureProjection::ProjectZ);
        assert_eq!(TextureProjection::parse("z"), TextureProjection::ProjectZ);
        assert_eq!(TextureProjection::parse("cylindrical"), TextureProjection::Cylindrical);
        assert_eq!(TextureProjection::parse("spherical"), TextureProjection::Spherical);
        assert_eq!(TextureProjection::parse("cone"), TextureProjection::Conical);
        assert_eq!(TextureProjection::parse("conical"), TextureProjection::Conical);
        assert_eq!(TextureProjection::parse("circ"), TextureProjection::Circle);
        assert_eq!(TextureProjection::parse("circle"), TextureProjection::Circle);
        // Unknown defaults to ProjectZ
        assert_eq!(TextureProjection::parse("unknown"), TextureProjection::ProjectZ);
    }

    #[test]
    fn test_texture_projection_to_ofml_str_all() {
        assert_eq!(TextureProjection::ProjectY.to_ofml_str(), "prjy");
        assert_eq!(TextureProjection::ProjectZ.to_ofml_str(), "prjz");
        assert_eq!(TextureProjection::Cylindrical.to_ofml_str(), "cyl");
        assert_eq!(TextureProjection::Spherical.to_ofml_str(), "sph");
        assert_eq!(TextureProjection::Conical.to_ofml_str(), "cone");
        assert_eq!(TextureProjection::Circle.to_ofml_str(), "circ");
    }

    #[test]
    fn test_texture_projection_clone() {
        let proj = TextureProjection::Cylindrical;
        let cloned = proj.clone();
        assert_eq!(cloned, TextureProjection::Cylindrical);
    }

    #[test]
    fn test_texture_def_with_rotation() {
        let tex = TextureDef::new("image.png")
            .with_rotation(std::f32::consts::PI / 4.0);
        assert!((tex.rotation - std::f32::consts::PI / 4.0).abs() < 0.0001);
    }

    #[test]
    fn test_texture_def_clone() {
        let tex = TextureDef::new("clone.png")
            .with_scale(2.0, 3.0)
            .with_offset(0.1, 0.2);
        let cloned = tex.clone();
        assert_eq!(cloned.filename, "clone.png");
        assert_eq!(cloned.u_scale, 2.0);
        assert_eq!(cloned.v_scale, 3.0);
        assert_eq!(cloned.u_offset, 0.1);
        assert_eq!(cloned.v_offset, 0.2);
    }

    #[test]
    fn test_texture_def_debug() {
        let tex = TextureDef::new("debug.png");
        let debug_str = format!("{:?}", tex);
        assert!(debug_str.contains("debug.png"));
    }

    #[test]
    fn test_material_def_with_texture() {
        let tex = TextureDef::new("material_tex.png");
        let mat = MaterialDef::new("textured_mat").with_texture(tex);
        assert!(mat.texture.is_some());
        assert_eq!(mat.texture.unwrap().filename, "material_tex.png");
    }

    #[test]
    fn test_material_def_with_ambient() {
        let mat = MaterialDef::new("ambient_test").with_ambient(0.1, 0.2, 0.3, 0.4);
        assert_eq!(mat.ambient, [0.1, 0.2, 0.3, 0.4]);
    }

    #[test]
    fn test_material_def_with_specular() {
        let mat = MaterialDef::new("specular_test").with_specular(0.7, 0.8, 0.9, 1.0);
        assert_eq!(mat.specular, [0.7, 0.8, 0.9, 1.0]);
    }

    #[test]
    fn test_material_def_is_transparent_from_diffuse_alpha() {
        let mat = MaterialDef::new("alpha_transparent").with_diffuse(1.0, 1.0, 1.0, 0.5);
        assert!(mat.is_transparent());
    }

    #[test]
    fn test_material_def_is_not_transparent() {
        let mat = MaterialDef::new("opaque")
            .with_diffuse(1.0, 1.0, 1.0, 1.0)
            .with_transparency(0.0);
        assert!(!mat.is_transparent());
    }

    #[test]
    fn test_material_def_clone() {
        let mat = MaterialDef::new("cloneable")
            .with_diffuse(0.5, 0.5, 0.5, 1.0)
            .with_shininess(100.0);
        let cloned = mat.clone();
        assert_eq!(cloned.name, "cloneable");
        assert_eq!(cloned.shininess, 100.0);
    }

    #[test]
    fn test_material_def_debug() {
        let mat = MaterialDef::new("debug_mat");
        let debug_str = format!("{:?}", mat);
        assert!(debug_str.contains("debug_mat"));
    }

    #[test]
    fn test_material_def_default() {
        let mat = MaterialDef::default();
        assert_eq!(mat.name, "default");
    }

    #[test]
    fn test_material_color_from_name_all_colors() {
        // Test all color name patterns
        let white = MaterialDef::from_color_name("white_material");
        assert!(white.diffuse[0] > 0.8);

        let black = MaterialDef::from_color_name("black_material");
        assert!(black.diffuse[0] < 0.2);

        let red = MaterialDef::from_color_name("red_paint");
        assert!(red.diffuse[0] > red.diffuse[1] && red.diffuse[0] > red.diffuse[2]);

        let green = MaterialDef::from_color_name("green_surface");
        assert!(green.diffuse[1] > green.diffuse[0] && green.diffuse[1] > green.diffuse[2]);

        let blue = MaterialDef::from_color_name("blue_fabric");
        assert!(blue.diffuse[2] > blue.diffuse[0] && blue.diffuse[2] > blue.diffuse[1]);

        let gray = MaterialDef::from_color_name("gray_metal");
        let grey = MaterialDef::from_color_name("grey_plastic");
        assert!((gray.diffuse[0] - gray.diffuse[1]).abs() < 0.01);
        assert!((grey.diffuse[0] - grey.diffuse[1]).abs() < 0.01);

        let walnut = MaterialDef::from_color_name("walnut_finish");
        assert!(walnut.diffuse[0] > walnut.diffuse[2]); // Wood-like

        let steel = MaterialDef::from_color_name("steel_frame");
        let chrome = MaterialDef::from_color_name("chrome_handle");
        let metal = MaterialDef::from_color_name("metal_bracket");
        // Metal has higher specular
        assert!(chrome.specular[0] > 0.5);
        assert!(steel.specular[0] > 0.5 || metal.specular[0] > 0.5);

        let silver = MaterialDef::from_color_name("silver_trim");
        assert!(silver.diffuse[0] > 0.7);

        let gold = MaterialDef::from_color_name("gold_accent");
        assert!(gold.diffuse[0] > gold.diffuse[2]); // Yellow-ish
    }

    #[test]
    fn test_material_parse_color_invalid_components() {
        // Invalid number format
        let result = MaterialDef::parse_color("abc 0.5 0.5");
        assert!(result.is_err());

        let result = MaterialDef::parse_color("0.5 xyz 0.5");
        assert!(result.is_err());

        let result = MaterialDef::parse_color("0.5 0.5 zzz");
        assert!(result.is_err());
    }

    #[test]
    fn test_mat_parser_with_texture() {
        let content = r#"
            dif 0.5 0.5 0.5
            tex wood_grain.jpg
            prj prjy
        "#;

        let mat = MatParser::parse(content, "textured").unwrap();
        assert!(mat.texture.is_some());
        let tex = mat.texture.unwrap();
        assert_eq!(tex.filename, "wood_grain.jpg");
        assert_eq!(tex.projection, TextureProjection::ProjectY);
    }

    #[test]
    fn test_mat_parser_projection_without_texture() {
        // Projection line without a texture should be ignored
        let content = r#"
            dif 0.5 0.5 0.5
            prj prjy
        "#;

        let mat = MatParser::parse(content, "no_tex").unwrap();
        assert!(mat.texture.is_none());
    }

    #[test]
    fn test_mat_parser_comments() {
        let content = r#"
            # This is a comment
            // This is also a comment
            dif 0.8 0.2 0.1
            # Another comment
        "#;

        let mat = MatParser::parse(content, "commented").unwrap();
        assert!((mat.diffuse[0] - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_mat_parser_long_form_keys() {
        let content = r#"
            ambient 0.1 0.1 0.1
            diffuse 0.5 0.5 0.5
            specular 0.3 0.3 0.3
            shininess 50
            transparency 0.2
            texture myfile.png
            projection cylindrical
        "#;

        let mat = MatParser::parse(content, "long_form").unwrap();
        assert!((mat.ambient[0] - 0.1).abs() < 0.001);
        assert!((mat.diffuse[0] - 0.5).abs() < 0.001);
        assert!((mat.specular[0] - 0.3).abs() < 0.001);
        assert_eq!(mat.shininess, 50.0);
        assert!((mat.transparency - 0.2).abs() < 0.001);
        let tex = mat.texture.unwrap();
        assert_eq!(tex.filename, "myfile.png");
        assert_eq!(tex.projection, TextureProjection::Cylindrical);
    }

    #[test]
    fn test_mat_parser_invalid_shininess() {
        let content = "shi not_a_number";
        let result = MatParser::parse(content, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_mat_parser_invalid_transparency() {
        let content = "tra invalid";
        let result = MatParser::parse(content, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_mat_parser_invalid_color() {
        let content = "dif 0.5 0.5"; // Too few components
        let result = MatParser::parse(content, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_mat_parser_inline_without_dollar() {
        // Should work without leading $
        let inline = "amb 0.3 0.3 0.3; dif 0.7 0.7 0.7";
        let mat = MatParser::parse_inline(inline, "no_dollar").unwrap();
        assert!((mat.ambient[0] - 0.3).abs() < 0.001);
        assert!((mat.diffuse[0] - 0.7).abs() < 0.001);
    }

    #[test]
    fn test_mat_parser_inline_transparency() {
        let inline = "$ tra 0.5";
        let mat = MatParser::parse_inline(inline, "transparent").unwrap();
        assert_eq!(mat.transparency, 0.5);
    }

    #[test]
    fn test_mat_parser_inline_specular() {
        let inline = "$ spe 0.9 0.9 0.9";
        let mat = MatParser::parse_inline(inline, "shiny").unwrap();
        assert!((mat.specular[0] - 0.9).abs() < 0.001);
    }

    #[test]
    fn test_mat_parser_inline_empty() {
        let inline = "";
        let mat = MatParser::parse_inline(inline, "empty").unwrap();
        // Should return material with default values
        assert_eq!(mat.name, "empty");
    }

    #[test]
    fn test_mat_parser_inline_unknown_key() {
        let inline = "$ unknown 1 2 3";
        // Should not fail, just ignore unknown keys
        let mat = MatParser::parse_inline(inline, "unknown_key").unwrap();
        assert_eq!(mat.name, "unknown_key");
    }

    #[test]
    fn test_mat_parser_parse_color_tokens_too_few() {
        // Test private function edge case via inline parsing
        let inline = "$ amb 0.5 0.5"; // Only 2 components
        let mat = MatParser::parse_inline(inline, "short").unwrap();
        // Should not update ambient (needs 3+ components)
        assert_eq!(mat.ambient, [0.2, 0.2, 0.2, 1.0]); // Default
    }

    #[test]
    fn test_material_resolver_resolve_namespace() {
        let mut resolver = MaterialResolver::new();
        let mat = MaterialDef::new("test_material").with_diffuse(1.0, 0.0, 0.0, 1.0);
        resolver.register(mat);

        // Should resolve without namespace
        let resolved = resolver.resolve("::some::namespace::test_material");
        assert_eq!(resolved.diffuse[0], 1.0);
    }

    #[test]
    fn test_material_resolver_names() {
        let mut resolver = MaterialResolver::new();
        resolver.register(MaterialDef::new("mat_a"));
        resolver.register(MaterialDef::new("mat_b"));

        let names = resolver.names();
        assert_eq!(names.len(), 2);
        assert!(names.iter().any(|n| *n == "mat_a"));
        assert!(names.iter().any(|n| *n == "mat_b"));
    }

    #[test]
    fn test_material_resolver_set_fallback() {
        let mut resolver = MaterialResolver::new();
        let custom_fallback = MaterialDef::new("custom_fallback").with_diffuse(0.0, 1.0, 0.0, 1.0);
        resolver.set_fallback(custom_fallback);

        let fallback = resolver.resolve("nonexistent");
        assert_eq!(fallback.name, "custom_fallback");
        assert_eq!(fallback.diffuse[1], 1.0); // Green
    }

    #[test]
    fn test_material_resolver_default() {
        let resolver = MaterialResolver::default();
        // Should have fallback material
        let fallback = resolver.resolve("nonexistent");
        assert_eq!(fallback.name, "fallback");
    }

    #[test]
    fn test_material_resolver_resolve_or_create_namespace() {
        let mut resolver = MaterialResolver::new();
        let mat = MaterialDef::new("metal_chrome").with_diffuse(0.8, 0.8, 0.8, 1.0);
        resolver.register(mat);

        let resolved = resolver.resolve_or_create("::ns::metal_chrome");
        assert_eq!(resolved.diffuse[0], 0.8);
    }

    #[test]
    fn test_ebase_material_entry_debug() {
        let entry = EbaseMaterialEntry {
            name: "debug_entry".to_string(),
            ambient: None,
            diffuse: None,
            specular: None,
            shininess: None,
            transparency: None,
            texture: None,
            projection: None,
        };
        let debug_str = format!("{:?}", entry);
        assert!(debug_str.contains("debug_entry"));
    }

    #[test]
    fn test_ebase_material_entry_clone() {
        let entry = EbaseMaterialEntry {
            name: "cloneable_entry".to_string(),
            ambient: Some("0.5 0.5 0.5".to_string()),
            diffuse: None,
            specular: None,
            shininess: Some(64.0),
            transparency: None,
            texture: None,
            projection: None,
        };
        let cloned = entry.clone();
        assert_eq!(cloned.name, "cloneable_entry");
        assert_eq!(cloned.shininess, Some(64.0));
    }

    #[test]
    fn test_ebase_material_entry_with_invalid_color() {
        // Entry with invalid color string should use defaults
        let entry = EbaseMaterialEntry {
            name: "invalid_color".to_string(),
            ambient: Some("invalid".to_string()),
            diffuse: Some("also invalid".to_string()),
            specular: Some("not a color".to_string()),
            shininess: None,
            transparency: None,
            texture: None,
            projection: None,
        };

        let mat = entry.to_material_def();
        assert_eq!(mat.name, "invalid_color");
        // Should have default values since colors are invalid
        assert_eq!(mat.ambient, [0.2, 0.2, 0.2, 1.0]);
        assert_eq!(mat.diffuse, [0.6, 0.6, 0.6, 1.0]);
        assert_eq!(mat.specular, [0.3, 0.3, 0.3, 1.0]);
    }

    #[test]
    fn test_parse_ebase_mat_table_unknown_property() {
        let data = "material_name	unknown_prop=some_value	dif=0.5 0.5 0.5";
        let entries = parse_ebase_mat_table(data);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].diffuse, Some("0.5 0.5 0.5".to_string()));
    }

    #[test]
    fn test_parse_ebase_mat_table_no_value() {
        // Property without = sign should be ignored
        let data = "material_name	noequal	dif=0.5 0.5 0.5";
        let entries = parse_ebase_mat_table(data);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].diffuse, Some("0.5 0.5 0.5".to_string()));
    }

    #[test]
    fn test_material_from_color_name_default_grey() {
        // Test with a color name that doesn't match any known keyword
        let mat = MaterialDef::from_color_name("unknown_xyz");
        // Should default to grey [0.5, 0.5, 0.5, 1.0]
        assert!((mat.diffuse[0] - 0.5).abs() < 0.01);
        assert!((mat.diffuse[1] - 0.5).abs() < 0.01);
        assert!((mat.diffuse[2] - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_mat_parser_single_keyword_line() {
        // Test line with just a keyword and no value
        let content = r#"
            amb 0.2 0.2 0.2
            dif
            spe 0.3 0.3 0.3
        "#;
        // Should parse successfully, skipping the single-keyword line
        let mat = MatParser::parse(content, "test");
        assert!(mat.is_ok());
        let mat = mat.unwrap();
        assert_eq!(mat.ambient, [0.2, 0.2, 0.2, 1.0]);
        assert_eq!(mat.specular, [0.3, 0.3, 0.3, 1.0]);
    }

    #[test]
    fn test_material_from_color_name_chrome() {
        let mat = MaterialDef::from_color_name("metal_chrome_finish");
        // Chrome should have high specular
        assert!(mat.specular[0] > 0.7);
        assert!(mat.specular[1] > 0.7);
        assert!(mat.specular[2] > 0.7);
    }

    #[test]
    fn test_material_from_color_name_silver() {
        // Use "silver" without "metal" to hit the silver branch
        let mat = MaterialDef::from_color_name("silver_trim");
        // Silver should have light grey diffuse [0.75, 0.75, 0.75]
        assert!((mat.diffuse[0] - 0.75).abs() < 0.01, "silver diffuse r={}", mat.diffuse[0]);
        assert!((mat.diffuse[1] - 0.75).abs() < 0.01, "silver diffuse g={}", mat.diffuse[1]);
        assert!((mat.diffuse[2] - 0.75).abs() < 0.01, "silver diffuse b={}", mat.diffuse[2]);
    }

    #[test]
    fn test_material_from_color_name_gold() {
        let mat = MaterialDef::from_color_name("gold_trim");
        // Gold should have more red/yellow than blue
        assert!(mat.diffuse[0] > mat.diffuse[2]);
        assert!(mat.diffuse[1] > mat.diffuse[2]);
    }
}
