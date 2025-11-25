//! Toast systems

use bevy::prelude::*;
use crate::styles::dimensions;
use crate::theme::UiTheme;
use super::types::*;

/// System to spawn the toast container if it doesn't exist
pub fn ensure_toast_container(
    mut commands: Commands,
    settings: Res<ToastSettings>,
    container_query: Query<Entity, With<ToastContainer>>,
) {
    if container_query.is_empty() {
        // Determine container position styles
        let (top, right, bottom, left, align_items) = match settings.position {
            ToastPosition::TopLeft => (Val::Px(16.0), Val::Auto, Val::Auto, Val::Px(16.0), AlignItems::FlexStart),
            ToastPosition::TopCenter => (Val::Px(16.0), Val::Auto, Val::Auto, Val::Auto, AlignItems::Center),
            ToastPosition::TopRight => (Val::Px(16.0), Val::Px(16.0), Val::Auto, Val::Auto, AlignItems::FlexEnd),
            ToastPosition::BottomLeft => (Val::Auto, Val::Auto, Val::Px(16.0), Val::Px(16.0), AlignItems::FlexStart),
            ToastPosition::BottomCenter => (Val::Auto, Val::Auto, Val::Px(16.0), Val::Auto, AlignItems::Center),
            ToastPosition::BottomRight => (Val::Auto, Val::Px(16.0), Val::Px(16.0), Val::Auto, AlignItems::FlexEnd),
        };

        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                top,
                right,
                bottom,
                left,
                width: Val::Px(settings.width),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(settings.gap),
                align_items,
                ..default()
            },
            GlobalZIndex(settings.z_index),
            ToastContainer {
                position: settings.position,
            },
        ));
    }
}

/// System to spawn new toasts from the queue
pub fn spawn_toasts(
    mut commands: Commands,
    settings: Res<ToastSettings>,
    mut queue: ResMut<ToastQueue>,
    container_query: Query<Entity, With<ToastContainer>>,
    active_toasts: Query<&ActiveToast>,
    theme: Option<Res<UiTheme>>,
) {
    let Ok(container) = container_query.single() else {
        return;
    };

    // Resolve colors from theme or use defaults
    let colors = if let Some(ref theme) = theme {
        ToastColors::from_theme(theme)
    } else {
        ToastColors::default_colors()
    };

    // Count visible toasts
    let visible_count = active_toasts.iter().filter(|t| !t.exiting).count();

    // Spawn new toasts up to the limit
    while visible_count < settings.max_visible {
        let Some(toast) = queue.pop() else {
            break;
        };

        let bg_color = colors.background;
        let accent_color = colors.accent_for_variant(toast.variant);

        let duration_secs = toast.duration.as_secs_f32();
        let toast_clone = toast.clone();

        commands.entity(container).with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(12.0)),
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(12.0),
                        align_items: AlignItems::FlexStart,
                        border: UiRect::left(Val::Px(4.0)),
                        ..default()
                    },
                    BackgroundColor(bg_color),
                    BorderColor::all(accent_color),
                    BorderRadius::all(Val::Px(6.0)),
                    ActiveToast {
                        toast: toast_clone,
                        time_remaining: duration_secs,
                        animation_progress: 0.0,
                        exiting: false,
                    },
                    Interaction::default(),
                ))
                .with_children(|toast_node| {
                    // Content container
                    toast_node
                        .spawn((
                            Node {
                                flex_grow: 1.0,
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(4.0),
                                ..default()
                            },
                        ))
                        .with_children(|content| {
                            // Title if present
                            if let Some(title) = &toast.title {
                                content.spawn((
                                    Text::new(title),
                                    TextFont {
                                        font_size: dimensions::FONT_SIZE_MEDIUM,
                                        ..default()
                                    },
                                    TextColor(colors.text_primary),
                                ));
                            }

                            // Message
                            content.spawn((
                                Text::new(&toast.message),
                                TextFont {
                                    font_size: dimensions::FONT_SIZE_SMALL,
                                    ..default()
                                },
                                TextColor(if toast.title.is_some() {
                                    colors.text_secondary
                                } else {
                                    colors.text_primary
                                }),
                            ));

                            // Action button if present
                            if let Some(action_text) = &toast.action {
                                content
                                    .spawn((
                                        Node {
                                            margin: UiRect::top(Val::Px(8.0)),
                                            padding: UiRect::new(
                                                Val::Px(12.0),
                                                Val::Px(12.0),
                                                Val::Px(6.0),
                                                Val::Px(6.0),
                                            ),
                                            ..default()
                                        },
                                        BackgroundColor(accent_color),
                                        BorderRadius::all(Val::Px(4.0)),
                                        Interaction::default(),
                                        ToastActionButton,
                                    ))
                                    .with_children(|btn| {
                                        btn.spawn((
                                            Text::new(action_text),
                                            TextFont {
                                                font_size: dimensions::FONT_SIZE_SMALL,
                                                ..default()
                                            },
                                            TextColor(colors.text_on_button),
                                        ));
                                    });
                            }
                        });

                    // Dismiss button if dismissible
                    if toast.dismissible {
                        toast_node
                            .spawn((
                                Node {
                                    padding: UiRect::all(Val::Px(4.0)),
                                    ..default()
                                },
                                Interaction::default(),
                                ToastDismissButton,
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    Text::new("✕"),
                                    TextFont {
                                        font_size: dimensions::FONT_SIZE_SMALL,
                                        ..default()
                                    },
                                    TextColor(colors.text_secondary),
                                ));
                            });
                    }
                });
        });

        break; // Only spawn one per frame for smoother animation
    }
}

/// Marker for toast dismiss button
#[derive(Component)]
pub struct ToastDismissButton;

/// Marker for toast action button
#[derive(Component)]
pub struct ToastActionButton;

/// System to update toast timers and handle auto-dismiss
pub fn update_toast_timers(
    time: Res<Time>,
    mut toast_query: Query<&mut ActiveToast>,
) {
    for mut toast in toast_query.iter_mut() {
        if toast.exiting {
            continue;
        }

        // Update animation
        if toast.animation_progress < 1.0 {
            toast.animation_progress = (toast.animation_progress + time.delta_secs() * 5.0).min(1.0);
        }

        // Update timer
        toast.time_remaining -= time.delta_secs();
        if toast.time_remaining <= 0.0 {
            toast.exiting = true;
        }
    }
}

/// System to handle toast dismiss button clicks
pub fn handle_toast_dismiss(
    dismiss_query: Query<(&ChildOf, &Interaction), (With<ToastDismissButton>, Changed<Interaction>)>,
    mut toast_query: Query<&mut ActiveToast>,
) {
    for (child_of, interaction) in dismiss_query.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok(mut toast) = toast_query.get_mut(child_of.parent()) {
                toast.exiting = true;
            }
        }
    }
}

/// System to handle toast action button clicks
pub fn handle_toast_action(
    dismiss_query: Query<(&ChildOf, &Interaction), (With<ToastActionButton>, Changed<Interaction>)>,
    toast_query: Query<&ActiveToast>,
    mut events: MessageWriter<ToastActionEvent>,
    mut dismiss_events: MessageWriter<DismissToastEvent>,
) {
    for (child_of, interaction) in dismiss_query.iter() {
        if *interaction == Interaction::Pressed {
            let toast_entity = child_of.parent();
            if let Ok(toast) = toast_query.get(toast_entity) {
                if let Some(action) = &toast.toast.action {
                    events.write(ToastActionEvent {
                        entity: toast_entity,
                        action: action.clone(),
                    });
                    dismiss_events.write(DismissToastEvent {
                        entity: toast_entity,
                    });
                }
            }
        }
    }
}

/// System to despawn exiting toasts
pub fn despawn_exiting_toasts(
    mut commands: Commands,
    toast_query: Query<(Entity, &ActiveToast)>,
) {
    for (entity, toast) in toast_query.iter() {
        if toast.exiting {
            commands.entity(entity).despawn();
        }
    }
}

/// System to handle dismiss events
pub fn handle_dismiss_events(
    mut events: MessageReader<DismissToastEvent>,
    mut toast_query: Query<&mut ActiveToast>,
) {
    for event in events.read() {
        if let Ok(mut toast) = toast_query.get_mut(event.entity) {
            toast.exiting = true;
        }
    }
}
