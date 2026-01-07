//! TUI event loop runner
//!
//! This module contains the main event loop for the TUI application.

use std::collections::{HashMap, HashSet};
use std::io::Stdout;
use std::path::Path;

use ratatui::{backend::CrosstermBackend, Terminal};

use ofml_lib::ebase::{EBaseReader};
use ofml_lib::oap::engine::ConfigurationEngine;
use ofml_lib::oap::ocd;

use ofml_tui::app::{App, CompositeComponent, PackagingInfo, TableInfo, TableRow};
use ofml_tui::{ui::render, config_store, Message, Screen};

/// Run the TUI event loop
pub fn run_event_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
    engine: &mut ConfigurationEngine,
) -> Result<(), String> {
    use crossterm::event::{self, Event, KeyCode};
    use ofml_lib::oap::catalog::load_smart_catalog;
    use ofml_lib::oap::families::{FamilyConfiguration, ProductFamily};

    loop {
        terminal
            .draw(|f| render(f, app))
            .map_err(|e| e.to_string())?;

        if let Event::Key(key) = event::read().map_err(|e| e.to_string())? {
            use crossterm::event::KeyModifiers;
            let is_ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

            let msg = match key.code {
                // Undo/Redo with Ctrl+Z / Ctrl+Y
                KeyCode::Char('z') if is_ctrl && !app.search_active => Some(Message::Undo),
                KeyCode::Char('y') if is_ctrl && !app.search_active => Some(Message::Redo),
                KeyCode::Char('u') if !app.search_active => Some(Message::Undo), // vim-style undo
                KeyCode::Char('q') if !app.search_active => Some(Message::Quit),
                KeyCode::Char('?') if !app.search_active => Some(Message::ShowHelp),
                KeyCode::F(12) => Some(Message::ToggleDebug),
                KeyCode::Char('/') if !app.search_active => Some(Message::ToggleSearch),
                KeyCode::Char('e') if !app.search_active => Some(Message::Export),
                // Geometry export with Ctrl+G (only in FamilyConfig)
                KeyCode::Char('g') if is_ctrl && !app.search_active && app.screen == Screen::FamilyConfig => {
                    Some(Message::ExportGeometry)
                }
                // Save/Load configuration shortcuts
                KeyCode::Char('s') if is_ctrl && !app.search_active => Some(Message::SaveConfig),
                KeyCode::Char('l') if is_ctrl && !app.search_active => Some(Message::LoadConfig),
                KeyCode::Char('o') if is_ctrl && !app.search_active => Some(Message::LoadConfig),
                // Toggle favorite
                KeyCode::Char('f')
                    if !app.search_active && app.screen == Screen::FamilyConfig =>
                {
                    Some(Message::ToggleFavorite)
                }
                KeyCode::Char('t') if !app.search_active && app.selected_manufacturer.is_some() => {
                    // Load tables for current manufacturer
                    if let Some(ref mfr) = app.selected_manufacturer {
                        app.status_message = Some("Lade Tabellen...".to_string());
                        let _ = terminal.draw(|f| render(f, app));
                        app.tables = load_manufacturer_tables(Path::new(&mfr.path));
                        app.table_list_state.select(Some(0));
                        app.status_message = Some(format!("{} Tabellen geladen", app.tables.len()));
                    }
                    Some(Message::ShowTables)
                }
                KeyCode::Up => Some(Message::NavigateUp),
                KeyCode::Down => Some(Message::NavigateDown),
                KeyCode::PageUp => Some(Message::NavigatePageUp),
                KeyCode::PageDown => Some(Message::NavigatePageDown),
                KeyCode::Home if !app.search_active => Some(Message::NavigateToTop),
                KeyCode::End if !app.search_active => Some(Message::NavigateToBottom),
                KeyCode::Char('g') if !app.search_active => Some(Message::NavigateToTop),
                KeyCode::Char('G') if !app.search_active => Some(Message::NavigateToBottom),
                KeyCode::Char('h') if !app.search_active => Some(Message::GoHome),
                KeyCode::Char('T') if !app.search_active => Some(Message::ToggleTheme),
                KeyCode::Left if app.screen == Screen::FamilyConfig => {
                    Some(Message::CyclePropertyOption(-1))
                }
                KeyCode::Right if app.screen == Screen::FamilyConfig => {
                    Some(Message::CyclePropertyOption(1))
                }
                KeyCode::Left if app.screen == Screen::TableView => {
                    Some(Message::ScrollTableHorizontal(-1))
                }
                KeyCode::Right if app.screen == Screen::TableView => {
                    Some(Message::ScrollTableHorizontal(1))
                }
                KeyCode::Enter => {
                    if let Some(idx) = app.get_selected_index() {
                        match app.screen {
                            Screen::Manufacturers => {
                                // Load catalog and product families for the selected manufacturer
                                if idx < app.manufacturers.len() {
                                    // Clone manufacturer data to avoid borrow issues
                                    let manufacturer_id = app.manufacturers[idx].id.clone();
                                    let manufacturer_name = app.manufacturers[idx].name.clone();
                                    let manufacturer_path = app.manufacturers[idx].path.clone();

                                    app.status_message =
                                        Some(format!("Lade {}...", manufacturer_name));
                                    let _ = terminal.draw(|f| render(f, app));

                                    // Try to load XCF catalog (master or aggregated from series)
                                    let data_path = Path::new(&app.data_path);
                                    let catalog = load_smart_catalog(
                                        data_path,
                                        &manufacturer_path,
                                        &manufacturer_id,
                                        "de",
                                    );
                                    let has_catalog = catalog.is_some();
                                    app.set_catalog(catalog);

                                    // Load product families using ConfigurationEngine
                                    let families = engine.load_families(&manufacturer_id);

                                    let configurable_count =
                                        families.iter().filter(|f| f.is_configurable).count();
                                    let with_props_count = families
                                        .iter()
                                        .filter(|f| !f.prop_classes.is_empty())
                                        .count();

                                    app.families = families.to_vec();

                                    // Set status message
                                    if has_catalog {
                                        let cat_stats =
                                            app.catalog.as_ref().map(|c| c.stats()).unwrap_or_else(
                                                || ofml_lib::oap::catalog::CatalogStats {
                                                    total_nodes: 0,
                                                    folder_count: 0,
                                                    article_count: 0,
                                                    text_entries: 0,
                                                    languages: vec![],
                                                },
                                            );
                                        app.status_message = Some(format!(
                                            "Katalog: {} Kategorien, {} Artikel",
                                            cat_stats.folder_count, cat_stats.article_count
                                        ));
                                        // Navigate to catalog view
                                        app.screen = Screen::Catalog;
                                    } else {
                                        app.status_message = Some(format!(
                                            "{} Produktfamilien ({} konfigurierbar, {} mit Eigenschaften)",
                                            app.families.len(),
                                            configurable_count,
                                            with_props_count
                                        ));
                                        // Navigate to families view
                                        app.screen = Screen::Families;
                                    }
                                }
                                Some(Message::SelectManufacturer(idx))
                            }
                            Screen::Catalog => {
                                // Handle catalog selection
                                if idx < app.catalog_children.len() {
                                    let node = app.catalog_children[idx].clone();
                                    match node.node_type {
                                        ofml_lib::oap::catalog::NodeType::Folder => {
                                            app.enter_catalog_folder(&node);
                                            app.status_message = Some(format!(
                                                "{} - {} Einträge",
                                                node.name,
                                                node.children.len()
                                            ));
                                        }
                                        ofml_lib::oap::catalog::NodeType::Article => {
                                            // Find family matching this article and configure it
                                            // Use case-insensitive matching and try multiple strategies
                                            let node_id_upper = node.id.to_uppercase();
                                            let matching_family = app.families.iter().find(|f| {
                                                f.article_nrs.iter().any(|nr| {
                                                    let nr_upper = nr.to_uppercase();
                                                    // Exact match (case-insensitive)
                                                    nr_upper == node_id_upper ||
                                                    // Family article contains catalog node id
                                                    nr_upper.contains(&node_id_upper) ||
                                                    // Catalog node id contains family article
                                                    node_id_upper.contains(&nr_upper)
                                                })
                                            });

                                            if let Some(family) = matching_family {
                                                if let Some(ref manufacturer) =
                                                    app.selected_manufacturer
                                                {
                                                    // Load properties for this family
                                                    let properties = engine.get_family_properties(
                                                        &manufacturer.id,
                                                        &family.id,
                                                    );
                                                    app.family_properties = properties.clone();
                                                    app.selected_family = Some(family.clone());

                                                    // Create configuration
                                                    let mut config = FamilyConfiguration::new(
                                                        &family.id,
                                                        &properties,
                                                    );

                                                    // Apply variant settings if node has a variant code
                                                    if let Some(ref variant_code) =
                                                        node.variant_code
                                                    {
                                                        if let Some(ref catalog) = app.catalog {
                                                            if let Some(variant_def) = catalog
                                                                .get_variant(&node.id, variant_code)
                                                            {
                                                                // Parse and apply property settings
                                                                // Format: "PROPERTYCLASS.PROPERTY=VALUE"
                                                                for setting in
                                                                    &variant_def.property_settings
                                                                {
                                                                    if let Some(eq_pos) =
                                                                        setting.rfind('=')
                                                                    {
                                                                        let prop_path =
                                                                            &setting[..eq_pos];
                                                                        let value =
                                                                            &setting[eq_pos + 1..];
                                                                        // Extract property name (after last dot)
                                                                        if let Some(dot_pos) =
                                                                            prop_path.rfind('.')
                                                                        {
                                                                            let prop_key =
                                                                                &prop_path
                                                                                    [dot_pos + 1..];
                                                                            config.set(
                                                                                prop_key, value,
                                                                            );
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }

                                                    app.family_config = Some(config.clone());

                                                    // Calculate price
                                                    app.family_price = engine
                                                        .calculate_family_price(
                                                            &manufacturer.id,
                                                            family,
                                                            &config,
                                                            app.price_date,
                                                        );

                                                    // Load packaging info
                                                    if let Some(pkg) = engine
                                                        .get_packaging_for_article(
                                                            &manufacturer.id,
                                                            &family.base_article_nr,
                                                        )
                                                    {
                                                        // Convert units to standard cm/kg/m³
                                                        let (w, d, h) = match pkg
                                                            .measure_unit
                                                            .to_lowercase()
                                                            .as_str()
                                                        {
                                                            "mm" => (
                                                                pkg.width / 10.0,
                                                                pkg.depth / 10.0,
                                                                pkg.height / 10.0,
                                                            ),
                                                            "m" => (
                                                                pkg.width * 100.0,
                                                                pkg.depth * 100.0,
                                                                pkg.height * 100.0,
                                                            ),
                                                            _ => (pkg.width, pkg.depth, pkg.height),
                                                        };
                                                        let weight = match pkg
                                                            .weight_unit
                                                            .to_lowercase()
                                                            .as_str()
                                                        {
                                                            "g" => pkg.net_weight / 1000.0,
                                                            _ => pkg.net_weight,
                                                        };
                                                        app.packaging_info = Some(PackagingInfo {
                                                            weight_kg: weight,
                                                            dimensions_cm: (w, d, h),
                                                            volume_m3: pkg.volume,
                                                        });
                                                    } else {
                                                        app.packaging_info = None;
                                                    }

                                                    // Load data version
                                                    app.data_version =
                                                        engine.get_data_version(&manufacturer.id);

                                                    // Check data validity
                                                    app.data_validity_warning = engine
                                                        .get_data_validity_warning(
                                                            &manufacturer.id,
                                                            app.price_date,
                                                        );

                                                    // Load variant code separator
                                                    app.varcode_separator =
                                                        engine.get_varcode_separator(&manufacturer.id);

                                                    // Load composite components if this is a composite product
                                                    let components = engine.get_composite_components(
                                                        &manufacturer.id,
                                                        &family.base_article_nr,
                                                    );
                                                    app.composite_components = components
                                                        .into_iter()
                                                        .map(|c| CompositeComponent {
                                                            item_id: c.item_id,
                                                            position: c.item_pos,
                                                            quantity: c.quantity,
                                                            description: String::new(),
                                                        })
                                                        .collect();

                                                    app.screen = Screen::FamilyConfig;

                                                    // Track in history
                                                    let _ = config_store::add_to_history(
                                                        &manufacturer.id,
                                                        &family.id,
                                                        &family.base_article_nr,
                                                        &family.name,
                                                    );

                                                    let price_str = app
                                                        .family_price
                                                        .as_ref()
                                                        .map(|p| {
                                                            format!(
                                                                "{:.2} {}",
                                                                p.base_price, p.currency
                                                            )
                                                        })
                                                        .unwrap_or_else(|| "Preis n/a".to_string());
                                                    app.status_message = Some(format!(
                                                        "{} - {}",
                                                        family.name, price_str
                                                    ));
                                                }
                                            } else {
                                                // No matching family found - try to create a minimal family from the catalog node
                                                // This allows viewing articles that aren't in the family list
                                                if let Some(ref manufacturer) =
                                                    app.selected_manufacturer
                                                {
                                                    // Get series from catalog node or path
                                                    let series = node
                                                        .series_ref
                                                        .clone()
                                                        .or_else(|| {
                                                            // Try to extract series from breadcrumb
                                                            app.catalog_path
                                                                .first()
                                                                .map(|s| s.id.to_lowercase())
                                                        })
                                                        .unwrap_or_else(|| node.id.clone());

                                                    // Create a minimal family for this article
                                                    let minimal_family = ProductFamily {
                                                        id: node.id.clone(),
                                                        name: node.name.clone(),
                                                        description: node.name.clone(),
                                                        long_description: String::new(),
                                                        series: series.clone(),
                                                        base_article_nr: node.id.clone(),
                                                        prop_classes: vec![],
                                                        variant_count: 1,
                                                        is_configurable: false,
                                                        article_nrs: vec![node.id.clone()],
                                                        article_descriptions: vec![node
                                                            .name
                                                            .clone()],
                                                        article_long_descriptions: vec![],
                                                    };

                                                    // Try to load properties using the series
                                                    let properties = engine.get_family_properties(
                                                        &manufacturer.id,
                                                        &series,
                                                    );
                                                    app.family_properties = properties.clone();
                                                    app.selected_family =
                                                        Some(minimal_family.clone());

                                                    // Create configuration
                                                    let config = FamilyConfiguration::new(
                                                        &minimal_family.id,
                                                        &properties,
                                                    );
                                                    app.family_config = Some(config.clone());

                                                    // Calculate price
                                                    app.family_price = engine
                                                        .calculate_family_price(
                                                            &manufacturer.id,
                                                            &minimal_family,
                                                            &config,
                                                            app.price_date,
                                                        );

                                                    // Load packaging info
                                                    if let Some(pkg) = engine
                                                        .get_packaging_for_article(
                                                            &manufacturer.id,
                                                            &node.id,
                                                        )
                                                    {
                                                        let (w, d, h) = match pkg
                                                            .measure_unit
                                                            .to_lowercase()
                                                            .as_str()
                                                        {
                                                            "mm" => (
                                                                pkg.width / 10.0,
                                                                pkg.depth / 10.0,
                                                                pkg.height / 10.0,
                                                            ),
                                                            "m" => (
                                                                pkg.width * 100.0,
                                                                pkg.depth * 100.0,
                                                                pkg.height * 100.0,
                                                            ),
                                                            _ => (pkg.width, pkg.depth, pkg.height),
                                                        };
                                                        let weight = match pkg
                                                            .weight_unit
                                                            .to_lowercase()
                                                            .as_str()
                                                        {
                                                            "g" => pkg.net_weight / 1000.0,
                                                            _ => pkg.net_weight,
                                                        };
                                                        app.packaging_info = Some(PackagingInfo {
                                                            weight_kg: weight,
                                                            dimensions_cm: (w, d, h),
                                                            volume_m3: pkg.volume,
                                                        });
                                                    } else {
                                                        app.packaging_info = None;
                                                    }

                                                    // Load data version
                                                    app.data_version =
                                                        engine.get_data_version(&manufacturer.id);

                                                    // Check data validity
                                                    app.data_validity_warning = engine
                                                        .get_data_validity_warning(
                                                            &manufacturer.id,
                                                            app.price_date,
                                                        );

                                                    // Load variant code separator
                                                    app.varcode_separator =
                                                        engine.get_varcode_separator(&manufacturer.id);

                                                    // Load composite components
                                                    let components = engine.get_composite_components(
                                                        &manufacturer.id,
                                                        &node.id,
                                                    );
                                                    app.composite_components = components
                                                        .into_iter()
                                                        .map(|c| CompositeComponent {
                                                            item_id: c.item_id,
                                                            position: c.item_pos,
                                                            quantity: c.quantity,
                                                            description: String::new(),
                                                        })
                                                        .collect();

                                                    app.screen = Screen::FamilyConfig;

                                                    // Track in history
                                                    let _ = config_store::add_to_history(
                                                        &manufacturer.id,
                                                        &minimal_family.id,
                                                        &minimal_family.base_article_nr,
                                                        &node.name,
                                                    );

                                                    let price_str = app
                                                        .family_price
                                                        .as_ref()
                                                        .map(|p| {
                                                            format!(
                                                                "{:.2} {}",
                                                                p.base_price, p.currency
                                                            )
                                                        })
                                                        .unwrap_or_else(|| "Preis n/a".to_string());
                                                    app.status_message = Some(format!(
                                                        "{} - {}",
                                                        node.name, price_str
                                                    ));
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                Some(Message::SelectCatalogNode(idx))
                            }
                            Screen::Families => {
                                // Load properties for the selected product family
                                if idx < app.families.len() {
                                    let family = &app.families[idx];

                                    if let Some(ref manufacturer) = app.selected_manufacturer {
                                        app.status_message =
                                            Some(format!("Lade {}...", family.name));
                                        let _ = terminal.draw(|f| render(f, app));

                                        // Load family properties
                                        let properties = engine
                                            .get_family_properties(&manufacturer.id, &family.id);
                                        app.family_properties = properties.clone();

                                        // Create configuration with default values
                                        let config =
                                            FamilyConfiguration::new(&family.id, &properties);
                                        app.family_config = Some(config.clone());

                                        // Calculate initial price
                                        app.family_price = engine.calculate_family_price(
                                            &manufacturer.id,
                                            family,
                                            &config,
                                            app.price_date,
                                        );

                                        // Load packaging info
                                        if let Some(pkg) = engine.get_packaging_for_article(
                                            &manufacturer.id,
                                            &family.base_article_nr,
                                        ) {
                                            // Convert units to standard cm/kg/m³
                                            let (w, d, h) = match pkg
                                                .measure_unit
                                                .to_lowercase()
                                                .as_str()
                                            {
                                                "mm" => (
                                                    pkg.width / 10.0,
                                                    pkg.depth / 10.0,
                                                    pkg.height / 10.0,
                                                ),
                                                "m" => (
                                                    pkg.width * 100.0,
                                                    pkg.depth * 100.0,
                                                    pkg.height * 100.0,
                                                ),
                                                _ => (pkg.width, pkg.depth, pkg.height),
                                            };
                                            let weight = match pkg
                                                .weight_unit
                                                .to_lowercase()
                                                .as_str()
                                            {
                                                "g" => pkg.net_weight / 1000.0,
                                                _ => pkg.net_weight,
                                            };
                                            app.packaging_info = Some(PackagingInfo {
                                                weight_kg: weight,
                                                dimensions_cm: (w, d, h),
                                                volume_m3: pkg.volume,
                                            });
                                        } else {
                                            app.packaging_info = None;
                                        }

                                        // Load data version
                                        app.data_version =
                                            engine.get_data_version(&manufacturer.id);

                                        // Check data validity
                                        app.data_validity_warning = engine
                                            .get_data_validity_warning(&manufacturer.id, app.price_date);

                                        // Load variant code separator
                                        app.varcode_separator =
                                            engine.get_varcode_separator(&manufacturer.id);

                                        // Load composite components
                                        let components = engine.get_composite_components(
                                            &manufacturer.id,
                                            &family.base_article_nr,
                                        );
                                        app.composite_components = components
                                            .into_iter()
                                            .map(|c| CompositeComponent {
                                                item_id: c.item_id,
                                                position: c.item_pos,
                                                quantity: c.quantity,
                                                description: String::new(),
                                            })
                                            .collect();

                                        let price_str = app
                                            .family_price
                                            .as_ref()
                                            .map(|p| format!("{:.2} {}", p.base_price, p.currency))
                                            .unwrap_or_else(|| "Preis n/a".to_string());

                                        // Track in history
                                        let _ = config_store::add_to_history(
                                            &manufacturer.id,
                                            &family.id,
                                            &family.base_article_nr,
                                            &family.name,
                                        );

                                        app.status_message = Some(format!(
                                            "{} Eigenschaften, {}",
                                            app.family_properties.len(),
                                            price_str
                                        ));
                                    }
                                }
                                Some(Message::SelectFamily(idx))
                            }
                            Screen::FamilyConfig => {
                                // Handle Enter in FamilyConfig when no properties (select article)
                                if app.family_properties.is_empty() {
                                    // Extract data first to avoid borrow issues
                                    let article_data = app.selected_family.as_ref().and_then(|f| {
                                        let idx = app.focused_article_index;
                                        f.article_nrs.get(idx).map(|nr| {
                                            let mut selected = f.clone();
                                            selected.base_article_nr = nr.clone();
                                            (nr.clone(), selected)
                                        })
                                    });

                                    if let Some((article_nr, selected_family)) = article_data {
                                        if let Some(ref manufacturer) = app.selected_manufacturer {
                                            // Recalculate price for the selected article
                                            if let Some(ref config) = app.family_config {
                                                app.family_price = engine.calculate_family_price(
                                                    &manufacturer.id,
                                                    &selected_family,
                                                    config,
                                                    app.price_date,
                                                );
                                            }

                                            // Update the selected family to reflect the chosen article
                                            app.selected_family = Some(selected_family);

                                            let price_str = app
                                                .family_price
                                                .as_ref()
                                                .map(|p| {
                                                    format!("{:.2} {}", p.base_price, p.currency)
                                                })
                                                .unwrap_or_else(|| "Preis n/a".to_string());

                                            app.status_message = Some(format!(
                                                "Artikel {} ausgewählt - {}",
                                                article_nr, price_str
                                            ));
                                        }
                                    }
                                }
                                None
                            }
                            Screen::Articles => {
                                // Load configuration for the selected article (legacy mode)
                                if idx < app.articles.len() {
                                    let article = &app.articles[idx];

                                    if let Some(ref manufacturer) = app.selected_manufacturer {
                                        app.status_message =
                                            Some(format!("Lade {}...", article.short_description));
                                        let _ = terminal.draw(|f| render(f, app));

                                        // Always look up base price from OCD first
                                        let pdata_files = ocd::find_pdata_files(Path::new(&manufacturer.path));
                                        let mut found_price = None;

                                        for pdata_path in &pdata_files {
                                            if let Ok(reader) =
                                                ocd::OcdReader::from_ebase(pdata_path)
                                            {
                                                if let Some(ocd_price) =
                                                    reader.get_base_price(&article.id)
                                                {
                                                    use chrono::NaiveDate;
                                                    use rust_decimal::Decimal;

                                                    let base_price =
                                                        Decimal::from_f32_retain(ocd_price.price)
                                                            .unwrap_or(Decimal::ZERO);

                                                    let price_date = NaiveDate::parse_from_str(
                                                        &ocd_price.date_from,
                                                        "%Y%m%d",
                                                    )
                                                    .unwrap_or(app.price_date);
                                                    let valid_to = NaiveDate::parse_from_str(
                                                        &ocd_price.date_to,
                                                        "%Y%m%d",
                                                    )
                                                    .ok();

                                                    found_price = Some(
                                                        ofml_lib::oap::PriceResult::new(
                                                            base_price,
                                                            vec![],
                                                            ocd_price.currency.clone(),
                                                            app.price_date,
                                                            price_date,
                                                            valid_to,
                                                        ),
                                                    );
                                                    break;
                                                }
                                            }
                                        }

                                        // Create base configuration
                                        let mut config =
                                            ofml_lib::oap::config::Configuration::new(
                                                article.id.clone(),
                                                manufacturer.id.clone(),
                                            );
                                        config.article_number =
                                            Some(article.base_article_number.clone());
                                        config.price = found_price;

                                        // Try to load CLS properties if configurable
                                        let status_msg = if article.has_configuration {
                                            match engine
                                                .load_configuration(&manufacturer.id, &article.id)
                                            {
                                                Ok(loaded_config) => {
                                                    // Merge properties from CLS
                                                    config.properties = loaded_config.properties;
                                                    let prop_count =
                                                        config.properties.definitions.len();

                                                    let price_str = config
                                                        .price
                                                        .as_ref()
                                                        .map(|p| {
                                                            format!(
                                                                "{:.2} {}",
                                                                p.base_price, p.currency
                                                            )
                                                        })
                                                        .unwrap_or_else(|| "Preis n/a".to_string());

                                                    if prop_count > 0 {
                                                        format!(
                                                            "{} Eigenschaften, {}",
                                                            prop_count, price_str
                                                        )
                                                    } else {
                                                        format!(
                                                            "Keine Eigenschaften, {}",
                                                            price_str
                                                        )
                                                    }
                                                }
                                                Err(_) => {
                                                    let price_str = config
                                                        .price
                                                        .as_ref()
                                                        .map(|p| {
                                                            format!(
                                                                "{:.2} {}",
                                                                p.base_price, p.currency
                                                            )
                                                        })
                                                        .unwrap_or_else(|| "Preis n/a".to_string());
                                                    format!("CLS nicht geladen, {}", price_str)
                                                }
                                            }
                                        } else {
                                            let price_str = config
                                                .price
                                                .as_ref()
                                                .map(|p| {
                                                    format!("{:.2} {}", p.base_price, p.currency)
                                                })
                                                .unwrap_or_else(|| {
                                                    "Preis nicht verfügbar".to_string()
                                                });
                                            format!("Nicht konfigurierbar, {}", price_str)
                                        };

                                        app.status_message = Some(status_msg);
                                        app.configuration = Some(config);
                                    }
                                }
                                Some(Message::SelectArticle(idx))
                            }
                            Screen::Tables => {
                                // Load table contents for selected table
                                if idx < app.tables.len() {
                                    let table_info = app.tables[idx].clone();
                                    app.status_message =
                                        Some(format!("Lade Tabelle {}...", table_info.name));
                                    let _ = terminal.draw(|f| render(f, app));

                                    // Load table rows
                                    app.table_rows =
                                        load_table_rows(&table_info.source_path, &table_info.name);
                                    app.table_row_list_state.select(Some(0));
                                    app.table_scroll_x = 0;
                                    app.status_message =
                                        Some(format!("{} Zeilen geladen", app.table_rows.len()));
                                }
                                Some(Message::SelectTable(idx))
                            }
                            Screen::SavedConfigs => {
                                // Load selected saved configuration
                                if idx < app.saved_configs.len() {
                                    let (_, saved_config) = &app.saved_configs[idx];
                                    let manufacturer = saved_config.manufacturer.clone();
                                    let series = saved_config.series.clone();
                                    let saved_properties = saved_config.properties.clone();

                                    // Find and load the manufacturer
                                    if let Some(mfr_idx) = app
                                        .manufacturers
                                        .iter()
                                        .position(|m| m.id == manufacturer)
                                    {
                                        let mfr = app.manufacturers[mfr_idx].clone();
                                        app.selected_manufacturer = Some(mfr.clone());

                                        // Load families
                                        let families = engine.load_families(&manufacturer);
                                        app.families = families.to_vec();

                                        // Find the matching family
                                        if let Some(family) = app
                                            .families
                                            .iter()
                                            .find(|f| f.id == series)
                                            .cloned()
                                        {
                                            app.selected_family = Some(family.clone());

                                            // Get family properties
                                            let properties =
                                                engine.get_family_properties(&manufacturer, &family.id);
                                            app.family_properties = properties.clone();

                                            // Create configuration with saved property values
                                            let mut fam_config =
                                                FamilyConfiguration::new(&family.id, &properties);

                                            // Apply saved property values
                                            for (key, value) in saved_properties {
                                                fam_config.set(&key, &value);
                                            }

                                            // Calculate price
                                            app.family_price = engine.calculate_family_price(
                                                &manufacturer,
                                                &family,
                                                &fam_config,
                                                app.price_date,
                                            );
                                            app.family_config = Some(fam_config);

                                            app.screen = Screen::FamilyConfig;

                                            // Track in history
                                            let _ = config_store::add_to_history(
                                                &manufacturer,
                                                &family.id,
                                                &family.base_article_nr,
                                                &family.name,
                                            );

                                            app.status_message =
                                                Some("Konfiguration geladen".to_string());
                                        } else {
                                            app.status_message =
                                                Some("Produktfamilie nicht gefunden".to_string());
                                        }
                                    } else {
                                        app.status_message =
                                            Some("Hersteller nicht gefunden".to_string());
                                    }
                                }
                                Some(Message::SelectSavedConfig(idx))
                            }
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
                KeyCode::Esc => {
                    if app.search_active {
                        Some(Message::ToggleSearch)
                    } else {
                        Some(Message::GoBack)
                    }
                }
                KeyCode::Char(c) if app.search_active => {
                    let mut query = app.search_query.clone();
                    query.push(c);
                    Some(Message::UpdateSearch(query))
                }
                KeyCode::Backspace if app.search_active => {
                    let mut query = app.search_query.clone();
                    query.pop();
                    Some(Message::UpdateSearch(query))
                }
                _ => None,
            };

            if let Some(m) = msg {
                app.update(m.clone());

                // Handle geometry export
                if matches!(m, Message::ExportGeometry) {
                    if let (Some(ref manufacturer), Some(ref family)) = (
                        &app.selected_manufacturer,
                        &app.selected_family,
                    ) {
                        app.status_message = Some("Exportiere Geometrie...".to_string());
                        let _ = terminal.draw(|f| render(f, app));

                        match export_family_geometry(Path::new(&manufacturer.path), &family.id, &family.base_article_nr) {
                            Ok(path) => {
                                app.status_message = Some(format!(
                                    "GLB exportiert: {}",
                                    path.file_name()
                                        .and_then(|s| s.to_str())
                                        .unwrap_or("export.glb")
                                ));
                            }
                            Err(e) => {
                                app.status_message = Some(format!("Export fehlgeschlagen: {}", e));
                            }
                        }
                    } else {
                        app.status_message = Some("Keine Familie ausgewählt".to_string());
                    }
                }

                // Handle price recalculation and property dependency updates on property change
                if matches!(m, Message::CyclePropertyOption(_)) {
                    if let (Some(ref manufacturer), Some(ref family), Some(ref config)) = (
                        &app.selected_manufacturer,
                        &app.selected_family,
                        &app.family_config,
                    ) {
                        // Refresh properties with current selections to update TABLE-dependent options
                        let updated_properties = engine.get_family_properties_with_selections(
                            &manufacturer.id,
                            &family.id,
                            &config.selections,
                        );
                        app.family_properties = updated_properties;

                        // Recalculate price
                        app.family_price = engine.calculate_family_price(
                            &manufacturer.id,
                            family,
                            config,
                            app.price_date,
                        );
                    }
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

/// Export geometry for a product family to a GLB file

/// Load all tables from manufacturer's pdata.ebase files
pub fn load_manufacturer_tables(manufacturer_path: &Path) -> Vec<TableInfo> {
    let mut all_tables: HashMap<String, TableInfo> = HashMap::new();

    let standard_tables: HashSet<&str> = [
        "ocd_article", "ocd_articletext", "ocd_artshorttext", "ocd_artlongtext",
        "ocd_price", "ocd_pricetext", "ocd_property", "ocd_propertyclass",
        "ocd_propertyvalue", "ocd_propertyvaluetext", "ocd_propvaluetext",
        "ocd_variantcondition", "ocd_relation", "ocd_relationobj",
        "ocd_propertygroup", "ocd_article2propgroup", "ocd_composite",
        "ocd_billofitems", "propvalue2varcond",
    ].into_iter().collect();

    fn find_ebase_files(path: &Path, files: &mut Vec<std::path::PathBuf>) {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    find_ebase_files(&p, files);
                } else if p.file_name().is_some_and(|n| n == "pdata.ebase") {
                    files.push(p);
                }
            }
        }
    }

    let mut ebase_files = Vec::new();
    find_ebase_files(manufacturer_path, &mut ebase_files);

    for ebase_path in ebase_files {
        if let Ok(reader) = EBaseReader::open(&ebase_path) {
            for table_name in reader.table_names() {
                if all_tables.contains_key(table_name) {
                    continue;
                }

                let is_standard = standard_tables.contains(table_name);
                let row_count = reader
                    .get_table(table_name)
                    .map(|t| t.record_count as usize)
                    .unwrap_or(0);
                let columns: Vec<String> = reader
                    .get_table(table_name)
                    .map(|t| t.columns.iter().map(|c| c.name.clone()).collect())
                    .unwrap_or_default();

                all_tables.insert(
                    table_name.to_string(),
                    TableInfo {
                        name: table_name.to_string(),
                        row_count,
                        columns,
                        is_standard,
                        source_path: ebase_path.to_string_lossy().to_string(),
                    },
                );
            }
        }
    }

    let mut result: Vec<TableInfo> = all_tables.into_values().collect();
    result.sort_by(|a, b| match (a.is_standard, b.is_standard) {
        (false, true) => std::cmp::Ordering::Less,
        (true, false) => std::cmp::Ordering::Greater,
        _ => a.name.cmp(&b.name),
    });

    result
}

/// Load rows from a specific table
pub fn load_table_rows(ebase_path: &str, table_name: &str) -> Vec<TableRow> {

    let mut rows = Vec::new();
    let path = Path::new(ebase_path);

    if let Ok(mut reader) = EBaseReader::open(path) {
        let columns: Vec<String> = reader
            .get_table(table_name)
            .map(|t| t.columns.iter().map(|c| c.name.clone()).collect())
            .unwrap_or_default();

        if let Ok(records) = reader.read_records(table_name, Some(500)) {
            for record in records.iter() {
                let values: Vec<String> = columns
                    .iter()
                    .map(|col| {
                        record
                            .get(col.as_str())
                            .map(value_to_string)
                            .unwrap_or_default()
                    })
                    .collect();
                rows.push(TableRow { values });
            }
        }
    }

    rows
}

fn value_to_string(v: &ofml_lib::ebase::Value) -> String {
    use ofml_lib::ebase::Value;
    match v {
        Value::Int(i) => i.to_string(),
        Value::UInt(u) => u.to_string(),
        Value::Float(f) => format!("{:.4}", f),
        Value::String(s) => s.clone(),
        Value::Blob(id) => format!("[blob:{}]", id),
        Value::Null => String::new(),
    }
}

/// Export family geometry to GLB file
pub fn export_family_geometry(
    manufacturer_path: &Path,
    series_id: &str,
    article_nr: &str,
) -> Result<std::path::PathBuf, String> {
    use ofml_lib::operations::{assemble_product, export_to_glb, ProductConfig};

    let series_path = manufacturer_path.join(series_id);
    if !series_path.exists() {
        return Err(format!("Series directory not found: {}", series_id));
    }

    let product_path = find_product_path(&series_path)?;

    let config = ProductConfig {
        article: Some(article_nr.to_string()),
        properties: std::collections::HashMap::new(),
    };

    let result = assemble_product(&product_path, &config)
        .map_err(|e| format!("Geometry assembly failed: {}", e))?;

    if result.geometry_loaded == 0 {
        return Err("No geometry found".to_string());
    }

    let glb_data = export_to_glb(&result.scene)
        .map_err(|e| format!("GLB export failed: {}", e))?;

    let output_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let safe_article = article_nr.replace([':', '/', '\\'], "_");
    let output_path = output_dir.join(format!("{}_{}.glb", series_id, safe_article));

    std::fs::write(&output_path, glb_data)
        .map_err(|e| format!("Could not write file: {}", e))?;

    Ok(output_path)
}

fn find_product_path(series_path: &Path) -> Result<std::path::PathBuf, String> {
    let candidates = [
        series_path.join("DE/1"),
        series_path.join("1"),
        series_path.to_path_buf(),
    ];

    for candidate in &candidates {
        if candidate.join("db/odb.ebase").exists() || has_alb_files(candidate) {
            return Ok(candidate.clone());
        }
    }

    Ok(series_path.to_path_buf())
}

fn has_alb_files(path: &Path) -> bool {
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Some(ext) = entry.path().extension() {
                if ext.to_string_lossy().to_lowercase() == "alb" {
                    return true;
                }
            }
        }
    }
    false
}
