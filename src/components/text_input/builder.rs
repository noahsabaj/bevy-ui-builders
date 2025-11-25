//! TextInputBuilder implementation

use bevy::prelude::*;
use crate::animation::AnimationCategory;
use crate::components::button::{ButtonBuilder, ButtonSize};
use crate::styles::ButtonStyle;
use crate::theme::UiTheme;
use crate::traits::{UiBuilder, LayoutBuilder, BuilderBase};
use super::types::*;
use super::native_input::{
    NativeTextInput, TextBuffer, TextInputVisual,
    TextInputSettings, TabBehavior,
};

/// Builder for creating text inputs with managed focus
pub struct TextInputBuilder {
    value: String,
    placeholder: Option<String>,
    font_size: f32,
    padding: UiRect,
    focus_type: TextInputFocus,
    inactive: bool,
    retain_on_submit: bool,
    filter: Option<TextInputFilter>,
    show_clear_button: bool,
    validation_rules: Option<Vec<crate::ValidationRule>>,
    // Theme-resolved colors (set via .themed())
    themed_colors: Option<TextInputColors>,
    base: BuilderBase,
}

impl TextInputBuilder {
    /// Create a new text input builder
    pub fn new() -> Self {
        let mut base = BuilderBase::new();
        // Default dimensions
        base.node.width = Val::Px(300.0);
        base.node.height = Val::Px(40.0);

        Self {
            value: String::new(),
            placeholder: None,
            font_size: 16.0,
            padding: UiRect::all(Val::Px(10.0)),
            focus_type: TextInputFocus::Independent,
            inactive: false,
            retain_on_submit: true,
            filter: None,
            show_clear_button: false,
            validation_rules: None,
            themed_colors: None,
            base,
        }
    }

    /// Apply theme colors to this builder.
    ///
    /// Call this method to use theme-aware styling. If not called,
    /// sensible defaults (matching the dark theme) will be used.
    ///
    /// # Example
    /// ```ignore
    /// fn setup(mut commands: Commands, theme: Res<UiTheme>) {
    ///     commands.spawn(Node::default()).with_children(|parent| {
    ///         TextInputBuilder::new()
    ///             .themed(&theme)
    ///             .with_placeholder("Enter text...")
    ///             .build(parent);
    ///     });
    /// }
    /// ```
    pub fn themed(mut self, theme: &UiTheme) -> Self {
        self.themed_colors = Some(TextInputColors::from_theme(theme));
        self
    }

    /// Resolve colors (themed > default)
    fn resolve_colors(&self) -> TextInputColors {
        self.themed_colors.clone()
            .unwrap_or_else(TextInputColors::default_colors)
    }

    /// Set the initial value of the text input
    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    /// Set placeholder text (currently just sets initial value)
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Set the font size
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Set the width of the input
    pub fn with_width(mut self, width: Val) -> Self {
        self.base.node.width = width;
        self
    }

    /// Set the height of the input
    pub fn with_height(mut self, height: Val) -> Self {
        self.base.node.height = height;
        self
    }

    /// Set padding
    pub fn with_padding(mut self, padding: UiRect) -> Self {
        self.padding = padding;
        self
    }

    /// Make this input part of an exclusive focus group
    pub fn with_focus_group(mut self, group: FocusGroupId) -> Self {
        self.focus_type = TextInputFocus::ExclusiveGroup(group);
        self
    }

    /// Make this input independent (doesn't affect other inputs)
    pub fn independent(mut self) -> Self {
        self.focus_type = TextInputFocus::Independent;
        self
    }

    /// Start with the input inactive (not focused)
    pub fn inactive(mut self) -> Self {
        self.inactive = true;
        self
    }

    /// Set whether to retain text on submit
    pub fn retain_on_submit(mut self, retain: bool) -> Self {
        self.retain_on_submit = retain;
        self
    }

    /// Set input filter for validation
    pub fn with_filter(mut self, filter_type: InputFilter) -> Self {
        self.filter = Some(TextInputFilter {
            filter_type,
            max_length: None,
            transform: InputTransform::None,
        });
        self
    }

    /// Set maximum length for input
    pub fn with_max_length(mut self, max_length: usize) -> Self {
        if let Some(ref mut filter) = self.filter {
            filter.max_length = Some(max_length);
        } else {
            self.filter = Some(TextInputFilter {
                filter_type: InputFilter::None,
                max_length: Some(max_length),
                transform: InputTransform::None,
            });
        }
        self
    }

    /// Set text transformation
    pub fn with_transform(mut self, transform: InputTransform) -> Self {
        if let Some(ref mut filter) = self.filter {
            filter.transform = transform;
        } else {
            self.filter = Some(TextInputFilter {
                filter_type: InputFilter::None,
                max_length: None,
                transform,
            });
        }
        self
    }

    /// Convenience method for numeric-only input (0-9)
    pub fn numeric_only(mut self) -> Self {
        self.filter = Some(TextInputFilter {
            filter_type: InputFilter::Numeric,
            max_length: None,
            transform: InputTransform::None,
        });
        self
    }

    /// Convenience method for integer input (with optional negative)
    pub fn integer_only(mut self) -> Self {
        self.filter = Some(TextInputFilter {
            filter_type: InputFilter::Integer,
            max_length: None,
            transform: InputTransform::None,
        });
        self
    }

    /// Convenience method for decimal input
    pub fn decimal_only(mut self) -> Self {
        self.filter = Some(TextInputFilter {
            filter_type: InputFilter::Decimal,
            max_length: None,
            transform: InputTransform::None,
        });
        self
    }

    /// Convenience method for alphabetic-only input
    pub fn alphabetic_only(mut self) -> Self {
        self.filter = Some(TextInputFilter {
            filter_type: InputFilter::Alphabetic,
            max_length: None,
            transform: InputTransform::None,
        });
        self
    }

    /// Convenience method for alphanumeric-only input
    pub fn alphanumeric_only(mut self) -> Self {
        self.filter = Some(TextInputFilter {
            filter_type: InputFilter::Alphanumeric,
            max_length: None,
            transform: InputTransform::None,
        });
        self
    }

    /// Show a clear button inside the input
    pub fn with_clear_button(mut self) -> Self {
        self.show_clear_button = true;
        self
    }

    /// Add validation rules to this text input
    pub fn with_validation(mut self, rules: Vec<crate::ValidationRule>) -> Self {
        self.validation_rules = Some(rules);
        self
    }

    /// Build and spawn the text input entity (proxy to UiBuilder::build)
    pub fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        UiBuilder::build(self, parent)
    }
}

impl UiBuilder for TextInputBuilder {
    fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        // Resolve colors (themed > default)
        let colors = self.resolve_colors();

        // If we need a clear button, create a container
        let entity = if self.show_clear_button {
            // Container uses base.node properties
            let mut container_node = self.base.node.clone();
            container_node.flex_direction = FlexDirection::Row;
            container_node.column_gap = Val::Px(5.0);
            // Ensure container doesn't have padding/border that interferes with input look?
            // Actually base.node might have them if user set them via LayoutBuilder.
            // But we want padding on the input, not container.
            // LayoutBuilder::padding sets self.padding, not base.node.padding (see impl below).

            let container_id = parent
                .spawn((
                    container_node,
                    BackgroundColor(Color::NONE),
                ))
                .id();

            let mut text_input_id = None;

            parent
                .commands()
                .entity(container_id)
                .with_children(|container| {
                    let mut entity_commands = container.spawn((
                        // Node components for layout
                        Node {
                            flex_grow: 1.0, // Take remaining space
                            height: Val::Percent(100.0),
                            padding: self.padding,
                            border: UiRect::all(Val::Px(2.0)),
                            justify_content: JustifyContent::Start,
                            align_items: AlignItems::Center,
                            overflow: Overflow::visible(),  // Prevent cursor clipping
                            ..default()
                        },
                        BackgroundColor(colors.background),
                        BorderColor::all(colors.border),
                        BorderRadius::all(Val::Px(5.0)),
                        // Native text input components
                        NativeTextInput,
                        TextBuffer {
                            content: self.value.clone(),
                            cursor_pos: self.value.chars().count(),
                            is_focused: false,
                        },
                        TextInputVisual {
                            font: TextFont {
                                font_size: self.font_size,
                                ..default()
                            },
                            text_color: colors.text,
                            placeholder: self.placeholder.clone().unwrap_or_default(),
                            placeholder_color: colors.placeholder,
                            cursor_color: Color::WHITE,  // White cursor for maximum visibility
                            selection_color: colors.selection,
                            mask_char: None,
                        },
                        TextInputSettings {
                            multiline: false,
                            max_length: self.filter.as_ref().and_then(|f| f.max_length),
                            retain_on_submit: self.retain_on_submit,
                            read_only: self.inactive,
                            tab_behavior: TabBehavior::NextField,
                        },
                        // Focus management
                        self.focus_type.clone(),
                        // Make it a button so it can be clicked
                        Button,
                        // Animation category for inputs
                        AnimationCategory::Input,
                    ));

                    // Add filter if specified
                    if let Some(filter) = self.filter.clone() {
                        entity_commands.insert(filter);
                    }
                    
                    // Add validation if specified
                    if let Some(rules) = self.validation_rules.clone() {
                        entity_commands.insert((
                            crate::validation::Validated::new(rules),
                            crate::validation::ValidationState::default(),
                        ));
                    }

                    text_input_id = Some(entity_commands.id());

                    // Add clear button
                    let clear_button = ButtonBuilder::new("x")
                        .style(ButtonStyle::Ghost)
                        .size(ButtonSize::Small)
                        .build(container);

                    // Add component to track which text input this button clears
                    if let Some(input_id) = text_input_id {
                        container
                            .commands()
                            .entity(clear_button)
                            .insert(ClearButtonTarget(input_id));
                    }
                });

            container_id
        } else {
            // No clear button, build normally
            // Merge base.node with input specific props
            let mut input_node = self.base.node.clone();
            input_node.padding = self.padding;
            input_node.border = UiRect::all(Val::Px(2.0));
            input_node.justify_content = JustifyContent::Start;
            input_node.align_items = AlignItems::Center;
            input_node.overflow = Overflow::visible();

            let mut entity_commands = parent.spawn((
                input_node,
                BackgroundColor(colors.background),
                BorderColor::all(colors.border),
                BorderRadius::all(Val::Px(5.0)),
                // Native text input components
                NativeTextInput,
                TextBuffer {
                    content: self.value.clone(),
                    cursor_pos: self.value.chars().count(),
                    is_focused: false,
                },
                TextInputVisual {
                    font: TextFont {
                        font_size: self.font_size,
                        ..default()
                    },
                    text_color: colors.text,
                    placeholder: self.placeholder.clone().unwrap_or_default(),
                    placeholder_color: colors.placeholder,
                    cursor_color: Color::WHITE,  // White cursor for maximum visibility
                    selection_color: colors.selection,
                    mask_char: None,
                },
                TextInputSettings {
                    multiline: false,
                    max_length: self.filter.as_ref().and_then(|f| f.max_length),
                    retain_on_submit: self.retain_on_submit,
                    read_only: self.inactive,
                    tab_behavior: TabBehavior::NextField,
                },
                // Focus management
                self.focus_type.clone(),
                // Make it a button so it can be clicked
                Button,
                // Animation category for inputs
                AnimationCategory::Input,
            ));

            // Add filter if specified
            if let Some(filter) = self.filter.clone() {
                entity_commands.insert(filter);
            }

            // Add validation if specified
            if let Some(rules) = self.validation_rules.clone() {
                entity_commands.insert((
                    crate::validation::Validated::new(rules),
                    crate::validation::ValidationState::default(),
                ));
            }

            entity_commands.id()
        };
        
        // Apply hooks
        for hook in self.base.hooks {
            hook(&mut parent.commands().entity(entity));
        }

        entity
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

impl LayoutBuilder for TextInputBuilder {
    fn node(mut self, node: Node) -> Self {
        self.base.node = node;
        self
    }

    fn margin(mut self, margin: UiRect) -> Self {
        self.base.node.margin = margin;
        self
    }

    fn padding(mut self, padding: UiRect) -> Self {
        self.padding = padding;
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

/// Convenience function to create a text input builder
pub fn text_input() -> TextInputBuilder {
    TextInputBuilder::new()
}