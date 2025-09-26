//! Event definitions for native text input

use bevy::prelude::*;

/// Event triggered when text input is submitted (Enter key)
#[derive(Event)]
pub struct TextInputSubmitEvent {
    /// Entity that triggered the submit
    pub entity: Entity,
    /// The submitted text
    pub text: String,
}

/// Event triggered when text input content changes
#[derive(Event)]
pub struct TextInputChangeEvent {
    /// Entity that changed
    pub entity: Entity,
    /// The new text content
    pub text: String,
}