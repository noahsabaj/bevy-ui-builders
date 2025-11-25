//! Tooltip systems

use bevy::prelude::*;
use crate::styles::dimensions;
use crate::theme::UiTheme;
use crate::traits::TooltipPosition;
use super::types::*;

// Default colors (dark theme) for when no theme resource is available
mod defaults {
    use bevy::prelude::Color;

    pub const BACKGROUND: Color = Color::srgb(0.12, 0.12, 0.12);
    pub const BORDER: Color = Color::srgb(0.3, 0.3, 0.3);
    pub const TEXT_PRIMARY: Color = Color::srgb(0.95, 0.95, 0.95);
    pub const TEXT_SECONDARY: Color = Color::srgb(0.7, 0.7, 0.7);
}

/// System to track hover time on elements with tooltips
pub fn track_tooltip_hover(
    mut query: Query<(&Interaction, &mut TooltipState), (With<HasTooltip>, Changed<Interaction>)>,
) {
    for (interaction, mut state) in query.iter_mut() {
        match interaction {
            Interaction::Hovered => {
                // Start tracking hover time
                if !state.visible {
                    state.hover_time = 0.0;
                }
            }
            Interaction::None | Interaction::Pressed => {
                // Reset hover tracking
                state.hover_time = 0.0;
            }
        }
    }
}

/// System to update hover time
pub fn update_tooltip_hover_time(
    time: Res<Time>,
    mut query: Query<(&Interaction, &mut TooltipState), With<HasTooltip>>,
) {
    for (interaction, mut state) in query.iter_mut() {
        if matches!(interaction, Interaction::Hovered) && !state.visible {
            state.hover_time += time.delta_secs();
        }
    }
}

/// System to show tooltips after delay
pub fn show_tooltips(
    mut commands: Commands,
    settings: Res<TooltipSettings>,
    theme: Option<Res<UiTheme>>,
    mut query: Query<(
        Entity,
        &HasTooltip,
        &mut TooltipState,
        &GlobalTransform,
        &ComputedNode,
        &Interaction,
    )>,
) {
    // Resolve colors from theme or defaults
    let (bg_color, border_color, text_primary, text_secondary) = if let Some(ref theme) = theme {
        (
            theme.colors.surface.tertiary,
            theme.colors.border.default,
            theme.colors.text.primary,
            theme.colors.text.secondary,
        )
    } else {
        (
            defaults::BACKGROUND,
            defaults::BORDER,
            defaults::TEXT_PRIMARY,
            defaults::TEXT_SECONDARY,
        )
    };

    for (entity, tooltip, mut state, transform, computed, interaction) in query.iter_mut() {
        // Only show when hovered and not already visible
        if !matches!(interaction, Interaction::Hovered) || state.visible {
            continue;
        }

        // Check if delay has passed
        let delay_secs = tooltip.delay.as_secs_f32();
        if state.hover_time < delay_secs {
            continue;
        }

        // Calculate position
        let element_pos = transform.translation().truncate();
        let element_size = computed.size();

        let tooltip_pos = calculate_tooltip_position(
            element_pos,
            element_size,
            tooltip.position,
            settings.offset,
        );

        // Spawn tooltip entity
        let tooltip_entity = commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(tooltip_pos.x),
                    top: Val::Px(tooltip_pos.y),
                    max_width: Val::Px(tooltip.max_width),
                    padding: UiRect::all(Val::Px(8.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.0),
                    ..default()
                },
                BackgroundColor(bg_color),
                BorderColor::all(border_color),
                BorderRadius::all(Val::Px(4.0)),
                GlobalZIndex(settings.z_index),
                TooltipEntity { target: entity },
            ))
            .with_children(|parent| {
                // Title/main text
                parent.spawn((
                    Text::new(tooltip.content.title()),
                    TextFont {
                        font_size: dimensions::FONT_SIZE_SMALL,
                        ..default()
                    },
                    TextColor(text_primary),
                ));

                // Description (if rich tooltip)
                if let Some(desc) = tooltip.content.description() {
                    parent.spawn((
                        Text::new(desc),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_SMALL,
                            ..default()
                        },
                        TextColor(text_secondary),
                    ));
                }
            })
            .id();

        state.visible = true;
        state.tooltip_entity = Some(tooltip_entity);
    }
}

/// System to hide tooltips when no longer hovering
pub fn hide_tooltips(
    mut commands: Commands,
    mut query: Query<(&Interaction, &mut TooltipState), With<HasTooltip>>,
) {
    for (interaction, mut state) in query.iter_mut() {
        if !matches!(interaction, Interaction::Hovered) && state.visible {
            // Despawn tooltip entity
            if let Some(tooltip_entity) = state.tooltip_entity.take() {
                commands.entity(tooltip_entity).despawn();
            }
            state.visible = false;
            state.hover_time = 0.0;
        }
    }
}

/// System to clean up tooltips when target entities are despawned
pub fn cleanup_orphaned_tooltips(
    mut commands: Commands,
    tooltip_query: Query<(Entity, &TooltipEntity)>,
    target_query: Query<Entity>,
) {
    for (entity, tooltip) in tooltip_query.iter() {
        if target_query.get(tooltip.target).is_err() {
            commands.entity(entity).despawn();
        }
    }
}

/// Calculate the position for a tooltip based on the element and preferred position
fn calculate_tooltip_position(
    element_pos: Vec2,
    element_size: Vec2,
    position: TooltipPosition,
    offset: f32,
) -> Vec2 {
    match position {
        TooltipPosition::Top => Vec2::new(
            element_pos.x + element_size.x / 2.0,
            element_pos.y - offset,
        ),
        TooltipPosition::Bottom => Vec2::new(
            element_pos.x + element_size.x / 2.0,
            element_pos.y + element_size.y + offset,
        ),
        TooltipPosition::Left => Vec2::new(
            element_pos.x - offset,
            element_pos.y + element_size.y / 2.0,
        ),
        TooltipPosition::Right => Vec2::new(
            element_pos.x + element_size.x + offset,
            element_pos.y + element_size.y / 2.0,
        ),
        TooltipPosition::Auto => {
            // Default to bottom for auto
            Vec2::new(
                element_pos.x + element_size.x / 2.0,
                element_pos.y + element_size.y + offset,
            )
        }
    }
}
