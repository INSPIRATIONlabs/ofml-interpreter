//! OAP Actions processing
//!
//! This module handles OAP actions that modify configuration state:
//! - OAPCreateObjectAction: Create child articles
//! - OAPPropEditAction: Propagate property changes
//! - OAPShowMediaAction: Reference images/previews

use super::config::Configuration;
use serde::{Deserialize, Serialize};

/// OAP action types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OAPAction {
    /// Create a child object/sub-article
    CreateObject(OAPCreateObjectAction),
    /// Edit/propagate property values
    PropEdit(OAPPropEditAction),
    /// Show media reference
    ShowMedia(OAPShowMediaAction),
}

/// Action to create a child object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAPCreateObjectAction {
    /// Class name of the object to create
    pub class_name: String,
    /// Properties to set on the new object
    pub properties: Vec<(String, String)>,
    /// Parent relationship type
    pub relationship: String,
}

/// Action to edit properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAPPropEditAction {
    /// Target property name
    pub property_name: String,
    /// New value expression
    pub value_expression: String,
    /// Condition for when to apply
    pub condition: Option<String>,
}

/// Action to show media
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAPShowMediaAction {
    /// Media type (image, video, etc.)
    pub media_type: String,
    /// Path or URL to media
    pub media_path: String,
}

/// Result of processing an OAP action
#[derive(Debug, Clone)]
pub struct OAPActionResult {
    /// Type of action executed
    pub action_type: String,
    /// Whether action completed successfully
    pub success: bool,
    /// New object if OAPCreateObjectAction
    pub created_object: Option<Box<Configuration>>,
    /// Property names changed by OAPPropEditAction
    pub modified_properties: Vec<String>,
    /// Media path/URL if OAPShowMediaAction
    pub media_reference: Option<String>,
    /// Error description if failed
    pub error_message: Option<String>,
}

impl OAPActionResult {
    /// Create a successful result for CreateObject
    pub fn create_object_success(config: Configuration) -> Self {
        Self {
            action_type: "create".to_string(),
            success: true,
            created_object: Some(Box::new(config)),
            modified_properties: vec![],
            media_reference: None,
            error_message: None,
        }
    }

    /// Create a successful result for PropEdit
    pub fn prop_edit_success(modified: Vec<String>) -> Self {
        Self {
            action_type: "edit".to_string(),
            success: true,
            created_object: None,
            modified_properties: modified,
            media_reference: None,
            error_message: None,
        }
    }

    /// Create a successful result for ShowMedia
    pub fn show_media_success(path: String) -> Self {
        Self {
            action_type: "media".to_string(),
            success: true,
            created_object: None,
            modified_properties: vec![],
            media_reference: Some(path),
            error_message: None,
        }
    }

    /// Create a failure result
    pub fn failure(action_type: &str, error: String) -> Self {
        Self {
            action_type: action_type.to_string(),
            success: false,
            created_object: None,
            modified_properties: vec![],
            media_reference: None,
            error_message: Some(error),
        }
    }
}

/// OAP action processor
pub struct OAPActionProcessor {
    // State for action processing
}

impl OAPActionProcessor {
    /// Create a new action processor
    pub fn new() -> Self {
        Self {}
    }

    /// Process an OAP action on a configuration
    pub fn process(&self, action: &OAPAction, config: &mut Configuration) -> OAPActionResult {
        match action {
            OAPAction::CreateObject(create_action) => {
                self.process_create_object(create_action, config)
            }
            OAPAction::PropEdit(edit_action) => self.process_prop_edit(edit_action, config),
            OAPAction::ShowMedia(media_action) => self.process_show_media(media_action),
        }
    }

    /// Process OAPCreateObjectAction
    fn process_create_object(
        &self,
        action: &OAPCreateObjectAction,
        parent: &mut Configuration,
    ) -> OAPActionResult {
        // Create a new sub-article configuration
        let mut sub_config =
            Configuration::new(action.class_name.clone(), parent.manufacturer_id.clone());

        // Apply initial properties
        for (name, value) in &action.properties {
            // Parse and set property value
            use crate::property::PropertyValue;
            // Try to parse as different types
            let pv = if let Ok(i) = value.parse::<i64>() {
                PropertyValue::Int(i)
            } else if let Ok(f) = value.parse::<f64>() {
                PropertyValue::Float(f)
            } else if value == "true" {
                PropertyValue::Bool(true)
            } else if value == "false" {
                PropertyValue::Bool(false)
            } else {
                PropertyValue::String(value.clone())
            };

            sub_config.properties.values.insert(name.clone(), pv);
        }

        // Update variant code
        sub_config.update_variant_code();

        // Add to parent's sub-articles
        parent.sub_articles.push(sub_config.clone());

        OAPActionResult::create_object_success(sub_config)
    }

    /// Process OAPPropEditAction
    fn process_prop_edit(
        &self,
        action: &OAPPropEditAction,
        config: &mut Configuration,
    ) -> OAPActionResult {
        // Check condition if present
        if let Some(ref _condition) = action.condition {
            // TODO: Evaluate condition expression
        }

        // Parse and set the new value
        use crate::property::PropertyValue;
        let pv = if let Ok(i) = action.value_expression.parse::<i64>() {
            PropertyValue::Int(i)
        } else if let Ok(f) = action.value_expression.parse::<f64>() {
            PropertyValue::Float(f)
        } else if action.value_expression == "true" {
            PropertyValue::Bool(true)
        } else if action.value_expression == "false" {
            PropertyValue::Bool(false)
        } else {
            PropertyValue::String(action.value_expression.clone())
        };

        config
            .properties
            .values
            .insert(action.property_name.clone(), pv);

        // Update variant code
        config.update_variant_code();

        OAPActionResult::prop_edit_success(vec![action.property_name.clone()])
    }

    /// Process OAPShowMediaAction
    fn process_show_media(&self, action: &OAPShowMediaAction) -> OAPActionResult {
        // Just return the media reference
        OAPActionResult::show_media_success(action.media_path.clone())
    }
}

impl Default for OAPActionProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_object_action() {
        let processor = OAPActionProcessor::new();
        let mut config = Configuration::new("Parent".to_string(), "vitra".to_string());

        let action = OAPAction::CreateObject(OAPCreateObjectAction {
            class_name: "ChildPart".to_string(),
            properties: vec![("color".to_string(), "white".to_string())],
            relationship: "child".to_string(),
        });

        let result = processor.process(&action, &mut config);
        assert!(result.success);
        assert!(result.created_object.is_some());
        assert_eq!(config.sub_articles.len(), 1);
        assert_eq!(config.sub_articles[0].article_id, "ChildPart");
    }

    #[test]
    fn test_prop_edit_action() {
        let processor = OAPActionProcessor::new();
        let mut config = Configuration::new("Article".to_string(), "vitra".to_string());

        let action = OAPAction::PropEdit(OAPPropEditAction {
            property_name: "height".to_string(),
            value_expression: "720".to_string(),
            condition: None,
        });

        let result = processor.process(&action, &mut config);
        assert!(result.success);
        assert_eq!(result.modified_properties, vec!["height"]);

        // Check property was set
        use crate::property::PropertyValue;
        assert_eq!(
            config.properties.values.get("height"),
            Some(&PropertyValue::Int(720))
        );
    }

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
        assert_eq!(
            result.media_reference,
            Some("/images/preview.jpg".to_string())
        );
    }

    #[test]
    fn test_action_result_failure() {
        let result = OAPActionResult::failure("create", "Class not found".to_string());
        assert!(!result.success);
        assert_eq!(result.error_message, Some("Class not found".to_string()));
    }
}
