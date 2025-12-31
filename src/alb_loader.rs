//! Multi-ALB Loader for OFML Package Dependencies
//!
//! This module handles loading multiple ALB (Archive Library) files in the correct
//! dependency order and resolving cross-package class references.
//!
//! ## Architecture
//!
//! OFML manufacturers typically organize their data in multiple ALBs:
//! - `global` - Global manufacturer settings and resources
//! - `basics` - Base classes shared across products
//! - Product-specific ALBs (e.g., `workit`, `joyn`)
//!
//! The loader ensures that dependencies are loaded before dependents.

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::interpreter::Interpreter;
use crate::parser::Parser;

/// ALB Archive password (standard OFML encryption key)
const ALB_PASSWORD: &[u8] = b"Gur#Ynzo$Yvrf%Qbja&Ba*Oebnqjnl.";

/// Result type for ALB operations
pub type AlbResult<T> = Result<T, AlbError>;

/// ALB loading errors
#[derive(Debug)]
pub enum AlbError {
    Io(std::io::Error),
    Zip(zip::result::ZipError),
    Parse(String),
    MissingDependency(String),
    CircularDependency(String),
}

impl std::fmt::Display for AlbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlbError::Io(e) => write!(f, "IO error: {}", e),
            AlbError::Zip(e) => write!(f, "ZIP error: {}", e),
            AlbError::Parse(msg) => write!(f, "Parse error: {}", msg),
            AlbError::MissingDependency(dep) => write!(f, "Missing dependency: {}", dep),
            AlbError::CircularDependency(msg) => write!(f, "Circular dependency: {}", msg),
        }
    }
}

impl std::error::Error for AlbError {}

impl From<std::io::Error> for AlbError {
    fn from(e: std::io::Error) -> Self {
        AlbError::Io(e)
    }
}

impl From<zip::result::ZipError> for AlbError {
    fn from(e: zip::result::ZipError) -> Self {
        AlbError::Zip(e)
    }
}

/// CLS source file with package information
#[derive(Debug, Clone)]
pub struct ClsSource {
    /// Original filename
    pub filename: String,
    /// Package declaration (e.g., "::vitra::workit")
    pub package: String,
    /// Import statements
    pub imports: Vec<String>,
    /// Source code
    pub source: String,
    /// Classes defined in this file
    pub classes: Vec<String>,
    /// Parent classes referenced (for dependency ordering)
    pub parent_classes: Vec<String>,
}

impl ClsSource {
    /// Parse CLS source to extract package, imports, and class definitions
    pub fn parse(filename: &str, source: &str) -> Self {
        let mut package = String::new();
        let mut imports = Vec::new();
        let mut classes = Vec::new();
        let mut parent_classes = Vec::new();

        for line in source.lines() {
            let line = line.trim();

            // Package declaration
            if line.starts_with("package ") {
                package = line
                    .strip_prefix("package ")
                    .unwrap_or("")
                    .trim_end_matches(';')
                    .trim()
                    .to_string();
            }

            // Import statements
            if line.starts_with("import ") {
                let import = line
                    .strip_prefix("import ")
                    .unwrap_or("")
                    .trim_end_matches(';')
                    .trim()
                    .to_string();
                imports.push(import);
            }

            // Class definitions with inheritance
            if line.contains("class ") && (line.contains(": ") || line.contains(":")) {
                // Extract class name and parent
                if let Some(class_part) = line.split("class ").nth(1) {
                    let parts: Vec<&str> = class_part.split(':').collect();
                    if !parts.is_empty() {
                        let class_name = parts[0].trim().to_string();
                        if !class_name.is_empty() && class_name != "{" {
                            classes.push(class_name);
                        }
                    }
                    if parts.len() > 1 {
                        let parent = parts[1].trim().trim_start_matches(':').trim();
                        // Remove anything after the class name (like {)
                        let parent = parent.split_whitespace().next().unwrap_or(parent);
                        let parent = parent.trim_end_matches('{').trim();
                        if !parent.is_empty() {
                            parent_classes.push(parent.to_string());
                        }
                    }
                }
            }
        }

        Self {
            filename: filename.to_string(),
            package,
            imports,
            source: source.to_string(),
            classes,
            parent_classes,
        }
    }

    /// Get the fully qualified class names
    pub fn qualified_classes(&self) -> Vec<String> {
        self.classes
            .iter()
            .map(|c| {
                if self.package.is_empty() {
                    c.clone()
                } else {
                    format!("{}::{}", self.package, c)
                }
            })
            .collect()
    }
}

/// Multi-ALB loader for loading manufacturer packages with dependencies
#[derive(Debug)]
pub struct AlbLoader {
    /// Base directory for OFML data
    pub data_dir: PathBuf,
    /// Loaded CLS sources by package
    pub sources: HashMap<String, Vec<ClsSource>>,
    /// Class name to package mapping
    pub class_to_package: HashMap<String, String>,
    /// Package to ALB path mapping
    pub package_to_alb: HashMap<String, PathBuf>,
    /// Loaded ALBs (to avoid reloading)
    loaded_albs: HashSet<PathBuf>,
}

impl AlbLoader {
    /// Create a new ALB loader
    pub fn new(data_dir: impl AsRef<Path>) -> Self {
        Self {
            data_dir: data_dir.as_ref().to_path_buf(),
            sources: HashMap::new(),
            class_to_package: HashMap::new(),
            package_to_alb: HashMap::new(),
            loaded_albs: HashSet::new(),
        }
    }

    /// Load all ALBs for a manufacturer with dependencies
    pub fn load_manufacturer(&mut self, manufacturer: &str) -> AlbResult<()> {
        let mfr_dir = self.data_dir.join(manufacturer);
        if !mfr_dir.exists() {
            return Err(AlbError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Manufacturer directory not found: {}", manufacturer),
            )));
        }

        // Find all ALBs for this manufacturer
        let mut albs = Vec::new();
        self.find_albs(&mfr_dir, &mut albs)?;

        // Load ALBs in dependency order (global, basics first)
        let priority_order = ["global", "basics", "extension"];

        // Sort ALBs by priority
        albs.sort_by(|a, b| {
            let a_name = a.file_stem().unwrap_or_default().to_string_lossy();
            let b_name = b.file_stem().unwrap_or_default().to_string_lossy();

            let a_priority = priority_order
                .iter()
                .position(|&p| a_name.contains(p))
                .unwrap_or(99);
            let b_priority = priority_order
                .iter()
                .position(|&p| b_name.contains(p))
                .unwrap_or(99);

            a_priority.cmp(&b_priority)
        });

        // Load each ALB
        for alb_path in albs {
            self.load_alb(&alb_path)?;
        }

        Ok(())
    }

    /// Find all ALB files in a directory recursively
    fn find_albs(&self, dir: &Path, albs: &mut Vec<PathBuf>) -> AlbResult<()> {
        if !dir.is_dir() {
            return Ok(());
        }

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                self.find_albs(&path, albs)?;
            } else if path.extension().map_or(false, |e| e == "alb") {
                albs.push(path);
            }
        }

        Ok(())
    }

    /// Load a single ALB file
    pub fn load_alb(&mut self, path: &Path) -> AlbResult<Vec<ClsSource>> {
        let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());

        if self.loaded_albs.contains(&canonical) {
            // Already loaded, return existing sources
            return Ok(Vec::new());
        }

        let file = File::open(path)?;
        let mut archive = zip::ZipArchive::new(file)?;
        let mut cls_sources = Vec::new();

        for i in 0..archive.len() {
            let mut file = match archive.by_index_decrypt(i, ALB_PASSWORD) {
                Ok(f) => f,
                Err(_) => continue,
            };

            let name = file.name().to_string();
            if !name.to_lowercase().ends_with(".cls") {
                continue;
            }

            let mut source = String::new();
            if file.read_to_string(&mut source).is_ok() {
                let cls = ClsSource::parse(&name, &source);

                // Register classes
                for class_name in &cls.classes {
                    self.class_to_package
                        .insert(class_name.clone(), cls.package.clone());
                }

                // Register package to ALB mapping
                if !cls.package.is_empty() {
                    self.package_to_alb
                        .insert(cls.package.clone(), path.to_path_buf());
                }

                cls_sources.push(cls);
            }
        }

        // Store sources by package
        for cls in &cls_sources {
            self.sources
                .entry(cls.package.clone())
                .or_insert_with(Vec::new)
                .push(cls.clone());
        }

        self.loaded_albs.insert(canonical);
        Ok(cls_sources)
    }

    /// Resolve a class name to its fully qualified name
    pub fn resolve_class(
        &self,
        name: &str,
        current_package: &str,
        imports: &[String],
    ) -> Option<String> {
        // Already fully qualified
        if name.starts_with("::") {
            return Some(name.to_string());
        }

        // Check current package
        if self.class_to_package.get(name) == Some(&current_package.to_string()) {
            return Some(format!("{}::{}", current_package, name));
        }

        // Check imports
        for import in imports {
            let import_pkg = import.trim_end_matches("::*");
            if let Some(pkg) = self.class_to_package.get(name) {
                if pkg.starts_with(import_pkg) || import_pkg.ends_with(&format!("::{}", name)) {
                    return Some(format!("{}::{}", pkg, name));
                }
            }
        }

        // Check if it's a known class in any package
        if let Some(pkg) = self.class_to_package.get(name) {
            return Some(format!("{}::{}", pkg, name));
        }

        // Could be a framework class
        None
    }

    /// Get all CLS sources sorted by dependency order
    pub fn get_sorted_sources(&self) -> Vec<&ClsSource> {
        // First, collect and sort by filename to ensure deterministic initial order
        let mut all_sources: Vec<&ClsSource> = self.sources.values().flatten().collect();
        all_sources.sort_by(|a, b| a.filename.cmp(&b.filename));

        // Build a map from class name to source index
        let mut class_to_idx: HashMap<&str, usize> = HashMap::new();
        for (i, src) in all_sources.iter().enumerate() {
            for class in &src.classes {
                class_to_idx.insert(class.as_str(), i);
            }
        }

        // Calculate priority scores for each source
        let priority_packages = ["::ofml::oi", "::ofml::xoi", "::ofml::go"];
        let mut scores: Vec<(usize, i32, &str)> = all_sources
            .iter()
            .enumerate()
            .map(|(i, src)| {
                let mut score = 0i32;

                // Package priority (lower is better)
                let pkg_priority = priority_packages
                    .iter()
                    .position(|&p| src.package.starts_with(p))
                    .map(|p| p as i32)
                    .unwrap_or(99);
                score += pkg_priority * 10000;

                // Basics comes before others
                if !src.package.contains("basics") {
                    score += 5000;
                }

                // Parent dependencies: if parent is in our sources, we should come after it
                for parent in &src.parent_classes {
                    if let Some(&parent_idx) = class_to_idx.get(parent.as_str()) {
                        if parent_idx != i {
                            // We depend on something, increase our score slightly
                            score += 100;
                        }
                    }
                }

                (i, score, src.filename.as_str())
            })
            .collect();

        // Sort by score, then by filename for deterministic ordering
        scores.sort_by(|a, b| match a.1.cmp(&b.1) {
            std::cmp::Ordering::Equal => a.2.cmp(&b.2),
            other => other,
        });

        // Return sources in sorted order
        scores.iter().map(|&(i, _, _)| all_sources[i]).collect()
    }

    /// Load all sources into an interpreter
    pub fn load_into_interpreter(&self, interp: &mut Interpreter) -> AlbResult<usize> {
        let sources = self.get_sorted_sources();
        let mut loaded = 0;
        let mut errors = Vec::new();

        for cls in sources {
            match Parser::new(&cls.source) {
                Ok(mut parser) => match parser.parse() {
                    Ok(ast) => {
                        if let Err(e) = interp.execute(&ast) {
                            errors.push(format!("{}: {}", cls.filename, e));
                        } else {
                            loaded += 1;
                        }
                    }
                    Err(e) => {
                        errors.push(format!("{}: parse error: {}", cls.filename, e));
                    }
                },
                Err(e) => {
                    errors.push(format!("{}: lex error: {}", cls.filename, e));
                }
            }
        }

        if !errors.is_empty() && loaded == 0 {
            return Err(AlbError::Parse(errors.join("\n")));
        }

        Ok(loaded)
    }

    /// Get statistics about loaded packages
    pub fn stats(&self) -> LoaderStats {
        LoaderStats {
            packages: self.sources.len(),
            classes: self.class_to_package.len(),
            files: self.sources.values().map(|v| v.len()).sum(),
            albs: self.loaded_albs.len(),
        }
    }
}

/// Statistics about loaded packages
#[derive(Debug, Clone)]
pub struct LoaderStats {
    pub packages: usize,
    pub classes: usize,
    pub files: usize,
    pub albs: usize,
}

/// Load manufacturer data with all dependencies
pub fn load_manufacturer_with_deps(
    data_dir: &Path,
    manufacturer: &str,
    product_alb: Option<&Path>,
) -> AlbResult<AlbLoader> {
    let mut loader = AlbLoader::new(data_dir);

    // Load ofml framework ALBs first if available
    let ofml_path = data_dir.join("ofml");
    if ofml_path.exists() {
        let _ = loader.load_manufacturer("ofml");
    }

    // Load manufacturer
    loader.load_manufacturer(manufacturer)?;

    // Load specific product ALB if provided
    if let Some(alb) = product_alb {
        loader.load_alb(alb)?;
    }

    Ok(loader)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cls_source_parse() {
        let source = r#"
package ::vitra::workit;
import ::ofml::oi::*;
import ::ofml::xoi::*;

public class WorkitDesk : xOiBTGPlElement3
{
    public func initialize(pFa, pNa)
    {
        xOiBTGPlElement3::initialize(pFa, pNa);
    }
}
        "#;

        let cls = ClsSource::parse("workit.cls", source);
        assert_eq!(cls.package, "::vitra::workit");
        assert_eq!(cls.imports.len(), 2);
        assert!(cls.imports.contains(&"::ofml::oi::*".to_string()));
        assert!(cls.imports.contains(&"::ofml::xoi::*".to_string()));
        assert_eq!(cls.classes.len(), 1);
        assert!(cls.classes.contains(&"WorkitDesk".to_string()));
        assert!(cls.parent_classes.contains(&"xOiBTGPlElement3".to_string()));
    }

    #[test]
    fn test_cls_source_qualified_classes() {
        let cls = ClsSource {
            filename: "test.cls".to_string(),
            package: "::test::package".to_string(),
            imports: vec![],
            source: String::new(),
            classes: vec!["MyClass".to_string(), "OtherClass".to_string()],
            parent_classes: vec![],
        };

        let qualified = cls.qualified_classes();
        assert!(qualified.contains(&"::test::package::MyClass".to_string()));
        assert!(qualified.contains(&"::test::package::OtherClass".to_string()));
    }

    #[test]
    fn test_alb_loader_new() {
        let loader = AlbLoader::new("/tmp");
        assert_eq!(loader.data_dir, PathBuf::from("/tmp"));
        assert!(loader.sources.is_empty());
        assert!(loader.class_to_package.is_empty());
    }

    #[test]
    fn test_resolve_class_fully_qualified() {
        let loader = AlbLoader::new("/tmp");
        let result = loader.resolve_class("::ofml::xoi::xOiBTGPlElement3", "", &[]);
        assert_eq!(result, Some("::ofml::xoi::xOiBTGPlElement3".to_string()));
    }
}
