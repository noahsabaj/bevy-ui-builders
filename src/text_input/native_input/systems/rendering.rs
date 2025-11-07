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