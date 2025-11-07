//! Scrolling systems for handling mouse wheel input and visual feedback

use bevy::prelude::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::ui::ComputedNode;
use super::types::*;
use super::builder::ScrollbarThumb;

/// Handle mouse wheel scrolling for scrollable containers
pub fn handle_mouse_wheel_scroll(
    mut scroll_events: MessageReader<MouseWheel>,
    mut scroll_query: Query<(&mut ScrollPosition, &ScrollConfig, &Interaction), With<ScrollView>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut scrollbar_state: Query<&mut ScrollbarState>,
) {
    // Accumulate all scroll events
    let mut total_delta = Vec2::ZERO;

    for event in scroll_events.read() {
        let delta = match event.unit {
            MouseScrollUnit::Line => Vec2::new(event.x * 21.0, event.y * 21.0), // Standard line height
            MouseScrollUnit::Pixel => Vec2::new(event.x, event.y),
        };
        total_delta += delta;
    }

    if total_delta == Vec2::ZERO {
        return;
    }

    // Check for shift key (horizontal scroll)
    let shift_held = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);

    // Find hovered scroll container
    for (mut position, config, interaction) in scroll_query.iter_mut() {
        if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
            // Apply scroll with sensitivity
            if shift_held {
                position.0.x -= total_delta.x * config.scroll_sensitivity;
            } else {
                position.0.y -= total_delta.y * config.scroll_sensitivity;
            }

            // Clamp to valid range (Bevy handles this internally but we help)
            position.0.x = position.0.x.max(0.0);
            position.0.y = position.0.y.max(0.0);

            // Reset scrollbar visibility timer
            for mut state in scrollbar_state.iter_mut() {
                state.time_since_interaction = 0.0;
            }

            break; // Only scroll one container
        }
    }
}

/// Update scrollbar visibility and opacity based on configuration
pub fn update_scrollbar_visibility(
    mut scrollbar_query: Query<(&mut BackgroundColor, &mut ScrollbarState)>,
    scroll_query: Query<&ScrollConfig, With<ScrollView>>,
    mut thumb_query: Query<&mut BackgroundColor, (With<ScrollbarThumb>, Without<ScrollbarState>)>,
    time: Res<Time>,
) {
    for (mut scrollbar_bg, mut state) in scrollbar_query.iter_mut() {
        // Get config from target container
        let Ok(config) = scroll_query.get(state.scroll_container) else { continue };

        // Update timer
        state.time_since_interaction += time.delta_secs();

        // Calculate target opacity based on visibility mode
        let target_opacity = match config.scrollbar_visibility {
            ScrollbarVisibility::Always => 1.0,
            ScrollbarVisibility::AutoHide { timeout_secs } => {
                if state.time_since_interaction < timeout_secs {
                    0.5
                } else {
                    0.0
                }
            }
            ScrollbarVisibility::OnHover => {
                if state.time_since_interaction < 0.1 {
                    0.5
                } else {
                    0.0
                }
            }
            ScrollbarVisibility::Never => 0.0,
        };

        // Smooth opacity transition
        let fade_speed = 5.0;
        state.opacity += (target_opacity - state.opacity) * fade_speed * time.delta_secs();

        // Update scrollbar track color
        scrollbar_bg.0 = scrollbar_bg.0.with_alpha(state.opacity * 0.2);

        // Update thumb color (find thumb child)
        for mut thumb_bg in thumb_query.iter_mut() {
            thumb_bg.0 = thumb_bg.0.with_alpha(state.opacity * 0.6);
        }
    }
}

/// Update scrollbar thumb position and size based on scroll position
pub fn update_scrollbar_thumb_position(
    scrollbar_query: Query<(&ScrollbarState, &Children)>,
    scroll_query: Query<(&ScrollPosition, &ComputedNode, &Children), With<ScrollView>>,
    mut thumb_query: Query<&mut Node, With<ScrollbarThumb>>,
    child_query: Query<&ComputedNode, Without<ScrollView>>,
) {
    for (scrollbar_state, scrollbar_children) in scrollbar_query.iter() {
        // Get the scroll container this scrollbar belongs to
        let Ok((position, scroll_computed, scroll_children)) = scroll_query.get(scrollbar_state.scroll_container) else { continue };

        let viewport_height = scroll_computed.size().y;
        let current_scroll = position.0.y;

        // Calculate total content height
        let mut content_height: f32 = 0.0;
        for child in scroll_children.iter() {
            if let Ok(child_computed) = child_query.get(child) {
                content_height += child_computed.size().y;
            }
        }

        // Only show scrollbar if content overflows
        if content_height <= viewport_height {
            continue;
        }

        // Calculate thumb size as percentage
        let thumb_size_percentage = (viewport_height / content_height * 100.0).min(100.0).max(5.0);

        // Calculate thumb position as percentage
        let max_scroll = (content_height - viewport_height).max(0.0);
        let scroll_percentage = if max_scroll > 0.0 {
            (current_scroll / max_scroll * 100.0).min(100.0).max(0.0)
        } else {
            0.0
        };

        // Available travel distance for thumb (track height - thumb height)
        let available_travel = 100.0 - thumb_size_percentage;
        let thumb_top_percentage = (scroll_percentage / 100.0 * available_travel).min(available_travel).max(0.0);

        // Update thumb node
        for thumb_child in scrollbar_children.iter() {
            if let Ok(mut thumb_node) = thumb_query.get_mut(thumb_child) {
                thumb_node.top = Val::Percent(thumb_top_percentage);
                thumb_node.height = Val::Percent(thumb_size_percentage);
            }
        }
    }
}

/// Handle scrollbar thumb dragging
pub fn handle_scrollbar_thumb_drag(
    mut scroll_query: Query<(&mut ScrollPosition, &ComputedNode, &Children), With<ScrollView>>,
    thumb_query: Query<(&Interaction, &ChildOf), (With<ScrollbarThumb>, Changed<Interaction>)>,
    scrollbar_query: Query<(&ScrollbarState, &ComputedNode)>,
    child_query: Query<&ComputedNode, Without<ScrollView>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut drag_state: Local<Option<(Entity, f32, f32, f32)>>, // (container_entity, initial_mouse_y, initial_scroll_y, last_known_mouse_y)
) {
    let Ok(window) = windows.single() else { return };

    // Check for thumb interaction changes - start dragging
    for (interaction, child_of) in thumb_query.iter() {
        if *interaction == Interaction::Pressed && mouse_button.just_pressed(MouseButton::Left) {
            // Start dragging - find the scroll container (parent is the scrollbar)
            if let Ok((scrollbar_state, _)) = scrollbar_query.get(child_of.parent()) {
                if let Some(cursor_pos) = window.cursor_position() {
                    // Get initial scroll position
                    if let Ok((position, _, _)) = scroll_query.get(scrollbar_state.scroll_container) {
                        *drag_state = Some((scrollbar_state.scroll_container, cursor_pos.y, position.0.y, cursor_pos.y));
                    }
                }
            }
        }
    }

    // Handle ongoing drag - continues even when mouse leaves window
    if mouse_button.pressed(MouseButton::Left) {
        if let Some((container_entity, initial_mouse_y, initial_scroll_y, mut last_known_mouse_y)) = *drag_state {
            // Get current cursor position if available, otherwise use last known position
            let current_mouse_y = if let Some(cursor_pos) = window.cursor_position() {
                last_known_mouse_y = cursor_pos.y; // Update last known position
                cursor_pos.y
            } else {
                last_known_mouse_y // Cursor outside window, use last known position
            };

            // Find the scrollbar for this container
            for (scrollbar_state, scrollbar_computed) in scrollbar_query.iter() {
                if scrollbar_state.scroll_container == container_entity {
                    let track_height = scrollbar_computed.size().y;

                    if let Ok((mut position, scroll_computed, scroll_children)) = scroll_query.get_mut(container_entity) {
                        let viewport_height = scroll_computed.size().y;

                        // Calculate content height
                        let mut content_height: f32 = 0.0;
                        for child in scroll_children.iter() {
                            if let Ok(child_computed) = child_query.get(child) {
                                content_height += child_computed.size().y;
                            }
                        }

                        let max_scroll = (content_height - viewport_height).max(0.0);

                        // Industry standard 1:1 mapping: dragging thumb full range scrolls full content
                        // Calculate how much the mouse moved in pixels
                        let mouse_delta = current_mouse_y - initial_mouse_y;

                        // Convert to scroll offset with 1:1 track-to-content ratio
                        if track_height > 0.0 {
                            let scroll_ratio = max_scroll / track_height;
                            let new_scroll = initial_scroll_y + (mouse_delta * scroll_ratio);
                            position.0.y = new_scroll.clamp(0.0, max_scroll);
                        }
                    }

                    // Update drag state with new last known mouse position
                    *drag_state = Some((container_entity, initial_mouse_y, initial_scroll_y, last_known_mouse_y));
                    break;
                }
            }
        }
    } else {
        // Mouse released - stop dragging
        *drag_state = None;
    }
}

/// Handle keyboard navigation for scroll containers
pub fn handle_keyboard_scroll(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut scroll_query: Query<(&mut ScrollPosition, &ScrollConfig, &ComputedNode, &Interaction), With<ScrollView>>,
) {
    for (mut position, config, computed, interaction) in scroll_query.iter_mut() {
        // Only scroll if hovered or has focus
        if *interaction != Interaction::Hovered && *interaction != Interaction::Pressed {
            continue;
        }

        let viewport_height = computed.size().y;
        let scroll_amount = 40.0; // Small increment

        if keyboard.just_pressed(KeyCode::PageDown) {
            position.0.y += viewport_height * config.scroll_sensitivity;
        } else if keyboard.just_pressed(KeyCode::PageUp) {
            position.0.y -= viewport_height * config.scroll_sensitivity;
            position.0.y = position.0.y.max(0.0);
        } else if keyboard.just_pressed(KeyCode::Home) {
            position.0.y = 0.0;
        } else if keyboard.just_pressed(KeyCode::End) {
            // Scroll to bottom (Bevy will clamp)
            position.0.y = f32::MAX;
        } else if keyboard.pressed(KeyCode::ArrowDown) {
            position.0.y += scroll_amount * config.scroll_sensitivity;
        } else if keyboard.pressed(KeyCode::ArrowUp) {
            position.0.y -= scroll_amount * config.scroll_sensitivity;
            position.0.y = position.0.y.max(0.0);
        }
    }
}

/// Auto-scroll to focused text inputs to keep them visible
pub fn auto_scroll_to_focused_input(
    mut scroll_query: Query<(&mut ScrollPosition, &ScrollConfig, &ComputedNode, &Children), With<ScrollView>>,
    input_query: Query<(&GlobalTransform, &ComputedNode), With<crate::text_input::NativeTextInput>>,
    focus_query: Query<&crate::text_input::TextBuffer>,
) {
    for (mut position, config, scroll_computed, children) in scroll_query.iter_mut() {
        if !config.auto_scroll_to_focus {
            continue;
        }

        let viewport_height = scroll_computed.size().y;
        let current_scroll = position.0.y;

        // Check each child for focus
        for child in children.iter() {
            if let Ok(buffer) = focus_query.get(child) {
                if buffer.is_focused {
                    if let Ok((input_transform, input_computed)) = input_query.get(child) {
                        // Get input position relative to scroll container
                        let input_y = input_transform.translation().y;
                        let input_height = input_computed.size().y;

                        // Check if input is outside viewport
                        if input_y < current_scroll {
                            // Scroll up to show input
                            position.0.y = input_y;
                        } else if input_y + input_height > current_scroll + viewport_height {
                            // Scroll down to show input
                            position.0.y = (input_y + input_height - viewport_height).max(0.0);
                        }
                    }
                }
            }
        }
    }
}

/// Apply kinetic scrolling with momentum
pub fn apply_kinetic_scrolling(
    mut scroll_query: Query<(&mut ScrollPosition, &ScrollConfig), With<ScrollView>>,
    mut kinetic_query: Query<&mut KineticScrollState>,
    time: Res<Time>,
) {
    for ((mut position, config), mut kinetic) in scroll_query.iter_mut().zip(kinetic_query.iter_mut()) {
        if !kinetic.active || !config.enable_kinetic_scroll {
            continue;
        }

        // Apply velocity
        position.0.x += kinetic.velocity.x * time.delta_secs();
        position.0.y += kinetic.velocity.y * time.delta_secs();

        // Clamp to valid range
        position.0.x = position.0.x.max(0.0);
        position.0.y = position.0.y.max(0.0);

        // Exponential decay for natural deceleration
        let decay_rate = 0.95_f32.powf(time.delta_secs() * 60.0); // Normalized to 60fps
        kinetic.velocity *= decay_rate;

        // Stop if velocity too low
        if kinetic.velocity.length() < 1.0 {
            kinetic.active = false;
            kinetic.velocity = Vec2::ZERO;
        }
    }
}

/// Handle drag-to-scroll interactions (basic implementation)
pub fn handle_drag_scroll(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut scroll_query: Query<(&mut ScrollPosition, &ScrollConfig, &Interaction, Entity), (With<ScrollView>, With<DragScrollTarget>)>,
    mut kinetic_query: Query<&mut KineticScrollState>,
    mut last_mouse_pos: Local<Option<Vec2>>,
    windows: Query<&Window>,
    time: Res<Time>,
) {
    let Ok(window) = windows.single() else { return };
    let Some(cursor_pos) = window.cursor_position() else {
        *last_mouse_pos = None;
        return;
    };

    // Check if mouse button pressed
    if mouse_button.pressed(MouseButton::Left) {
        for (mut position, config, interaction, entity) in scroll_query.iter_mut() {
            if !config.enable_drag_scroll {
                continue;
            }

            if *interaction == Interaction::Pressed {
                // Calculate drag delta
                if let Some(last_pos) = *last_mouse_pos {
                    let delta = cursor_pos - last_pos;

                    // Scroll in opposite direction of drag (natural scrolling)
                    position.0.x -= delta.x;
                    position.0.y -= delta.y;

                    // Clamp
                    position.0.x = position.0.x.max(0.0);
                    position.0.y = position.0.y.max(0.0);

                    // Update velocity for kinetic scrolling
                    if let Ok(mut kinetic) = kinetic_query.get_mut(entity) {
                        kinetic.velocity = -delta / time.delta_secs();
                        kinetic.last_position = Some(cursor_pos);
                        kinetic.last_time = Some(time.elapsed_secs());
                    }
                }

                *last_mouse_pos = Some(cursor_pos);
            }
        }
    } else {
        // Mouse released - activate kinetic scrolling
        if last_mouse_pos.is_some() {
            for (_, config, _, entity) in scroll_query.iter() {
                if config.enable_kinetic_scroll {
                    if let Ok(mut kinetic) = kinetic_query.get_mut(entity) {
                        kinetic.active = true;
                    }
                }
            }
        }
        *last_mouse_pos = None;
    }
}

/// Clamp scroll position to valid bounds based on content size
pub fn clamp_scroll_bounds(
    mut scroll_query: Query<(&mut ScrollPosition, &ComputedNode, &Children), (With<ScrollView>, Changed<ScrollPosition>)>,
    child_query: Query<&ComputedNode, Without<ScrollView>>,
) {
    for (mut position, scroll_computed, children) in scroll_query.iter_mut() {
        let viewport_size = scroll_computed.size();

        // Calculate total content size
        let mut max_content_x: f32 = 0.0;
        let mut max_content_y: f32 = 0.0;

        for child in children.iter() {
            if let Ok(child_computed) = child_query.get(child) {
                let child_size = child_computed.size();
                max_content_x = max_content_x.max(child_size.x);
                max_content_y += child_size.y; // Assuming vertical layout
            }
        }

        // Calculate max offset
        let max_offset_x = (max_content_x - viewport_size.x).max(0.0);
        let max_offset_y = (max_content_y - viewport_size.y).max(0.0);

        // Clamp position
        position.0.x = position.0.x.clamp(0.0, max_offset_x);
        position.0.y = position.0.y.clamp(0.0, max_offset_y);
    }
}
