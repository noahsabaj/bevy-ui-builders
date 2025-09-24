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
    Number { min: Option<f32>, max: Option<f32> },
    /// Slider with range
    Slider { min: f32, max: f32, step: Option<f32> },
    /// Dropdown selection
    Dropdown { options: Vec<String> },
    /// Checkbox
    Checkbox,
    /// Radio button group
    RadioGroup { options: Vec<String> },
    /// Multi-line text area
    TextArea { rows: usize },
    /// Date picker
    Date,
    /// File upload
    File { accept: Option<String> },
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
    /// Pattern matching (regex)
    Pattern(String),
    /// Email validation
    Email,
    /// Custom validation function
    Custom(fn(&str) -> Result<(), String>),
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
#[derive(Event)]
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
    Grid { columns: usize },
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