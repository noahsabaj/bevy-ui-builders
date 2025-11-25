//! Form types and components

use bevy::prelude::*;
use std::collections::HashMap;

/// Field types available in forms
#[derive(Debug, Clone)]
pub enum FieldType {
    /// Single line text input
    Text,
    /// Password input (masked)
    Password,
    /// Email input with validation
    Email,
    /// Number input
    Number {
        /// Optional minimum value
        min: Option<f32>,
        /// Optional maximum value
        max: Option<f32>
    },
    /// Slider with range
    Slider {
        /// Minimum value
        min: f32,
        /// Maximum value
        max: f32,
        /// Step size
        step: Option<f32>
    },
    /// Dropdown selection
    Dropdown {
        /// Available options
        options: Vec<String>
    },
    /// Checkbox
    Checkbox,
    /// Radio button group
    RadioGroup {
        /// Available options
        options: Vec<String>
    },
    /// Multi-line text area
    TextArea {
        /// Number of visible rows
        rows: usize
    },
    /// Date picker
    Date,
    /// File upload
    File {
        /// Accepted file types (e.g. ".png, .jpg")
        accept: Option<String>
    },
}

/// Validation rules for form fields
#[derive(Debug, Clone)]
pub enum ValidationRule {
    /// Field is required
    Required,
    /// Minimum length for text
    MinLength(usize),
    /// Maximum length for text
    MaxLength(usize),
    /// Numeric range validation
    Range {
        /// Minimum value
        min: f32,
        /// Maximum value
        max: f32
    },
    /// Pattern matching (regex)
    Pattern(String),
    /// Email validation
    Email,
    /// Custom validation function
    Custom(fn(&str) -> Result<(), String>),
}

impl ValidationRule {
    /// Validate a value against this rule
    pub fn validate(&self, value: &str) -> Result<(), String> {
        match self {
            Self::Required => {
                if value.trim().is_empty() {
                    Err("This field is required".to_string())
                } else {
                    Ok(())
                }
            }
            Self::MinLength(min_len) => {
                if value.len() < *min_len {
                    Err(format!("Minimum {} characters required", min_len))
                } else {
                    Ok(())
                }
            }
            Self::MaxLength(max_len) => {
                if value.len() > *max_len {
                    Err(format!("Maximum {} characters allowed", max_len))
                } else {
                    Ok(())
                }
            }
            Self::Range { min, max } => {
                match value.parse::<f32>() {
                    Ok(num) if num < *min => Err(format!("Must be at least {}", min)),
                    Ok(num) if num > *max => Err(format!("Must be at most {}", max)),
                    Ok(_) => Ok(()),
                    Err(_) => Err("Invalid number".to_string()),
                }
            }
            Self::Pattern(pattern) => {
                // Simple pattern matching - could be extended with regex crate
                if value.contains(pattern) {
                    Ok(())
                } else {
                    Err(format!("Must match pattern: {}", pattern))
                }
            }
            Self::Email => {
                // Basic email validation
                if value.contains('@') && value.contains('.') {
                    Ok(())
                } else {
                    Err("Invalid email address".to_string())
                }
            }
            Self::Custom(validator) => {
                validator(value)
            }
        }
    }
}

/// A field in the form
#[derive(Debug, Clone)]
pub struct FormField {
    /// Unique name for the field
    pub name: String,
    /// Display label
    pub label: String,
    /// Field type
    pub field_type: FieldType,
    /// Validation rules
    pub validations: Vec<ValidationRule>,
    /// Placeholder text
    pub placeholder: Option<String>,
    /// Help text shown below field
    pub help_text: Option<String>,
    /// Whether field is disabled
    pub disabled: bool,
    /// Default value
    pub default_value: Option<String>,
}

/// Form submission result
#[derive(Debug, Clone)]
pub struct FormData {
    /// Field values by name
    pub values: HashMap<String, String>,
}

/// Component marking a form root
#[derive(Component)]
pub struct FormRoot {
    /// Form identifier
    pub id: String,
    /// Fields in the form
    pub fields: Vec<FormField>,
    /// Whether form is currently valid
    pub is_valid: bool,
    /// Current values
    pub values: HashMap<String, String>,
}

/// Component marking a form field
#[derive(Component)]
pub struct FormFieldMarker {
    /// Field name
    pub field_name: String,
    /// Field type
    pub field_type: FieldType,
}

/// Component for form submit button
#[derive(Component)]
pub struct FormSubmitButton {
    /// Form entity this button submits
    pub form_entity: Entity,
}

/// Event fired when form is submitted
#[derive(Message)]
pub struct FormSubmitEvent {
    /// Form identifier
    pub form_id: String,
    /// Submitted data
    pub data: FormData,
}

/// Form layout options
#[derive(Debug, Clone)]
pub enum FormLayout {
    /// Fields stacked vertically
    Vertical,
    /// Fields in horizontal rows
    Horizontal,
    /// Grid layout with columns
    Grid {
        /// Number of columns in the grid
        columns: usize
    },
}

/// When to trigger validation
#[derive(Debug, Clone)]
pub enum ValidationTrigger {
    /// Validate on blur
    OnBlur,
    /// Validate on every change
    OnChange,
    /// Validate only on submit
    OnSubmit,
}