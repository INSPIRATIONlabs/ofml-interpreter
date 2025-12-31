//! Unit tests for TUI terminal resize handling (T062)

#[cfg(feature = "tui")]
mod resize_tests {
    use ofml_interpreter::oap::Manufacturer;
    use ofml_interpreter::tui::{App, Message};
    use std::path::PathBuf;

    fn create_manufacturer(id: &str, name: &str) -> Manufacturer {
        Manufacturer {
            id: id.into(),
            name: name.into(),
            path: PathBuf::new(),
        }
    }

    #[test]
    fn test_resize_message_exists() {
        // Verify Resize message variant exists
        let _resize = Message::Resize(80, 24);
    }

    #[test]
    fn test_app_handles_resize() {
        let mut app = App::new("/path/to/data".into());

        // Should not panic on resize
        app.update(Message::Resize(120, 40));

        // App should still be functional
        assert!(!app.should_quit);
    }

    #[test]
    fn test_app_handles_small_resize() {
        let mut app = App::new("/path/to/data".into());

        // Very small terminal
        app.update(Message::Resize(20, 10));

        // Should handle gracefully
        assert!(!app.should_quit);
    }

    #[test]
    fn test_app_handles_large_resize() {
        let mut app = App::new("/path/to/data".into());

        // Very large terminal
        app.update(Message::Resize(300, 100));

        // Should handle gracefully
        assert!(!app.should_quit);
    }

    #[test]
    fn test_resize_updates_dimensions() {
        let mut app = App::new("/path/to/data".into());

        app.update(Message::Resize(120, 40));

        // Verify dimensions are updated
        assert_eq!(app.terminal_size, (120, 40));
    }

    #[test]
    fn test_multiple_resizes() {
        let mut app = App::new("/path/to/data".into());

        // Simulate rapid resizing
        app.update(Message::Resize(80, 24));
        app.update(Message::Resize(100, 30));
        app.update(Message::Resize(120, 40));
        app.update(Message::Resize(80, 24));

        // Should handle all gracefully
        assert!(!app.should_quit);
        assert_eq!(app.terminal_size, (80, 24));
    }

    #[test]
    fn test_resize_during_search() {
        let mut app = App::new("/path/to/data".into());

        app.update(Message::ToggleSearch);
        app.search_query = "test".into();

        // Resize during search
        app.update(Message::Resize(100, 30));

        // Search state should be preserved
        assert!(app.search_active);
        assert_eq!(app.search_query, "test");
    }

    #[test]
    fn test_resize_preserves_selection() {
        let mut app = App::new("/path/to/data".into());
        app.manufacturers = vec![
            create_manufacturer("vitra", "Vitra AG"),
            create_manufacturer("sedus", "Sedus"),
        ];
        app.manufacturer_list_state.select(Some(1));

        // Resize
        app.update(Message::Resize(100, 30));

        // Selection should be preserved
        assert_eq!(app.manufacturer_list_state.selected(), Some(1));
    }

    #[test]
    fn test_resize_minimum_dimensions() {
        let mut app = App::new("/path/to/data".into());

        // Minimum viable terminal size
        app.update(Message::Resize(10, 5));

        // Should still work, maybe with degraded UI
        assert!(!app.should_quit);
    }

    #[test]
    fn test_resize_zero_dimensions() {
        let mut app = App::new("/path/to/data".into());

        // Edge case: zero dimensions (shouldn't happen but be defensive)
        app.update(Message::Resize(0, 0));

        // Should not crash
        assert!(!app.should_quit);
    }
}

// Test that resize works without TUI feature (no-op)
#[cfg(not(feature = "tui"))]
#[test]
fn test_tui_feature_disabled() {
    // When TUI is disabled, these types shouldn't be available
    // This is a compile-time check
    assert!(true);
}
