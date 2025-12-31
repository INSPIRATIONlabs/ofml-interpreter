//! Unit tests for TUI search/filter functionality (T029a)

#[cfg(feature = "tui")]
mod search_tests {
    use ofml_interpreter::oap::Manufacturer;
    use ofml_interpreter::tui::{App, Message};
    use std::path::PathBuf;

    fn create_test_manufacturers() -> Vec<Manufacturer> {
        vec![
            Manufacturer {
                id: "vitra".into(),
                name: "Vitra AG".into(),
                path: PathBuf::new(),
            },
            Manufacturer {
                id: "sedus".into(),
                name: "Sedus Stoll AG".into(),
                path: PathBuf::new(),
            },
            Manufacturer {
                id: "vs".into(),
                name: "VS Vereinigte".into(),
                path: PathBuf::new(),
            },
        ]
    }

    #[test]
    fn test_search_mode_toggle() {
        let mut app = App::new("/path/to/data".into());
        assert!(!app.search_active);
        assert!(app.search_query.is_empty());

        app.update(Message::ToggleSearch);
        assert!(app.search_active);

        app.update(Message::ToggleSearch);
        assert!(!app.search_active);
    }

    #[test]
    fn test_search_input() {
        let mut app = App::new("/path/to/data".into());
        app.manufacturers = create_test_manufacturers();
        app.update(Message::ToggleSearch);

        // Simulate typing via UpdateSearch
        app.update(Message::UpdateSearch("v".into()));
        assert_eq!(app.search_query, "v");

        app.update(Message::UpdateSearch("vi".into()));
        assert_eq!(app.search_query, "vi");
    }

    #[test]
    fn test_search_clears_on_exit() {
        let mut app = App::new("/path/to/data".into());
        app.update(Message::ToggleSearch);
        app.search_query = "test".into();

        // Exit search mode
        app.update(Message::ToggleSearch);
        // Query should be cleared
        assert!(app.search_query.is_empty());
    }

    #[test]
    fn test_filter_updates_on_search() {
        let mut app = App::new("/path/to/data".into());
        app.manufacturers = create_test_manufacturers();

        // Search for "vitra"
        app.update(Message::UpdateSearch("vitra".into()));

        // Should have filtered indices
        assert_eq!(app.filtered_indices.len(), 1);
        assert_eq!(app.filtered_indices[0], 0); // vitra is at index 0
    }

    #[test]
    fn test_filter_case_insensitive() {
        let mut app = App::new("/path/to/data".into());
        app.manufacturers = create_test_manufacturers();

        // Search with uppercase
        app.update(Message::UpdateSearch("VITRA".into()));

        // Should still find it
        assert_eq!(app.filtered_indices.len(), 1);
    }

    #[test]
    fn test_filter_matches_name() {
        let mut app = App::new("/path/to/data".into());
        app.manufacturers = create_test_manufacturers();

        // Search by name
        app.update(Message::UpdateSearch("Stoll".into()));

        // Should find sedus
        assert_eq!(app.filtered_indices.len(), 1);
        assert_eq!(app.manufacturers[app.filtered_indices[0]].id, "sedus");
    }

    #[test]
    fn test_filter_no_match() {
        let mut app = App::new("/path/to/data".into());
        app.manufacturers = create_test_manufacturers();

        app.update(Message::UpdateSearch("nonexistent".into()));

        assert!(app.filtered_indices.is_empty());
    }

    #[test]
    fn test_filter_empty_query() {
        let mut app = App::new("/path/to/data".into());
        app.manufacturers = create_test_manufacturers();

        // Set and then clear query
        app.update(Message::UpdateSearch("vitra".into()));
        assert!(!app.filtered_indices.is_empty());

        app.update(Message::UpdateSearch("".into()));
        assert!(app.filtered_indices.is_empty());
    }

    #[test]
    fn test_selection_resets_on_filter() {
        let mut app = App::new("/path/to/data".into());
        app.manufacturers = create_test_manufacturers();
        app.manufacturer_list_state.select(Some(2));

        // Apply filter
        app.update(Message::UpdateSearch("v".into()));

        // Selection should reset to 0
        assert_eq!(app.manufacturer_list_state.selected(), Some(0));
    }
}
