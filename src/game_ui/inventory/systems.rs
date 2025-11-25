//! Inventory systems

use bevy::prelude::*;
use crate::theme::UiTheme;
use super::types::*;

/// Default colors for inventory (dark theme fallback)
mod defaults {
    use bevy::prelude::Color;
    pub const BACKGROUND_LIGHT: Color = Color::srgb(0.15, 0.15, 0.17);
    pub const BACKGROUND_TERTIARY: Color = Color::srgb(0.12, 0.12, 0.14);
    pub const GHOST_HOVER: Color = Color::srgba(1.0, 1.0, 1.0, 0.05);
    pub const GHOST_PRESSED: Color = Color::srgba(1.0, 1.0, 1.0, 0.1);
}

/// System to handle slot hover effects
pub fn handle_slot_hover(
    mut slot_query: Query<(&InventorySlot, &Interaction, &mut BackgroundColor), Changed<Interaction>>,
    _settings: Res<InventorySettings>,
    theme: Option<Res<UiTheme>>,
) {
    // Resolve colors from theme or use defaults
    let (bg_light, bg_tertiary, ghost_hover, ghost_pressed) = if let Some(ref theme) = theme {
        (
            theme.colors.surface.secondary,
            theme.colors.surface.tertiary,
            theme.colors.ghost.hover,
            theme.colors.ghost.pressed,
        )
    } else {
        (
            defaults::BACKGROUND_LIGHT,
            defaults::BACKGROUND_TERTIARY,
            defaults::GHOST_HOVER,
            defaults::GHOST_PRESSED,
        )
    };

    for (slot, interaction, mut bg_color) in slot_query.iter_mut() {
        let base_color = if slot.item.is_some() {
            bg_light
        } else {
            bg_tertiary
        };

        *bg_color = match interaction {
            Interaction::Hovered => BackgroundColor(ghost_hover),
            Interaction::Pressed => BackgroundColor(ghost_pressed),
            Interaction::None => BackgroundColor(base_color),
        };
    }
}

/// System to handle slot clicks
pub fn handle_slot_clicks(
    slot_query: Query<(Entity, &InventorySlot, &Interaction), Changed<Interaction>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut click_events: MessageWriter<SlotClickEvent>,
) {
    for (entity, slot, interaction) in slot_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let button = if mouse.pressed(MouseButton::Left) {
            MouseButton::Left
        } else if mouse.pressed(MouseButton::Right) {
            MouseButton::Right
        } else {
            continue;
        };

        click_events.write(SlotClickEvent {
            grid: slot.grid,
            slot: entity,
            index: slot.index,
            button,
        });
    }
}

/// System to handle drag start
pub fn handle_drag_start(
    slot_query: Query<(Entity, &InventorySlot, &Interaction), Changed<Interaction>>,
    mouse: Res<ButtonInput<MouseButton>>,
    settings: Res<InventorySettings>,
    mut drag_state: ResMut<InventoryDragState>,
    mut drag_events: MessageWriter<ItemDragStartEvent>,
) {
    if !settings.drag_drop_enabled {
        return;
    }

    // Only start drag on left mouse press
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    for (entity, slot, interaction) in slot_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        // Can only drag if slot has an item
        let Some(item) = slot.item else {
            continue;
        };

        drag_state.dragging = Some(DragInfo {
            item,
            from_slot: entity,
            grid: slot.grid,
        });

        drag_events.write(ItemDragStartEvent {
            grid: slot.grid,
            from_slot: entity,
            item,
        });

        break;
    }
}

/// System to handle drag end/drop
pub fn handle_drop(
    slot_query: Query<(Entity, &InventorySlot, &Interaction)>,
    mouse: Res<ButtonInput<MouseButton>>,
    settings: Res<InventorySettings>,
    mut drag_state: ResMut<InventoryDragState>,
    mut drop_events: MessageWriter<ItemDropEvent>,
) {
    if !settings.drag_drop_enabled {
        return;
    }

    // Only process on left mouse release
    if !mouse.just_released(MouseButton::Left) {
        return;
    }

    let Some(drag_info) = drag_state.dragging.take() else {
        return;
    };

    // Find the slot being hovered over
    for (entity, slot, interaction) in slot_query.iter() {
        if *interaction != Interaction::Hovered && *interaction != Interaction::Pressed {
            continue;
        }

        // Found target slot
        drop_events.write(ItemDropEvent {
            grid: drag_info.grid,
            from_slot: drag_info.from_slot,
            to_slot: entity,
            item: drag_info.item,
        });

        return;
    }

    // Dropped outside any slot - return to original position
    // The game can handle this case by listening to the drag state reset
}

/// System to cancel drag on right-click
pub fn cancel_drag_on_right_click(
    mouse: Res<ButtonInput<MouseButton>>,
    mut drag_state: ResMut<InventoryDragState>,
) {
    if mouse.just_pressed(MouseButton::Right) {
        drag_state.dragging = None;
    }
}
