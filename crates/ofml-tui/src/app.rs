//! TUI Application state and event handling
//!
//! This module implements the Elm Architecture (TEA) pattern for the TUI:
//! - App: Application state
//! - Message: Events that can occur
//! - Screen: Current view state

use chrono::NaiveDate;
use ratatui::widgets::ListState;

use ofml_lib::oap::catalog::{CatalogNode, NodeType, XcfCatalog};
use ofml_lib::oap::engine::ConfigurationEngine;
use ofml_lib::oap::families::{FamilyConfiguration, FamilyProperty, ProductFamily};
use ofml_lib::oap::{Article, Configuration, Manufacturer, PriceResult};

/// Packaging information for display

#[derive(Debug, Clone)]
pub struct PackagingInfo {
    /// Weight in kg
    pub weight_kg: f64,
    /// Dimensions (width x depth x height) in cm
    pub dimensions_cm: (f64, f64, f64),
    /// Volume in m³
    pub volume_m3: f64,
}

/// Composite component information for display

#[derive(Debug, Clone)]
pub struct CompositeComponent {
    /// Item ID (article number)
    pub item_id: String,
    /// Position in the bill
    pub position: u16,
    /// Quantity
    pub quantity: f64,
    /// Description (if available)
    pub description: String,
}

/// Current screen/view in the TUI

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    /// List of manufacturers
    Manufacturers,
    /// XCF Catalog browser (hierarchical categories)
    Catalog,
    /// List of product families for selected manufacturer
    Families,
    /// Family configuration view with property selection
    FamilyConfig,
    /// List of articles for selected manufacturer (legacy)
    Articles,
    /// Property configuration for selected article (legacy)
    Properties,
    /// Custom table browser (shows all tables in pdata.ebase)
    Tables,
    /// Table contents viewer (shows rows in selected table)
    TableView,
    /// Saved configurations browser
    SavedConfigs,
    /// Help screen
    Help,
}

/// Information about a custom table

#[derive(Debug, Clone)]
pub struct TableInfo {
    /// Table name
    pub name: String,
    /// Number of rows
    pub row_count: usize,
    /// Column names
    pub columns: Vec<String>,
    /// Whether this is a standard OCD table
    pub is_standard: bool,
    /// Source file path
    pub source_path: String,
}

/// A row of data from a table

#[derive(Debug, Clone)]
pub struct TableRow {
    /// Column values as strings
    pub values: Vec<String>,
}

/// A single property change for undo/redo

#[derive(Debug, Clone)]
pub struct PropertyChange {
    /// Property key
    pub key: String,
    /// Old value
    pub old_value: String,
    /// New value
    pub new_value: String,
}

/// Messages/events that can occur in the TUI

#[derive(Debug, Clone)]
pub enum Message {
    /// Select a manufacturer by index
    SelectManufacturer(usize),
    /// Select a catalog node (enter folder or select article)
    SelectCatalogNode(usize),
    /// Select a product family by index
    SelectFamily(usize),
    /// Select an article by index
    SelectArticle(usize),
    /// Cycle through property options (Left/Right)
    CyclePropertyOption(i32),
    /// Edit a property value
    EditProperty(String, String),
    /// Navigate up in the list
    NavigateUp,
    /// Navigate down in the list
    NavigateDown,
    /// Navigate up by page (10 items)
    NavigatePageUp,
    /// Navigate down by page (10 items)
    NavigatePageDown,
    /// Navigate to top of list
    NavigateToTop,
    /// Navigate to bottom of list
    NavigateToBottom,
    /// Go back to previous screen
    GoBack,
    /// Go back to home screen (manufacturers)
    GoHome,
    /// Undo last property change
    Undo,
    /// Redo last undone property change
    Redo,
    /// Toggle search mode
    ToggleSearch,
    /// Update search query
    UpdateSearch(String),
    /// Set price date
    SetPriceDate(NaiveDate),
    /// Export configuration to JSON
    Export,
    /// Export geometry to GLB file
    ExportGeometry,
    /// Show help
    ShowHelp,
    /// Toggle debug console
    ToggleDebug,
    /// Show tables browser
    ShowTables,
    /// Select a table by index
    SelectTable(usize),
    /// Scroll table view left/right
    ScrollTableHorizontal(i32),
    /// Save current configuration
    SaveConfig,
    /// Load a saved configuration
    LoadConfig,
    /// Show saved configurations list
    ShowSavedConfigs,
    /// Select a saved configuration to load
    SelectSavedConfig(usize),
    /// Toggle favorite status for current family
    ToggleFavorite,
    /// Toggle theme (light/dark)
    ToggleTheme,
    /// Quit application
    Quit,
    /// Resize terminal
    Resize(u16, u16),
}

/// Main application state

pub struct App {
    /// Current screen
    pub screen: Screen,
    /// List of loaded manufacturers
    pub manufacturers: Vec<Manufacturer>,
    /// Manufacturer list selection state
    pub manufacturer_list_state: ListState,
    /// Currently selected manufacturer
    pub selected_manufacturer: Option<Manufacturer>,

    // === XCF Catalog Mode ===
    /// Loaded XCF catalog for current manufacturer
    pub catalog: Option<XcfCatalog>,
    /// Stack of catalog nodes for navigation (breadcrumb)
    pub catalog_path: Vec<CatalogNode>,
    /// Current catalog children being displayed
    pub catalog_children: Vec<CatalogNode>,
    /// Catalog list selection state
    pub catalog_list_state: ListState,

    // === Product Family Mode (new) ===
    /// List of product families for current manufacturer
    pub families: Vec<ProductFamily>,
    /// Family list selection state
    pub family_list_state: ListState,
    /// Currently selected product family
    pub selected_family: Option<ProductFamily>,
    /// Configurable properties for current family
    pub family_properties: Vec<FamilyProperty>,
    /// Current family configuration (selections)
    pub family_config: Option<FamilyConfiguration>,
    /// Current family price result
    pub family_price: Option<PriceResult>,
    /// Data warnings for current family (from OCD parsing)
    pub family_warnings: Vec<ofml_lib::oap::DataWarning>,
    /// Focused article index (when no OCD properties, navigate articles instead)
    pub focused_article_index: usize,
    /// Packaging info for current article
    pub packaging_info: Option<PackagingInfo>,
    /// Data version info for current manufacturer
    pub data_version: Option<String>,
    /// Data validity warning if data is outdated
    pub data_validity_warning: Option<String>,
    /// Variant code separator from code scheme (default is "_")
    pub varcode_separator: String,
    /// Composite product components (if current article is a composite)
    pub composite_components: Vec<CompositeComponent>,
    /// Undo stack for property changes
    pub undo_stack: Vec<PropertyChange>,
    /// Redo stack for undone property changes
    pub redo_stack: Vec<PropertyChange>,

    // === Legacy Article Mode ===
    /// List of articles for current manufacturer
    pub articles: Vec<Article>,
    /// Article list selection state
    pub article_list_state: ListState,
    /// Currently selected article
    pub selected_article: Option<Article>,
    /// Current configuration
    pub configuration: Option<Configuration>,
    /// Current price result
    pub price: Option<PriceResult>,

    // === Table Browser Mode ===
    /// List of tables for current manufacturer
    pub tables: Vec<TableInfo>,
    /// Table list selection state
    pub table_list_state: ListState,
    /// Currently selected table
    pub selected_table: Option<TableInfo>,
    /// Rows of the currently selected table
    pub table_rows: Vec<TableRow>,
    /// Table row selection state
    pub table_row_list_state: ListState,
    /// Horizontal scroll offset for table view
    pub table_scroll_x: usize,

    // === Saved Configurations ===
    /// List of saved configurations
    pub saved_configs: Vec<(std::path::PathBuf, super::config_store::SavedConfiguration)>,
    /// Saved configs list selection state
    pub saved_configs_list_state: ListState,

    // === Debug Console ===
    /// Debug mode enabled
    pub debug_mode: bool,
    /// Debug log messages (newest first)
    pub debug_log: Vec<String>,
    /// Maximum debug log entries
    pub debug_log_max: usize,

    // === Common State ===
    /// Price lookup date
    pub price_date: NaiveDate,
    /// Search mode active
    pub search_active: bool,
    /// Search query
    pub search_query: String,
    /// Filtered items for current list
    pub filtered_indices: Vec<usize>,
    /// Currently focused property index
    pub focused_property: usize,
    /// Status message
    pub status_message: Option<String>,
    /// Whether to quit
    pub should_quit: bool,
    /// Data path
    pub data_path: String,
    /// Terminal size
    pub terminal_size: (u16, u16),
    /// Configuration engine for price calculation
    pub engine: ConfigurationEngine,
    /// Current theme variant
    pub theme_variant: super::theme::ThemeVariant,
}


impl App {
    /// Create a new App instance
    pub fn new(data_path: String) -> Self {
        let mut manufacturer_list_state = ListState::default();
        manufacturer_list_state.select(Some(0));

        Self {
            screen: Screen::Manufacturers,
            manufacturers: Vec::new(),
            manufacturer_list_state,
            selected_manufacturer: None,
            // XCF catalog mode
            catalog: None,
            catalog_path: Vec::new(),
            catalog_children: Vec::new(),
            catalog_list_state: ListState::default(),
            // Product family mode
            families: Vec::new(),
            family_list_state: ListState::default(),
            selected_family: None,
            family_properties: Vec::new(),
            family_config: None,
            family_price: None,
            family_warnings: Vec::new(),
            focused_article_index: 0,
            packaging_info: None,
            data_version: None,
            data_validity_warning: None,
            varcode_separator: "_".to_string(),
            composite_components: Vec::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            // Legacy article mode
            articles: Vec::new(),
            article_list_state: ListState::default(),
            selected_article: None,
            configuration: None,
            price: None,
            // Table browser mode
            tables: Vec::new(),
            table_list_state: ListState::default(),
            selected_table: None,
            table_rows: Vec::new(),
            table_row_list_state: ListState::default(),
            table_scroll_x: 0,
            // Saved configurations
            saved_configs: Vec::new(),
            saved_configs_list_state: ListState::default(),
            // Debug console
            debug_mode: false,
            debug_log: Vec::new(),
            debug_log_max: 100,
            // Common state
            price_date: chrono::Local::now().date_naive(),
            search_active: false,
            search_query: String::new(),
            filtered_indices: Vec::new(),
            focused_property: 0,
            status_message: None,
            should_quit: false,
            engine: ConfigurationEngine::new(&data_path),
            data_path,
            terminal_size: (80, 24),
            theme_variant: super::theme::ThemeVariant::default(),
        }
    }

    /// Get the current theme
    pub fn theme(&self) -> super::theme::Theme {
        self.theme_variant.theme()
    }

    /// Set the catalog for the current manufacturer
    pub fn set_catalog(&mut self, catalog: Option<XcfCatalog>) {
        self.catalog = catalog;
        self.catalog_path.clear();
        self.catalog_children.clear();
        self.catalog_list_state = ListState::default();

        if let Some(ref cat) = self.catalog {
            // Start at root level
            self.catalog_children = cat.root.children.clone();
            if !self.catalog_children.is_empty() {
                self.catalog_list_state.select(Some(0));
            }
        }
    }

    /// Get the current catalog node (for display in breadcrumb)
    pub fn current_catalog_node(&self) -> Option<&CatalogNode> {
        self.catalog_path.last()
    }

    /// Get the catalog breadcrumb path as a string
    pub fn catalog_breadcrumb(&self) -> String {
        if self.catalog_path.is_empty() {
            return "Katalog".to_string();
        }
        self.catalog_path
            .iter()
            .map(|n| n.name.as_str())
            .collect::<Vec<_>>()
            .join(" > ")
    }

    /// Enter a catalog folder
    pub fn enter_catalog_folder(&mut self, node: &CatalogNode) {
        self.catalog_path.push(node.clone());
        self.catalog_children = node.children.clone();
        self.catalog_list_state = ListState::default();
        if !self.catalog_children.is_empty() {
            self.catalog_list_state.select(Some(0));
        }
    }

    /// Go back one level in the catalog
    pub fn exit_catalog_folder(&mut self) {
        if self.catalog_path.is_empty() {
            // At root, exit catalog view
            return;
        }

        self.catalog_path.pop();

        if self.catalog_path.is_empty() {
            // Back to root level
            if let Some(ref cat) = self.catalog {
                self.catalog_children = cat.root.children.clone();
            }
        } else {
            // Back to parent's children
            if let Some(parent) = self.catalog_path.last() {
                self.catalog_children = parent.children.clone();
            }
        }

        self.catalog_list_state = ListState::default();
        if !self.catalog_children.is_empty() {
            self.catalog_list_state.select(Some(0));
        }
    }

    /// Add a debug log message
    pub fn debug_log(&mut self, msg: String) {
        let timestamp = chrono::Local::now().format("%H:%M:%S%.3f");
        self.debug_log.insert(0, format!("[{}] {}", timestamp, msg));
        if self.debug_log.len() > self.debug_log_max {
            self.debug_log.pop();
        }
    }

    /// Recalculate price for current family configuration
    pub fn recalculate_price(&mut self) {
        if let (Some(ref mfr), Some(ref family), Some(ref config)) = (
            &self.selected_manufacturer,
            &self.selected_family,
            &self.family_config,
        ) {
            let old_price = self.family_price.as_ref().map(|p| p.total_price);

            self.family_price =
                self.engine
                    .calculate_family_price(&mfr.id, family, config, self.price_date);

            // Log debug info
            if self.debug_mode {
                // Collect price info first to avoid borrow issues
                let price_info = self.family_price.as_ref().map(|price| {
                        let surcharges: Vec<(String, String)> = price
                            .surcharges
                            .iter()
                            .map(|s| (s.name.clone(), s.amount.to_string()))
                            .collect();
                        (
                            price.total_price.to_string(),
                            price.currency.clone(),
                            price.surcharges.len(),
                            surcharges,
                        )
                    });

                if let Some((total, _currency, count, surcharges)) = price_info {
                    let old_str = old_price
                        .map(|p| format!("{}", p))
                        .unwrap_or_else(|| "N/A".to_string());
                    self.debug_log(format!(
                        "Price: {} -> {} EUR ({} surcharges)",
                        old_str, total, count
                    ));
                    for (name, amount) in surcharges {
                        self.debug_log(format!("  + {}: {} EUR", name, amount));
                    }
                } else {
                    self.debug_log("Price calculation returned None".to_string());
                }
            }
        }
    }

    /// Save the current family configuration to a JSON file
    pub fn save_current_configuration(&mut self) {
        if let (Some(ref mfr), Some(ref family), Some(ref config)) = (
            &self.selected_manufacturer,
            &self.selected_family,
            &self.family_config,
        ) {
            let saved = super::config_store::SavedConfiguration {
                manufacturer: mfr.id.clone(),
                series: family.id.clone(),
                article_nr: family.base_article_nr.clone(),
                properties: config.selections.clone(),
                variant_code: config.variant_code.clone(),
                description: family.name.clone(),
                price_date: Some(self.price_date.format("%Y-%m-%d").to_string()),
                saved_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            };

            match super::config_store::save_configuration(&saved) {
                Ok(path) => {
                    self.status_message = Some(format!(
                        "Gespeichert: {}",
                        path.file_name()
                            .and_then(|s| s.to_str())
                            .unwrap_or("config.json")
                    ));
                }
                Err(e) => {
                    self.status_message = Some(format!("Speichern fehlgeschlagen: {}", e));
                }
            }
        } else {
            self.status_message = Some("Keine Konfiguration zum Speichern".to_string());
        }
    }

    /// Process a message and update state
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::SelectManufacturer(idx) => {
                if idx < self.manufacturers.len() {
                    self.selected_manufacturer = Some(self.manufacturers[idx].clone());
                    // Screen is set in main.rs after loading catalog/families
                    // If catalog exists -> Screen::Catalog, else -> Screen::Families
                    self.family_list_state.select(Some(0));
                    self.catalog_list_state.select(Some(0));
                }
            }
            Message::SelectCatalogNode(idx) => {
                if idx < self.catalog_children.len() {
                    let node = self.catalog_children[idx].clone();
                    match node.node_type {
                        NodeType::Folder => {
                            // Enter the folder
                            self.enter_catalog_folder(&node);
                        }
                        NodeType::Article => {
                            // Find family for this article and configure it
                            // This is handled externally in main.rs
                            self.status_message =
                                Some(format!("Artikel: {} ({})", node.name, node.id));
                        }
                        NodeType::Root => {}
                    }
                }
            }
            Message::SelectFamily(idx) => {
                if idx < self.families.len() {
                    self.selected_family = Some(self.families[idx].clone());
                    self.screen = Screen::FamilyConfig;
                    self.focused_property = 0;
                    self.focused_article_index = 0;
                    // Properties and configuration are loaded in main.rs TUI loop
                }
            }
            Message::SelectArticle(idx) => {
                if idx < self.articles.len() {
                    self.selected_article = Some(self.articles[idx].clone());
                    self.screen = Screen::Properties;
                    self.focused_property = 0;
                    // Legacy article mode - configuration loaded externally
                }
            }
            Message::CyclePropertyOption(delta) => {
                // Cycle through options for the focused property
                let mut changed = false;
                let mut change_info: Option<(String, String, String, usize)> = None;

                if let Some(ref mut config) = self.family_config {
                    if let Some(prop) = self.family_properties.get(self.focused_property) {
                        if !prop.options.is_empty() {
                            let current_value = config
                                .selections
                                .get(&prop.key)
                                .cloned()
                                .unwrap_or_default();
                            let current_idx = prop
                                .options
                                .iter()
                                .position(|o| o.value == current_value)
                                .unwrap_or(0);
                            let new_idx = ((current_idx as i32 + delta)
                                .rem_euclid(prop.options.len() as i32))
                                as usize;
                            let new_value = prop.options[new_idx].value.clone();

                            if current_value != new_value {
                                config.set(&prop.key, &new_value);
                                changed = true;
                                change_info = Some((
                                    prop.key.clone(),
                                    current_value,
                                    new_value,
                                    prop.options.len(),
                                ));
                            }
                        }
                    }
                }
                if changed {
                    if let Some((key, old, new, opts)) = change_info {
                        // Push to undo stack and clear redo stack
                        self.undo_stack.push(PropertyChange {
                            key: key.clone(),
                            old_value: old.clone(),
                            new_value: new.clone(),
                        });
                        self.redo_stack.clear();

                        if self.debug_mode {
                            self.debug_log(format!(
                                "Property changed: {} = '{}' -> '{}' ({} options)",
                                key, old, new, opts
                            ));
                        }
                    }
                    self.recalculate_price();
                }
            }
            Message::EditProperty(name, value) => {
                if let Some(ref mut config) = self.configuration {
                    // Parse and set property value
                    use ofml_lib::property::PropertyValue;
                    let pv = if let Ok(i) = value.parse::<i64>() {
                        PropertyValue::Int(i)
                    } else if let Ok(f) = value.parse::<f64>() {
                        PropertyValue::Float(f)
                    } else if value == "true" {
                        PropertyValue::Bool(true)
                    } else if value == "false" {
                        PropertyValue::Bool(false)
                    } else {
                        PropertyValue::String(value)
                    };
                    config.properties.values.insert(name, pv);
                    config.update_variant_code();
                    // Legacy article mode price recalculation not yet implemented
                }
            }
            Message::NavigateUp => {
                self.navigate(-1);
            }
            Message::NavigateDown => {
                self.navigate(1);
            }
            Message::NavigatePageUp => {
                self.navigate(-10);
            }
            Message::NavigatePageDown => {
                self.navigate(10);
            }
            Message::NavigateToTop => {
                self.navigate_to(0);
            }
            Message::NavigateToBottom => {
                let max_idx = self.get_current_list_length().saturating_sub(1);
                self.navigate_to(max_idx);
            }
            Message::GoBack => {
                match self.screen {
                    Screen::Catalog => {
                        if !self.catalog_path.is_empty() {
                            // Navigate up in catalog hierarchy
                            self.exit_catalog_folder();
                        } else {
                            // At root of catalog, go back to manufacturer list
                            self.screen = Screen::Manufacturers;
                            self.selected_manufacturer = None;
                            self.catalog = None;
                            self.catalog_path.clear();
                            self.catalog_children.clear();
                        }
                    }
                    Screen::Families => {
                        // Check if we came from catalog
                        if self.catalog.is_some() {
                            self.screen = Screen::Catalog;
                        } else {
                            self.screen = Screen::Manufacturers;
                            self.selected_manufacturer = None;
                            self.families.clear();
                        }
                    }
                    Screen::FamilyConfig => {
                        // Go back to catalog if we have one, otherwise families
                        if self.catalog.is_some() && !self.catalog_path.is_empty() {
                            self.screen = Screen::Catalog;
                        } else if self.catalog.is_some() {
                            // At catalog root, restore children
                            if let Some(ref cat) = self.catalog {
                                self.catalog_children = cat.root.children.clone();
                            }
                            self.screen = Screen::Catalog;
                        } else {
                            self.screen = Screen::Families;
                        }
                        self.selected_family = None;
                        self.family_properties.clear();
                        self.family_config = None;
                        self.family_price = None;
                        self.focused_article_index = 0;
                    }
                    Screen::Articles => {
                        self.screen = Screen::Manufacturers;
                        self.selected_manufacturer = None;
                    }
                    Screen::Properties => {
                        self.screen = Screen::Articles;
                        self.selected_article = None;
                        self.configuration = None;
                    }
                    Screen::Help => {
                        // Return to previous screen
                        self.screen = Screen::Manufacturers;
                    }
                    Screen::Tables => {
                        // Return to families or catalog
                        if self.catalog.is_some() {
                            self.screen = Screen::Catalog;
                        } else {
                            self.screen = Screen::Families;
                        }
                        self.tables.clear();
                    }
                    Screen::TableView => {
                        self.screen = Screen::Tables;
                        self.selected_table = None;
                        self.table_rows.clear();
                        self.table_scroll_x = 0;
                    }
                    Screen::SavedConfigs => {
                        // Return to manufacturer list
                        self.screen = Screen::Manufacturers;
                        self.saved_configs.clear();
                    }
                    Screen::Manufacturers => {
                        // At top level, quit
                        self.should_quit = true;
                    }
                }
            }
            Message::GoHome => {
                // Reset to manufacturers screen
                self.screen = Screen::Manufacturers;
                self.selected_manufacturer = None;
                self.selected_family = None;
                self.selected_article = None;
                self.catalog = None;
                self.catalog_path.clear();
                self.catalog_children.clear();
                self.families.clear();
                self.family_properties.clear();
                self.family_config = None;
                self.family_price = None;
                self.articles.clear();
                self.configuration = None;
                self.tables.clear();
                self.table_rows.clear();
                self.search_query.clear();
                self.search_active = false;
                self.filtered_indices.clear();
                self.manufacturer_list_state.select(Some(0));
                // Also clear undo/redo stacks when going home
                self.undo_stack.clear();
                self.redo_stack.clear();
            }
            Message::Undo => {
                if let Some(change) = self.undo_stack.pop() {
                    // Apply the reverse change
                    if let Some(ref mut config) = self.family_config {
                        config.set(&change.key, &change.old_value);
                        // Move to redo stack (swap old/new for redo)
                        self.redo_stack.push(PropertyChange {
                            key: change.key.clone(),
                            old_value: change.new_value.clone(),
                            new_value: change.old_value.clone(),
                        });
                        self.recalculate_price();
                        self.status_message = Some(format!("Rückgängig: {}", change.key));
                        if self.debug_mode {
                            self.debug_log(format!(
                                "Undo: {} = '{}' (was '{}')",
                                change.key, change.old_value, change.new_value
                            ));
                        }
                    }
                } else {
                    self.status_message = Some("Nichts zum Rückgängigmachen".to_string());
                }
            }
            Message::Redo => {
                if let Some(change) = self.redo_stack.pop() {
                    // Reapply the change (old_value in redo is what we want to apply)
                    if let Some(ref mut config) = self.family_config {
                        config.set(&change.key, &change.old_value);
                        // Move back to undo stack
                        self.undo_stack.push(PropertyChange {
                            key: change.key.clone(),
                            old_value: change.new_value.clone(),
                            new_value: change.old_value.clone(),
                        });
                        self.recalculate_price();
                        self.status_message = Some(format!("Wiederherstellen: {}", change.key));
                        if self.debug_mode {
                            self.debug_log(format!(
                                "Redo: {} = '{}' (was '{}')",
                                change.key, change.old_value, change.new_value
                            ));
                        }
                    }
                } else {
                    self.status_message = Some("Nichts zum Wiederherstellen".to_string());
                }
            }
            Message::ToggleSearch => {
                self.search_active = !self.search_active;
                if !self.search_active {
                    self.search_query.clear();
                    self.update_filter();
                }
            }
            Message::UpdateSearch(query) => {
                self.search_query = query;
                self.update_filter();
            }
            Message::SetPriceDate(date) => {
                self.price_date = date;
                self.recalculate_price();
            }
            Message::Export => {
                // Try family configuration export first (new mode)
                if let (Some(ref mfr), Some(ref family), Some(ref config)) = (
                    &self.selected_manufacturer,
                    &self.selected_family,
                    &self.family_config,
                ) {
                    let article_nr = family.base_article_nr.as_str();
                    let series_id = &family.id;
                    let json = ofml_lib::oap::export_family_json(
                        &mfr.id,
                        series_id,
                        article_nr,
                        config,
                        self.family_price.as_ref(),
                        &self.family_warnings,
                    );
                    let filename = format!(
                        "{}_{}_{}_{}.json",
                        mfr.id,
                        series_id,
                        article_nr,
                        chrono::Utc::now().format("%Y%m%d_%H%M%S")
                    );
                    match std::fs::write(&filename, &json) {
                        Ok(_) => {
                            self.status_message = Some(format!("Export erfolgreich: {}", filename))
                        }
                        Err(e) => {
                            self.status_message = Some(format!("Export fehlgeschlagen: {}", e))
                        }
                    }
                }
                // Fallback to legacy configuration export
                else if let Some(ref config) = self.configuration {
                    let export_data = config.to_export_data();
                    let filename = format!(
                        "{}_{}_{}.json",
                        config.manufacturer_id,
                        config.article_id,
                        chrono::Utc::now().format("%Y%m%d_%H%M%S")
                    );
                    match serde_json::to_string_pretty(&export_data) {
                        Ok(json) => match std::fs::write(&filename, json) {
                            Ok(_) => {
                                self.status_message =
                                    Some(format!("Export erfolgreich: {}", filename))
                            }
                            Err(e) => {
                                self.status_message = Some(format!("Export fehlgeschlagen: {}", e))
                            }
                        },
                        Err(e) => self.status_message = Some(format!("JSON Fehler: {}", e)),
                    }
                } else {
                    self.status_message = Some("Keine Konfiguration zum Exportieren".to_string());
                }
            }
            Message::ShowHelp => {
                self.screen = Screen::Help;
            }
            Message::ToggleDebug => {
                self.debug_mode = !self.debug_mode;
                if self.debug_mode {
                    self.debug_log("Debug console enabled (F12 to hide)".to_string());
                    // Log current config state
                    if let Some(ref config) = self.family_config {
                        self.debug_log(format!("Current config: {} selections", config.selections.len()));
                    }
                    if let Some(ref price) = self.family_price {
                        self.debug_log(format!(
                            "Current price: {} EUR ({} surcharges)",
                            price.total_price, price.surcharges.len()
                        ));
                    }
                }
            }
            Message::ShowTables => {
                self.screen = Screen::Tables;
                self.table_list_state.select(Some(0));
            }
            Message::SelectTable(idx) => {
                if idx < self.tables.len() {
                    self.selected_table = Some(self.tables[idx].clone());
                    self.screen = Screen::TableView;
                    self.table_row_list_state.select(Some(0));
                    self.table_scroll_x = 0;
                    // Table rows are loaded in main.rs
                }
            }
            Message::ScrollTableHorizontal(delta) => {
                let new_offset = (self.table_scroll_x as i32 + delta).max(0) as usize;
                self.table_scroll_x = new_offset;
            }
            Message::SaveConfig => {
                self.save_current_configuration();
            }
            Message::LoadConfig | Message::ShowSavedConfigs => {
                // Load saved configurations list
                match super::config_store::list_configurations() {
                    Ok(configs) => {
                        self.saved_configs = configs;
                        if !self.saved_configs.is_empty() {
                            self.saved_configs_list_state.select(Some(0));
                        }
                        self.screen = Screen::SavedConfigs;
                    }
                    Err(e) => {
                        self.status_message = Some(format!("Fehler beim Laden: {}", e));
                    }
                }
            }
            Message::SelectSavedConfig(idx) => {
                if idx < self.saved_configs.len() {
                    let (_, config) = &self.saved_configs[idx];
                    // Store which configuration to load - actual loading happens in main.rs
                    self.status_message = Some(format!(
                        "Lade Konfiguration: {} {} ({})",
                        config.manufacturer, config.article_nr, config.variant_code
                    ));
                }
            }
            Message::ToggleFavorite => {
                if let (Some(ref mfr), Some(ref family)) =
                    (&self.selected_manufacturer, &self.selected_family)
                {
                    let is_fav = super::config_store::is_favorite(&mfr.id, &family.id);
                    if is_fav {
                        if let Err(e) = super::config_store::remove_favorite(&mfr.id, &family.id) {
                            self.status_message = Some(format!("Fehler: {}", e));
                        } else {
                            self.status_message = Some(format!("★ {} entfernt", family.name));
                        }
                    } else if let Err(e) =
                        super::config_store::add_favorite(&mfr.id, &family.id, &family.name)
                    {
                        self.status_message = Some(format!("Fehler: {}", e));
                    } else {
                        self.status_message = Some(format!("★ {} als Favorit", family.name));
                    }
                } else {
                    self.status_message =
                        Some("Wählen Sie erst eine Produktfamilie aus".to_string());
                }
            }
            Message::ToggleTheme => {
                self.theme_variant = self.theme_variant.toggle();
                let theme_name = match self.theme_variant {
                    super::theme::ThemeVariant::Dark => "Dunkel",
                    super::theme::ThemeVariant::Light => "Hell",
                };
                self.status_message = Some(format!("Theme: {}", theme_name));
            }
            Message::Quit => {
                self.should_quit = true;
            }
            Message::Resize(width, height) => {
                self.terminal_size = (width, height);
            }
            Message::ExportGeometry => {
                // Handled in main.rs TUI loop (needs file I/O)
            }
        }
    }

    /// Navigate up or down in the current list
    fn navigate(&mut self, delta: i32) {
        match self.screen {
            Screen::Manufacturers => {
                let len = if self.filtered_indices.is_empty() {
                    self.manufacturers.len()
                } else {
                    self.filtered_indices.len()
                };
                if len == 0 {
                    return;
                }
                let current = self.manufacturer_list_state.selected().unwrap_or(0) as i32;
                let new = (current + delta).rem_euclid(len as i32) as usize;
                self.manufacturer_list_state.select(Some(new));
            }
            Screen::Catalog => {
                let len = if self.filtered_indices.is_empty() {
                    self.catalog_children.len()
                } else {
                    self.filtered_indices.len()
                };
                if len == 0 {
                    return;
                }
                let current = self.catalog_list_state.selected().unwrap_or(0) as i32;
                let new = (current + delta).rem_euclid(len as i32) as usize;
                self.catalog_list_state.select(Some(new));
            }
            Screen::Families => {
                let len = if self.filtered_indices.is_empty() {
                    self.families.len()
                } else {
                    self.filtered_indices.len()
                };
                if len == 0 {
                    return;
                }
                let current = self.family_list_state.selected().unwrap_or(0) as i32;
                let new = (current + delta).rem_euclid(len as i32) as usize;
                self.family_list_state.select(Some(new));
            }
            Screen::FamilyConfig => {
                if !self.family_properties.is_empty() {
                    // Navigate through properties
                    let len = self.family_properties.len();
                    let current = self.focused_property as i32;
                    let new = (current + delta).rem_euclid(len as i32) as usize;
                    self.focused_property = new;
                } else if let Some(ref family) = self.selected_family {
                    // No properties - navigate through articles
                    let len = family.article_nrs.len();
                    if len == 0 {
                        return;
                    }
                    let current = self.focused_article_index as i32;
                    let new = (current + delta).rem_euclid(len as i32) as usize;
                    self.focused_article_index = new;
                }
            }
            Screen::Articles => {
                let len = if self.filtered_indices.is_empty() {
                    self.articles.len()
                } else {
                    self.filtered_indices.len()
                };
                if len == 0 {
                    return;
                }
                let current = self.article_list_state.selected().unwrap_or(0) as i32;
                let new = (current + delta).rem_euclid(len as i32) as usize;
                self.article_list_state.select(Some(new));
            }
            Screen::Properties => {
                if let Some(ref config) = self.configuration {
                    let len = config.properties.definitions.len();
                    if len == 0 {
                        return;
                    }
                    let current = self.focused_property as i32;
                    let new = (current + delta).rem_euclid(len as i32) as usize;
                    self.focused_property = new;
                }
            }
            Screen::Tables => {
                let len = if self.filtered_indices.is_empty() {
                    self.tables.len()
                } else {
                    self.filtered_indices.len()
                };
                if len == 0 {
                    return;
                }
                let current = self.table_list_state.selected().unwrap_or(0) as i32;
                let new = (current + delta).rem_euclid(len as i32) as usize;
                self.table_list_state.select(Some(new));
            }
            Screen::TableView => {
                let len = self.table_rows.len();
                if len == 0 {
                    return;
                }
                let current = self.table_row_list_state.selected().unwrap_or(0) as i32;
                let new = (current + delta).rem_euclid(len as i32) as usize;
                self.table_row_list_state.select(Some(new));
            }
            Screen::SavedConfigs => {
                let len = self.saved_configs.len();
                if len == 0 {
                    return;
                }
                let current = self.saved_configs_list_state.selected().unwrap_or(0) as i32;
                let new = (current + delta).rem_euclid(len as i32) as usize;
                self.saved_configs_list_state.select(Some(new));
            }
            Screen::Help => {}
        }
    }

    /// Get the length of the current list (for navigation)
    fn get_current_list_length(&self) -> usize {
        match self.screen {
            Screen::Manufacturers => {
                if self.filtered_indices.is_empty() {
                    self.manufacturers.len()
                } else {
                    self.filtered_indices.len()
                }
            }
            Screen::Catalog => {
                if self.filtered_indices.is_empty() {
                    self.catalog_children.len()
                } else {
                    self.filtered_indices.len()
                }
            }
            Screen::Families => {
                if self.filtered_indices.is_empty() {
                    self.families.len()
                } else {
                    self.filtered_indices.len()
                }
            }
            Screen::FamilyConfig => {
                if !self.family_properties.is_empty() {
                    self.family_properties.len()
                } else if let Some(ref family) = self.selected_family {
                    family.article_nrs.len()
                } else {
                    0
                }
            }
            Screen::Articles => {
                if self.filtered_indices.is_empty() {
                    self.articles.len()
                } else {
                    self.filtered_indices.len()
                }
            }
            Screen::Properties => self
                .configuration
                .as_ref()
                .map(|c| c.properties.definitions.len())
                .unwrap_or(0),
            Screen::Tables => {
                if self.filtered_indices.is_empty() {
                    self.tables.len()
                } else {
                    self.filtered_indices.len()
                }
            }
            Screen::TableView => self.table_rows.len(),
            Screen::SavedConfigs => self.saved_configs.len(),
            Screen::Help => 0,
        }
    }

    /// Navigate directly to a specific index
    fn navigate_to(&mut self, index: usize) {
        let len = self.get_current_list_length();
        if len == 0 {
            return;
        }
        let new_idx = index.min(len.saturating_sub(1));

        match self.screen {
            Screen::Manufacturers => {
                self.manufacturer_list_state.select(Some(new_idx));
            }
            Screen::Catalog => {
                self.catalog_list_state.select(Some(new_idx));
            }
            Screen::Families => {
                self.family_list_state.select(Some(new_idx));
            }
            Screen::FamilyConfig => {
                if !self.family_properties.is_empty() {
                    self.focused_property = new_idx;
                } else {
                    self.focused_article_index = new_idx;
                }
            }
            Screen::Articles => {
                self.article_list_state.select(Some(new_idx));
            }
            Screen::Properties => {
                self.focused_property = new_idx;
            }
            Screen::Tables => {
                self.table_list_state.select(Some(new_idx));
            }
            Screen::TableView => {
                self.table_row_list_state.select(Some(new_idx));
            }
            Screen::SavedConfigs => {
                self.saved_configs_list_state.select(Some(new_idx));
            }
            Screen::Help => {}
        }
    }

    /// Calculate fuzzy match score between query and target
    ///
    /// Returns None if no match, Some(score) otherwise.
    /// Higher scores indicate better matches.
    ///
    /// Scoring criteria:
    /// - 1000: Exact match (case-insensitive)
    /// - 500+: Prefix match (query at start of target)
    /// - 200+: Word boundary match (query appears after space/underscore/dash)
    /// - 100+: Substring match (query appears anywhere)
    /// - 10+: Subsequence match (all chars appear in order)
    /// - Bonus: Consecutive character matches
    /// - Penalty: Longer gaps between matches
    fn fuzzy_score(query: &str, target: &str) -> Option<i32> {
        if query.is_empty() {
            return Some(0);
        }

        let target_lower = target.to_lowercase();
        let query_lower = query.to_lowercase();

        // Exact match (highest priority)
        if target_lower == query_lower {
            return Some(1000);
        }

        // Prefix match
        if target_lower.starts_with(&query_lower) {
            return Some(500 + (100 - target_lower.len().min(100) as i32));
        }

        // Word boundary match (after space, underscore, or dash)
        let word_boundaries: Vec<usize> = target_lower
            .char_indices()
            .filter(|(i, c)| {
                *i == 0
                    || *c == ' '
                    || *c == '_'
                    || *c == '-'
                    || target_lower.chars().nth(i.saturating_sub(1)).map_or(false, |prev| {
                        prev == ' ' || prev == '_' || prev == '-'
                    })
            })
            .map(|(i, _)| i)
            .collect();

        for boundary in &word_boundaries {
            if target_lower[*boundary..].starts_with(&query_lower) {
                return Some(200 + (100 - target_lower.len().min(100) as i32));
            }
        }

        // Substring match
        if let Some(pos) = target_lower.find(&query_lower) {
            // Earlier position = better score
            return Some(100 + (50 - pos.min(50) as i32));
        }

        // Subsequence match with gap penalty
        let mut query_chars = query_lower.chars().peekable();
        let mut score = 10;
        let mut consecutive = 0;
        let mut last_match_idx: Option<usize> = None;

        for (idx, target_char) in target_lower.chars().enumerate() {
            if query_chars.peek().is_some_and(|&q| q == target_char) {
                query_chars.next();

                // Bonus for consecutive matches
                if last_match_idx.is_some_and(|last| idx == last + 1) {
                    consecutive += 1;
                    score += consecutive * 2;
                } else {
                    consecutive = 0;
                    // Penalty for gaps
                    if let Some(last) = last_match_idx {
                        let gap = idx - last - 1;
                        score -= (gap as i32).min(5);
                    }
                }

                // Bonus for word boundary matches
                if word_boundaries.contains(&idx) {
                    score += 3;
                }

                last_match_idx = Some(idx);
            }
        }

        if query_chars.peek().is_none() {
            Some(score.max(1))
        } else {
            None
        }
    }

    /// Get the best fuzzy match score from multiple targets
    fn fuzzy_score_any(query: &str, targets: &[&str]) -> Option<i32> {
        targets
            .iter()
            .filter_map(|t| Self::fuzzy_score(query, t))
            .max()
    }

    /// Update filter based on search query
    fn update_filter(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_indices.clear();
            return;
        }

        let query = &self.search_query;

        // Collect (index, score) pairs and sort by score descending
        let mut scored: Vec<(usize, i32)> = match self.screen {
            Screen::Manufacturers => self
                .manufacturers
                .iter()
                .enumerate()
                .filter_map(|(i, m)| {
                    Self::fuzzy_score_any(query, &[&m.id, &m.name]).map(|s| (i, s))
                })
                .collect(),
            Screen::Catalog => self
                .catalog_children
                .iter()
                .enumerate()
                .filter_map(|(i, n)| {
                    Self::fuzzy_score_any(query, &[&n.id, &n.name]).map(|s| (i, s))
                })
                .collect(),
            Screen::Families => self
                .families
                .iter()
                .enumerate()
                .filter_map(|(i, f)| {
                    Self::fuzzy_score_any(query, &[&f.id, &f.name, &f.description]).map(|s| (i, s))
                })
                .collect(),
            Screen::Articles => self
                .articles
                .iter()
                .enumerate()
                .filter_map(|(i, a)| {
                    Self::fuzzy_score_any(query, &[&a.id, &a.short_description]).map(|s| (i, s))
                })
                .collect(),
            Screen::Tables => self
                .tables
                .iter()
                .enumerate()
                .filter_map(|(i, t)| Self::fuzzy_score(query, &t.name).map(|s| (i, s)))
                .collect(),
            _ => Vec::new(),
        };

        // Sort by score descending (higher score = better match = first)
        scored.sort_by(|a, b| b.1.cmp(&a.1));
        self.filtered_indices = scored.into_iter().map(|(i, _)| i).collect();

        // Reset selection to first filtered item
        match self.screen {
            Screen::Manufacturers => {
                self.manufacturer_list_state.select(Some(0));
            }
            Screen::Catalog => {
                self.catalog_list_state.select(Some(0));
            }
            Screen::Families => {
                self.family_list_state.select(Some(0));
            }
            Screen::Articles => {
                self.article_list_state.select(Some(0));
            }
            Screen::Tables => {
                self.table_list_state.select(Some(0));
            }
            _ => {}
        }
    }

    /// Get the currently selected item index (accounting for filter)
    pub fn get_selected_index(&self) -> Option<usize> {
        match self.screen {
            Screen::Manufacturers => {
                let selected = self.manufacturer_list_state.selected()?;
                if self.filtered_indices.is_empty() {
                    Some(selected)
                } else {
                    self.filtered_indices.get(selected).copied()
                }
            }
            Screen::Catalog => {
                let selected = self.catalog_list_state.selected()?;
                if self.filtered_indices.is_empty() {
                    Some(selected)
                } else {
                    self.filtered_indices.get(selected).copied()
                }
            }
            Screen::Families => {
                let selected = self.family_list_state.selected()?;
                if self.filtered_indices.is_empty() {
                    Some(selected)
                } else {
                    self.filtered_indices.get(selected).copied()
                }
            }
            Screen::Articles => {
                let selected = self.article_list_state.selected()?;
                if self.filtered_indices.is_empty() {
                    Some(selected)
                } else {
                    self.filtered_indices.get(selected).copied()
                }
            }
            Screen::Tables => {
                let selected = self.table_list_state.selected()?;
                if self.filtered_indices.is_empty() {
                    Some(selected)
                } else {
                    self.filtered_indices.get(selected).copied()
                }
            }
            _ => None,
        }
    }
}


impl Default for App {
    fn default() -> Self {
        Self::new(String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_new() {
        let app = App::new("/workspace/ofmldata".to_string());
        assert_eq!(app.screen, Screen::Manufacturers);
        assert!(!app.should_quit);
        assert!(app.manufacturers.is_empty());
    }

    #[test]
    fn test_app_navigate() {
        let mut app = App::new("/workspace/ofmldata".to_string());
        app.manufacturers = vec![
            Manufacturer {
                id: "a".to_string(),
                name: "A".to_string(),
                path: std::path::PathBuf::new(),
            },
            Manufacturer {
                id: "b".to_string(),
                name: "B".to_string(),
                path: std::path::PathBuf::new(),
            },
        ];

        assert_eq!(app.manufacturer_list_state.selected(), Some(0));
        app.update(Message::NavigateDown);
        assert_eq!(app.manufacturer_list_state.selected(), Some(1));
        app.update(Message::NavigateDown);
        assert_eq!(app.manufacturer_list_state.selected(), Some(0)); // Wrap around
    }

    #[test]
    fn test_app_quit() {
        let mut app = App::new("/workspace/ofmldata".to_string());
        assert!(!app.should_quit);
        app.update(Message::Quit);
        assert!(app.should_quit);
    }

    #[test]
    fn test_app_search() {
        let mut app = App::new("/workspace/ofmldata".to_string());
        app.manufacturers = vec![
            Manufacturer {
                id: "vitra".to_string(),
                name: "Vitra AG".to_string(),
                path: std::path::PathBuf::new(),
            },
            Manufacturer {
                id: "sedus".to_string(),
                name: "Sedus Stoll".to_string(),
                path: std::path::PathBuf::new(),
            },
        ];

        app.update(Message::ToggleSearch);
        assert!(app.search_active);

        app.update(Message::UpdateSearch("vit".to_string()));
        assert_eq!(app.filtered_indices, vec![0]);

        app.update(Message::ToggleSearch);
        assert!(!app.search_active);
        assert!(app.filtered_indices.is_empty());
    }

    #[test]
    fn test_fuzzy_score() {
        // Exact match - highest score
        assert!(App::fuzzy_score("vitra", "vitra").unwrap() >= 1000);

        // Prefix match - high score
        let prefix_score = App::fuzzy_score("vit", "Vitra").unwrap();
        assert!(prefix_score >= 500);

        // Case insensitive
        assert!(App::fuzzy_score("VIT", "vitra").is_some());

        // Subsequence match - lower score
        let subseq_score = App::fuzzy_score("vs", "Vitra Sedus").unwrap();
        assert!(subseq_score >= 10);
        assert!(subseq_score < prefix_score); // Subsequence < prefix

        assert!(App::fuzzy_score("ai", "Aeris_Inspire").is_some());
        assert!(App::fuzzy_score("sd", "Sedus").is_some());

        // Non-matches should return None
        assert!(App::fuzzy_score("xyz", "Vitra").is_none());
        assert!(App::fuzzy_score("ab", "ba").is_none()); // Order matters

        // Verify scoring priority: exact > prefix > substring > subsequence
        let exact = App::fuzzy_score("vitra", "vitra").unwrap();
        let prefix = App::fuzzy_score("vit", "vitra").unwrap();
        let substring = App::fuzzy_score("itr", "vitra").unwrap();
        let subsequence = App::fuzzy_score("va", "vitra").unwrap();

        assert!(exact > prefix, "exact ({}) should beat prefix ({})", exact, prefix);
        assert!(prefix > substring, "prefix ({}) should beat substring ({})", prefix, substring);
        assert!(substring > subsequence, "substring ({}) should beat subsequence ({})", substring, subsequence);
    }

    #[test]
    fn test_fuzzy_search() {
        let mut app = App::new("/workspace/ofmldata".to_string());
        app.manufacturers = vec![
            Manufacturer {
                id: "vitra".to_string(),
                name: "Vitra AG".to_string(),
                path: std::path::PathBuf::new(),
            },
            Manufacturer {
                id: "sedus".to_string(),
                name: "Sedus Stoll".to_string(),
                path: std::path::PathBuf::new(),
            },
            Manufacturer {
                id: "bisley".to_string(),
                name: "Bisley".to_string(),
                path: std::path::PathBuf::new(),
            },
        ];

        app.update(Message::ToggleSearch);

        // Test fuzzy subsequence match
        app.update(Message::UpdateSearch("vs".to_string()));
        // "vs" matches "Vitra" (v...s) and "Sedus" (s...s would need 2 s's)
        // Actually "vs" only matches if v and s appear in order
        // In "Vitra AG" we have v-i-t-r-a, no 's' -> doesn't match
        // In "Sedus Stoll" we have s-e-d-u-s, has 's' but no 'v' before -> doesn't match
        // So "vs" shouldn't match anything in this set
        assert!(app.filtered_indices.is_empty());

        // Test fuzzy match with "ss" for Sedus Stoll
        app.update(Message::UpdateSearch("ss".to_string()));
        assert_eq!(app.filtered_indices, vec![1]); // Sedus has two s's
    }
}
