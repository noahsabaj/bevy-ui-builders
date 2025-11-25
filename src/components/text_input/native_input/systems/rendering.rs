//! Text and cursor rendering systems

use bevy::prelude::*;
use bevy::text::TextLayoutInfo;

use super::super::components::*;

/// Render text with embedded cursor (split into 3 spans)
pub fn render_text(
    mut text_inputs: Query<(
        &TextBuffer,
        &SelectionState,
        &TextInputVisual,
        &CursorVisual,
        &Children,
    ), With<NativeTextInput>>,
    text_inner_query: Query<Entity, With<TextInputInner>>,
    mut text_span_query: Query<(&mut TextSpan, &mut TextFont, &mut TextColor)>,
    children_query: Query<&Children>,
) {
    for (buffer, _selection, visual, cursor_visual, children) in text_inputs.iter_mut() {
        info!("render_text: buffer content = '{}', cursor_pos = {}", buffer.content, buffer.cursor_pos);

        // Find the TextInputInner entity (direct child)
        let mut text_inner_entity = None;
        for child in children.iter() {
            if text_inner_query.get(child).is_ok() {
                text_inner_entity = Some(child);
                break;
            }
        }

        if let Some(text_inner_entity) = text_inner_entity {
            // Get the 3 TextSpan children
            if let Ok(text_children) = children_query.get(text_inner_entity) {
                let text_children_vec: Vec<Entity> = text_children.iter().collect();

                if text_children_vec.len() >= 3 {
                    // Prepare the display text with optional masking
                    let display_text = if buffer.content.is_empty() && !buffer.is_focused {
                        // Show placeholder in first span
                        visual.placeholder.clone()
                    } else if let Some(mask) = visual.mask_char {
                        mask.to_string().repeat(buffer.content.chars().count())
                    } else {
                        buffer.content.clone()
                    };

                    // Split text at cursor position
                    let cursor_pos = buffer.cursor_pos.min(display_text.chars().count());
                    let before_cursor: String = display_text.chars().take(cursor_pos).collect();
                    let after_cursor: String = display_text.chars().skip(cursor_pos).collect();

                    // Determine cursor character
                    let cursor_char = if buffer.is_focused && cursor_visual.visible {
                        "|".to_string()
                    } else {
                        "".to_string()
                    };

                    // Update pre-cursor span (index 0)
                    if let Ok((mut text_span, mut font, mut color)) = text_span_query.get_mut(text_children_vec[0]) {
                        *text_span = TextSpan::new(before_cursor);
                        *font = visual.font.clone();
                        color.0 = if buffer.content.is_empty() && !buffer.is_focused {
                            visual.placeholder_color
                        } else {
                            visual.text_color
                        };
                    }

                    // Update cursor span (index 1)
                    if let Ok((mut text_span, mut font, mut color)) = text_span_query.get_mut(text_children_vec[1]) {
                        *text_span = TextSpan::new(cursor_char);
                        *font = visual.font.clone();
                        color.0 = visual.cursor_color;  // Always use cursor color
                    }

                    // Update post-cursor span (index 2)
                    if let Ok((mut text_span, mut font, mut color)) = text_span_query.get_mut(text_children_vec[2]) {
                        *text_span = TextSpan::new(after_cursor);
                        *font = visual.font.clone();
                        color.0 = visual.text_color;
                    }
                } else {
                    warn!("TextInputInner doesn't have 3 TextSpan children!");
                }
            }
        } else {
            warn!("Could not find text inner entity for input!");
        }
    }
}

// update_cursor_visual removed - cursor is now embedded in text spans

/// Render selection highlight
pub fn render_selection(
    mut commands: Commands,
    mut text_inputs: Query<(Entity, &Children, &SelectionState, &mut CursorVisual), With<NativeTextInput>>,
    text_inner_query: Query<(&TextLayoutInfo, &Children), With<TextInputInner>>,
    text_span_query: Query<&TextSpan>,
    primary_window: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    let scale_factor = primary_window.iter().next().map(|w| w.scale_factor()).unwrap_or(1.0);

    for (input_entity, children, selection, mut cursor_visual) in &mut text_inputs {
        // Clean up existing selection entities
        for entity in cursor_visual.selection_entities.drain(..) {
            commands.entity(entity).despawn();
        }

        if selection.has_selection() {
            // Find text layout (direct child)
            for child in children.iter() {
                if let Ok((text_layout, text_spans)) = text_inner_query.get(child) {
                    // Calculate selection bounds based on glyph positions
                    if let Some((start, end)) = selection.range() {
                        // We need to map character indices to glyph indices.
                        // Because of the 3-span structure (Pre, Cursor, Post), and the cursor being a separate span/glyph,
                        // the mapping is tricky.
                        // However, TextLayoutInfo aggregates all glyphs.
                        // If cursor is present (focused), we have an extra glyph at cursor_pos.

                        // Check if cursor is actually present in the text layout
                        // The cursor span is index 1.
                        let cursor_present = if let Some(cursor_entity) = text_spans.get(1) {
                             if let Ok(span) = text_span_query.get(*cursor_entity) {
                                 !span.0.is_empty()
                             } else {
                                 false
                             }
                        } else {
                            false
                        };

                        let cursor_pos = selection.cursor; // This is the cursor position in the buffer

                        // Map logical char index to glyph index
                        // If cursor is present at `cursor_pos`, then:
                        // Indices < cursor_pos map to same glyph index
                        // Indices >= cursor_pos map to index + 1
                        let map_index = |idx: usize, is_end: bool| -> usize {
                            if cursor_present {
                                if idx > cursor_pos || (idx == cursor_pos && !is_end) {
                                    idx + 1
                                } else {
                                    idx
                                }
                            } else {
                                idx
                            }
                        };

                        let start_glyph = map_index(start, false).min(text_layout.glyphs.len());
                        let end_glyph = map_index(end, true).min(text_layout.glyphs.len());

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

                        // Apply scale factor correction
                        let start_x = start_x / scale_factor;
                        let end_x = end_x / scale_factor;

                        // Add padding offset to align with text
                        let padding_offset = 10.0;

                        let width = end_x - start_x;
                        
                        if width > 0.0 {
                            let selection_entity = commands.spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(start_x + padding_offset),
                                    top: Val::Px(padding_offset), // Assuming vertical padding is also 10
                                    width: Val::Px(width),
                                    height: Val::Px(20.0), // Approximate line height
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(0.3, 0.5, 0.8, 0.3)), // Selection color
                                TextInputSelection { input_entity },
                                ZIndex(-1), // Render behind text
                            )).id();

                            cursor_visual.selection_entities.push(selection_entity);
                            
                            // Parent the selection to the input container so it moves with it
                            commands.entity(input_entity).add_child(selection_entity);
                        }
                    }
                    break; // Found the inner text, stop looking
                }
            }
        }
    }
}