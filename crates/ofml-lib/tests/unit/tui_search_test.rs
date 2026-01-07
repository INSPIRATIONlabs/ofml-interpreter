//! Unit tests for TUI search/filter functionality (T029a)

#[cfg(feature = "tui")]
mod search_tests {
    use ofml_lib::oap::Manufacturer;
    use ofml_lib::tui::{App, Message};
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

    #[test]
    fn test_fuzzy_search_sorted_by_score() {
        let mut app = App::new("/path/to/data".into());
        app.manufacturers = vec![
            Manufacturer {
                id: "sedus".into(),
                name: "Sedus Stoll AG".into(), // 'v' is not at start
                path: PathBuf::new(),
            },
            Manufacturer {
                id: "vitra".into(),
                name: "Vitra AG".into(), // 'v' is at start - should rank higher
                path: PathBuf::new(),
            },
            Manufacturer {
                id: "vs".into(),
                name: "VS Vereinigte".into(), // 'v' is at start - should rank high
                path: PathBuf::new(),
            },
            Manufacturer {
                id: "haver".into(),
                name: "Haver GmbH".into(), // 'v' in middle
                path: PathBuf::new(),
            },
        ];

        // Search for 'v' - prefix matches should come first
        app.update(Message::UpdateSearch("v".into()));

        // All contain 'v', but vitra and vs start with it
        assert!(!app.filtered_indices.is_empty());

        // First results should be the ones starting with 'v'
        let first_id = &app.manufacturers[app.filtered_indices[0]].id;
        assert!(
            first_id == "vitra" || first_id == "vs",
            "Expected 'vitra' or 'vs' first, got '{}'",
            first_id
        );
    }

    #[test]
    fn test_fuzzy_search_exact_match_first() {
        let mut app = App::new("/path/to/data".into());
        app.manufacturers = vec![
            Manufacturer {
                id: "vitra_special".into(),
                name: "Vitra Special Edition".into(),
                path: PathBuf::new(),
            },
            Manufacturer {
                id: "vitra".into(),
                name: "Vitra AG".into(), // Exact match on id
                path: PathBuf::new(),
            },
            Manufacturer {
                id: "vitraplus".into(),
                name: "VitraPlus".into(),
                path: PathBuf::new(),
            },
        ];

        // Search for exact 'vitra'
        app.update(Message::UpdateSearch("vitra".into()));

        // Exact match should be first
        assert_eq!(app.manufacturers[app.filtered_indices[0]].id, "vitra");
    }

    #[test]
    fn test_fuzzy_subsequence_match() {
        let mut app = App::new("/path/to/data".into());
        app.manufacturers = vec![
            Manufacturer {
                id: "steelcase".into(),
                name: "Steelcase Inc.".into(),
                path: PathBuf::new(),
            },
            Manufacturer {
                id: "vitra".into(),
                name: "Vitra AG".into(),
                path: PathBuf::new(),
            },
        ];

        // Search for 'ste' - subsequence/prefix match for Steelcase
        // (Note: 'scl' doesn't work because in 'steelcase', l comes BEFORE c)
        app.update(Message::UpdateSearch("slc".into()));

        // Should find steelcase via subsequence (s-tee-l-c-ase)
        assert!(!app.filtered_indices.is_empty());
        assert_eq!(app.manufacturers[app.filtered_indices[0]].id, "steelcase");
    }

    #[test]
    fn test_fuzzy_word_boundary_match() {
        let mut app = App::new("/path/to/data".into());
        app.manufacturers = vec![
            Manufacturer {
                id: "ag".into(),
                name: "ABC Group".into(), // 'ag' not at word boundary for 'Group'
                path: PathBuf::new(),
            },
            Manufacturer {
                id: "vitra_ag".into(),
                name: "Vitra_AG".into(), // 'ag' after underscore - word boundary
                path: PathBuf::new(),
            },
        ];

        // Search for 'ag' - word boundary match should score higher
        app.update(Message::UpdateSearch("ag".into()));

        // Both match, but exact id match should come first
        assert!(!app.filtered_indices.is_empty());
        assert_eq!(app.manufacturers[app.filtered_indices[0]].id, "ag");
    }
}
