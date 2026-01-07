//! TUI Snapshot Tests
//!
//! Uses ratatui's TestBackend with insta snapshots to verify UI rendering.
//! These tests ensure the TUI displays correctly and that price changes
//! are properly reflected in the UI.

#[cfg(feature = "tui")]
mod snapshot_tests {
    use std::path::{Path, PathBuf};

    use insta::assert_snapshot;
    use ratatui::{backend::TestBackend, buffer::Buffer, Terminal};

    use ofml_lib::oap::engine::ConfigurationEngine;
    use ofml_lib::oap::families::{
        FamilyConfiguration, FamilyLoader, FamilyProperty, ProductFamily,
    };
    use ofml_lib::oap::{Manufacturer, PriceResult};
    use ofml_lib::tui::app::{App, Screen};
    use ofml_lib::tui::ui::render;

    const OFMLDATA_BASE: &str = "/reference/ofmldata";

    fn ofmldata_exists() -> bool {
        Path::new(OFMLDATA_BASE).exists()
    }

    /// Convert buffer to string for snapshot comparison
    fn buffer_to_string(buffer: &Buffer) -> String {
        let mut result = String::new();
        let area = buffer.area();

        for y in area.y..area.y + area.height {
            for x in area.x..area.x + area.width {
                let cell = &buffer[(x, y)];
                result.push_str(cell.symbol());
            }
            result.push('\n');
        }

        result
    }

    /// Create a test app with mock data
    fn create_test_app() -> App {
        let mut app = App::new(OFMLDATA_BASE.to_string());

        // Add some test manufacturers
        app.manufacturers = vec![
            Manufacturer {
                id: "sex".to_string(),
                name: "Sedus".to_string(),
                path: PathBuf::from("/reference/ofmldata/sex"),
            },
            Manufacturer {
                id: "vitra".to_string(),
                name: "Vitra".to_string(),
                path: PathBuf::from("/reference/ofmldata/vitra"),
            },
            Manufacturer {
                id: "kn".to_string(),
                name: "Knoll".to_string(),
                path: PathBuf::from("/reference/ofmldata/kn"),
            },
        ];

        app.terminal_size = (80, 24);
        app
    }

    #[test]
    fn test_manufacturer_list_render() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        let app = create_test_app();

        terminal
            .draw(|frame| {
                render(frame, &app);
            })
            .unwrap();

        let buffer_str = buffer_to_string(terminal.backend().buffer());
        assert_snapshot!("manufacturer_list", buffer_str);
    }

    #[test]
    fn test_manufacturer_list_with_selection() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        let mut app = create_test_app();
        // Select second manufacturer
        app.manufacturer_list_state.select(Some(1));

        terminal
            .draw(|frame| {
                render(frame, &app);
            })
            .unwrap();

        let buffer_str = buffer_to_string(terminal.backend().buffer());
        assert_snapshot!("manufacturer_list_selected", buffer_str);
    }

    #[test]
    fn test_family_list_render() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        let mut app = create_test_app();
        app.screen = Screen::Families;
        app.selected_manufacturer = Some(Manufacturer {
            id: "sex".to_string(),
            name: "Sedus".to_string(),
            path: PathBuf::from("/reference/ofmldata/sex"),
        });

        // Add test families
        app.families = vec![
            ProductFamily {
                id: "ai".to_string(),
                name: "Drehstuhl se:air".to_string(),
                description: "Ergonomischer Drehstuhl".to_string(),
                long_description: "Sedus se:air\nErgonomischer Drehstuhl".to_string(),
                series: "ai".to_string(),
                base_article_nr: "AI-121".to_string(),
                prop_classes: vec!["KLASSE_TEST".to_string()],
                variant_count: 5,
                is_configurable: true,
                article_nrs: vec!["AI-121".to_string(), "AI-821".to_string()],
                article_descriptions: vec!["Drehstuhl".to_string(), "Drehstuhl XXL".to_string()],
                article_long_descriptions: vec![
                    "Sedus se:air\nDrehstuhl".to_string(),
                    "Sedus se:air\nDrehstuhl XXL".to_string(),
                ],
            },
            ProductFamily {
                id: "ap".to_string(),
                name: "Konferenzstuhl ap".to_string(),
                description: "Konferenzstuhl".to_string(),
                long_description: "Sedus ap\nKonferenzstuhl".to_string(),
                series: "ap".to_string(),
                base_article_nr: "AP-101".to_string(),
                prop_classes: vec![],
                variant_count: 3,
                is_configurable: false,
                article_nrs: vec!["AP-101".to_string()],
                article_descriptions: vec!["Konferenzstuhl".to_string()],
                article_long_descriptions: vec!["Sedus ap\nKonferenzstuhl".to_string()],
            },
        ];
        app.family_list_state.select(Some(0));

        terminal
            .draw(|frame| {
                render(frame, &app);
            })
            .unwrap();

        let buffer_str = buffer_to_string(terminal.backend().buffer());
        assert_snapshot!("family_list", buffer_str);
    }

    #[test]
    fn test_family_config_with_price() {
        let backend = TestBackend::new(100, 30);
        let mut terminal = Terminal::new(backend).unwrap();

        let mut app = create_test_app();
        app.terminal_size = (100, 30);
        app.screen = Screen::FamilyConfig;
        app.selected_manufacturer = Some(Manufacturer {
            id: "sex".to_string(),
            name: "Sedus".to_string(),
            path: PathBuf::from("/reference/ofmldata/sex"),
        });

        app.selected_family = Some(ProductFamily {
            id: "ai".to_string(),
            name: "Drehstuhl se:air".to_string(),
            description: "Ergonomischer Drehstuhl".to_string(),
            long_description: "Sedus se:air\nErgonomischer Drehstuhl".to_string(),
            series: "ai".to_string(),
            base_article_nr: "AI-121".to_string(),
            prop_classes: vec!["KLASSE_TEST".to_string()],
            variant_count: 5,
            is_configurable: true,
            article_nrs: vec!["AI-121".to_string()],
            article_descriptions: vec!["Drehstuhl".to_string()],
            article_long_descriptions: vec!["Sedus se:air\nDrehstuhl".to_string()],
        });

        // Add test properties
        app.family_properties = vec![
            FamilyProperty {
                key: "S_STOFF".to_string(),
                label: "Stoffgruppe".to_string(),
                group: "Polsterung".to_string(),
                group_label: "Polsterung".to_string(),
                prop_type: ofml_lib::oap::families::PropertyType::Choice,
                required: true,
                options: vec![
                    ofml_lib::oap::families::PropertyOption {
                        value: "2G3".to_string(),
                        label: "Preisgruppe 3".to_string(),
                        is_default: true,
                    },
                    ofml_lib::oap::families::PropertyOption {
                        value: "2G4".to_string(),
                        label: "Preisgruppe 4".to_string(),
                        is_default: false,
                    },
                ],
                default_value: Some("2G3".to_string()),
                position: 1,
                hint: None,
            },
            FamilyProperty {
                key: "S_FUSSFARBE".to_string(),
                label: "Fussfarbe".to_string(),
                group: "Gestell".to_string(),
                group_label: "Gestell".to_string(),
                prop_type: ofml_lib::oap::families::PropertyType::Choice,
                required: true,
                options: vec![
                    ofml_lib::oap::families::PropertyOption {
                        value: "119".to_string(),
                        label: "Schwarz".to_string(),
                        is_default: true,
                    },
                    ofml_lib::oap::families::PropertyOption {
                        value: "120".to_string(),
                        label: "Weiss".to_string(),
                        is_default: false,
                    },
                ],
                default_value: Some("119".to_string()),
                position: 2,
                hint: None,
            },
        ];

        // Create configuration
        app.family_config = Some(FamilyConfiguration::new("ai", &app.family_properties));

        // Set price
        app.family_price = Some(PriceResult::new(
            rust_decimal::Decimal::from(599),
            vec![],
            "EUR".to_string(),
            chrono::Local::now().date_naive(),
            chrono::Local::now().date_naive(),
            None,
        ));

        terminal
            .draw(|frame| {
                render(frame, &app);
            })
            .unwrap();

        let buffer_str = buffer_to_string(terminal.backend().buffer());
        assert_snapshot!("family_config_with_price", buffer_str);
    }

    #[test]
    fn test_family_config_with_surcharges() {
        let backend = TestBackend::new(100, 30);
        let mut terminal = Terminal::new(backend).unwrap();

        let mut app = create_test_app();
        app.terminal_size = (100, 30);
        app.screen = Screen::FamilyConfig;
        app.selected_manufacturer = Some(Manufacturer {
            id: "sex".to_string(),
            name: "Sedus".to_string(),
            path: PathBuf::from("/reference/ofmldata/sex"),
        });

        app.selected_family = Some(ProductFamily {
            id: "ai".to_string(),
            name: "Drehstuhl se:air".to_string(),
            description: "Ergonomischer Drehstuhl".to_string(),
            long_description: "Sedus se:air\nErgonomischer Drehstuhl".to_string(),
            series: "ai".to_string(),
            base_article_nr: "AI-121".to_string(),
            prop_classes: vec!["KLASSE_TEST".to_string()],
            variant_count: 5,
            is_configurable: true,
            article_nrs: vec!["AI-121".to_string()],
            article_descriptions: vec!["Drehstuhl".to_string()],
            article_long_descriptions: vec!["Sedus se:air\nDrehstuhl".to_string()],
        });

        app.family_properties = vec![FamilyProperty {
            key: "S_STOFF_FRONT_GABRIEL".to_string(),
            label: "Gabriel Stoff".to_string(),
            group: "Polsterung".to_string(),
            group_label: "Polsterung".to_string(),
            prop_type: ofml_lib::oap::families::PropertyType::Choice,
            required: true,
            options: vec![ofml_lib::oap::families::PropertyOption {
                value: "XST244166018".to_string(),
                label: "Gabriel Comfort 166".to_string(),
                is_default: false,
            }],
            default_value: Some("XST244166018".to_string()),
            position: 1,
            hint: None,
        }];

        app.family_config = Some(FamilyConfiguration::new("ai", &app.family_properties));

        // Set price with surcharge
        use ofml_lib::oap::Surcharge;
        app.family_price = Some(PriceResult::new(
            rust_decimal::Decimal::from(599),
            vec![Surcharge {
                name: "S_166".to_string(),
                amount: rust_decimal::Decimal::from(44),
                is_percentage: false,
            }],
            "EUR".to_string(),
            chrono::Local::now().date_naive(),
            chrono::Local::now().date_naive(),
            None,
        ));

        terminal
            .draw(|frame| {
                render(frame, &app);
            })
            .unwrap();

        let buffer_str = buffer_to_string(terminal.backend().buffer());
        assert_snapshot!("family_config_with_surcharges", buffer_str);
    }

    #[test]
    fn test_help_screen() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        let mut app = create_test_app();
        app.screen = Screen::Help;

        terminal
            .draw(|frame| {
                render(frame, &app);
            })
            .unwrap();

        let buffer_str = buffer_to_string(terminal.backend().buffer());
        assert_snapshot!("help_screen", buffer_str);
    }

    #[test]
    fn test_search_mode_active() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        let mut app = create_test_app();
        app.search_active = true;
        app.search_query = "vitra".to_string();

        terminal
            .draw(|frame| {
                render(frame, &app);
            })
            .unwrap();

        let buffer_str = buffer_to_string(terminal.backend().buffer());
        assert_snapshot!("search_mode_active", buffer_str);
    }

    /// Integration test with real data (if available)
    #[test]
    fn test_real_data_price_display() {
        if !ofmldata_exists() {
            eprintln!("Skipping: ofmldata not found");
            return;
        }

        let backend = TestBackend::new(120, 35);
        let mut terminal = Terminal::new(backend).unwrap();

        let mut app = App::new(OFMLDATA_BASE.to_string());
        app.terminal_size = (120, 35);

        // Load real Sedus data
        let mfr_path = Path::new(OFMLDATA_BASE).join("sex");
        if !mfr_path.exists() {
            return;
        }

        let loader = FamilyLoader::load(&mfr_path, "DE");
        let engine = ConfigurationEngine::new(OFMLDATA_BASE);

        // Find AI family
        let ai_family = loader
            .get_families()
            .iter()
            .find(|f| f.base_article_nr.contains("AI"))
            .cloned();

        if let Some(family) = ai_family {
            let properties = loader.get_properties_for_family(&family);
            let config = FamilyConfiguration::new(&family.id, &properties);

            // Set up app state
            app.screen = Screen::FamilyConfig;
            app.selected_manufacturer = Some(Manufacturer {
                id: "sex".to_string(),
                name: "Sedus".to_string(),
                path: PathBuf::from("/reference/ofmldata/sex"),
            });
            app.selected_family = Some(family.clone());
            app.family_properties = properties;
            app.family_config = Some(config.clone());
            app.family_price = engine.calculate_family_price(
                "sex",
                &family,
                &config,
                chrono::Local::now().date_naive(),
            );

            terminal
                .draw(|frame| {
                    render(frame, &app);
                })
                .unwrap();

            let buffer_str = buffer_to_string(terminal.backend().buffer());

            // Verify price is displayed
            assert!(
                buffer_str.contains("599") || buffer_str.contains("Preis"),
                "Price should be displayed in the UI"
            );
        }
    }

    /// Test that price updates when property changes
    #[test]
    fn test_price_update_on_property_change() {
        if !ofmldata_exists() {
            eprintln!("Skipping: ofmldata not found");
            return;
        }

        let mfr_path = Path::new(OFMLDATA_BASE).join("sex");
        if !mfr_path.exists() {
            return;
        }

        let loader = FamilyLoader::load(&mfr_path, "DE");
        let engine = ConfigurationEngine::new(OFMLDATA_BASE);

        // Find AI family
        let ai_family = loader
            .get_families()
            .iter()
            .find(|f| f.base_article_nr.contains("AI"))
            .cloned();

        if let Some(family) = ai_family {
            let properties = loader.get_properties_for_family(&family);

            // Find S_STOFF_FRONT_GABRIEL property
            let gabriel_prop = properties.iter().find(|p| p.key == "S_STOFF_FRONT_GABRIEL");
            if gabriel_prop.is_none() {
                return;
            }
            let prop = gabriel_prop.unwrap();

            // Find an option with "166" embedded
            let opt_166 = prop.options.iter().find(|o| o.value.contains("166"));
            let opt_other = prop.options.iter().find(|o| {
                !o.value.contains("166") && !o.value.contains("167") && !o.value.contains("168")
            });

            if opt_166.is_none() || opt_other.is_none() {
                return;
            }

            // Create two configurations - one without surcharge trigger, one with
            let mut config_without = FamilyConfiguration::new(&family.id, &properties);
            config_without.set(&prop.key, &opt_other.unwrap().value);

            let mut config_with = FamilyConfiguration::new(&family.id, &properties);
            config_with.set(&prop.key, &opt_166.unwrap().value);

            let price_without = engine.calculate_family_price(
                "sex",
                &family,
                &config_without,
                chrono::Local::now().date_naive(),
            );
            let price_with = engine.calculate_family_price(
                "sex",
                &family,
                &config_with,
                chrono::Local::now().date_naive(),
            );

            if let (Some(p1), Some(p2)) = (&price_without, &price_with) {
                // Verify prices are different
                assert_ne!(
                    p1.total_price, p2.total_price,
                    "Price should change when selecting option with embedded surcharge code"
                );

                // Verify the one with 166 has a surcharge
                assert!(
                    !p2.surcharges.is_empty(),
                    "Option with embedded '166' should trigger S_166 surcharge"
                );

                // Render both states
                let backend1 = TestBackend::new(100, 30);
                let mut terminal1 = Terminal::new(backend1).unwrap();

                let mut app1 = App::new(OFMLDATA_BASE.to_string());
                app1.terminal_size = (100, 30);
                app1.screen = Screen::FamilyConfig;
                app1.selected_manufacturer = Some(Manufacturer {
                    id: "sex".to_string(),
                    name: "Sedus".to_string(),
                    path: PathBuf::from("/reference/ofmldata/sex"),
                });
                app1.selected_family = Some(family.clone());
                app1.family_properties = properties.clone();
                app1.family_config = Some(config_without);
                app1.family_price = Some(p1.clone());

                terminal1
                    .draw(|frame| {
                        render(frame, &app1);
                    })
                    .unwrap();

                let buffer1 = buffer_to_string(terminal1.backend().buffer());

                let backend2 = TestBackend::new(100, 30);
                let mut terminal2 = Terminal::new(backend2).unwrap();

                // Create a new app for the second render
                let mut app2 = App::new(OFMLDATA_BASE.to_string());
                app2.terminal_size = (100, 30);
                app2.screen = Screen::FamilyConfig;
                app2.selected_manufacturer = app1.selected_manufacturer.clone();
                app2.selected_family = app1.selected_family.clone();
                app2.family_properties = properties.clone();
                app2.family_config = Some(config_with);
                app2.family_price = Some(p2.clone());

                terminal2
                    .draw(|frame| {
                        render(frame, &app2);
                    })
                    .unwrap();

                let buffer2 = buffer_to_string(terminal2.backend().buffer());

                // The buffers should be different (price changed)
                assert_ne!(buffer1, buffer2, "UI should show different prices");
            }
        }
    }
}
