//! Unit tests for TUI property form validation (T040)

#[cfg(feature = "tui")]
mod form_tests {
    use ofml_interpreter::property::{PropertyDef, PropertyState, PropertyType, PropertyValue};
    use ofml_interpreter::tui::widgets::form::PropertyFormState;

    fn create_int_property() -> PropertyDef {
        PropertyDef {
            name: "height".to_string(),
            label: "Höhe".to_string(),
            prop_type: PropertyType::Int {
                min: Some(620),
                max: Some(820),
            },
            state: PropertyState::Enabled,
            sort_order: 1,
            description: None,
            category: None,
        }
    }

    fn create_float_property() -> PropertyDef {
        PropertyDef {
            name: "width".to_string(),
            label: "Breite".to_string(),
            prop_type: PropertyType::Float {
                min: Some(0.5),
                max: Some(2.5),
            },
            state: PropertyState::Enabled,
            sort_order: 2,
            description: None,
            category: None,
        }
    }

    fn create_choice_property() -> PropertyDef {
        PropertyDef {
            name: "color".to_string(),
            label: "Farbe".to_string(),
            prop_type: PropertyType::Choice {
                options: vec!["white".to_string(), "black".to_string(), "oak".to_string()],
            },
            state: PropertyState::Enabled,
            sort_order: 3,
            description: None,
            category: None,
        }
    }

    fn create_bool_property() -> PropertyDef {
        PropertyDef {
            name: "enabled".to_string(),
            label: "Aktiviert".to_string(),
            prop_type: PropertyType::Bool,
            state: PropertyState::Enabled,
            sort_order: 4,
            description: None,
            category: None,
        }
    }

    #[test]
    fn test_form_state_default() {
        let state = PropertyFormState::default();
        assert!(state.input.is_empty());
        assert_eq!(state.cursor, 0);
        assert!(!state.editing);
        assert!(state.error.is_none());
    }

    #[test]
    fn test_form_state_from_int_value() {
        let state = PropertyFormState::new(&PropertyValue::Int(720));
        assert_eq!(state.input, "720");
        assert_eq!(state.cursor, 3);
    }

    #[test]
    fn test_form_state_from_float_value() {
        let state = PropertyFormState::new(&PropertyValue::Float(1.5));
        assert_eq!(state.input, "1.5");
    }

    #[test]
    fn test_form_state_from_bool_value() {
        let state = PropertyFormState::new(&PropertyValue::Bool(true));
        assert_eq!(state.input, "ja");

        let state = PropertyFormState::new(&PropertyValue::Bool(false));
        assert_eq!(state.input, "nein");
    }

    #[test]
    fn test_form_state_from_string_value() {
        let state = PropertyFormState::new(&PropertyValue::String("test".to_string()));
        assert_eq!(state.input, "test");
    }

    #[test]
    fn test_form_state_from_symbol_value() {
        let state = PropertyFormState::new(&PropertyValue::Symbol("white".to_string()));
        assert_eq!(state.input, "white");
    }

    #[test]
    fn test_form_insert_character() {
        let mut state = PropertyFormState::default();
        state.insert('7');
        state.insert('2');
        state.insert('0');
        assert_eq!(state.input, "720");
        assert_eq!(state.cursor, 3);
    }

    #[test]
    fn test_form_backspace() {
        let mut state = PropertyFormState::default();
        state.input = "720".to_string();
        state.cursor = 3;

        state.backspace();
        assert_eq!(state.input, "72");
        assert_eq!(state.cursor, 2);
    }

    #[test]
    fn test_form_backspace_at_start() {
        let mut state = PropertyFormState::default();
        state.input = "720".to_string();
        state.cursor = 0;

        state.backspace();
        // Should not change anything
        assert_eq!(state.input, "720");
        assert_eq!(state.cursor, 0);
    }

    #[test]
    fn test_form_delete() {
        let mut state = PropertyFormState::default();
        state.input = "720".to_string();
        state.cursor = 0;

        state.delete();
        assert_eq!(state.input, "20");
        assert_eq!(state.cursor, 0);
    }

    #[test]
    fn test_form_delete_at_end() {
        let mut state = PropertyFormState::default();
        state.input = "720".to_string();
        state.cursor = 3;

        state.delete();
        // Should not change anything
        assert_eq!(state.input, "720");
    }

    #[test]
    fn test_form_cursor_movement() {
        let mut state = PropertyFormState::default();
        state.input = "720".to_string();
        state.cursor = 1;

        state.left();
        assert_eq!(state.cursor, 0);

        state.left();
        assert_eq!(state.cursor, 0); // Can't go below 0

        state.right();
        assert_eq!(state.cursor, 1);

        state.right();
        state.right();
        assert_eq!(state.cursor, 3);

        state.right();
        assert_eq!(state.cursor, 3); // Can't exceed length
    }

    #[test]
    fn test_validate_int_valid() {
        let def = create_int_property();
        let mut state = PropertyFormState::default();
        state.input = "720".to_string();

        let result = state.validate(&def);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PropertyValue::Int(720));
    }

    #[test]
    fn test_validate_int_below_min() {
        let def = create_int_property();
        let mut state = PropertyFormState::default();
        state.input = "500".to_string();

        let result = state.validate(&def);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("mindestens"));
    }

    #[test]
    fn test_validate_int_above_max() {
        let def = create_int_property();
        let mut state = PropertyFormState::default();
        state.input = "900".to_string();

        let result = state.validate(&def);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("höchstens"));
    }

    #[test]
    fn test_validate_int_invalid_format() {
        let def = create_int_property();
        let mut state = PropertyFormState::default();
        state.input = "abc".to_string();

        let result = state.validate(&def);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Ungültige Ganzzahl"));
    }

    #[test]
    fn test_validate_float_valid() {
        let def = create_float_property();
        let mut state = PropertyFormState::default();
        state.input = "1.5".to_string();

        let result = state.validate(&def);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PropertyValue::Float(1.5));
    }

    #[test]
    fn test_validate_float_below_min() {
        let def = create_float_property();
        let mut state = PropertyFormState::default();
        state.input = "0.1".to_string();

        let result = state.validate(&def);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_choice_valid() {
        let def = create_choice_property();
        let mut state = PropertyFormState::default();
        state.input = "white".to_string();

        let result = state.validate(&def);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PropertyValue::Symbol("white".to_string()));
    }

    #[test]
    fn test_validate_choice_invalid() {
        let def = create_choice_property();
        let mut state = PropertyFormState::default();
        state.input = "red".to_string();

        let result = state.validate(&def);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_bool_ja() {
        let def = create_bool_property();
        let mut state = PropertyFormState::default();
        state.input = "ja".to_string();

        let result = state.validate(&def);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PropertyValue::Bool(true));
    }

    #[test]
    fn test_validate_bool_nein() {
        let def = create_bool_property();
        let mut state = PropertyFormState::default();
        state.input = "nein".to_string();

        let result = state.validate(&def);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PropertyValue::Bool(false));
    }

    #[test]
    fn test_validate_bool_true() {
        let def = create_bool_property();
        let mut state = PropertyFormState::default();
        state.input = "true".to_string();

        let result = state.validate(&def);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PropertyValue::Bool(true));
    }

    #[test]
    fn test_validate_bool_invalid() {
        let def = create_bool_property();
        let mut state = PropertyFormState::default();
        state.input = "maybe".to_string();

        let result = state.validate(&def);
        assert!(result.is_err());
    }

    #[test]
    fn test_cycle_choice_forward() {
        let def = create_choice_property();
        let mut state = PropertyFormState::default();
        state.input = "white".to_string();

        state.cycle_choice(&def, true);
        assert_eq!(state.input, "black");

        state.cycle_choice(&def, true);
        assert_eq!(state.input, "oak");

        state.cycle_choice(&def, true);
        assert_eq!(state.input, "white"); // Wraps around
    }

    #[test]
    fn test_cycle_choice_backward() {
        let def = create_choice_property();
        let mut state = PropertyFormState::default();
        state.input = "white".to_string();

        state.cycle_choice(&def, false);
        assert_eq!(state.input, "oak"); // Wraps to last

        state.cycle_choice(&def, false);
        assert_eq!(state.input, "black");
    }

    #[test]
    fn test_toggle_bool() {
        let mut state = PropertyFormState::default();
        state.input = "ja".to_string();

        state.toggle_bool();
        assert_eq!(state.input, "nein");

        state.toggle_bool();
        assert_eq!(state.input, "ja");
    }

    #[test]
    fn test_error_clears_on_input() {
        let mut state = PropertyFormState::default();
        state.error = Some("Previous error".to_string());

        state.insert('1');
        assert!(state.error.is_none());
    }

    #[test]
    fn test_error_clears_on_backspace() {
        let mut state = PropertyFormState::default();
        state.input = "abc".to_string();
        state.cursor = 3;
        state.error = Some("Previous error".to_string());

        state.backspace();
        assert!(state.error.is_none());
    }
}
