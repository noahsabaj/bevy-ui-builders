//! Mouse click handling

use bevy::prelude::*;
use bevy::ecs::system::ParamSet;
use bevy::text::TextLayoutInfo;
use bevy::ui::RelativeCursorPosition;

use super::super::super::super::components::*;
use super::selection::calculate_char_index_from_position;

/// Handle mouse input for selection
pub fn handle_mouse_input(
    mut param_set: ParamSet<(
        Query<(
            Entity,
            &mut TextBuffer,
            &mut SelectionState,
            &mut CursorVisual,
            &Interaction,
            &RelativeCursorPosition,
            &Children,
        ), (With<NativeTextInput>, Changed<Interaction>)>,
        Query<(Entity, &mut TextBuffer, &mut SelectionState, &mut CursorVisual), With<NativeTextInput>>,
    )>,
    text_query: Query<&TextLayoutInfo, With<TextInputInner>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    // Collect pressed entities first using the first query
    let pressed_entities: Vec<Entity> = {
        let p0 = param_set.p0();
        p0.iter()
            .filter(|(_, _, _, _, interaction, _, _)| **interaction == Interaction::Pressed)
            .map(|(entity, _, _, _, _, _, _)| entity)
            .collect()
    };

    // Unfocus all other inputs
    if !pressed_entities.is_empty() {
        let mut p1 = param_set.p1();
        for (entity, mut buffer, mut selection, mut cursor_visual) in p1.iter_mut() {
            if !pressed_entities.contains(&entity) {
                buffer.is_focused = false;
                selection.clear();
                cursor_visual.visible = false;
            }
        }
    }

    // Now handle the actual interaction
    let mut p0 = param_set.p0();
    for (_entity, mut buffer, mut selection, mut cursor_visual, interaction, cursor_pos, children) in p0.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Check if already focused
                let was_already_focused = buffer.is_focused;

                // Focus the input (now exclusive)
                info!("Setting focus to entity {:?} (was_already_focused: {})", _entity, was_already_focused);
                buffer.is_focused = true;
                cursor_visual.visible = true;  // Make cursor immediately visible
                cursor_visual.blink_timer.reset();  // Reset blink timer

                // Find the text inner entity (direct child)
                let mut text_entity = None;
                for child in children.iter() {
                    if text_query.get(child).is_ok() {
                        text_entity = Some(child);
                        break;
                    }
                }

                if let Some(text_entity) = text_entity {
                    if let Ok(text_layout) = text_query.get(text_entity) {
                        if let Some(normalized_pos) = cursor_pos.normalized {
                            info!("Mouse click normalized position: {:?}", normalized_pos);
                            info!("Text layout glyphs count: {}", text_layout.glyphs.len());
                            info!("Buffer content: '{}', length: {}", buffer.content, buffer.content.len());

                            // Only update cursor position if we have glyph data or if it wasn't focused before
                            // If no glyphs and already focused, keep cursor where it was
                            if !text_layout.glyphs.is_empty() || !was_already_focused {
                                // Calculate click position in the text
                                let char_index = calculate_char_index_from_position(
                                    normalized_pos,
                                    text_layout,
                                    &buffer.content,
                                );

                                info!("Calculated char_index: {}", char_index);

                                let shift = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);

                                if shift && mouse.just_pressed(MouseButton::Left) {
                                    // Extend selection
                                    if selection.anchor.is_none() {
                                        selection.start_selection(buffer.cursor_pos);
                                    }
                                    selection.update_selection(char_index);
                                } else if mouse.just_pressed(MouseButton::Left) {
                                    // Start new selection or just move cursor
                                    info!("Setting cursor position to: {} (was: {})", char_index, buffer.cursor_pos);
                                    buffer.cursor_pos = char_index;
                                    selection.clear();
                                    // Store anchor for potential drag selection
                                    selection.anchor = Some(char_index);
                                }
                            } else {
                                info!("Keeping cursor position at {} (no glyphs available yet)", buffer.cursor_pos);
                            }
                        } else {
                            info!("No normalized cursor position available");
                        }
                    } else {
                        info!("Could not get text layout for text entity");
                    }
                }
            }
            _ => {}
        }
    }
}