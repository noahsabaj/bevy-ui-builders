//! Main keyboard input handler that ties all modules together

use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;

use super::super::super::super::components::*;
use super::super::super::super::events::*;

use super::navigation::{handle_arrow_left, handle_arrow_right, handle_home, handle_end};
use super::editing::{
    handle_character_input, handle_backspace, handle_delete,
    handle_delete_word_backward, handle_delete_word_forward,
    handle_delete_to_line_start, handle_delete_to_line_end,
};
use super::selection::handle_select_all;
use super::clipboard::{handle_cut, handle_copy, handle_paste};
use super::undo_redo::{handle_undo, handle_redo};
use super::special::handle_enter;
use super::super::super::super::types::TabBehavior;

/// Handle keyboard input for text editing
pub fn handle_keyboard_input(
    mut keyboard_events: MessageReader<KeyboardInput>,
    mut text_inputs: Query<(
        Entity,
        &mut TextBuffer,
        &mut SelectionState,
        &mut CursorVisual,
        &mut UndoHistory,
        &TextInputSettings,
    ), With<NativeTextInput>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut submit_events: MessageWriter<TextInputSubmitEvent>,
    mut change_events: MessageWriter<TextInputChangeEvent>,
) {
    for event in keyboard_events.read() {
        // Only handle key press events
        if !event.state.is_pressed() {
            continue;
        }

        info!("Keyboard event received: {:?}", event.logical_key);

        // Check modifiers
        let ctrl = keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight);
        let shift = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
        let alt = keyboard.pressed(KeyCode::AltLeft) || keyboard.pressed(KeyCode::AltRight);
        let cmd = keyboard.pressed(KeyCode::SuperLeft) || keyboard.pressed(KeyCode::SuperRight);

        for (entity, mut buffer, mut selection, mut cursor_visual, mut history, settings) in text_inputs.iter_mut() {
            // Skip if not focused or read-only
            if !buffer.is_focused || settings.read_only {
                continue;
            }

            // Reset cursor blink on any input
            cursor_visual.blink_timer.reset();
            cursor_visual.visible = true;

            // Handle navigation keys
            match event.key_code {
                KeyCode::ArrowLeft => handle_arrow_left(&mut buffer, &mut selection, shift),
                KeyCode::ArrowRight => handle_arrow_right(&mut buffer, &mut selection, shift),
                KeyCode::Home => handle_home(&mut buffer, &mut selection, shift),
                KeyCode::End => handle_end(&mut buffer, &mut selection, shift),
                KeyCode::Backspace if !ctrl && !shift => handle_backspace(&mut buffer, &mut selection, &mut history),
                KeyCode::Delete if !ctrl && !shift => handle_delete(&mut buffer, &mut selection, &mut history),
                KeyCode::Backspace if ctrl && !shift => handle_delete_word_backward(&mut buffer, &mut selection, &mut history),
                KeyCode::Delete if ctrl && !shift => handle_delete_word_forward(&mut buffer, &mut selection, &mut history),
                KeyCode::Backspace if ctrl && shift => handle_delete_to_line_start(&mut buffer, &mut selection, &mut history),
                KeyCode::Delete if ctrl && shift => handle_delete_to_line_end(&mut buffer, &mut selection, &mut history),
                KeyCode::KeyA if ctrl => handle_select_all(&mut buffer, &mut selection),
                KeyCode::KeyZ if ctrl && !shift => handle_undo(&mut buffer, &mut selection, &mut history),
                KeyCode::KeyY if ctrl => handle_redo(&mut buffer, &mut selection, &mut history),
                KeyCode::KeyZ if ctrl && shift => handle_redo(&mut buffer, &mut selection, &mut history),
                KeyCode::KeyX if ctrl => handle_cut(&mut buffer, &mut selection, &mut history),
                KeyCode::KeyC if ctrl => handle_copy(&buffer, &selection),
                KeyCode::KeyV if ctrl => handle_paste(&mut buffer, &mut selection, &mut history),
                KeyCode::Tab if settings.tab_behavior == TabBehavior::NextField => {
                    // Tab navigation is handled in a separate system
                    continue;
                }
                KeyCode::Enter => {
                    handle_enter(entity, &mut buffer, &mut selection, settings, &mut history, &mut submit_events);
                }
                _ => {
                    // Handle regular character input
                    if !ctrl && !alt && !cmd {
                        handle_character_input(&event.logical_key, &mut buffer, &mut selection, &mut history, settings);
                    }
                }
            }

            // Emit change event
            change_events.write(TextInputChangeEvent {
                entity,
                text: buffer.content.clone(),
            });
        }
    }
}