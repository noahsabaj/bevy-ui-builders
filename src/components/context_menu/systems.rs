//! Context menu systems

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::styles::dimensions;
use crate::theme::UiTheme;
use super::types::*;

/// System to detect right-clicks and open context menus
pub fn detect_context_menu_trigger(
    mouse: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    trigger_query: Query<(Entity, &ContextMenuTrigger, &GlobalTransform, &ComputedNode)>,
    mut open_menu: ResMut<OpenContextMenu>,
    mut commands: Commands,
    settings: Res<ContextMenuSettings>,
    existing_menus: Query<Entity, With<ContextMenu>>,
    theme: Option<Res<UiTheme>>,
) {
    // Only handle right-click
    if !mouse.just_pressed(MouseButton::Right) {
        return;
    }

    let Ok(window) = window_query.single() else { return };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    // Resolve colors from theme or use defaults
    let colors = if let Some(ref theme) = theme {
        ContextMenuColors::from_theme(theme)
    } else {
        ContextMenuColors::default_colors()
    };

    // Check if cursor is over any trigger
    for (entity, trigger, transform, computed) in trigger_query.iter() {
        let pos = transform.translation().truncate();
        let size = computed.size();

        // Check if cursor is within bounds
        let min = pos;
        let max = pos + size;

        if cursor_pos.x >= min.x && cursor_pos.x <= max.x
            && cursor_pos.y >= min.y && cursor_pos.y <= max.y
        {
            // Close any existing menu
            for menu_entity in existing_menus.iter() {
                commands.entity(menu_entity).despawn();
            }

            // Spawn new context menu
            spawn_context_menu(
                &mut commands,
                &settings,
                entity,
                &trigger.items,
                cursor_pos,
                &colors,
            );

            open_menu.menu = Some(entity);
            open_menu.trigger = Some(entity);

            return;
        }
    }
}

/// Spawn a context menu at the given position
fn spawn_context_menu(
    commands: &mut Commands,
    settings: &ContextMenuSettings,
    trigger: Entity,
    items: &[MenuItem],
    position: Vec2,
    colors: &ContextMenuColors,
) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(position.x),
                top: Val::Px(position.y),
                flex_direction: FlexDirection::Column,
                min_width: Val::Px(180.0),
                padding: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(colors.background),
            BorderColor::all(colors.border),
            BorderRadius::all(Val::Px(6.0)),
            GlobalZIndex(settings.z_index),
            ContextMenu {
                trigger,
                position,
            },
        ))
        .with_children(|menu| {
            for (index, item) in items.iter().enumerate() {
                spawn_menu_item(menu, item, index, trigger, colors);
            }
        });
}

/// Spawn a single menu item
fn spawn_menu_item(
    parent: &mut ChildSpawnerCommands,
    item: &MenuItem,
    index: usize,
    menu_entity: Entity,
    colors: &ContextMenuColors,
) {
    match item {
        MenuItem::Action { label, shortcut, disabled, id: _ } => {
            let text_color = if *disabled {
                colors.text_disabled
            } else {
                colors.text_primary
            };

            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        padding: UiRect::new(
                            Val::Px(12.0),
                            Val::Px(12.0),
                            Val::Px(8.0),
                            Val::Px(8.0),
                        ),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    BorderRadius::all(Val::Px(4.0)),
                    ContextMenuItem {
                        menu: menu_entity,
                        item: item.clone(),
                        index,
                    },
                    if *disabled {
                        Interaction::None
                    } else {
                        Interaction::default()
                    },
                ))
                .with_children(|row| {
                    // Label
                    row.spawn((
                        Text::new(label),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_SMALL,
                            ..default()
                        },
                        TextColor(text_color),
                    ));

                    // Shortcut
                    if let Some(sc) = shortcut {
                        row.spawn((
                            Text::new(sc),
                            TextFont {
                                font_size: dimensions::FONT_SIZE_SMALL,
                                ..default()
                            },
                            TextColor(colors.text_muted),
                        ));
                    }
                });
        }

        MenuItem::Separator => {
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(1.0),
                    margin: UiRect::new(
                        Val::Px(8.0),
                        Val::Px(8.0),
                        Val::Px(4.0),
                        Val::Px(4.0),
                    ),
                    ..default()
                },
                BackgroundColor(colors.border),
            ));
        }

        MenuItem::Submenu { label, items: _ } => {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        padding: UiRect::new(
                            Val::Px(12.0),
                            Val::Px(12.0),
                            Val::Px(8.0),
                            Val::Px(8.0),
                        ),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    BorderRadius::all(Val::Px(4.0)),
                    ContextMenuItem {
                        menu: menu_entity,
                        item: item.clone(),
                        index,
                    },
                    Interaction::default(),
                ))
                .with_children(|row| {
                    row.spawn((
                        Text::new(label),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_SMALL,
                            ..default()
                        },
                        TextColor(colors.text_primary),
                    ));

                    // Arrow indicator
                    row.spawn((
                        Text::new("▶"),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_SMALL,
                            ..default()
                        },
                        TextColor(colors.text_secondary),
                    ));
                });
        }

        MenuItem::Checkbox { label, checked, id: _ } => {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        padding: UiRect::new(
                            Val::Px(12.0),
                            Val::Px(12.0),
                            Val::Px(8.0),
                            Val::Px(8.0),
                        ),
                        column_gap: Val::Px(8.0),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    BorderRadius::all(Val::Px(4.0)),
                    ContextMenuItem {
                        menu: menu_entity,
                        item: item.clone(),
                        index,
                    },
                    Interaction::default(),
                ))
                .with_children(|row| {
                    // Checkbox indicator
                    row.spawn((
                        Text::new(if *checked { "✓" } else { " " }),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_SMALL,
                            ..default()
                        },
                        TextColor(colors.primary),
                        Node {
                            width: Val::Px(16.0),
                            ..default()
                        },
                    ));

                    // Label
                    row.spawn((
                        Text::new(label),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_SMALL,
                            ..default()
                        },
                        TextColor(colors.text_primary),
                    ));
                });
        }
    }
}

/// System to handle menu item hover effects
pub fn handle_menu_item_hover(
    mut item_query: Query<(&Interaction, &mut BackgroundColor), (With<ContextMenuItem>, Changed<Interaction>)>,
    theme: Option<Res<UiTheme>>,
) {
    // Resolve colors from theme or use defaults
    let colors = if let Some(ref theme) = theme {
        ContextMenuColors::from_theme(theme)
    } else {
        ContextMenuColors::default_colors()
    };

    for (interaction, mut bg_color) in item_query.iter_mut() {
        *bg_color = match interaction {
            Interaction::Hovered => BackgroundColor(colors.hover),
            Interaction::Pressed => BackgroundColor(colors.pressed),
            Interaction::None => BackgroundColor(Color::NONE),
        };
    }
}

/// System to handle menu item clicks
pub fn handle_menu_item_click(
    mut commands: Commands,
    item_query: Query<(&ContextMenuItem, &Interaction), Changed<Interaction>>,
    menu_query: Query<&ContextMenu>,
    mut action_events: MessageWriter<ContextMenuActionEvent>,
    mut checkbox_events: MessageWriter<ContextMenuCheckboxEvent>,
    mut open_menu: ResMut<OpenContextMenu>,
) {
    for (menu_item, interaction) in item_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let Ok(menu) = menu_query.get(menu_item.menu) else {
            continue;
        };

        match &menu_item.item {
            MenuItem::Action { id, disabled, .. } => {
                if !disabled {
                    action_events.write(ContextMenuActionEvent {
                        id: id.clone(),
                        trigger: menu.trigger,
                    });

                    // Close menu
                    close_all_menus(&mut commands, &mut open_menu);
                }
            }

            MenuItem::Checkbox { id, checked, .. } => {
                checkbox_events.write(ContextMenuCheckboxEvent {
                    id: id.clone(),
                    checked: !checked,
                    trigger: menu.trigger,
                });

                // Close menu
                close_all_menus(&mut commands, &mut open_menu);
            }

            MenuItem::Submenu { .. } => {
                // TODO: Open submenu on hover/click
            }

            MenuItem::Separator => {}
        }
    }
}

/// System to close context menu when clicking outside
pub fn close_menu_on_outside_click(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    menu_query: Query<(Entity, &GlobalTransform, &ComputedNode), With<ContextMenu>>,
    mut open_menu: ResMut<OpenContextMenu>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = window_query.single() else { return };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    // Check if click is outside all menus
    for (_entity, transform, computed) in menu_query.iter() {
        let pos = transform.translation().truncate();
        let size = computed.size();

        if cursor_pos.x >= pos.x && cursor_pos.x <= pos.x + size.x
            && cursor_pos.y >= pos.y && cursor_pos.y <= pos.y + size.y
        {
            // Click is inside a menu, don't close
            return;
        }
    }

    // Close all menus
    close_all_menus(&mut commands, &mut open_menu);
}

/// System to close context menu on Escape key
pub fn close_menu_on_escape(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut open_menu: ResMut<OpenContextMenu>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        close_all_menus(&mut commands, &mut open_menu);
    }
}

/// Helper to close all menus
fn close_all_menus(commands: &mut Commands, open_menu: &mut OpenContextMenu) {
    if let Some(menu_entity) = open_menu.menu.take() {
        if let Ok(mut entity_commands) = commands.get_entity(menu_entity) {
            entity_commands.despawn();
        }
    }
    open_menu.trigger = None;
}
