//! XCF Catalog Loader - Parses eXtensible Catalog Format for product hierarchies
//!
//! XCF is the standard format used by pcon.configurator/pcon.basket for organizing
//! products into human-readable category hierarchies. The format consists of CSV files:
//!
//! - `structure.csv` - Defines the tree hierarchy with folder/article nodes
//! - `text.csv` - Multilingual labels for nodes
//! - `article.csv` - Maps articles to their series directories
//! - `resource.csv` - Resource references (images, icons)
//! - `variant.csv` - Product variant definitions
//!
//! Catalogs can be stored as:
//! - Plain CSV files in a `cat/` directory
//! - ZIP archive (`xcf.zip`) containing CSV files

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use encoding_rs::WINDOWS_1252;
use serde::{Deserialize, Serialize};

/// Type of node in the catalog tree
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum NodeType {
    /// Folder/category node
    #[default]
    Folder,
    /// Article/product node
    Article,
    /// Root node (virtual)
    Root,
}

/// A node in the catalog tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogNode {
    /// Node identifier (@FOLDERXXXX for folders, article_nr for articles)
    pub id: String,
    /// Display name (from text.csv)
    pub name: String,
    /// Node type (Folder or Article)
    pub node_type: NodeType,
    /// Hierarchy depth (1 = top level)
    pub depth: u8,
    /// Series reference for articles (e.g., "::bisley::lf")
    pub series_ref: Option<String>,
    /// Variant code for articles (e.g., "1503", "1513" for different configurations)
    pub variant_code: Option<String>,
    /// Child nodes
    pub children: Vec<CatalogNode>,
}

impl CatalogNode {
    /// Create a new folder node
    pub fn folder(id: &str, name: &str, depth: u8) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            node_type: NodeType::Folder,
            depth,
            series_ref: None,
            variant_code: None,
            children: Vec::new(),
        }
    }

    /// Create a new article node
    pub fn article(id: &str, name: &str, depth: u8, series_ref: Option<String>) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            node_type: NodeType::Article,
            depth,
            series_ref,
            variant_code: None,
            children: Vec::new(),
        }
    }

    /// Create a new article node with variant code
    pub fn article_with_variant(
        id: &str,
        name: &str,
        depth: u8,
        series_ref: Option<String>,
        variant_code: Option<String>,
    ) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            node_type: NodeType::Article,
            depth,
            series_ref,
            variant_code,
            children: Vec::new(),
        }
    }

    /// Create root node
    pub fn root() -> Self {
        Self {
            id: "@ROOT".to_string(),
            name: "Root".to_string(),
            node_type: NodeType::Root,
            depth: 0,
            series_ref: None,
            variant_code: None,
            children: Vec::new(),
        }
    }

    /// Count total nodes (including self)
    pub fn count(&self) -> usize {
        1 + self.children.iter().map(|c| c.count()).sum::<usize>()
    }

    /// Count folders only
    pub fn folder_count(&self) -> usize {
        let self_count = if self.node_type == NodeType::Folder {
            1
        } else {
            0
        };
        self_count
            + self
                .children
                .iter()
                .map(|c| c.folder_count())
                .sum::<usize>()
    }

    /// Count articles only
    pub fn article_count(&self) -> usize {
        let self_count = if self.node_type == NodeType::Article {
            1
        } else {
            0
        };
        self_count
            + self
                .children
                .iter()
                .map(|c| c.article_count())
                .sum::<usize>()
    }

    /// Find a node by ID (recursive)
    pub fn find(&self, id: &str) -> Option<&CatalogNode> {
        if self.id == id {
            return Some(self);
        }
        for child in &self.children {
            if let Some(found) = child.find(id) {
                return Some(found);
            }
        }
        None
    }

    /// Get path to node (list of ancestor names)
    pub fn path_to(&self, id: &str) -> Option<Vec<String>> {
        if self.id == id {
            return Some(vec![self.name.clone()]);
        }
        for child in &self.children {
            if let Some(mut path) = child.path_to(id) {
                path.insert(0, self.name.clone());
                return Some(path);
            }
        }
        None
    }
}

/// Structure entry from structure.csv
#[derive(Debug, Clone)]
struct StructureEntry {
    id: String,
    /// Variant code (e.g., "1503", "1513", or "default")
    variant_code: Option<String>,
    depth: u8,
    node_type: NodeType,
}

/// Variant definition from variant.csv - maps article variants to property settings
#[derive(Debug, Clone)]
pub struct VariantDefinition {
    /// Article ID
    pub article_id: String,
    /// Variant code
    pub variant_code: String,
    /// Property settings in format "PROPERTY_CLASS.PROPERTY=VALUE"
    pub property_settings: Vec<String>,
}

/// XCF Catalog - parsed catalog data
#[derive(Debug, Clone)]
pub struct XcfCatalog {
    /// Root node of the catalog tree
    pub root: CatalogNode,
    /// Text labels by (id, variant_code, language) - variant_code empty for folders
    texts: HashMap<(String, String, String), String>,
    /// Article to series mapping
    articles: HashMap<String, String>,
    /// Variant definitions - property settings for article variants
    pub variants: HashMap<(String, String), VariantDefinition>,
    /// Source path
    pub source_path: PathBuf,
}

impl XcfCatalog {
    /// Get text for a node in specified language (without variant)
    pub fn get_text(&self, id: &str, language: &str) -> Option<&str> {
        self.get_text_variant(id, "", language)
    }

    /// Get text for a node with variant in specified language
    pub fn get_text_variant(&self, id: &str, variant: &str, language: &str) -> Option<&str> {
        self.texts
            .get(&(id.to_string(), variant.to_string(), language.to_string()))
            .map(|s| s.as_str())
    }

    /// Get text with fallback to German then English
    pub fn get_text_with_fallback(&self, id: &str, language: &str) -> Option<&str> {
        self.get_text_variant_with_fallback(id, "", language)
    }

    /// Get text with variant and fallback to German then English
    pub fn get_text_variant_with_fallback(
        &self,
        id: &str,
        variant: &str,
        language: &str,
    ) -> Option<&str> {
        self.get_text_variant(id, variant, language)
            .or_else(|| self.get_text_variant(id, variant, "de"))
            .or_else(|| self.get_text_variant(id, variant, "en"))
    }

    /// Get variant definition for an article
    pub fn get_variant(&self, article_id: &str, variant_code: &str) -> Option<&VariantDefinition> {
        self.variants
            .get(&(article_id.to_string(), variant_code.to_string()))
    }

    /// Get series reference for an article
    pub fn get_series_ref(&self, article_id: &str) -> Option<&str> {
        self.articles.get(article_id).map(|s| s.as_str())
    }

    /// Get all available languages
    pub fn languages(&self) -> Vec<String> {
        let mut langs: Vec<String> = self.texts.keys().map(|(_, _, lang)| lang.clone()).collect();
        langs.sort();
        langs.dedup();
        langs
    }

    /// Get statistics
    pub fn stats(&self) -> CatalogStats {
        CatalogStats {
            total_nodes: self.root.count(),
            folder_count: self.root.folder_count(),
            article_count: self.root.article_count(),
            text_entries: self.texts.len(),
            languages: self.languages(),
        }
    }
}

/// Catalog statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogStats {
    pub total_nodes: usize,
    pub folder_count: usize,
    pub article_count: usize,
    pub text_entries: usize,
    pub languages: Vec<String>,
}

/// XCF Catalog Loader
pub struct CatalogLoader;

impl CatalogLoader {
    /// Load catalog from a directory or ZIP file
    ///
    /// Automatically detects format:
    /// - If path is a directory, looks for CSV files
    /// - If path is a ZIP file, extracts and parses
    /// - If path is a directory with xcf.zip, loads from ZIP
    pub fn load(path: &Path, language: &str) -> Result<XcfCatalog, CatalogError> {
        // Check for xcf.zip in directory
        let zip_path = path.join("xcf.zip");
        if zip_path.exists() {
            return Self::load_from_zip(&zip_path, language);
        }

        // Check if path itself is a ZIP
        if path.extension().map(|e| e == "zip").unwrap_or(false) {
            return Self::load_from_zip(path, language);
        }

        // Check for CSV files in directory
        let structure_path = path.join("structure.csv");
        if structure_path.exists() {
            return Self::load_from_csv_dir(path, language);
        }

        // Try cat/ subdirectory
        let cat_path = path.join("cat");
        if cat_path.exists() {
            let cat_zip = cat_path.join("xcf.zip");
            if cat_zip.exists() {
                return Self::load_from_zip(&cat_zip, language);
            }
            let cat_structure = cat_path.join("structure.csv");
            if cat_structure.exists() {
                return Self::load_from_csv_dir(&cat_path, language);
            }
        }

        Err(CatalogError::NotFound(path.to_path_buf()))
    }

    /// Load catalog from CSV files in a directory
    pub fn load_from_csv_dir(dir: &Path, language: &str) -> Result<XcfCatalog, CatalogError> {
        let structure_path = dir.join("structure.csv");
        let text_path = dir.join("text.csv");
        let article_path = dir.join("article.csv");
        let variant_path = dir.join("variant.csv");

        // Read structure (with encoding conversion)
        let structure_content = Self::read_file_with_encoding(&structure_path)?;
        let structure_entries = Self::parse_structure_csv(&structure_content)?;

        // Read texts
        let texts = if text_path.exists() {
            let text_content = Self::read_file_with_encoding(&text_path)?;
            Self::parse_text_csv(&text_content)?
        } else {
            HashMap::new()
        };

        // Read articles
        let articles = if article_path.exists() {
            let article_content = Self::read_file_with_encoding(&article_path)?;
            Self::parse_article_csv(&article_content)?
        } else {
            HashMap::new()
        };

        // Read variants
        let variants = if variant_path.exists() {
            let variant_content = Self::read_file_with_encoding(&variant_path)?;
            Self::parse_variant_csv(&variant_content)?
        } else {
            HashMap::new()
        };

        // Build tree
        let root = Self::build_tree(&structure_entries, &texts, &articles, language);

        Ok(XcfCatalog {
            root,
            texts,
            articles,
            variants,
            source_path: dir.to_path_buf(),
        })
    }

    /// Read a file with automatic encoding detection (UTF-8 or Windows-1252)
    fn read_file_with_encoding(path: &Path) -> Result<String, CatalogError> {
        let bytes = std::fs::read(path)
            .map_err(|e| CatalogError::IoError(path.to_path_buf(), e.to_string()))?;

        // Try UTF-8 first
        if let Ok(content) = String::from_utf8(bytes.clone()) {
            return Ok(content);
        }

        // Fall back to Windows-1252 (common for German/European text)
        let (content, _, had_errors) = WINDOWS_1252.decode(&bytes);
        if had_errors {
            return Err(CatalogError::IoError(
                path.to_path_buf(),
                "Failed to decode file content".to_string(),
            ));
        }

        Ok(content.into_owned())
    }

    /// Load catalog from ZIP archive
    pub fn load_from_zip(zip_path: &Path, language: &str) -> Result<XcfCatalog, CatalogError> {
        let file = File::open(zip_path)
            .map_err(|e| CatalogError::IoError(zip_path.to_path_buf(), e.to_string()))?;
        let mut archive =
            zip::ZipArchive::new(file).map_err(|e| CatalogError::ZipError(e.to_string()))?;

        let mut structure_bytes = Vec::new();
        let mut text_bytes = Vec::new();
        let mut article_bytes = Vec::new();
        let mut variant_bytes = Vec::new();

        // Extract files from archive
        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .map_err(|e| CatalogError::ZipError(e.to_string()))?;
            let name = file.name().to_lowercase();

            if name.ends_with("structure.csv") {
                file.read_to_end(&mut structure_bytes)
                    .map_err(|e| CatalogError::IoError(zip_path.to_path_buf(), e.to_string()))?;
            } else if name.ends_with("text.csv") {
                file.read_to_end(&mut text_bytes)
                    .map_err(|e| CatalogError::IoError(zip_path.to_path_buf(), e.to_string()))?;
            } else if name.ends_with("article.csv") {
                file.read_to_end(&mut article_bytes)
                    .map_err(|e| CatalogError::IoError(zip_path.to_path_buf(), e.to_string()))?;
            } else if name.ends_with("variant.csv") {
                file.read_to_end(&mut variant_bytes)
                    .map_err(|e| CatalogError::IoError(zip_path.to_path_buf(), e.to_string()))?;
            }
        }

        if structure_bytes.is_empty() {
            return Err(CatalogError::MissingFile("structure.csv".to_string()));
        }

        // Decode with encoding detection
        let structure_content = Self::decode_bytes(&structure_bytes);
        let text_content = Self::decode_bytes(&text_bytes);
        let article_content = Self::decode_bytes(&article_bytes);
        let variant_content = Self::decode_bytes(&variant_bytes);

        let structure_entries = Self::parse_structure_csv(&structure_content)?;
        let texts = if !text_content.is_empty() {
            Self::parse_text_csv(&text_content)?
        } else {
            HashMap::new()
        };
        let articles = if !article_content.is_empty() {
            Self::parse_article_csv(&article_content)?
        } else {
            HashMap::new()
        };
        let variants = if !variant_content.is_empty() {
            Self::parse_variant_csv(&variant_content)?
        } else {
            HashMap::new()
        };

        let root = Self::build_tree(&structure_entries, &texts, &articles, language);

        Ok(XcfCatalog {
            root,
            texts,
            articles,
            variants,
            source_path: zip_path.to_path_buf(),
        })
    }

    /// Decode bytes with automatic encoding detection
    fn decode_bytes(bytes: &[u8]) -> String {
        // Try UTF-8 first
        if let Ok(content) = String::from_utf8(bytes.to_vec()) {
            return content;
        }
        // Fall back to Windows-1252
        let (content, _, _) = WINDOWS_1252.decode(bytes);
        content.into_owned()
    }

    /// Parse structure.csv content
    /// Format: "ID";variant_code;DEPTH;TYPE;
    fn parse_structure_csv(content: &str) -> Result<Vec<StructureEntry>, CatalogError> {
        let mut entries = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let fields: Vec<&str> = Self::parse_csv_line(line);
            if fields.len() < 4 {
                continue;
            }

            let id = fields[0].trim_matches('"').to_string();
            if id.is_empty() || id == "ID" {
                continue; // Skip header
            }

            // Capture variant code (second field) - "default" means no variant
            let variant_str = fields[1].trim_matches('"');
            let variant_code = if variant_str.is_empty() || variant_str == "default" {
                None
            } else {
                Some(variant_str.to_string())
            };

            let depth: u8 = fields[2].trim_matches('"').parse().unwrap_or(1);
            let type_str = fields[3].trim_matches('"');
            let node_type = match type_str {
                "F" => NodeType::Folder,
                "A" => NodeType::Article,
                _ => NodeType::Folder,
            };

            entries.push(StructureEntry {
                id,
                variant_code,
                depth,
                node_type,
            });
        }

        Ok(entries)
    }

    /// Parse text.csv content
    /// Format: "ID";variant_code;LANG;"TEXT"
    fn parse_text_csv(
        content: &str,
    ) -> Result<HashMap<(String, String, String), String>, CatalogError> {
        let mut texts = HashMap::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let fields: Vec<&str> = Self::parse_csv_line(line);
            if fields.len() < 4 {
                continue;
            }

            let id = fields[0].trim_matches('"').to_string();
            if id.is_empty() || id == "ID" {
                continue; // Skip header
            }

            // Variant code (second field) - normalize "default" to empty string
            let variant_str = fields[1].trim_matches('"');
            let variant = if variant_str == "default" {
                String::new()
            } else {
                variant_str.to_string()
            };

            let language = fields[2].trim_matches('"').to_string();
            let text = fields[3].trim_matches('"').to_string();

            if !language.is_empty() && !text.is_empty() {
                texts.insert((id, variant, language), text);
            }
        }

        Ok(texts)
    }

    /// Parse variant.csv content
    /// Format: "ARTICLE_ID";variant_code;"PROPERTY_CLASS.PROPERTY=VALUE"
    fn parse_variant_csv(
        content: &str,
    ) -> Result<HashMap<(String, String), VariantDefinition>, CatalogError> {
        let mut variants = HashMap::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let fields: Vec<&str> = Self::parse_csv_line(line);
            if fields.len() < 3 {
                continue;
            }

            let article_id = fields[0].trim_matches('"').to_string();
            if article_id.is_empty() || article_id == "ID" {
                continue;
            }

            let variant_code = fields[1].trim_matches('"').to_string();
            let settings_str = fields[2].trim_matches('"');

            // Parse property settings (may be multiple, separated by semicolons within the field)
            let property_settings: Vec<String> = settings_str
                .split(';')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();

            variants.insert(
                (article_id.clone(), variant_code.clone()),
                VariantDefinition {
                    article_id,
                    variant_code,
                    property_settings,
                },
            );
        }

        Ok(variants)
    }

    /// Parse article.csv content
    /// Format: "ARTICLE_NR";default;0;;S;15;::mfr::series
    fn parse_article_csv(content: &str) -> Result<HashMap<String, String>, CatalogError> {
        let mut articles = HashMap::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let fields: Vec<&str> = Self::parse_csv_line(line);
            if fields.len() < 7 {
                continue;
            }

            let id = fields[0].trim_matches('"').to_string();
            if id.is_empty() || id == "ID" || id.starts_with('@') {
                continue; // Skip headers and folder references
            }

            let series_ref = fields[6].trim_matches('"').to_string();
            if !series_ref.is_empty() {
                articles.insert(id, series_ref);
            }
        }

        Ok(articles)
    }

    /// Parse a CSV line handling quoted fields with semicolons
    fn parse_csv_line(line: &str) -> Vec<&str> {
        let mut fields = Vec::new();
        let mut start = 0;
        let mut in_quotes = false;

        for (i, c) in line.char_indices() {
            match c {
                '"' => in_quotes = !in_quotes,
                ';' if !in_quotes => {
                    fields.push(&line[start..i]);
                    start = i + 1;
                }
                _ => {}
            }
        }
        // Add last field
        if start < line.len() {
            fields.push(&line[start..]);
        }

        fields
    }

    /// Build tree from structure entries
    fn build_tree(
        entries: &[StructureEntry],
        texts: &HashMap<(String, String, String), String>,
        articles: &HashMap<String, String>,
        language: &str,
    ) -> CatalogNode {
        let mut root = CatalogNode::root();
        let mut stack: Vec<(u8, usize)> = vec![(0, 0)]; // (depth, index in parent's children)

        for entry in entries {
            // Get variant string for text lookup
            let variant_str = entry.variant_code.as_deref().unwrap_or("");

            // Get display name - try with variant first, then without
            let name = texts
                .get(&(
                    entry.id.clone(),
                    variant_str.to_string(),
                    language.to_string(),
                ))
                .or_else(|| {
                    texts.get(&(entry.id.clone(), variant_str.to_string(), "de".to_string()))
                })
                .or_else(|| {
                    texts.get(&(entry.id.clone(), variant_str.to_string(), "en".to_string()))
                })
                // Fall back to no variant
                .or_else(|| texts.get(&(entry.id.clone(), String::new(), language.to_string())))
                .or_else(|| texts.get(&(entry.id.clone(), String::new(), "de".to_string())))
                .or_else(|| texts.get(&(entry.id.clone(), String::new(), "en".to_string())))
                .cloned()
                .unwrap_or_else(|| entry.id.clone());

            // Get series reference for articles
            let series_ref = if entry.node_type == NodeType::Article {
                articles.get(&entry.id).cloned()
            } else {
                None
            };

            let node = CatalogNode {
                id: entry.id.clone(),
                name,
                node_type: entry.node_type,
                depth: entry.depth,
                series_ref,
                variant_code: entry.variant_code.clone(),
                children: Vec::new(),
            };

            // Find parent at correct depth
            while stack.len() > 1
                && stack
                    .last()
                    .map(|(d, _)| *d >= entry.depth)
                    .unwrap_or(false)
            {
                stack.pop();
            }

            // Add to parent
            if stack.is_empty() {
                root.children.push(node);
                stack.push((entry.depth, root.children.len() - 1));
            } else {
                // Navigate to the parent node
                let mut current = &mut root;
                for (_, idx) in stack.iter().skip(1) {
                    current = &mut current.children[*idx];
                }
                current.children.push(node);
                let new_idx = current.children.len() - 1;
                stack.push((entry.depth, new_idx));
            }
        }

        root
    }
}

/// Catalog loader errors
#[derive(Debug)]
pub enum CatalogError {
    NotFound(PathBuf),
    IoError(PathBuf, String),
    ZipError(String),
    ParseError(String),
    MissingFile(String),
}

impl std::fmt::Display for CatalogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CatalogError::NotFound(path) => write!(f, "Catalog not found: {}", path.display()),
            CatalogError::IoError(path, msg) => {
                write!(f, "IO error reading {}: {}", path.display(), msg)
            }
            CatalogError::ZipError(msg) => write!(f, "ZIP error: {}", msg),
            CatalogError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            CatalogError::MissingFile(name) => write!(f, "Missing required file: {}", name),
        }
    }
}

impl std::error::Error for CatalogError {}

/// Catalog with metadata for sorting/selection
#[derive(Debug)]
pub struct CatalogInfo {
    /// Path to the catalog directory
    pub path: PathBuf,
    /// Whether this is a master catalog
    pub is_master: bool,
    /// Series/directory name
    pub name: String,
}

/// Find all catalog directories for a manufacturer
pub fn find_manufacturer_catalogs(mfr_path: &Path) -> Vec<CatalogInfo> {
    let mut catalogs = Vec::new();

    // Check for master catalog at manufacturer level (e.g., bisley/catalog/)
    let master_cat = mfr_path.join("catalog");
    if master_cat.exists() {
        // Look for ANY/1/cat or DE/1/cat etc.
        for region in std::fs::read_dir(&master_cat).into_iter().flatten().flatten() {
            let region_path = region.path();
            if region_path.is_dir() {
                for version in std::fs::read_dir(&region_path).into_iter().flatten().flatten() {
                    let cat_path = version.path().join("cat");
                    if cat_path.exists() {
                        catalogs.push(CatalogInfo {
                            path: cat_path,
                            is_master: true,
                            name: "catalog".to_string(),
                        });
                    }
                }
            }
        }
    }

    // Check each series for catalogs
    if let Ok(entries) = std::fs::read_dir(mfr_path) {
        for entry in entries.flatten() {
            let series_path = entry.path();
            if series_path.is_dir() {
                let series_name = series_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();

                // Skip non-series directories
                if series_name == "catalog" || series_name.starts_with('.') {
                    continue;
                }

                // Check if this is a master catalog (e.g., desks_m_cat, seating_m_cat)
                let is_master = series_name.ends_with("_m_cat");

                // Look for cat directories in various locations
                for region in ["DE", "ANY", "EN"] {
                    for version in ["1", "2"] {
                        let cat_path = series_path.join(region).join(version).join("cat");
                        if cat_path.exists() {
                            let xcf_zip = cat_path.join("xcf.zip");
                            let structure_csv = cat_path.join("structure.csv");
                            if xcf_zip.exists() || structure_csv.exists() {
                                catalogs.push(CatalogInfo {
                                    path: cat_path,
                                    is_master,
                                    name: series_name.clone(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    // Sort: master catalogs first, then by name
    catalogs.sort_by(|a, b| match (a.is_master, b.is_master) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.cmp(&b.name),
    });

    catalogs
}

/// Load the best available catalog for a manufacturer
///
/// Priority: master catalogs (catalog/, *_m_cat) > series catalogs
pub fn load_manufacturer_catalog(mfr_path: &Path, language: &str) -> Option<XcfCatalog> {
    let catalogs = find_manufacturer_catalogs(mfr_path);

    // Try master catalogs first (already sorted first)
    for cat_info in &catalogs {
        if cat_info.is_master {
            if let Ok(catalog) = CatalogLoader::load(&cat_info.path, language) {
                return Some(catalog);
            }
        }
    }

    // Fall back to first available series catalog
    for cat_info in &catalogs {
        if let Ok(catalog) = CatalogLoader::load(&cat_info.path, language) {
            return Some(catalog);
        }
    }

    None
}

/// Load only the master catalog for a manufacturer (no fallback to series catalogs)
///
/// Returns None if no master catalog exists (like Sedus which has per-series catalogs)
pub fn load_master_catalog(mfr_path: &Path, language: &str) -> Option<XcfCatalog> {
    let catalogs = find_manufacturer_catalogs(mfr_path);

    // Only try master catalogs
    for cat_info in &catalogs {
        if cat_info.is_master {
            if let Ok(catalog) = CatalogLoader::load(&cat_info.path, language) {
                return Some(catalog);
            }
        }
    }

    None
}

/// Check if a manufacturer has a master catalog
pub fn has_master_catalog(mfr_path: &Path) -> bool {
    let catalogs = find_manufacturer_catalogs(mfr_path);
    catalogs.iter().any(|c| c.is_master)
}

/// Load all available catalogs for a manufacturer
pub fn load_all_manufacturer_catalogs(
    mfr_path: &Path,
    language: &str,
) -> Vec<(String, XcfCatalog)> {
    let catalog_infos = find_manufacturer_catalogs(mfr_path);
    let mut result = Vec::new();

    for info in catalog_infos {
        if info.is_master {
            if let Ok(catalog) = CatalogLoader::load(&info.path, language) {
                result.push((info.name, catalog));
            }
        }
    }

    result
}

// ============================================================================
// Registry Parser - for manufacturer metadata and series display names
// ============================================================================

/// Parsed manufacturer registry information
#[derive(Debug, Clone, Default)]
pub struct ManufacturerRegistry {
    /// Manufacturer ID (e.g., "sex")
    pub manufacturer: String,
    /// Manufacturer ID code (e.g., "SE")
    pub manufacturer_id: String,
    /// Display name
    pub manufacturer_name: String,
    /// Series name mappings: series_id -> display_name
    pub series_names: HashMap<String, String>,
}

impl ManufacturerRegistry {
    /// Load registry from a .cfg file (like SE.cfg)
    pub fn load(path: &Path) -> Option<Self> {
        let content = CatalogLoader::read_file_with_encoding(path).ok()?;
        Self::parse(&content, "de")
    }

    /// Load registry with specific language
    pub fn load_with_language(path: &Path, language: &str) -> Option<Self> {
        let content = CatalogLoader::read_file_with_encoding(path).ok()?;
        Self::parse(&content, language)
    }

    /// Parse registry content
    fn parse(content: &str, language: &str) -> Option<Self> {
        let mut registry = ManufacturerRegistry::default();
        let mut current_section = String::new();
        let mut lang_series_names: HashMap<String, String> = HashMap::new();

        for line in content.lines() {
            let line = line.trim();

            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Section header
            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len() - 1].to_string();
                continue;
            }

            // Key=Value pairs
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                match current_section.as_str() {
                    "general" => match key {
                        "manufacturer" => registry.manufacturer = value.to_string(),
                        "manufacturer_id" => registry.manufacturer_id = value.to_string(),
                        "manufacturer_name" => {
                            if registry.manufacturer_name.is_empty() {
                                registry.manufacturer_name = value.to_string();
                            }
                        }
                        _ if key.starts_with("series_name.") => {
                            if let Some(series_id) = key.strip_prefix("series_name.") {
                                registry
                                    .series_names
                                    .insert(series_id.to_lowercase(), value.to_string());
                            }
                        }
                        _ => {}
                    },
                    section if section == language => {
                        // Language-specific section overrides
                        match key {
                            "manufacturer_name" => {
                                registry.manufacturer_name = value.to_string();
                            }
                            _ if key.starts_with("series_name.") => {
                                if let Some(series_id) = key.strip_prefix("series_name.") {
                                    lang_series_names
                                        .insert(series_id.to_lowercase(), value.to_string());
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }

        // Override with language-specific names
        for (k, v) in lang_series_names {
            registry.series_names.insert(k, v);
        }

        if registry.manufacturer.is_empty() {
            return None;
        }

        Some(registry)
    }

    /// Get display name for a series
    pub fn get_series_name(&self, series_id: &str) -> Option<&str> {
        self.series_names
            .get(&series_id.to_lowercase())
            .map(|s| s.as_str())
    }
}

/// Find the manufacturer registry file
pub fn find_manufacturer_registry(data_path: &Path, manufacturer_id: &str) -> Option<PathBuf> {
    let registry_dir = data_path.join("registry");

    // Try uppercase ID first (like SE.cfg for Sedus)
    let uppercase_cfg = registry_dir.join(format!("{}.cfg", manufacturer_id.to_uppercase()));
    if uppercase_cfg.exists() {
        return Some(uppercase_cfg);
    }

    // Try manufacturer code patterns
    // Read from Manufacturers.ebase or iterate registry files
    if let Ok(entries) = std::fs::read_dir(&registry_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "cfg") {
                if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                    // Skip series-specific configs (like sex_ai_DE_1.cfg)
                    if name.contains('_') {
                        continue;
                    }
                    // Check if this is a manufacturer registry
                    if let Some(reg) = ManufacturerRegistry::load(&path) {
                        if reg.manufacturer == manufacturer_id {
                            return Some(path);
                        }
                    }
                }
            }
        }
    }

    None
}

/// Build an aggregated catalog from individual series catalogs
///
/// For manufacturers without a master catalog (like Sedus), this aggregates
/// all series catalogs into a single virtual catalog using the registry
/// for display names.
pub fn build_aggregated_catalog(
    data_path: &Path,
    mfr_path: &Path,
    manufacturer_id: &str,
    language: &str,
) -> Option<XcfCatalog> {
    // Load manufacturer registry for series names
    let registry = find_manufacturer_registry(data_path, manufacturer_id)
        .and_then(|p| ManufacturerRegistry::load_with_language(&p, language));

    let catalog_infos = find_manufacturer_catalogs(mfr_path);

    if catalog_infos.is_empty() {
        return None;
    }

    // Build root node with series as children
    let mut root = CatalogNode {
        id: manufacturer_id.to_string(),
        name: registry
            .as_ref()
            .map(|r| r.manufacturer_name.clone())
            .unwrap_or_else(|| manufacturer_id.to_string()),
        node_type: NodeType::Root,
        depth: 0,
        series_ref: None,
        variant_code: None,
        children: Vec::new(),
    };

    let mut texts: HashMap<(String, String, String), String> = HashMap::new();
    let mut articles = HashMap::new();
    let mut variants: HashMap<(String, String), VariantDefinition> = HashMap::new();

    // Add each series as a top-level folder
    for cat_info in &catalog_infos {
        // Get display name from registry or catalog
        let display_name = registry
            .as_ref()
            .and_then(|r| r.get_series_name(&cat_info.name).map(|s| s.to_string()))
            .unwrap_or_else(|| cat_info.name.clone());

        // Try to load the series catalog
        if let Ok(series_catalog) = CatalogLoader::load(&cat_info.path, language) {
            // If series catalog has meaningful content, use its structure
            if !series_catalog.root.children.is_empty() {
                // Check if the catalog has a single top-level folder with same/similar name
                // If so, use that folder's children directly to avoid duplication like "se:air > se:air"
                let effective_children = if series_catalog.root.children.len() == 1 {
                    let first_child = &series_catalog.root.children[0];
                    if first_child.node_type == NodeType::Folder
                        && (first_child.name.to_lowercase() == display_name.to_lowercase()
                            || first_child
                                .name
                                .to_lowercase()
                                .contains(&cat_info.name.to_lowercase()))
                    {
                        // Skip the intermediate folder, use its children
                        first_child.children.clone()
                    } else {
                        series_catalog.root.children.clone()
                    }
                } else {
                    series_catalog.root.children.clone()
                };

                let mut series_node = CatalogNode {
                    id: cat_info.name.clone(),
                    name: display_name.clone(),
                    node_type: NodeType::Folder,
                    depth: 1,
                    series_ref: Some(cat_info.name.clone()),
                    variant_code: None,
                    children: effective_children,
                };

                // Adjust depth of children
                adjust_depths(&mut series_node, 1);

                root.children.push(series_node);

                // Merge texts, articles, and variants
                for ((node_id, variant, lang), text) in series_catalog.texts {
                    texts.insert((node_id, variant, lang), text);
                }
                for (article_id, series) in series_catalog.articles {
                    articles.insert(article_id, series);
                }
                for (key, variant_def) in series_catalog.variants {
                    variants.insert(key, variant_def);
                }
            }
        } else {
            // No catalog content, just add as empty folder
            root.children.push(CatalogNode {
                id: cat_info.name.clone(),
                name: display_name,
                node_type: NodeType::Folder,
                depth: 1,
                series_ref: Some(cat_info.name.clone()),
                variant_code: None,
                children: Vec::new(),
            });
        }
    }

    // Sort children by display name
    root.children.sort_by(|a, b| a.name.cmp(&b.name));

    Some(XcfCatalog {
        root,
        texts,
        articles,
        variants,
        source_path: mfr_path.to_path_buf(),
    })
}

/// Adjust node depths recursively
fn adjust_depths(node: &mut CatalogNode, parent_depth: u8) {
    node.depth = parent_depth + 1;
    for child in &mut node.children {
        adjust_depths(child, node.depth);
    }
}

/// Smart catalog loader - returns master catalog if available, otherwise aggregated
pub fn load_smart_catalog(
    data_path: &Path,
    mfr_path: &Path,
    manufacturer_id: &str,
    language: &str,
) -> Option<XcfCatalog> {
    // First, try to load master catalog
    if let Some(catalog) = load_master_catalog(mfr_path, language) {
        return Some(catalog);
    }

    // Fall back to aggregated catalog from series + registry
    build_aggregated_catalog(data_path, mfr_path, manufacturer_id, language)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv_line() {
        let line = r#""@FOLDER123";default;1;F;"#;
        let fields = CatalogLoader::parse_csv_line(line);
        // Trailing semicolon may or may not add empty field - we care about actual data fields
        assert!(fields.len() >= 4);
        assert_eq!(fields[0], "\"@FOLDER123\"");
        assert_eq!(fields[2], "1");
        assert_eq!(fields[3], "F");

        // Test without trailing semicolon
        let line2 = r#""@FOLDER456";default;2;A"#;
        let fields2 = CatalogLoader::parse_csv_line(line2);
        assert_eq!(fields2.len(), 4);
        assert_eq!(fields2[0], "\"@FOLDER456\"");
        assert_eq!(fields2[3], "A");
    }

    #[test]
    fn test_parse_structure_csv() {
        let content = r#"
"@FOLDER1";default;1;F;
"@FOLDER2";default;2;F;
"ARTICLE1";default;3;A;
"#;
        let entries = CatalogLoader::parse_structure_csv(content).unwrap();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].id, "@FOLDER1");
        assert_eq!(entries[0].depth, 1);
        assert_eq!(entries[0].node_type, NodeType::Folder);
        assert_eq!(entries[2].node_type, NodeType::Article);
    }

    #[test]
    fn test_parse_text_csv() {
        let content = r#"
"@FOLDER1";default;de;"Kategorie Eins"
"@FOLDER1";default;en;"Category One"
"@FOLDER2";default;de;"Unterkategorie"
"#;
        let texts = CatalogLoader::parse_text_csv(content).unwrap();
        assert_eq!(texts.len(), 3);
        // "default" is normalized to empty string
        assert_eq!(
            texts.get(&("@FOLDER1".to_string(), String::new(), "de".to_string())),
            Some(&"Kategorie Eins".to_string())
        );
        assert_eq!(
            texts.get(&("@FOLDER1".to_string(), String::new(), "en".to_string())),
            Some(&"Category One".to_string())
        );
    }

    #[test]
    fn test_build_tree() {
        let entries = vec![
            StructureEntry {
                id: "@FOLDER1".to_string(),
                variant_code: None,
                depth: 1,
                node_type: NodeType::Folder,
            },
            StructureEntry {
                id: "@FOLDER2".to_string(),
                variant_code: None,
                depth: 2,
                node_type: NodeType::Folder,
            },
            StructureEntry {
                id: "ART1".to_string(),
                variant_code: None,
                depth: 3,
                node_type: NodeType::Article,
            },
            StructureEntry {
                id: "@FOLDER3".to_string(),
                variant_code: None,
                depth: 1,
                node_type: NodeType::Folder,
            },
        ];
        let mut texts = HashMap::new();
        texts.insert(
            ("@FOLDER1".to_string(), String::new(), "de".to_string()),
            "Hauptkategorie".to_string(),
        );
        texts.insert(
            ("@FOLDER2".to_string(), String::new(), "de".to_string()),
            "Unterkategorie".to_string(),
        );

        let root = CatalogLoader::build_tree(&entries, &texts, &HashMap::new(), "de");

        assert_eq!(root.children.len(), 2); // Two top-level folders
        assert_eq!(root.children[0].name, "Hauptkategorie");
        assert_eq!(root.children[0].children.len(), 1); // One child
        assert_eq!(root.children[0].children[0].name, "Unterkategorie");
        assert_eq!(root.children[0].children[0].children.len(), 1); // One article
    }

    #[test]
    fn test_load_bisley_catalog() {
        let path = Path::new("/reference/ofmldata/bisley/catalog/ANY/1/cat");
        if !path.exists() {
            println!("Bisley catalog not found, skipping test");
            return;
        }

        let catalog = CatalogLoader::load(path, "de").expect("Should load catalog");
        let stats = catalog.stats();

        println!("\n=== Bisley Catalog Stats ===");
        println!("Total nodes: {}", stats.total_nodes);
        println!("Folders: {}", stats.folder_count);
        println!("Articles: {}", stats.article_count);
        println!("Languages: {:?}", stats.languages);

        // Print top-level categories
        println!("\nTop-level categories:");
        for child in &catalog.root.children {
            println!(
                "  {} - {} ({} children)",
                child.id,
                child.name,
                child.children.len()
            );
        }

        assert!(stats.folder_count > 0);
    }

    #[test]
    fn test_load_kn_catalog() {
        let path = Path::new("/reference/ofmldata/kn/desks_m_cat/DE/2/cat");
        if !path.exists() {
            println!("K+N catalog not found, skipping test");
            return;
        }

        let catalog = CatalogLoader::load(path, "de").expect("Should load catalog");
        let stats = catalog.stats();

        println!("\n=== König+Neurath Catalog Stats ===");
        println!("Total nodes: {}", stats.total_nodes);
        println!("Folders: {}", stats.folder_count);
        println!("Articles: {}", stats.article_count);

        // Print structure
        println!("\nCatalog structure:");
        fn print_tree(node: &CatalogNode, indent: usize) {
            let prefix = "  ".repeat(indent);
            let type_char = match node.node_type {
                NodeType::Folder => "📁",
                NodeType::Article => "📄",
                NodeType::Root => "🏠",
            };
            println!("{}{} {}", prefix, type_char, node.name);
            for child in node.children.iter().take(3) {
                print_tree(child, indent + 1);
            }
            if node.children.len() > 3 {
                println!("{}  ... and {} more", prefix, node.children.len() - 3);
            }
        }
        print_tree(&catalog.root, 0);
    }

    #[test]
    fn test_find_manufacturer_catalogs() {
        let bisley_path = Path::new("/reference/ofmldata/bisley");
        if !bisley_path.exists() {
            println!("Bisley not found, skipping");
            return;
        }

        let catalogs = find_manufacturer_catalogs(bisley_path);
        println!("\nFound {} catalogs for Bisley:", catalogs.len());
        for cat in &catalogs {
            println!(
                "  {} (master: {}) -> {}",
                cat.name,
                cat.is_master,
                cat.path.display()
            );
        }

        assert!(!catalogs.is_empty());
        // Master catalog should be first
        assert!(catalogs[0].is_master, "First catalog should be master");
    }

    #[test]
    fn test_find_kn_master_catalogs() {
        let kn_path = Path::new("/reference/ofmldata/kn");
        if !kn_path.exists() {
            println!("K+N not found, skipping");
            return;
        }

        let catalogs = find_manufacturer_catalogs(kn_path);
        let master_catalogs: Vec<_> = catalogs.iter().filter(|c| c.is_master).collect();

        println!("\nFound {} master catalogs for K+N:", master_catalogs.len());
        for cat in &master_catalogs {
            println!("  {} -> {}", cat.name, cat.path.display());
        }

        // Should find desks_m_cat, konferenz_m_cat, seating_m_cat
        assert!(
            master_catalogs.len() >= 3,
            "K+N should have at least 3 master catalogs"
        );

        // Master catalogs should be first in the list
        assert!(catalogs[0].is_master, "First catalog should be master");
    }

    #[test]
    fn test_load_sedus_registry() {
        let data_path = Path::new("/reference/ofmldata");
        if !data_path.exists() {
            println!("OFML data not found, skipping");
            return;
        }

        let reg_path = find_manufacturer_registry(data_path, "sex");
        assert!(reg_path.is_some(), "Should find Sedus registry");

        let reg = ManufacturerRegistry::load_with_language(&reg_path.unwrap(), "de");
        assert!(reg.is_some(), "Should parse Sedus registry");

        let reg = reg.unwrap();
        println!("\nSedus Registry:");
        println!(
            "  Manufacturer: {} ({})",
            reg.manufacturer_name, reg.manufacturer_id
        );
        println!("  Series names: {}", reg.series_names.len());

        // Check some known series names
        assert_eq!(reg.get_series_name("ai"), Some("se:air"));
        assert_eq!(reg.get_series_name("qb"), Some("quarterback"));
        assert!(reg.series_names.len() > 30, "Sedus should have many series");
    }

    #[test]
    fn test_build_sedus_aggregated_catalog() {
        let data_path = Path::new("/reference/ofmldata");
        let mfr_path = data_path.join("sex");
        if !mfr_path.exists() {
            println!("Sedus data not found, skipping");
            return;
        }

        let catalog = load_smart_catalog(data_path, &mfr_path, "sex", "de");
        assert!(
            catalog.is_some(),
            "Should build aggregated catalog for Sedus"
        );

        let catalog = catalog.unwrap();
        let stats = catalog.stats();
        println!("\nSedus Aggregated Catalog:");
        println!("  Categories: {}", stats.folder_count);
        println!("  Articles: {}", stats.article_count);
        println!("  Top-level entries: {}", catalog.root.children.len());

        // Should have many series as top-level entries
        assert!(
            catalog.root.children.len() > 20,
            "Sedus should have many series"
        );

        // Check that series names are resolved
        let has_se_air = catalog.root.children.iter().any(|c| c.name == "se:air");
        let has_quarterback = catalog
            .root
            .children
            .iter()
            .any(|c| c.name == "quarterback");
        assert!(has_se_air, "Should have se:air series with display name");
        assert!(
            has_quarterback,
            "Should have quarterback series with display name"
        );

        println!("\nFirst 10 series:");
        for child in catalog.root.children.iter().take(10) {
            println!("  {} -> {}", child.id, child.name);
        }
    }
}
