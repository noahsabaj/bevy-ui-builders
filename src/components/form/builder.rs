//! FormBuilder implementation

use bevy::prelude::*;
use std::collections::HashMap;
use crate::styles::dimensions;
use crate::traits::{UiBuilder, LayoutBuilder, BuilderBase};
use super::types::{
    FormField, FieldType, ValidationRule, FormLayout,
    FormRoot, FormSubmitButton
};
use super::field::spawn_form_field;

/// Default colors for forms (dark theme fallback)
mod defaults {
    use bevy::prelude::Color;
    pub const BACKGROUND_SECONDARY: Color = Color::srgb(0.08, 0.08, 0.1);
    pub const TEXT_PRIMARY: Color = Color::srgb(0.95, 0.95, 0.95);
}

/// Builder for creating complete forms
pub struct FormBuilder {
    id: String,
    title: Option<String>,
    fields: Vec<FormField>,
    submit_text: String,
    cancel_text: Option<String>,
    layout: FormLayout,
    base: BuilderBase,
}

impl FormBuilder {
    /// Create a new form builder
    pub fn new(id: impl Into<String>) -> Self {
        let mut base = BuilderBase::new();
        base.node.width = Val::Px(400.0);
        base.node.flex_direction = FlexDirection::Column;
        base.node.row_gap = Val::Px(dimensions::SPACING_MEDIUM);
        base.node.padding = UiRect::all(Val::Px(dimensions::PADDING_LARGE));

        Self {
            id: id.into(),
            title: None,
            fields: Vec::new(),
            submit_text: "Submit".to_string(),
            cancel_text: None,
            layout: FormLayout::Vertical,
            base,
        }
    }

    /// Set the form title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Add a text field
    pub fn text_field(mut self, name: impl Into<String>, label: impl Into<String>) -> Self {
        self.fields.push(FormField {
            name: name.into(),
            label: label.into(),
            field_type: FieldType::Text,
            validations: Vec::new(),
            placeholder: None,
            help_text: None,
            disabled: false,
            default_value: None,
        });
        self
    }

    /// Add a password field
    pub fn password_field(mut self, name: impl Into<String>, label: impl Into<String>) -> Self {
        self.fields.push(FormField {
            name: name.into(),
            label: label.into(),
            field_type: FieldType::Password,
            validations: Vec::new(),
            placeholder: None,
            help_text: None,
            disabled: false,
            default_value: None,
        });
        self
    }

    /// Add an email field
    pub fn email_field(mut self, name: impl Into<String>, label: impl Into<String>) -> Self {
        let field = FormField {
            name: name.into(),
            label: label.into(),
            field_type: FieldType::Email,
            validations: vec![ValidationRule::Email],
            placeholder: Some("email@example.com".to_string()),
            help_text: None,
            disabled: false,
            default_value: None,
        };
        self.fields.push(field);
        self
    }

    /// Add a number field
    pub fn number_field(
        mut self,
        name: impl Into<String>,
        label: impl Into<String>,
        min: Option<f32>,
        max: Option<f32>,
    ) -> Self {
        self.fields.push(FormField {
            name: name.into(),
            label: label.into(),
            field_type: FieldType::Number { min, max },
            validations: Vec::new(),
            placeholder: None,
            help_text: None,
            disabled: false,
            default_value: None,
        });
        self
    }

    /// Add a slider field
    pub fn slider_field(
        mut self,
        name: impl Into<String>,
        label: impl Into<String>,
        min: f32,
        max: f32,
    ) -> Self {
        self.fields.push(FormField {
            name: name.into(),
            label: label.into(),
            field_type: FieldType::Slider { min, max, step: None },
            validations: Vec::new(),
            placeholder: None,
            help_text: None,
            disabled: false,
            default_value: Some(min.to_string()),
        });
        self
    }

    /// Add a dropdown field
    pub fn dropdown_field(
        mut self,
        name: impl Into<String>,
        label: impl Into<String>,
        options: Vec<String>,
    ) -> Self {
        self.fields.push(FormField {
            name: name.into(),
            label: label.into(),
            field_type: FieldType::Dropdown { options },
            validations: Vec::new(),
            placeholder: Some("Select an option".to_string()),
            help_text: None,
            disabled: false,
            default_value: None,
        });
        self
    }

    /// Add a checkbox field
    pub fn checkbox_field(mut self, name: impl Into<String>, label: impl Into<String>) -> Self {
        self.fields.push(FormField {
            name: name.into(),
            label: label.into(),
            field_type: FieldType::Checkbox,
            validations: Vec::new(),
            placeholder: None,
            help_text: None,
            disabled: false,
            default_value: Some("false".to_string()),
        });
        self
    }

    /// Make the last added field required
    pub fn required(mut self) -> Self {
        if let Some(field) = self.fields.last_mut() {
            field.validations.push(ValidationRule::Required);
        }
        self
    }

    /// Add validation to the last field
    pub fn validate(mut self, rule: ValidationRule) -> Self {
        if let Some(field) = self.fields.last_mut() {
            field.validations.push(rule);
        }
        self
    }

    /// Add placeholder to the last field
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        if let Some(field) = self.fields.last_mut() {
            field.placeholder = Some(text.into());
        }
        self
    }

    /// Add help text to the last field
    pub fn help_text(mut self, text: impl Into<String>) -> Self {
        if let Some(field) = self.fields.last_mut() {
            field.help_text = Some(text.into());
        }
        self
    }

    /// Set submit button text
    pub fn submit_text(mut self, text: impl Into<String>) -> Self {
        self.submit_text = text.into();
        self
    }

    /// Add cancel button
    pub fn cancel_text(mut self, text: impl Into<String>) -> Self {
        self.cancel_text = Some(text.into());
        self
    }

    /// Set form layout
    pub fn layout(mut self, layout: FormLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Set form width
    pub fn width(mut self, width: Val) -> Self {
        self.base.node.width = width;
        self
    }

    /// Build the form (proxy to UiBuilder::build)
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        UiBuilder::build(self, parent)
    }
}

impl UiBuilder for FormBuilder {
    fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        let form_entity = parent
            .spawn((
                FormRoot {
                    id: self.id.clone(),
                    fields: self.fields.clone(),
                    is_valid: false,
                    values: HashMap::new(),
                },
                self.base.node,
                BackgroundColor(defaults::BACKGROUND_SECONDARY),
                BorderRadius::all(Val::Px(dimensions::BORDER_RADIUS_MEDIUM)),
                Transform::default(), // Required to prevent B0004 warnings
            ))
            .id();

        let form_entity_copy = form_entity;

        parent.commands().entity(form_entity).with_children(|form| {
            // Add title if provided
            if let Some(title) = self.title {
                form.spawn((
                    Text::new(title),
                    TextFont {
                        font_size: dimensions::FONT_SIZE_HEADING,
                        ..default()
                    },
                    TextColor(defaults::TEXT_PRIMARY),
                    Node {
                        margin: UiRect::bottom(Val::Px(dimensions::SPACING_LARGE)),
                        ..default()
                    },
                ));
            }

            // Add fields
            for field in &self.fields {
                spawn_form_field(form, field);
            }

            // Add buttons
            form.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::End,
                    column_gap: Val::Px(dimensions::SPACING_MEDIUM),
                    margin: UiRect::top(Val::Px(dimensions::SPACING_LARGE)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                Transform::default(), // Required to prevent B0004 warnings
            ))
            .with_children(|buttons| {
                // Cancel button if specified
                if let Some(cancel_text) = self.cancel_text {
                    crate::components::button::secondary_button(cancel_text).build(buttons);
                }

                // Submit button
                let submit_button = crate::components::button::primary_button(self.submit_text)
                    .build(buttons);

                buttons.commands()
                    .entity(submit_button)
                    .insert(FormSubmitButton { form_entity: form_entity_copy });
            });
        });

        // Apply hooks
        for hook in self.base.hooks {
            hook(&mut parent.commands().entity(form_entity));
        }

        form_entity
    }

    fn insert(mut self, bundle: impl Bundle + Clone) -> Self {
        self.base.hooks.push(Box::new(move |cmds| {
            cmds.insert(bundle.clone());
        }));
        self
    }

    fn id(mut self, id: Entity) -> Self {
        self.base.entity = Some(id);
        self
    }
}

impl LayoutBuilder for FormBuilder {
    fn node(mut self, node: Node) -> Self {
        self.base.node = node;
        self
    }

    fn margin(mut self, margin: UiRect) -> Self {
        self.base.node.margin = margin;
        self
    }

    fn padding(mut self, padding: UiRect) -> Self {
        self.base.node.padding = padding;
        self
    }

    fn width(mut self, width: Val) -> Self {
        self.base.node.width = width;
        self
    }

    fn height(mut self, height: Val) -> Self {
        self.base.node.height = height;
        self
    }
}