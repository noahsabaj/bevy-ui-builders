//! Slider interaction systems

use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use super::types::*;

/// Handle slider dragging interaction
pub fn handle_slider_interaction(
    mut sliders: Query<
        (
            Entity,
            &Interaction,
            &mut Slider,
            &Node,
            &RelativeCursorPosition,
            &Children,
        ),
        With<SliderTrack>,
    >,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut dragged_slider: Local<Option<Entity>>,
) {
    // If mouse was released, stop dragging
    if !mouse_button.pressed(MouseButton::Left) {
        *dragged_slider = None;
    }

    for (entity, interaction, mut slider, _node, cursor_pos, _children) in &mut sliders {
        // Start dragging on press
        if *interaction == Interaction::Pressed
            && mouse_button.pressed(MouseButton::Left)
            && dragged_slider.is_none()
        {
            *dragged_slider = Some(entity);
        }

        if *dragged_slider == Some(entity) {
            if let Some(cursor_pos) = cursor_pos.normalized {
                let normalized_x = cursor_pos.x.clamp(0.0, 1.0);
                slider.set_normalized(normalized_x);
            }
        }
    }
}

/// Update slider visuals when value changes
pub fn update_slider_visuals(
    sliders: Query<(&Slider, &SliderConfig, &Children), Changed<Slider>>,
    mut fills: Query<&mut Node, (With<SliderFill>, Without<SliderHandle>)>,
    mut handles: Query<&mut Node, (With<SliderHandle>, Without<SliderFill>)>,
    mut value_texts: Query<&mut Text>,
) {
    for (slider, config, children) in &sliders {
        for child in children.iter() {
            if let Ok(mut fill_node) = fills.get_mut(child) {
                fill_node.width = Val::Percent(slider.normalized() * 100.0);
            }

            if let Ok(mut handle_node) = handles.get_mut(child) {
                let handle_offset = slider.normalized() * 100.0;
                handle_node.left = Val::Percent(handle_offset.min(95.0));
            }
        }

        if let Some(value_text_entity) = slider.value_text_entity {
            if let Ok(mut text) = value_texts.get_mut(value_text_entity) {
                **text = config.value_format.format(slider.value);
            }
        }
    }
}

/// Handle clicks on slider increment/decrement buttons
pub fn handle_slider_button_clicks(
    button_query: Query<(&Interaction, &SliderButtonAction), (Changed<Interaction>, With<Button>)>,
    mut slider_query: Query<&mut Slider>,
) {
    for (interaction, action) in &button_query {
        if *interaction == Interaction::Pressed {
            if let Ok(mut slider) = slider_query.get_mut(action.slider_entity) {
                let new_value = (slider.value + action.delta).clamp(slider.min, slider.max);

                // Apply step if configured
                if let Some(step) = slider.step {
                    let steps = ((new_value - slider.min) / step).round();
                    slider.value = slider.min + steps * step;
                } else {
                    slider.value = new_value;
                }
            }
        }
    }
}