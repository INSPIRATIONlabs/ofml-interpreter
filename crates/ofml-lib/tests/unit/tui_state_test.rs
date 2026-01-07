//! Unit tests for TUI app state transitions (T029)

#[cfg(feature = "tui")]
mod tui_tests {
    use ofml_lib::oap::Manufacturer;
    use ofml_lib::tui::{App, Message, Screen};
    use std::path::PathBuf;

    fn create_manufacturer(id: &str, name: &str) -> Manufacturer {
        Manufacturer {
            id: id.into(),
            name: name.into(),
            path: PathBuf::new(),
        }
    }

    #[test]
    fn test_app_initial_state() {
        let app = App::new("/path/to/data".into());
        assert!(matches!(app.screen, Screen::Manufacturers));
        assert!(!app.should_quit);
    }

    #[test]
    fn test_app_quit_message() {
        let mut app = App::new("/path/to/data".into());
        app.update(Message::Quit);
        assert!(app.should_quit);
    }

    #[test]
    fn test_app_navigate_down() {
        let mut app = App::new("/path/to/data".into());
        app.manufacturers = vec![
            create_manufacturer("vitra", "Vitra AG"),
            create_manufacturer("sedus", "Sedus"),
        ];

        assert_eq!(app.manufacturer_list_state.selected(), Some(0));
        app.update(Message::NavigateDown);
        assert_eq!(app.manufacturer_list_state.selected(), Some(1));
    }

    #[test]
    fn test_app_navigate_up() {
        let mut app = App::new("/path/to/data".into());
        app.manufacturers = vec![
            create_manufacturer("vitra", "Vitra AG"),
            create_manufacturer("sedus", "Sedus"),
        ];
        app.manufacturer_list_state.select(Some(1));

        app.update(Message::NavigateUp);
        assert_eq!(app.manufacturer_list_state.selected(), Some(0));
    }

    #[test]
    fn test_app_navigate_wraps() {
        let mut app = App::new("/path/to/data".into());
        app.manufacturers = vec![create_manufacturer("vitra", "Vitra AG")];
        app.manufacturer_list_state.select(Some(0));

        // Navigate down wraps to beginning
        app.update(Message::NavigateDown);
        assert_eq!(app.manufacturer_list_state.selected(), Some(0));

        // Navigate up from 0 wraps to end
        app.update(Message::NavigateUp);
        assert_eq!(app.manufacturer_list_state.selected(), Some(0));
    }

    #[test]
    fn test_app_show_help() {
        let mut app = App::new("/path/to/data".into());

        app.update(Message::ShowHelp);
        assert!(matches!(app.screen, Screen::Help));
    }

    #[test]
    fn test_app_toggle_search() {
        let mut app = App::new("/path/to/data".into());
        assert!(!app.search_active);

        app.update(Message::ToggleSearch);
        assert!(app.search_active);

        app.update(Message::ToggleSearch);
        assert!(!app.search_active);
    }

    #[test]
    fn test_app_go_back_from_articles() {
        let mut app = App::new("/path/to/data".into());
        app.screen = Screen::Articles;
        app.selected_manufacturer = Some(create_manufacturer("vitra", "Vitra AG"));

        app.update(Message::GoBack);
        assert!(matches!(app.screen, Screen::Manufacturers));
    }

    #[test]
    fn test_app_go_back_from_properties() {
        let mut app = App::new("/path/to/data".into());
        app.screen = Screen::Properties;

        app.update(Message::GoBack);
        assert!(matches!(app.screen, Screen::Articles));
    }

    #[test]
    fn test_app_go_back_from_manufacturers_quits() {
        let mut app = App::new("/path/to/data".into());
        app.screen = Screen::Manufacturers;

        app.update(Message::GoBack);
        assert!(app.should_quit);
    }

    #[test]
    fn test_screen_enum_variants() {
        // Verify all screen variants exist
        let _manufacturers = Screen::Manufacturers;
        let _articles = Screen::Articles;
        let _properties = Screen::Properties;
        let _help = Screen::Help;
    }

    #[test]
    fn test_message_enum_variants() {
        // Verify key message variants exist
        let _quit = Message::Quit;
        let _up = Message::NavigateUp;
        let _down = Message::NavigateDown;
        let _back = Message::GoBack;
        let _help = Message::ShowHelp;
        let _search = Message::ToggleSearch;
        let _export = Message::Export;
        let _resize = Message::Resize(80, 24);
    }

    #[test]
    fn test_new_message_variants() {
        // Verify new message variants for config/favorites/history features exist
        let _save_config = Message::SaveConfig;
        let _load_config = Message::LoadConfig;
        let _show_saved = Message::ShowSavedConfigs;
        let _select_saved = Message::SelectSavedConfig(0);
        let _toggle_fav = Message::ToggleFavorite;
    }

    #[test]
    fn test_screen_saved_configs_exists() {
        // Verify SavedConfigs screen variant exists
        let _saved_configs = Screen::SavedConfigs;
    }

    #[test]
    fn test_app_show_saved_configs() {
        let mut app = App::new("/path/to/data".into());

        app.update(Message::ShowSavedConfigs);
        assert!(matches!(app.screen, Screen::SavedConfigs));
    }

    #[test]
    fn test_app_go_back_from_saved_configs() {
        let mut app = App::new("/path/to/data".into());
        app.screen = Screen::SavedConfigs;

        app.update(Message::GoBack);
        // Should go back to manufacturers
        assert!(matches!(app.screen, Screen::Manufacturers));
    }
}

// Compile-time test that TUI types exist when feature is enabled
#[cfg(feature = "tui")]
#[test]
fn test_tui_types_exist() {
    use ofml_lib::tui::{App, Message, Screen};

    let _ = std::mem::size_of::<App>();
    let _ = std::mem::size_of::<Message>();
    let _ = std::mem::size_of::<Screen>();
}
