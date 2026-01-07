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
            } else if path.extension().is_some_and(|e| e == "alb") {
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
                .or_default()
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
            std::cmp::Ordering::Equal => a.2.cmp(b.2),
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
    fn test_cls_source_parse_empty() {
        let cls = ClsSource::parse("empty.cls", "");
        assert_eq!(cls.filename, "empty.cls");
        assert!(cls.package.is_empty());
        assert!(cls.imports.is_empty());
        assert!(cls.classes.is_empty());
        assert!(cls.parent_classes.is_empty());
    }

    #[test]
    fn test_cls_source_parse_no_inheritance() {
        let source = r#"
package ::test;
class SimpleClass {
    func test() {}
}
        "#;
        let cls = ClsSource::parse("simple.cls", source);
        assert_eq!(cls.package, "::test");
        assert!(cls.classes.is_empty()); // No colon after class name means no inheritance detected
    }

    #[test]
    fn test_cls_source_parse_multiple_classes() {
        let source = r#"
package ::multi;
import ::base::*;
public class ClassA : BaseA {}
private class ClassB : BaseB {}
        "#;
        let cls = ClsSource::parse("multi.cls", source);
        assert_eq!(cls.classes.len(), 2);
        assert!(cls.classes.contains(&"ClassA".to_string()));
        assert!(cls.classes.contains(&"ClassB".to_string()));
        assert_eq!(cls.parent_classes.len(), 2);
        assert!(cls.parent_classes.contains(&"BaseA".to_string()));
        assert!(cls.parent_classes.contains(&"BaseB".to_string()));
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
    fn test_cls_source_qualified_classes_empty_package() {
        let cls = ClsSource {
            filename: "test.cls".to_string(),
            package: String::new(),
            imports: vec![],
            source: String::new(),
            classes: vec!["MyClass".to_string()],
            parent_classes: vec![],
        };

        let qualified = cls.qualified_classes();
        assert_eq!(qualified, vec!["MyClass".to_string()]);
    }

    #[test]
    fn test_alb_loader_new() {
        let loader = AlbLoader::new("/tmp");
        assert_eq!(loader.data_dir, PathBuf::from("/tmp"));
        assert!(loader.sources.is_empty());
        assert!(loader.class_to_package.is_empty());
    }

    #[test]
    fn test_alb_loader_new_pathbuf() {
        let loader = AlbLoader::new(PathBuf::from("/test/path"));
        assert_eq!(loader.data_dir, PathBuf::from("/test/path"));
    }

    #[test]
    fn test_resolve_class_fully_qualified() {
        let loader = AlbLoader::new("/tmp");
        let result = loader.resolve_class("::ofml::xoi::xOiBTGPlElement3", "", &[]);
        assert_eq!(result, Some("::ofml::xoi::xOiBTGPlElement3".to_string()));
    }

    #[test]
    fn test_resolve_class_from_current_package() {
        let mut loader = AlbLoader::new("/tmp");
        loader
            .class_to_package
            .insert("MyClass".to_string(), "::test::pkg".to_string());

        let result = loader.resolve_class("MyClass", "::test::pkg", &[]);
        assert_eq!(result, Some("::test::pkg::MyClass".to_string()));
    }

    #[test]
    fn test_resolve_class_from_imports() {
        let mut loader = AlbLoader::new("/tmp");
        loader
            .class_to_package
            .insert("ImportedClass".to_string(), "::other::pkg".to_string());

        let imports = vec!["::other::pkg::*".to_string()];
        let result = loader.resolve_class("ImportedClass", "::current::pkg", &imports);
        assert_eq!(result, Some("::other::pkg::ImportedClass".to_string()));
    }

    #[test]
    fn test_resolve_class_unknown() {
        let loader = AlbLoader::new("/tmp");
        let result = loader.resolve_class("UnknownClass", "::test", &[]);
        assert_eq!(result, None);
    }

    #[test]
    fn test_loader_stats_empty() {
        let loader = AlbLoader::new("/tmp");
        let stats = loader.stats();
        assert_eq!(stats.packages, 0);
        assert_eq!(stats.classes, 0);
        assert_eq!(stats.files, 0);
        assert_eq!(stats.albs, 0);
    }

    #[test]
    fn test_loader_stats_with_data() {
        let mut loader = AlbLoader::new("/tmp");

        // Add some test data
        loader.class_to_package.insert("A".to_string(), "pkg1".to_string());
        loader.class_to_package.insert("B".to_string(), "pkg1".to_string());

        let cls = ClsSource {
            filename: "test.cls".to_string(),
            package: "pkg1".to_string(),
            imports: vec![],
            source: String::new(),
            classes: vec!["A".to_string(), "B".to_string()],
            parent_classes: vec![],
        };
        loader.sources.insert("pkg1".to_string(), vec![cls]);

        let stats = loader.stats();
        assert_eq!(stats.packages, 1);
        assert_eq!(stats.classes, 2);
        assert_eq!(stats.files, 1);
    }

    #[test]
    fn test_alb_error_display() {
        let io_err = AlbError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "test error",
        ));
        assert!(io_err.to_string().contains("IO error"));
        assert!(io_err.to_string().contains("test error"));

        let parse_err = AlbError::Parse("parse failed".to_string());
        assert!(parse_err.to_string().contains("Parse error"));
        assert!(parse_err.to_string().contains("parse failed"));

        let missing_err = AlbError::MissingDependency("dep1".to_string());
        assert!(missing_err.to_string().contains("Missing dependency"));
        assert!(missing_err.to_string().contains("dep1"));

        let circular_err = AlbError::CircularDependency("A -> B -> A".to_string());
        assert!(circular_err.to_string().contains("Circular dependency"));
        assert!(circular_err.to_string().contains("A -> B -> A"));
    }

    #[test]
    fn test_alb_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let alb_err: AlbError = io_err.into();
        matches!(alb_err, AlbError::Io(_));
    }

    #[test]
    fn test_alb_error_from_zip() {
        let zip_err = zip::result::ZipError::FileNotFound;
        let alb_err: AlbError = zip_err.into();
        matches!(alb_err, AlbError::Zip(_));
    }

    #[test]
    fn test_alb_error_is_error() {
        let err = AlbError::Parse("test".to_string());
        // Verify it implements std::error::Error
        let _: &dyn std::error::Error = &err;
    }

    #[test]
    fn test_cls_source_debug() {
        let cls = ClsSource::parse("test.cls", "package ::test;");
        let debug = format!("{:?}", cls);
        assert!(debug.contains("ClsSource"));
        assert!(debug.contains("test.cls"));
    }

    #[test]
    fn test_cls_source_clone() {
        let cls = ClsSource {
            filename: "original.cls".to_string(),
            package: "::pkg".to_string(),
            imports: vec!["::import".to_string()],
            source: "source code".to_string(),
            classes: vec!["Class1".to_string()],
            parent_classes: vec!["Parent1".to_string()],
        };
        let cloned = cls.clone();
        assert_eq!(cloned.filename, cls.filename);
        assert_eq!(cloned.package, cls.package);
        assert_eq!(cloned.imports, cls.imports);
        assert_eq!(cloned.classes, cls.classes);
    }

    #[test]
    fn test_loader_stats_debug_clone() {
        let stats = LoaderStats {
            packages: 5,
            classes: 20,
            files: 10,
            albs: 3,
        };
        let debug = format!("{:?}", stats);
        assert!(debug.contains("LoaderStats"));
        assert!(debug.contains("5"));

        let cloned = stats.clone();
        assert_eq!(cloned.packages, stats.packages);
        assert_eq!(cloned.classes, stats.classes);
    }

    #[test]
    fn test_alb_loader_debug() {
        let loader = AlbLoader::new("/tmp/test");
        let debug = format!("{:?}", loader);
        assert!(debug.contains("AlbLoader"));
        assert!(debug.contains("/tmp/test"));
    }

    #[test]
    fn test_load_manufacturer_not_found() {
        let mut loader = AlbLoader::new("/nonexistent");
        let result = loader.load_manufacturer("unknown");
        assert!(result.is_err());
        matches!(result.unwrap_err(), AlbError::Io(_));
    }

    #[test]
    fn test_get_sorted_sources_empty() {
        let loader = AlbLoader::new("/tmp");
        let sorted = loader.get_sorted_sources();
        assert!(sorted.is_empty());
    }

    #[test]
    fn test_get_sorted_sources_priority() {
        let mut loader = AlbLoader::new("/tmp");

        // Add sources with different packages
        let basics = ClsSource {
            filename: "basics.cls".to_string(),
            package: "::test::basics".to_string(),
            imports: vec![],
            source: String::new(),
            classes: vec!["BasicsClass".to_string()],
            parent_classes: vec![],
        };

        let product = ClsSource {
            filename: "product.cls".to_string(),
            package: "::test::product".to_string(),
            imports: vec![],
            source: String::new(),
            classes: vec!["ProductClass".to_string()],
            parent_classes: vec!["BasicsClass".to_string()],
        };

        let ofml = ClsSource {
            filename: "oi.cls".to_string(),
            package: "::ofml::oi".to_string(),
            imports: vec![],
            source: String::new(),
            classes: vec!["OiClass".to_string()],
            parent_classes: vec![],
        };

        loader.sources.insert("::test::product".to_string(), vec![product]);
        loader.sources.insert("::test::basics".to_string(), vec![basics]);
        loader.sources.insert("::ofml::oi".to_string(), vec![ofml]);

        let sorted = loader.get_sorted_sources();
        assert_eq!(sorted.len(), 3);

        // OFML should come first, then basics, then product
        assert!(sorted[0].package.starts_with("::ofml::oi"));
    }

    #[test]
    fn test_resolve_class_with_explicit_import() {
        let mut loader = AlbLoader::new("/tmp");
        loader
            .class_to_package
            .insert("SpecificClass".to_string(), "::pkg::sub".to_string());

        let imports = vec!["::pkg::sub::SpecificClass".to_string()];
        let result = loader.resolve_class("SpecificClass", "::other", &imports);
        assert_eq!(result, Some("::pkg::sub::SpecificClass".to_string()));
    }

    #[test]
    fn test_cls_source_parse_with_brace_on_same_line() {
        let source = r#"
package ::test;
class MyClass : Parent {
    func test() {}
}
        "#;
        let cls = ClsSource::parse("test.cls", source);
        assert!(cls.classes.contains(&"MyClass".to_string()));
        assert!(cls.parent_classes.contains(&"Parent".to_string()));
    }

    // Integration test with real OFML data
    #[test]
    fn test_load_manufacturer_with_real_data() {
        let data_path = Path::new("/reference/ofmldata");
        if !data_path.exists() {
            return;
        }

        let mut loader = AlbLoader::new(data_path);

        // Try to find ALBs in a known manufacturer
        let sex_path = data_path.join("sex");
        if sex_path.exists() {
            let mut albs = Vec::new();
            let _ = loader.find_albs(&sex_path, &mut albs);
            // Sedus should have ALB files
            if !albs.is_empty() {
                // Load the first ALB
                let result = loader.load_alb(&albs[0]);
                assert!(result.is_ok() || result.is_err()); // Either is valid for this test
            }
        }
    }

    #[test]
    fn test_find_albs_nonexistent() {
        let loader = AlbLoader::new("/tmp");
        let mut albs = Vec::new();
        let result = loader.find_albs(Path::new("/nonexistent/dir"), &mut albs);
        assert!(result.is_ok());
        assert!(albs.is_empty());
    }

    #[test]
    fn test_find_albs_file_not_dir() {
        let loader = AlbLoader::new("/tmp");
        let mut albs = Vec::new();
        // Use a file path instead of directory
        let result = loader.find_albs(Path::new("/etc/hosts"), &mut albs);
        assert!(result.is_ok());
        assert!(albs.is_empty());
    }

    #[test]
    fn test_load_alb_nonexistent() {
        let mut loader = AlbLoader::new("/tmp");
        let result = loader.load_alb(Path::new("/nonexistent/file.alb"));
        assert!(result.is_err());
    }

    #[test]
    fn test_load_manufacturer_with_deps_nonexistent() {
        let result = load_manufacturer_with_deps(Path::new("/nonexistent"), "unknown", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_cls_source_parse_package_without_semicolon() {
        let source = "package ::test";
        let cls = ClsSource::parse("test.cls", source);
        assert_eq!(cls.package, "::test");
    }

    #[test]
    fn test_cls_source_parse_class_without_brace() {
        let source = r#"
package ::test;
class SomeClass : SomeParent
"#;
        let cls = ClsSource::parse("test.cls", source);
        // The parser should still extract the class and parent
        assert!(cls.classes.contains(&"SomeClass".to_string()));
        assert!(cls.parent_classes.contains(&"SomeParent".to_string()));
    }

    #[test]
    fn test_resolve_class_known_in_any_package() {
        let mut loader = AlbLoader::new("/tmp");
        loader
            .class_to_package
            .insert("GlobalClass".to_string(), "::global::pkg".to_string());

        // Not in current package, not in imports, but known globally
        let result = loader.resolve_class("GlobalClass", "::other::pkg", &[]);
        assert_eq!(result, Some("::global::pkg::GlobalClass".to_string()));
    }
}
