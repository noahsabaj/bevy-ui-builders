//! DialogueBoxBuilder implementation

use bevy::prelude::*;
use crate::styles::dimensions;
use crate::traits::{UiBuilder, LayoutBuilder, BuilderBase};
use super::types::*;

/// Default colors for dialogue boxes (dark theme fallback)
mod defaults {
    use bevy::prelude::Color;
    pub const BORDER_DEFAULT: Color = Color::srgb(0.3, 0.3, 0.3);
    pub const TEXT_TITLE: Color = Color::srgb(1.0, 1.0, 1.0);
    pub const TEXT_PRIMARY: Color = Color::srgb(0.95, 0.95, 0.95);
    pub const TEXT_DISABLED: Color = Color::srgb(0.4, 0.4, 0.4);
}

/// Builder for creating RPG-style dialogue boxes
///
/// # Examples
///
/// ```ignore
/// use bevy_ui_builders::game_ui::dialogue::*;
///
/// // Basic dialogue
/// DialogueBoxBuilder::new()
///     .speaker("NPC Name")
///     .text("Hello, traveler! Welcome to our village.")
///     .build(parent);
///
/// // Dialogue with portrait and choices
/// DialogueBoxBuilder::new()
///     .speaker("Quest Giver")
///     .portrait(portrait_handle)
///     .text("Will you help us defeat the dragon?")
///     .choice("yes", "Yes, I'll help!")
///     .choice("no", "Sorry, too dangerous.")
///     .typing_effect(true)
///     .build(parent);
/// ```
pub struct DialogueBoxBuilder {
    speaker: Option<String>,
    text: String,
    portrait: Option<Handle<Image>>,
    choices: Vec<DialogueChoiceConfig>,
    style: DialogueStyle,
    typing_effect: bool,
    typing_speed: f32,
    width: Val,
    base: BuilderBase,
}

impl DialogueBoxBuilder {
    /// Create a new dialogue box builder
    pub fn new() -> Self {
        Self {
            speaker: None,
            text: String::new(),
            portrait: None,
            choices: Vec::new(),
            style: DialogueStyle::Standard,
            typing_effect: true,
            typing_speed: 30.0,
            width: Val::Percent(80.0),
            base: BuilderBase::new(),
        }
    }

    /// Set the speaker name
    pub fn speaker(mut self, name: impl Into<String>) -> Self {
        self.speaker = Some(name.into());
        self
    }

    /// Set the dialogue text
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    /// Set the portrait image
    pub fn portrait(mut self, image: Handle<Image>) -> Self {
        self.portrait = Some(image);
        self
    }

    /// Add a choice
    pub fn choice(mut self, id: impl Into<String>, text: impl Into<String>) -> Self {
        self.choices.push(DialogueChoiceConfig::new(id, text));
        self
    }

    /// Add a choice with full configuration
    pub fn choice_with_config(mut self, config: DialogueChoiceConfig) -> Self {
        self.choices.push(config);
        self
    }

    /// Set the visual style
    pub fn style(mut self, style: DialogueStyle) -> Self {
        self.style = style;
        self
    }

    /// Enable or disable typing effect
    pub fn typing_effect(mut self, enabled: bool) -> Self {
        self.typing_effect = enabled;
        self
    }

    /// Set typing speed (characters per second)
    pub fn typing_speed(mut self, speed: f32) -> Self {
        self.typing_speed = speed;
        self
    }

    /// Set the dialogue box width
    pub fn box_width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }
}

impl Default for DialogueBoxBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl UiBuilder for DialogueBoxBuilder {
    fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        let dialogue_entity = parent.spawn_empty().id();
        let full_text = self.text.clone();

        parent.commands().entity(dialogue_entity).insert((
            Node {
                width: self.width,
                padding: UiRect::all(Val::Px(16.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(12.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(32.0),
                left: Val::Percent(10.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.05, 0.08, 0.95)),
            BorderColor::all(defaults::BORDER_DEFAULT),
            BorderRadius::all(Val::Px(8.0)),
            DialogueBox {
                typing_effect: self.typing_effect,
                typing_speed: self.typing_speed,
                is_typing: self.typing_effect,
                chars_revealed: if self.typing_effect { 0 } else { self.text.len() },
            },
        )).with_children(|dialogue| {
            // Header row (speaker + portrait)
            if self.speaker.is_some() || self.portrait.is_some() {
                dialogue.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(12.0),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|header| {
                    // Portrait
                    if let Some(image) = self.portrait {
                        header.spawn((
                            Node {
                                width: Val::Px(64.0),
                                height: Val::Px(64.0),
                                ..default()
                            },
                            ImageNode::new(image),
                            BorderRadius::all(Val::Px(8.0)),
                            DialoguePortrait {
                                dialogue: dialogue_entity,
                            },
                        ));
                    }

                    // Speaker name
                    if let Some(speaker) = &self.speaker {
                        header.spawn((
                            Text::new(speaker),
                            TextFont {
                                font_size: dimensions::FONT_SIZE_LARGE,
                                ..default()
                            },
                            TextColor(defaults::TEXT_TITLE),
                            DialogueSpeaker {
                                dialogue: dialogue_entity,
                            },
                        ));
                    }
                });
            }

            // Dialogue text
            let display_text = if self.typing_effect {
                String::new()
            } else {
                self.text.clone()
            };

            dialogue.spawn((
                Text::new(display_text),
                TextFont {
                    font_size: dimensions::FONT_SIZE_MEDIUM,
                    ..default()
                },
                TextColor(defaults::TEXT_PRIMARY),
                DialogueText {
                    dialogue: dialogue_entity,
                    full_text,
                },
            ));

            // Choices
            if !self.choices.is_empty() {
                dialogue.spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(8.0),
                        margin: UiRect::top(Val::Px(8.0)),
                        ..default()
                    },
                ))
                .with_children(|choices_container| {
                    for (index, choice) in self.choices.iter().enumerate() {
                        let text_color = if choice.disabled {
                            defaults::TEXT_DISABLED
                        } else {
                            defaults::TEXT_PRIMARY
                        };

                        choices_container.spawn((
                            Node {
                                padding: UiRect::new(
                                    Val::Px(16.0),
                                    Val::Px(16.0),
                                    Val::Px(8.0),
                                    Val::Px(8.0),
                                ),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.8)),
                            BorderRadius::all(Val::Px(4.0)),
                            DialogueChoice {
                                dialogue: dialogue_entity,
                                index,
                                choice_id: choice.id.clone(),
                            },
                            if choice.disabled {
                                Interaction::None
                            } else {
                                Interaction::default()
                            },
                        ))
                        .with_children(|choice_btn| {
                            choice_btn.spawn((
                                Text::new(&choice.text),
                                TextFont {
                                    font_size: dimensions::FONT_SIZE_MEDIUM,
                                    ..default()
                                },
                                TextColor(text_color),
                            ));
                        });
                    }
                });
            }
        });

        dialogue_entity
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

impl LayoutBuilder for DialogueBoxBuilder {
    fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    fn margin(mut self, margin: UiRect) -> Self {
        self.base.node.margin = margin;
        self
    }

    fn padding(self, _padding: UiRect) -> Self {
        self
    }
}

/// Convenience function to create a dialogue box builder
pub fn dialogue_box() -> DialogueBoxBuilder {
    DialogueBoxBuilder::new()
}
