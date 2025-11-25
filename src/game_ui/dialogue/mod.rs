//! Dialogue box component
//!
//! Provides RPG-style dialogue boxes with portraits and choices.
//!
//! # Features
//!
//! - Speaker name and portrait
//! - Typing effect animation
//! - Multiple choice support
//! - Click to skip typing
//! - Customizable styles
//!
//! # Examples
//!
//! ```ignore
//! use bevy_ui_builders::game_ui::dialogue::*;
//!
//! // Create a dialogue with choices
//! DialogueBoxBuilder::new()
//!     .speaker("Quest Giver")
//!     .text("Will you help us defeat the dragon?")
//!     .choice("yes", "Yes, I'll help!")
//!     .choice("no", "Sorry, too dangerous.")
//!     .typing_effect(true)
//!     .build(parent);
//!
//! // Handle choice selection
//! fn handle_choices(mut events: MessageReader<DialogueChoiceEvent>) {
//!     for event in events.read() {
//!         match event.choice_id.as_str() {
//!             "yes" => { /* Accept quest */ }
//!             "no" => { /* Decline quest */ }
//!             _ => {}
//!         }
//!     }
//! }
//! ```

mod builder;
mod plugin;
mod systems;
mod types;

pub use builder::{DialogueBoxBuilder, dialogue_box};
pub use plugin::DialoguePlugin;
pub use types::{
    DialogueBox, DialogueSpeaker, DialoguePortrait, DialogueText,
    DialogueChoice, DialogueStyle, DialogueChoiceConfig,
    DialogueAdvanceEvent, DialogueChoiceEvent, DialogueTypingCompleteEvent,
    DialogueSettings,
};
