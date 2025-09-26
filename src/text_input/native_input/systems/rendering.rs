//! Text and cursor rendering systems

use bevy::prelude::*;
use bevy::text::TextLayoutInfo;

use super::super::components::*;
use super::super::types::*;

/// Render text without cursor (cursor is a separate entity)
pub fn render_text(
    mut text_inputs: Query<(
        &TextBuffer,
        &SelectionState,
        &TextInputVisual,
        &Children,
    ), With<NativeTextInput>>,
    text_inner_query: Query<Entity, With<TextInputInner>>,
    mut text_span_query: Query<(&mut TextSpan, &mut TextFont, &mut TextColor)>,
    children_query: Query<&Children>,
) {
    for (buffer, _selection, visual, children) in text_inputs.iter_mut() {
        info!("render_text: buffer content = '{}'", buffer.content);

        // Find the TextInputInner entity (direct child)
        let mut text_inner_entity = None;
        for child in children.iter() {
            if text_inner_query.get(child).is_ok() {
                text_inner_entity = Some(child);
                break;
            }
        }

        if let Some(text_inner_entity) = text_inner_entity {
            // Now find the TextSpan child of the TextInputInner
            if let Ok(text_children) = children_query.get(text_inner_entity) {
                for text_child in text_children.iter() {
                    if let Ok((mut text_span, mut font, mut color)) = text_span_query.get_mut(text_child) {
                        // Build display text without cursor
                        let display_text = if buffer.content.is_empty() && !buffer.is_focused {
                            // Show placeholder
                            visual.placeholder.clone()
                        } else {
                            // Show actual text with optional masking
                            if let Some(mask) = visual.mask_char {
                                mask.to_string().repeat(buffer.content.chars().count())
                            } else {
                                buffer.content.clone()
                            }
                        };

                        // Update the text content and styling
                        info!("Updating text span with: '{}'", display_text);

                        // Update text content
                        *text_span = TextSpan::new(display_text);

                        // Update font
                        *font = visual.font.clone();

                        // Update color based on state
                        color.0 = if buffer.content.is_empty() && !buffer.is_focused {
                            visual.placeholder_color
                        } else {
                            visual.text_color
                        };

                        break;  // Only update the first TextSpan
                    }
                }
            }
        } else {
            warn!("Could not find text inner entity for input!");
        }
    }
}

/// Update visual cursor position and style
pub fn update_cursor_visual(
    text_inputs: Query<(
        &TextBuffer,
        &CursorVisual,
        &TextInputVisual,
        &Children,
    ), With<NativeTextInput>>,  // Removed Changed filter to ensure all cursors update
    text_query: Query<&TextLayoutInfo, With<TextInputInner>>,
    mut cursor_query: Query<(&mut Node, &mut BackgroundColor, &mut Visibility), With<TextInputCursor>>,
    _children_query: Query<&Children>,
) {
    for (buffer, cursor_visual, input_visual, children) in text_inputs.iter() {
        if let Some(cursor_entity) = cursor_visual.cursor_entity {
            if let Ok((mut node, mut bg_color, mut visibility)) = cursor_query.get_mut(cursor_entity) {
                // Update visibility based on focus and blink state
                *visibility = if buffer.is_focused && cursor_visual.visible {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };

                // Set cursor color from visual settings
                bg_color.0 = input_visual.cursor_color;

                // Find text layout info (direct child)
                for child in children.iter() {
                    if let Ok(layout) = text_query.get(child) {
                        // Calculate cursor position based on layout
                        // The cursor_pos is a character index, and we need to map it to glyph positions
                        // Note: Bevy's glyphs may not have 1:1 correspondence with characters

                        let cursor_x = if buffer.cursor_pos == 0 || layout.glyphs.is_empty() {
                            // At the beginning or empty text
                            0.0
                        } else {
                            // Find the glyph that corresponds to our character position
                            // For now, we'll assume 1:1 mapping (will need refinement for complex text)
                            let glyph_index = (buffer.cursor_pos - 1).min(layout.glyphs.len() - 1);

                            // Position after the glyph at the calculated index
                            if let Some(glyph) = layout.glyphs.get(glyph_index) {
                                glyph.position.x + glyph.size.x
                            } else {
                                // Fallback to end of text
                                layout.glyphs.last()
                                    .map(|g| g.position.x + g.size.x)
                                    .unwrap_or(0.0)
                            }
                        };

                        // Update cursor style and position
                        // Add padding offset (10px default) to align with text
                        let padding_offset = 10.0;
                        match cursor_visual.style {
                            CursorStyle::Line => {
                                node.width = Val::Px(2.0);
                                node.height = Val::Percent(100.0);
                                node.left = Val::Px(cursor_x + padding_offset);
                                node.top = Val::Px(0.0);
                            }
                            CursorStyle::Block => {
                                // Get character width at cursor position
                                let char_width = if buffer.cursor_pos < layout.glyphs.len() {
                                    layout.glyphs.get(buffer.cursor_pos)
                                        .map(|g| g.size.x)
                                        .unwrap_or(10.0)
                                } else {
                                    10.0 // Default width for end of text
                                };

                                node.width = Val::Px(char_width);
                                node.height = Val::Percent(100.0);
                                node.left = Val::Px(cursor_x + padding_offset);
                                node.top = Val::Px(0.0);

                                // Make block cursor semi-transparent
                                bg_color.0 = input_visual.cursor_color.with_alpha(0.5);
                            }
                            CursorStyle::Underline => {
                                // Get character width at cursor position
                                let char_width = if buffer.cursor_pos < layout.glyphs.len() {
                                    layout.glyphs.get(buffer.cursor_pos)
                                        .map(|g| g.size.x)
                                        .unwrap_or(10.0)
                                } else {
                                    10.0 // Default width for end of text
                                };

                                node.width = Val::Px(char_width);
                                node.height = Val::Px(2.0);
                                node.left = Val::Px(cursor_x + padding_offset);
                                node.bottom = Val::Px(0.0);
                                node.top = Val::Auto;
                            }
                        }

                        return; // Exit all loops once we've updated the cursor
                    }
                }
            }
        }
    }
}

/// Render selection highlight
pub fn render_selection(
    mut text_inputs: Query<(
        Entity,
        &SelectionState,
        &TextInputVisual,
        &Children,
        &mut CursorVisual,
    ), (With<NativeTextInput>, Or<(Changed<SelectionState>, Changed<TextBuffer>)>)>,
    text_query: Query<&TextLayoutInfo, With<TextInputInner>>,
    mut commands: Commands,
    _children_query: Query<&Children>,
) {
    for (input_entity, selection, visual, children, mut cursor_visual) in text_inputs.iter_mut() {
        // Clean up existing selection entities
        for entity in cursor_visual.selection_entities.drain(..) {
            commands.entity(entity).despawn();
        }

        if selection.has_selection() {
            // Find text layout (direct child)
            for child in children.iter() {
                if let Ok(text_layout) = text_query.get(child) {
                    if let Some((start, end)) = selection.range() {
                        // Map character positions to glyph positions
                        // For now, assume 1:1 mapping (will need refinement for complex text)
                        let start_glyph = start.min(text_layout.glyphs.len());
                        let end_glyph = end.min(text_layout.glyphs.len());

                        // Calculate selection bounds
                        let start_x = if start_glyph == 0 {
                            0.0
                        } else if start_glyph > 0 {
                            text_layout.glyphs.get(start_glyph - 1)
                                .map(|g| g.position.x + g.size.x)
                                .unwrap_or(0.0)
                        } else {
                            0.0
                        };

                        let end_x = if end_glyph == 0 {
                            0.0
                        } else if end_glyph > 0 && end_glyph <= text_layout.glyphs.len() {
                            text_layout.glyphs.get(end_glyph - 1)
                                .map(|g| g.position.x + g.size.x)
                                .unwrap_or_else(|| {
                                    text_layout.glyphs.last()
                                        .map(|g| g.position.x + g.size.x)
                                        .unwrap_or(0.0)
                                })
                        } else {
                            text_layout.glyphs.last()
                                .map(|g| g.position.x + g.size.x)
                                .unwrap_or(0.0)
                        };

                        // Add padding offset to align with text
                        let padding_offset = 10.0;

                        // Spawn selection overlay as child of the input
                        let selection_entity = commands.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(start_x + padding_offset),
                                top: Val::Px(0.0),
                                width: Val::Px(end_x - start_x),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            BackgroundColor(visual.selection_color),
                            ZIndex(-1), // Behind text but above background
                            TextInputSelection {
                                input_entity,
                            },
                            Name::new("TextInputSelection"),
                        )).id();

                        // Parent it to the main input entity
                        commands.entity(input_entity).add_child(selection_entity);

                        // Track the selection entity
                        cursor_visual.selection_entities.push(selection_entity);
                    }
                    break;
                }
            }
        }
    }
}