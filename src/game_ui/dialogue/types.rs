//! Dialogue types and components

use bevy::prelude::*;

/// Component marking a dialogue box
#[derive(Component, Clone, Debug)]
pub struct DialogueBox {
    /// Whether to use typing effect
    pub typing_effect: bool,
    /// Characters per second for typing effect
    pub typing_speed: f32,
    /// Whether the dialogue is currently typing
    pub is_typing: bool,
    /// Characters revealed so far
    pub chars_revealed: usize,
}

/// Component for the speaker name
#[derive(Component, Clone, Debug)]
pub struct DialogueSpeaker {
    /// The dialogue box this belongs to
    pub dialogue: Entity,
}

/// Component for the portrait image
#[derive(Component, Clone, Debug)]
pub struct DialoguePortrait {
    /// The dialogue box this belongs to
    pub dialogue: Entity,
}

/// Component for the dialogue text
#[derive(Component, Clone, Debug)]
pub struct DialogueText {
    /// The dialogue box this belongs to
    pub dialogue: Entity,
    /// Full text content
    pub full_text: String,
}

/// Component for a dialogue choice button
#[derive(Component, Clone, Debug)]
pub struct DialogueChoice {
    /// The dialogue box this belongs to
    pub dialogue: Entity,
    /// Choice index
    pub index: usize,
    /// Choice identifier (for game logic)
    pub choice_id: String,
}

/// Style variants for dialogue boxes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DialogueStyle {
    /// Standard RPG dialogue box
    #[default]
    Standard,
    /// Clean modern style
    Modern,
    /// Fantasy/medieval style
    Fantasy,
    /// Sci-fi style
    SciFi,
}

/// Configuration for a dialogue choice
#[derive(Clone, Debug)]
pub struct DialogueChoiceConfig {
    /// Display text
    pub text: String,
    /// Choice identifier
    pub id: String,
    /// Whether this choice is disabled
    pub disabled: bool,
}

impl DialogueChoiceConfig {
    /// Create a new choice
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            id: id.into(),
            disabled: false,
        }
    }

    /// Mark as disabled
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Message emitted when dialogue advances
#[derive(Message, Clone, Debug)]
pub struct DialogueAdvanceEvent {
    /// The dialogue box entity
    pub dialogue: Entity,
}

/// Message emitted when a choice is selected
#[derive(Message, Clone, Debug)]
pub struct DialogueChoiceEvent {
    /// The dialogue box entity
    pub dialogue: Entity,
    /// The choice index
    pub index: usize,
    /// The choice identifier
    pub choice_id: String,
}

/// Message emitted when typing completes
#[derive(Message, Clone, Debug)]
pub struct DialogueTypingCompleteEvent {
    /// The dialogue box entity
    pub dialogue: Entity,
}

/// Resource for dialogue settings
#[derive(Resource, Clone, Debug)]
pub struct DialogueSettings {
    /// Default typing speed (chars per second)
    pub default_typing_speed: f32,
    /// Whether to enable typing effect by default
    pub typing_effect_enabled: bool,
}

impl Default for DialogueSettings {
    fn default() -> Self {
        Self {
            default_typing_speed: 30.0,
            typing_effect_enabled: true,
        }
    }
}
