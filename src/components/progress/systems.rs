//! Progress bar update systems

use bevy::prelude::*;
use super::types::*;

/// System to update progress bar fills when value changes
pub fn update_progress_bars(
    mut bars: Query<(Entity, &ProgressBar), Changed<ProgressBar>>,
    children_query: Query<&Children>,
    mut fills: Query<&mut Node, With<ProgressBarFill>>,
    mut labels: Query<&mut Text, With<ProgressBarLabel>>,
) {
    for (entity, bar) in &mut bars {
        // Use the recursive helper to find and update fills/labels
        find_and_update_fill(entity, bar.value, &children_query, &mut fills, &mut labels);
    }
}

/// Recursively find progress bar fill in children hierarchy
fn find_and_update_fill(
    entity: Entity,
    value: f32,
    children_query: &Query<&Children>,
    fills: &mut Query<&mut Node, With<ProgressBarFill>>,
    labels: &mut Query<&mut Text, With<ProgressBarLabel>>,
) {
    // Try to update this entity if it's a fill
    if let Ok(mut fill_node) = fills.get_mut(entity) {
        let new_width = Val::Percent(value * 100.0);
        if fill_node.width != new_width {
            fill_node.width = new_width;
        }
    }

    // Try to update this entity if it's a label
    if let Ok(mut label_text) = labels.get_mut(entity) {
        let new_text = format!("{}%", (value * 100.0) as i32);
        if **label_text != new_text {
            **label_text = new_text;
        }
    }

    // Recursively check children
    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            find_and_update_fill(child, value, children_query, fills, labels);
        }
    }
}

/// Force update all progress bars regardless of change detection (for debugging)
pub fn force_update_progress_bars(
    bars: Query<(Entity, &ProgressBar)>,
    children_query: Query<&Children>,
    mut fills: Query<&mut Node, With<ProgressBarFill>>,
    mut labels: Query<&mut Text, With<ProgressBarLabel>>,
) {
    for (entity, bar) in bars.iter() {
        // Recursively search for fill and label components in the hierarchy
        find_and_update_fill(entity, bar.value, &children_query, &mut fills, &mut labels);
    }
}