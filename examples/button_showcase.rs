//! Button Showcase - Demonstrates all button styles, sizes, and features
//!
//! Run with: cargo run --example button_showcase --features button

use bevy::prelude::*;
use bevy_ui_builders::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiBuilderPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, button_interaction_system)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);

    // Root node
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(20.0)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(30.0),
            ..default()
        })
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Button Showcase"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Button Styles Section
            create_section(parent, "Button Styles", |section| {
                // Row of different button styles
                section
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(10.0),
                        ..default()
                    })
                    .with_children(|row| {
                        ButtonBuilder::new("Primary")
                            .style(ButtonStyle::Primary)
                            .build(row);

                        ButtonBuilder::new("Secondary")
                            .style(ButtonStyle::Secondary)
                            .build(row);

                        ButtonBuilder::new("Success")
                            .style(ButtonStyle::Success)
                            .build(row);

                        ButtonBuilder::new("Danger")
                            .style(ButtonStyle::Danger)
                            .build(row);

                        ButtonBuilder::new("Warning")
                            .style(ButtonStyle::Warning)
                            .build(row);

                        ButtonBuilder::new("Ghost")
                            .style(ButtonStyle::Ghost)
                            .build(row);
                    });
            });

            // Button Sizes Section
            create_section(parent, "Button Sizes", |section| {
                section
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(10.0),
                        align_items: AlignItems::Center,
                        ..default()
                    })
                    .with_children(|row| {
                        ButtonBuilder::new("Small")
                            .style(ButtonStyle::Primary)
                            .size(ButtonSize::Small)
                            .build(row);

                        ButtonBuilder::new("Medium")
                            .style(ButtonStyle::Primary)
                            .size(ButtonSize::Medium)
                            .build(row);

                        ButtonBuilder::new("Large")
                            .style(ButtonStyle::Primary)
                            .size(ButtonSize::Large)
                            .build(row);

                        ButtonBuilder::new("XLarge")
                            .style(ButtonStyle::Primary)
                            .size(ButtonSize::XLarge)
                            .build(row);
                    });
            });

            // Convenience Functions Section
            create_section(parent, "Convenience Functions", |section| {
                section
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(10.0),
                        ..default()
                    })
                    .with_children(|row| {
                        primary_button("Primary").build(row);
                        secondary_button("Secondary").build(row);
                        success_button("Success").build(row);
                        danger_button("Delete").build(row);
                        ghost_button("Ghost").build(row);
                    });
            });

            // Special Features Section
            create_section(parent, "Special Features", |section| {
                // Row 1: Icons and disabled state
                section
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(10.0),
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    })
                    .with_children(|row| {
                        ButtonBuilder::new("Save")
                            .style(ButtonStyle::Success)
                            .icon("💾")
                            .build(row);

                        ButtonBuilder::new("Settings")
                            .style(ButtonStyle::Secondary)
                            .icon("⚙️")
                            .build(row);

                        ButtonBuilder::new("Disabled")
                            .style(ButtonStyle::Primary)
                            .disabled()
                            .build(row);
                    });

                // Row 2: Custom width and hover effects
                section
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(10.0),
                        ..default()
                    })
                    .with_children(|row| {
                        ButtonBuilder::new("Custom Width")
                            .style(ButtonStyle::Primary)
                            .width(Val::Px(200.0))
                            .build(row);

                        ButtonBuilder::new("Hover Scale")
                            .style(ButtonStyle::Secondary)
                            .hover_scale(1.1)
                            .build(row);

                        ButtonBuilder::new("Hover Bright")
                            .style(ButtonStyle::Warning)
                            .hover_brightness(1.2)
                            .build(row);
                    });
            });

            // Interactive Demo Section
            create_section(parent, "Interactive Demo", |section| {
                section.spawn((
                    Text::new("Click any button to see console output"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.7, 0.7, 0.7)),
                ));

                section
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(10.0),
                        ..default()
                    })
                    .with_children(|row| {
                        primary_button("▶️ Play")
                            .size(ButtonSize::Large)
                            .build(row);

                        secondary_button("⏸ Pause")
                            .size(ButtonSize::Large)
                            .build(row);

                        danger_button("⏹ Stop")
                            .size(ButtonSize::Large)
                            .build(row);
                    });
            });
        });
}

fn create_section<F>(parent: &mut ChildSpawnerCommands, title: &str, content: F)
where
    F: FnOnce(&mut ChildSpawnerCommands),
{
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(15.0),
            padding: UiRect::all(Val::Px(20.0)),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        })
        .with_child((
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.5)),
            BorderColor::all(Color::srgba(0.3, 0.3, 0.3, 0.5)),
        ))
        .with_children(|section| {
            // Section title
            section.spawn((
                Text::new(title),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));

            content(section);
        });
}

// System to handle button interactions
fn button_interaction_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            info!("Button clicked!");
        }
    }
}