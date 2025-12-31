//! TUI Application state and event handling
//!
//! This module implements the Elm Architecture (TEA) pattern for the TUI:
//! - App: Application state
//! - Message: Events that can occur
//! - Screen: Current view state

#[cfg(feature = "tui")]
use chrono::NaiveDate;
#[cfg(feature = "tui")]
use ratatui::widgets::ListState;

#[cfg(feature = "tui")]
use crate::oap::families::{FamilyConfiguration, FamilyProperty, ProductFamily};
#[cfg(feature = "tui")]
use crate::oap::{Article, Configuration, Manufacturer, PriceResult};

/// Current screen/view in the TUI
#[cfg(feature = "tui")]
#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    /// List of manufacturers
    Manufacturers,
    /// List of product families for selected manufacturer
    Families,
    /// Family configuration view with property selection
    FamilyConfig,
    /// List of articles for selected manufacturer (legacy)
    Articles,
    /// Property configuration for selected article (legacy)
    Properties,
    /// Help screen
    Help,
}

/// Messages/events that can occur in the TUI
#[cfg(feature = "tui")]
#[derive(Debug, Clone)]
pub enum Message {
    /// Select a manufacturer by index
    SelectManufacturer(usize),
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
    /// Go back to previous screen
    GoBack,
    /// Toggle search mode
    ToggleSearch,
    /// Update search query
    UpdateSearch(String),
    /// Set price date
    SetPriceDate(NaiveDate),
    /// Export configuration
    Export,
    /// Show help
    ShowHelp,
    /// Quit application
    Quit,
    /// Resize terminal
    Resize(u16, u16),
}

/// Main application state
#[cfg(feature = "tui")]
pub struct App {
    /// Current screen
    pub screen: Screen,
    /// List of loaded manufacturers
    pub manufacturers: Vec<Manufacturer>,
    /// Manufacturer list selection state
    pub manufacturer_list_state: ListState,
    /// Currently selected manufacturer
    pub selected_manufacturer: Option<Manufacturer>,

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
    /// Focused article index (when no OCD properties, navigate articles instead)
    pub focused_article_index: usize,

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
}

#[cfg(feature = "tui")]
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
            // Product family mode
            families: Vec::new(),
            family_list_state: ListState::default(),
            selected_family: None,
            family_properties: Vec::new(),
            family_config: None,
            family_price: None,
            focused_article_index: 0,
            // Legacy article mode
            articles: Vec::new(),
            article_list_state: ListState::default(),
            selected_article: None,
            configuration: None,
            price: None,
            // Common state
            price_date: chrono::Local::now().date_naive(),
            search_active: false,
            search_query: String::new(),
            filtered_indices: Vec::new(),
            focused_property: 0,
            status_message: None,
            should_quit: false,
            data_path,
            terminal_size: (80, 24),
        }
    }

    /// Process a message and update state
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::SelectManufacturer(idx) => {
                if idx < self.manufacturers.len() {
                    self.selected_manufacturer = Some(self.manufacturers[idx].clone());
                    self.screen = Screen::Families;
                    self.family_list_state.select(Some(0));
                    // Families are loaded in main.rs TUI loop
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
                    // TODO: Load configuration for article
                }
            }
            Message::CyclePropertyOption(delta) => {
                // Cycle through options for the focused property
                if let Some(ref mut config) = self.family_config {
                    if let Some(prop) = self.family_properties.get(self.focused_property) {
                        if !prop.options.is_empty() {
                            let current_value = config.selections.get(&prop.key).cloned().unwrap_or_default();
                            let current_idx = prop.options.iter().position(|o| o.value == current_value).unwrap_or(0);
                            let new_idx = ((current_idx as i32 + delta).rem_euclid(prop.options.len() as i32)) as usize;
                            config.set(&prop.key, &prop.options[new_idx].value);
                        }
                    }
                }
            }
            Message::EditProperty(name, value) => {
                if let Some(ref mut config) = self.configuration {
                    // Parse and set property value
                    use crate::property::PropertyValue;
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
                    // TODO: Update price
                }
            }
            Message::NavigateUp => {
                self.navigate(-1);
            }
            Message::NavigateDown => {
                self.navigate(1);
            }
            Message::GoBack => {
                match self.screen {
                    Screen::Families => {
                        self.screen = Screen::Manufacturers;
                        self.selected_manufacturer = None;
                        self.families.clear();
                    }
                    Screen::FamilyConfig => {
                        self.screen = Screen::Families;
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
                    Screen::Manufacturers => {
                        // At top level, quit
                        self.should_quit = true;
                    }
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
                // TODO: Update price lookup
            }
            Message::Export => {
                if let Some(ref config) = self.configuration {
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
                }
            }
            Message::ShowHelp => {
                self.screen = Screen::Help;
            }
            Message::Quit => {
                self.should_quit = true;
            }
            Message::Resize(width, height) => {
                self.terminal_size = (width, height);
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
            Screen::Help => {}
        }
    }

    /// Update filter based on search query
    fn update_filter(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_indices.clear();
            return;
        }

        let query = self.search_query.to_lowercase();
        self.filtered_indices = match self.screen {
            Screen::Manufacturers => self
                .manufacturers
                .iter()
                .enumerate()
                .filter(|(_, m)| {
                    m.id.to_lowercase().contains(&query) || m.name.to_lowercase().contains(&query)
                })
                .map(|(i, _)| i)
                .collect(),
            Screen::Families => self
                .families
                .iter()
                .enumerate()
                .filter(|(_, f)| {
                    f.id.to_lowercase().contains(&query)
                        || f.name.to_lowercase().contains(&query)
                        || f.description.to_lowercase().contains(&query)
                })
                .map(|(i, _)| i)
                .collect(),
            Screen::Articles => self
                .articles
                .iter()
                .enumerate()
                .filter(|(_, a)| {
                    a.id.to_lowercase().contains(&query)
                        || a.short_description.to_lowercase().contains(&query)
                })
                .map(|(i, _)| i)
                .collect(),
            _ => Vec::new(),
        };

        // Reset selection to first filtered item
        match self.screen {
            Screen::Manufacturers => {
                self.manufacturer_list_state.select(Some(0));
            }
            Screen::Families => {
                self.family_list_state.select(Some(0));
            }
            Screen::Articles => {
                self.article_list_state.select(Some(0));
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
            _ => None,
        }
    }
}

#[cfg(feature = "tui")]
impl Default for App {
    fn default() -> Self {
        Self::new(String::new())
    }
}

#[cfg(all(test, feature = "tui"))]
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
}
