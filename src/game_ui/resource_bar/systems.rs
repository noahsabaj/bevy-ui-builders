//! Resource bar systems

use bevy::prelude::*;
use super::types::*;

/// System to animate resource bar fill
pub fn animate_resource_bar_fill(
    time: Res<Time>,
    settings: Res<ResourceBarSettings>,
    bar_query: Query<&ResourceBar>,
    mut fill_query: Query<(&mut ResourceBarFill, &mut Node)>,
) {
    for (mut fill, mut node) in fill_query.iter_mut() {
        let Ok(bar) = bar_query.get(fill.bar) else {
            continue;
        };

        let target = bar.percentage();
        fill.target_percentage = target;

        if bar.animated {
            // Smoothly animate towards target
            let diff = target - fill.display_percentage;
            if diff.abs() > 0.001 {
                fill.display_percentage += diff * settings.animation_speed * time.delta_secs();
                fill.display_percentage = fill.display_percentage.clamp(0.0, 1.0);
            } else {
                fill.display_percentage = target;
            }
        } else {
            fill.display_percentage = target;
        }

        node.width = Val::Percent(fill.display_percentage * 100.0);
    }
}

/// System to animate damage indicator
pub fn animate_damage_indicator(
    time: Res<Time>,
    settings: Res<ResourceBarSettings>,
    bar_query: Query<&ResourceBar>,
    fill_query: Query<&ResourceBarFill>,
    mut indicator_query: Query<(&mut ResourceBarDamageIndicator, &mut Node)>,
) {
    for (mut indicator, mut node) in indicator_query.iter_mut() {
        let Ok(bar) = bar_query.get(indicator.bar) else {
            continue;
        };

        // Get current fill percentage
        let current_fill = fill_query
            .iter()
            .find(|f| f.bar == indicator.bar)
            .map(|f| f.display_percentage)
            .unwrap_or(bar.percentage());

        // If health decreased, show indicator at old position
        if indicator.display_percentage > current_fill {
            // Reset delay timer
            indicator.delay_timer = settings.damage_indicator_delay;
        }

        // Update delay timer
        if indicator.delay_timer > 0.0 {
            indicator.delay_timer -= time.delta_secs();
        } else {
            // Shrink indicator towards current fill
            let diff = current_fill - indicator.display_percentage;
            if diff.abs() > 0.001 {
                indicator.display_percentage += diff * settings.animation_speed * 0.5 * time.delta_secs();
                indicator.display_percentage = indicator.display_percentage.clamp(0.0, 1.0);
            } else {
                indicator.display_percentage = current_fill;
            }
        }

        node.width = Val::Percent(indicator.display_percentage * 100.0);
    }
}

/// System to update resource bar value (called by your game code)
pub fn update_resource_bar_value(
    mut bar_query: Query<&mut ResourceBar>,
    mut change_events: MessageWriter<ResourceBarChanged>,
) {
    // This system is a helper - games should modify ResourceBar.value directly
    // and this system will emit events for tracking
    for bar in bar_query.iter() {
        // The value is updated externally - we just need to make sure
        // the fill and indicator systems pick up the changes
    }
}
