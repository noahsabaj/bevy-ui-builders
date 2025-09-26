//! Scrolling systems for handling mouse wheel input and visual feedback

use bevy::prelude::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::picking::pointer::PointerInteraction;
use super::types::*;

/// Handle mouse wheel scrolling for scrollable containers
pub fn handle_mouse_wheel_scroll(
    mut scroll_events: EventReader<MouseWheel>,
    mut scroll_query: Query<(&mut ScrollState, &Node, &ScrollConfig, &GlobalTransform)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    // Get primary window
    let Ok(window) = window_query.get_single() else { return; };

    // Get camera for viewport calculations
    let Ok((camera, camera_transform)) = camera_query.get_single() else { return; };

    // Check if we have any scroll events
    let mut total_delta_y = 0.0;
    let mut total_delta_x = 0.0;

    for event in scroll_events.read() {
        // Calculate scroll delta based on unit type
        let delta = match event.unit {
            MouseScrollUnit::Line => {
                Vec2::new(event.x * 20.0, event.y * 20.0) // Line height approximation
            }
            MouseScrollUnit::Pixel => {
                Vec2::new(event.x, event.y)
            }
        };

        total_delta_x += delta.x;
        total_delta_y += delta.y;
    }

    // If no scroll events, return early
    if total_delta_x == 0.0 && total_delta_y == 0.0 {
        return;
    }

    // Get cursor position
    let Some(cursor_position) = window.cursor_position() else { return; };

    // Convert cursor position to world coordinates
    let Ok(cursor_world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) else { return; };

    // Check which scrollable container the cursor is over
    for (mut scroll_pos, node, config, transform) in scroll_query.iter_mut() {
        // Simple AABB check for hover detection
        let node_pos = transform.translation().truncate();
        // Estimate node size from computed values (we'll improve this later)
        let node_size = Vec2::new(200.0, 200.0); // Temporary approximation

        let min = node_pos - node_size / 2.0;
        let max = node_pos + node_size / 2.0;

        if cursor_world_pos.x >= min.x && cursor_world_pos.x <= max.x &&
           cursor_world_pos.y >= min.y && cursor_world_pos.y <= max.y {

            // Check if shift is held for horizontal scrolling
            let shift_held = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);

            // Apply scroll with sensitivity
            if shift_held {
                // Horizontal scroll
                scroll_pos.offset.x = (scroll_pos.offset.x - total_delta_x * config.sensitivity)
                    .max(0.0);
            } else {
                // Vertical scroll (inverted for natural scrolling)
                scroll_pos.offset.y = (scroll_pos.offset.y - total_delta_y * config.sensitivity)
                    .max(0.0);
            }

            // Mark as recently scrolled for scrollbar visibility
            scroll_pos.last_scroll_time = 0.0;

            // Only scroll one container at a time
            break;
        }
    }
}

/// Update scrollbar visual elements based on scroll position
pub fn update_scrollbar_visuals(
    mut scrollbar_query: Query<(&mut Visibility, &mut Node, &ScrollBarThumb)>,
    mut scroll_query: Query<(&mut ScrollState, &Node, &Children), Without<ScrollBarThumb>>,
    time: Res<Time>,
) {
    for (mut scroll_pos, container_node, children) in scroll_query.iter_mut() {
        // Update last scroll time
        scroll_pos.last_scroll_time += time.delta_secs();
        let time_since_scroll = scroll_pos.last_scroll_time;

        // Find scrollbar thumb in children
        for child in children.iter() {
            if let Ok((mut visibility, mut thumb_node, _)) = scrollbar_query.get_mut(child) {
                // Calculate scrollbar visibility
                if time_since_scroll < 1.0 {
                    *visibility = Visibility::Visible;

                    // Calculate thumb position and size
                    // TODO: Get actual content and viewport sizes
                    let content_height = 800.0; // Temporary
                    let viewport_height = 600.0; // Temporary

                    if content_height > viewport_height {
                        let scroll_percentage = scroll_pos.offset.y / (content_height - viewport_height);
                        let thumb_size_percentage = viewport_height / content_height * 100.0;

                        thumb_node.top = Val::Percent(scroll_percentage * (100.0 - thumb_size_percentage));
                        thumb_node.height = Val::Percent(thumb_size_percentage);
                    }
                } else if time_since_scroll > 2.0 {
                    // Fade out scrollbar after 2 seconds of inactivity
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}

/// Handle scrollbar thumb dragging
pub fn handle_scrollbar_interaction(
    mut interaction_query: Query<(&Interaction, &ScrollBarThumb), Changed<Interaction>>,
    mut scroll_query: Query<&mut ScrollState>,
) {
    for (interaction, thumb) in interaction_query.iter() {
        if let Ok(mut scroll_pos) = scroll_query.get_mut(thumb.scroll_container) {
            match *interaction {
                Interaction::Pressed => {
                    // TODO: Implement drag handling
                    info!("Scrollbar drag started");
                }
                Interaction::Hovered => {
                    // Show scrollbar when hovered
                    scroll_pos.last_scroll_time = 0.0;
                }
                _ => {}
            }
        }
    }
}

/// Smoothly animate scroll position changes
pub fn smooth_scroll_animation(
    mut scroll_query: Query<(&mut ScrollState, &ScrollConfig)>,
    time: Res<Time>,
) {
    for (mut scroll_pos, config) in scroll_query.iter_mut() {
        // Smooth scroll to target if set
        if let Some(target) = scroll_pos.target_offset {
            let current = scroll_pos.offset;
            let diff = target - current;

            // Lerp towards target
            let t = (config.animation_duration * time.delta_secs()).min(1.0);
            scroll_pos.offset = current + diff * t;

            // Clear target when reached
            if diff.length() < 0.1 {
                scroll_pos.target_offset = None;
            }
        }
    }
}

/// Auto-scroll to focused text inputs
pub fn auto_scroll_to_focused_input(
    mut scroll_query: Query<(&mut ScrollState, &Node, &Children)>,
    input_query: Query<(&GlobalTransform, &Node), With<crate::text_input::NativeTextInput>>,
    focus_query: Query<&crate::text_input::TextBuffer>,
) {
    for (mut scroll_pos, container_node, children) in scroll_query.iter_mut() {
        // Check if any child input is focused
        for child in children.iter() {
            if let Ok(buffer) = focus_query.get(child) {
                if buffer.is_focused {
                    if let Ok((input_transform, input_node)) = input_query.get(child) {
                        // Calculate if input is visible in viewport
                        let input_y = input_transform.translation().y;
                        let viewport_height = 600.0; // Temporary approximation

                        // Scroll to make input visible
                        if input_y < scroll_pos.offset.y {
                            scroll_pos.target_offset = Some(Vec2::new(scroll_pos.offset.x, input_y));
                        } else if input_y > scroll_pos.offset.y + viewport_height {
                            scroll_pos.target_offset = Some(Vec2::new(
                                scroll_pos.offset.x,
                                input_y - viewport_height + 40.0 // Approximate input height
                            ));
                        }
                    }
                }
            }
        }
    }
}

/// Update maximum scroll limits based on content size
pub fn update_scroll_limits(
    mut scroll_query: Query<(&mut ScrollState, &Node, &ComputedNode), Changed<ComputedNode>>,
) {
    for (mut scroll_pos, node, computed) in scroll_query.iter_mut() {
        // Calculate maximum scroll based on content size vs viewport size
        let content_size = computed.size();
        let viewport_size = Vec2::new(600.0, 400.0); // Temporary approximation

        scroll_pos.max_offset = Vec2::new(
            (content_size.x - viewport_size.x).max(0.0),
            (content_size.y - viewport_size.y).max(0.0),
        );

        // Clamp current scroll to new limits
        scroll_pos.offset = scroll_pos.offset.min(scroll_pos.max_offset);
    }
}