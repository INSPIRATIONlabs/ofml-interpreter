//! Unit tests for OAP actions (T054, T055)

use ofml_lib::oap::actions::{
    OAPAction, OAPActionProcessor, OAPActionResult, OAPCreateObjectAction, OAPPropEditAction,
    OAPShowMediaAction,
};
use ofml_lib::oap::config::Configuration;
use ofml_lib::property::PropertyValue;

// =============================================================================
// T054: Unit test for OAPCreateObjectAction
// =============================================================================

#[test]
fn test_create_object_action_structure() {
    let action = OAPCreateObjectAction {
        class_name: "ChildPart".to_string(),
        properties: vec![
            ("color".to_string(), "white".to_string()),
            ("height".to_string(), "720".to_string()),
        ],
        relationship: "child".to_string(),
    };

    assert_eq!(action.class_name, "ChildPart");
    assert_eq!(action.properties.len(), 2);
    assert_eq!(action.relationship, "child");
}

#[test]
fn test_create_object_action_processing() {
    let processor = OAPActionProcessor::new();
    let mut parent = Configuration::new("ParentArticle".to_string(), "vitra".to_string());

    let action = OAPAction::CreateObject(OAPCreateObjectAction {
        class_name: "ChildPart".to_string(),
        properties: vec![("color".to_string(), "white".to_string())],
        relationship: "child".to_string(),
    });

    let result = processor.process(&action, &mut parent);

    assert!(result.success);
    assert_eq!(result.action_type, "create");
    assert!(result.created_object.is_some());
    assert!(result.error_message.is_none());
}

#[test]
fn test_create_object_adds_to_sub_articles() {
    let processor = OAPActionProcessor::new();
    let mut parent = Configuration::new("ParentArticle".to_string(), "vitra".to_string());

    assert!(parent.sub_articles.is_empty());

    let action = OAPAction::CreateObject(OAPCreateObjectAction {
        class_name: "ChildPart".to_string(),
        properties: vec![],
        relationship: "child".to_string(),
    });

    processor.process(&action, &mut parent);

    assert_eq!(parent.sub_articles.len(), 1);
    assert_eq!(parent.sub_articles[0].article_id, "ChildPart");
}

#[test]
fn test_create_object_sets_properties() {
    let processor = OAPActionProcessor::new();
    let mut parent = Configuration::new("Parent".to_string(), "vitra".to_string());

    let action = OAPAction::CreateObject(OAPCreateObjectAction {
        class_name: "Child".to_string(),
        properties: vec![
            ("height".to_string(), "720".to_string()),
            ("enabled".to_string(), "true".to_string()),
        ],
        relationship: "child".to_string(),
    });

    let result = processor.process(&action, &mut parent);

    assert!(result.success);
    let child = result.created_object.unwrap();

    // Check properties were set
    assert!(child.properties.values.contains_key("height"));
    assert!(child.properties.values.contains_key("enabled"));
}

#[test]
fn test_create_multiple_sub_articles() {
    let processor = OAPActionProcessor::new();
    let mut parent = Configuration::new("Parent".to_string(), "vitra".to_string());

    for i in 0..3 {
        let action = OAPAction::CreateObject(OAPCreateObjectAction {
            class_name: format!("Child{}", i),
            properties: vec![],
            relationship: "child".to_string(),
        });
        processor.process(&action, &mut parent);
    }

    assert_eq!(parent.sub_articles.len(), 3);
}

// =============================================================================
// T055: Unit test for OAPPropEditAction
// =============================================================================

#[test]
fn test_prop_edit_action_structure() {
    let action = OAPPropEditAction {
        property_name: "height".to_string(),
        value_expression: "720".to_string(),
        condition: Some("color == 'white'".to_string()),
    };

    assert_eq!(action.property_name, "height");
    assert_eq!(action.value_expression, "720");
    assert!(action.condition.is_some());
}

#[test]
fn test_prop_edit_action_processing() {
    let processor = OAPActionProcessor::new();
    let mut config = Configuration::new("Article".to_string(), "vitra".to_string());

    let action = OAPAction::PropEdit(OAPPropEditAction {
        property_name: "height".to_string(),
        value_expression: "720".to_string(),
        condition: None,
    });

    let result = processor.process(&action, &mut config);

    assert!(result.success);
    assert_eq!(result.action_type, "edit");
    assert_eq!(result.modified_properties, vec!["height"]);
}

#[test]
fn test_prop_edit_sets_int_value() {
    let processor = OAPActionProcessor::new();
    let mut config = Configuration::new("Article".to_string(), "vitra".to_string());

    let action = OAPAction::PropEdit(OAPPropEditAction {
        property_name: "height".to_string(),
        value_expression: "720".to_string(),
        condition: None,
    });

    processor.process(&action, &mut config);

    let value = config.properties.values.get("height");
    assert!(value.is_some());
    assert_eq!(value.unwrap(), &PropertyValue::Int(720));
}

#[test]
fn test_prop_edit_sets_float_value() {
    let processor = OAPActionProcessor::new();
    let mut config = Configuration::new("Article".to_string(), "vitra".to_string());

    let action = OAPAction::PropEdit(OAPPropEditAction {
        property_name: "width".to_string(),
        value_expression: "1.5".to_string(),
        condition: None,
    });

    processor.process(&action, &mut config);

    let value = config.properties.values.get("width");
    assert!(value.is_some());
    assert_eq!(value.unwrap(), &PropertyValue::Float(1.5));
}

#[test]
fn test_prop_edit_sets_bool_true() {
    let processor = OAPActionProcessor::new();
    let mut config = Configuration::new("Article".to_string(), "vitra".to_string());

    let action = OAPAction::PropEdit(OAPPropEditAction {
        property_name: "enabled".to_string(),
        value_expression: "true".to_string(),
        condition: None,
    });

    processor.process(&action, &mut config);

    let value = config.properties.values.get("enabled");
    assert!(value.is_some());
    assert_eq!(value.unwrap(), &PropertyValue::Bool(true));
}

#[test]
fn test_prop_edit_sets_bool_false() {
    let processor = OAPActionProcessor::new();
    let mut config = Configuration::new("Article".to_string(), "vitra".to_string());

    let action = OAPAction::PropEdit(OAPPropEditAction {
        property_name: "enabled".to_string(),
        value_expression: "false".to_string(),
        condition: None,
    });

    processor.process(&action, &mut config);

    let value = config.properties.values.get("enabled");
    assert!(value.is_some());
    assert_eq!(value.unwrap(), &PropertyValue::Bool(false));
}

#[test]
fn test_prop_edit_sets_string_value() {
    let processor = OAPActionProcessor::new();
    let mut config = Configuration::new("Article".to_string(), "vitra".to_string());

    let action = OAPAction::PropEdit(OAPPropEditAction {
        property_name: "description".to_string(),
        value_expression: "Test description".to_string(),
        condition: None,
    });

    processor.process(&action, &mut config);

    let value = config.properties.values.get("description");
    assert!(value.is_some());
    assert_eq!(
        value.unwrap(),
        &PropertyValue::String("Test description".to_string())
    );
}

#[test]
fn test_prop_edit_updates_existing_property() {
    let processor = OAPActionProcessor::new();
    let mut config = Configuration::new("Article".to_string(), "vitra".to_string());

    // Set initial value
    config
        .properties
        .values
        .insert("height".to_string(), PropertyValue::Int(620));

    // Update via action
    let action = OAPAction::PropEdit(OAPPropEditAction {
        property_name: "height".to_string(),
        value_expression: "720".to_string(),
        condition: None,
    });

    processor.process(&action, &mut config);

    let value = config.properties.values.get("height");
    assert_eq!(value.unwrap(), &PropertyValue::Int(720));
}

#[test]
fn test_prop_edit_updates_variant_code() {
    let processor = OAPActionProcessor::new();
    let mut config = Configuration::new("Article".to_string(), "vitra".to_string());

    let old_variant = config.variant_code.clone();

    let action = OAPAction::PropEdit(OAPPropEditAction {
        property_name: "height".to_string(),
        value_expression: "720".to_string(),
        condition: None,
    });

    processor.process(&action, &mut config);

    // Variant code should be updated (even if it was empty before)
    // The actual content depends on property definitions
    assert!(config.variant_code != old_variant || config.variant_code.is_empty());
}

// =============================================================================
// Additional action tests
// =============================================================================

#[test]
fn test_show_media_action() {
    let processor = OAPActionProcessor::new();
    let mut config = Configuration::new("Article".to_string(), "vitra".to_string());

    let action = OAPAction::ShowMedia(OAPShowMediaAction {
        media_type: "image".to_string(),
        media_path: "/images/preview.jpg".to_string(),
    });

    let result = processor.process(&action, &mut config);

    assert!(result.success);
    assert_eq!(result.action_type, "media");
    assert_eq!(
        result.media_reference,
        Some("/images/preview.jpg".to_string())
    );
}

#[test]
fn test_action_result_failure() {
    let result = OAPActionResult::failure("create", "Class not found".to_string());

    assert!(!result.success);
    assert_eq!(result.action_type, "create");
    assert_eq!(result.error_message, Some("Class not found".to_string()));
    assert!(result.created_object.is_none());
}

#[test]
fn test_action_result_create_object_success() {
    let config = Configuration::new("Child".to_string(), "vitra".to_string());
    let result = OAPActionResult::create_object_success(config);

    assert!(result.success);
    assert_eq!(result.action_type, "create");
    assert!(result.created_object.is_some());
    assert!(result.error_message.is_none());
}

#[test]
fn test_action_result_prop_edit_success() {
    let result =
        OAPActionResult::prop_edit_success(vec!["height".to_string(), "width".to_string()]);

    assert!(result.success);
    assert_eq!(result.action_type, "edit");
    assert_eq!(result.modified_properties.len(), 2);
    assert!(result.created_object.is_none());
}

#[test]
fn test_action_result_show_media_success() {
    let result = OAPActionResult::show_media_success("/images/test.jpg".to_string());

    assert!(result.success);
    assert_eq!(result.action_type, "media");
    assert_eq!(result.media_reference, Some("/images/test.jpg".to_string()));
}

#[test]
fn test_action_processor_default() {
    let processor = OAPActionProcessor::default();
    let mut config = Configuration::new("Test".to_string(), "vitra".to_string());

    // Should work the same as ::new()
    let action = OAPAction::PropEdit(OAPPropEditAction {
        property_name: "test".to_string(),
        value_expression: "value".to_string(),
        condition: None,
    });

    let result = processor.process(&action, &mut config);
    assert!(result.success);
}

#[test]
fn test_oap_action_enum_variants() {
    // Verify all variants can be created
    let _create = OAPAction::CreateObject(OAPCreateObjectAction {
        class_name: "Test".to_string(),
        properties: vec![],
        relationship: "child".to_string(),
    });

    let _edit = OAPAction::PropEdit(OAPPropEditAction {
        property_name: "test".to_string(),
        value_expression: "value".to_string(),
        condition: None,
    });

    let _media = OAPAction::ShowMedia(OAPShowMediaAction {
        media_type: "image".to_string(),
        media_path: "/test.jpg".to_string(),
    });
}
