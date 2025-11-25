//! Demonstrates button selection states including:
//! - Toggle buttons (checkbox-like)
//! - Radio button groups (exclusive selection)
//! - Active state (current tab/page indicators)
//! - Custom selection colors
//! - Different button styles with selection

use bevy::prelude::*;
use bevy_ui_builders::prelude::*;
use bevy_ui_builders::*;
use bevy_ui_builders::components::button::{SelectionChanged, Selected};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiBuilderPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_selection_events,
            update_selection_counter,
            debug_button_groups,
        ))
        .run();
}

/// Debug system to log button group membership
fn debug_button_groups(
    groups: Query<(Entity, &ButtonGroupMembers), Changed<ButtonGroupMembers>>,
) {
    for (entity, members) in &groups {
        info!("Button group {:?} now has {} members: {:?}",
              entity, members.iter().count(), members.iter().collect::<Vec<_>>());
    }
}

#[derive(Component)]
struct SelectionCounter;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Create button group entity at world level (before any UI hierarchy)
    // This ensures it has no parent, avoiding B0004 warnings about missing GlobalTransform
    let radio_group = commands.spawn(ButtonGroupMembers::default()).id();

    // Root container - using UiContainer to prevent B0004 warnings
    commands
        .spawn((
            UiContainer::new(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(40.0),
                ..default()
            }),
            BackgroundColor(Color::srgb(0.1, 0.1, 0.12)),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Button Selection States Demo"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Section 1: Toggle Buttons (Independent Selection)
            section_container(parent, "Toggle Buttons (Checkbox Behavior)", |section| {
                ButtonBuilder::new("Feature A")
                    .selectable()
                    .selected(true)
                    .style(ButtonStyle::Primary)
                    .build(section);

                ButtonBuilder::new("Feature B")
                    .selectable()
                    .style(ButtonStyle::Success)
                    .build(section);

                ButtonBuilder::new("Feature C")
                    .selectable()
                    .style(ButtonStyle::Secondary)
                    .build(section);
            });

            // Section 2: Radio Button Group (Exclusive Selection)
            // Note: radio_group was created at world level above to avoid B0004 warnings
            section_container(parent, "Radio Button Group (Exclusive Selection)", |section| {
                ButtonBuilder::new("Option 1")
                    .in_group(radio_group)
                    .selected(true)
                    .style(ButtonStyle::Primary)
                    .build(section);

                ButtonBuilder::new("Option 2")
                    .in_group(radio_group)
                    .style(ButtonStyle::Primary)
                    .build(section);

                ButtonBuilder::new("Option 3")
                    .in_group(radio_group)
                    .style(ButtonStyle::Primary)
                    .build(section);
            });

            // Section 3: Tab Bar (Active State)
            section_container(parent, "Tab Bar (Active State)", |section| {
                ButtonBuilder::new("Home")
                    .selectable()
                    .active(true)
                    .style(ButtonStyle::Ghost)
                    .size(ButtonSize::Medium)
                    .build(section);

                ButtonBuilder::new("Settings")
                    .selectable()
                    .style(ButtonStyle::Ghost)
                    .size(ButtonSize::Medium)
                    .build(section);

                ButtonBuilder::new("Profile")
                    .selectable()
                    .style(ButtonStyle::Ghost)
                    .size(ButtonSize::Medium)
                    .build(section);

                ButtonBuilder::new("Help")
                    .selectable()
                    .style(ButtonStyle::Ghost)
                    .size(ButtonSize::Medium)
                    .build(section);
            });

            // Section 4: Mixed States (Active + Selected)
            section_container(parent, "Mixed States & Custom Colors", |section| {
                // Active button
                ButtonBuilder::new("Active")
                    .selectable()
                    .active(true)
                    .style(ButtonStyle::Warning)
                    .build(section);

                // Selected button
                ButtonBuilder::new("Selected")
                    .selectable()
                    .selected(true)
                    .style(ButtonStyle::Success)
                    .build(section);

                // Both active and selected (Active takes precedence visually)
                ButtonBuilder::new("Active + Selected")
                    .selectable()
                    .active(true)
                    .selected(true)
                    .style(ButtonStyle::Danger)
                    .build(section);

                // Manual toggle (doesn't auto-toggle on click)
                ButtonBuilder::new("Manual Toggle")
                    .selectable()
                    .manual_toggle()
                    .style(ButtonStyle::Secondary)
                    .build(section);
            });

            // Section 5: Different Sizes
            section_container(parent, "Selection with Different Sizes", |section| {
                ButtonBuilder::new("Small")
                    .selectable()
                    .selected(true)
                    .size(ButtonSize::Small)
                    .style(ButtonStyle::Primary)
                    .build(section);

                ButtonBuilder::new("Medium")
                    .selectable()
                    .size(ButtonSize::Medium)
                    .style(ButtonStyle::Primary)
                    .build(section);

                ButtonBuilder::new("Large")
                    .selectable()
                    .size(ButtonSize::Large)
                    .style(ButtonStyle::Primary)
                    .build(section);

                ButtonBuilder::new("XLarge")
                    .selectable()
                    .size(ButtonSize::XLarge)
                    .style(ButtonStyle::Primary)
                    .build(section);
            });

            // Selection counter
            parent.spawn((
                Text::new("Selected buttons: 3"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                SelectionCounter,
                Node {
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Instructions
            parent.spawn((
                Text::new("Click buttons to toggle selection • Radio groups allow only one selection • Active state shows current page/tab"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
                Node {
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
            ));
        });
}

/// Helper function to create a section container
/// Uses UiContainer to prevent B0004 warnings about missing GlobalTransform.
fn section_container(
    parent: &mut ChildSpawnerCommands,
    title: &str,
    children: impl FnOnce(&mut ChildSpawnerCommands),
) {
    parent
        .spawn((
            UiContainer::new(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(12.0),
                padding: UiRect::all(Val::Px(20.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            }),
            BackgroundColor(Color::srgba(0.15, 0.15, 0.17, 0.8)),
            BorderColor::all(Color::srgb(0.3, 0.3, 0.35)),
            BorderRadius::all(Val::Px(8.0)),
        ))
        .with_children(|section_parent| {
            // Section title
            section_parent.spawn((
                Text::new(title),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                },
            ));

            // Button container - using UiContainer for proper hierarchy
            section_parent
                .spawn((
                    UiContainer::row()
                        .gap(Val::Px(12.0))
                        .build(),
                    BackgroundColor(Color::NONE),
                ))
                .with_children(|button_container| {
                    children(button_container);
                });
        });
}

/// System to handle and log selection events
fn handle_selection_events(
    mut events: MessageReader<SelectionChanged>,
    query: Query<&Text>,
) {
    for event in events.read() {
        if let Ok(text) = query.get(event.entity) {
            let button_text = &text.0;
            info!(
                "Button '{}' {} (Entity: {:?})",
                button_text,
                if event.selected { "selected" } else { "deselected" },
                event.entity
            );
        }
    }
}

/// System to update the selection counter
fn update_selection_counter(
    selected_buttons: Query<(), With<Selected>>,
    mut counter_query: Query<&mut Text, With<SelectionCounter>>,
) {
    let count = selected_buttons.iter().count();

    for mut text in &mut counter_query {
        text.0 = format!("Selected buttons: {}", count);
    }
}
