//! Kitchen Sink - Complete UI combining all builders
//!
//! Run with: cargo run --example kitchen_sink --features all_builders

use bevy::prelude::*;
use bevy_ui_builders::*;
use bevy_ui_builders::panel::PanelStyle;
use bevy_ui_builders::label::LabelStyle;
use bevy_ui_builders::separator::{SeparatorStyle, Orientation};
use bevy_ui_builders::progress::ProgressBar;
use bevy_ui_builders::slider::{Slider, SliderBuilder, ValueFormat};
use bevy_ui_builders::dialog::presets;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiBuilderPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_progress, handle_buttons))
        .run();
}


fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);

    // Main container
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|main| {
            // Header
            create_header(main);

            // Main content area
            main.spawn(Node {
                flex_grow: 1.0,
                flex_direction: FlexDirection::Row,
                ..default()
            })
            .with_children(|content| {
                // Sidebar
                create_sidebar(content);

                // Main panel
                create_main_panel(content);

                // Right panel
                create_right_panel(content);
            });

            // Footer
            create_footer(main);
        });
}

fn create_header(parent: &mut ChildSpawnerCommands) {
    PanelBuilder::new()
        .style(PanelStyle::Dark)
        .height(Val::Px(60.0))
        .padding(UiRect::horizontal(Val::Px(20.0)))
        .build_with_children(parent, |header| {
            header
                .spawn(Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|row| {
                    // Logo/Title
                    label("Kitchen Sink Demo")
                        .style(LabelStyle::Title)
                        .font_size(24.0)
                        .build(row);

                    // Header buttons
                    row.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(10.0),
                        ..default()
                    })
                    .with_children(|buttons| {
                        ghost_button("Settings")
                            .size(ButtonSize::Small)
                            .build(buttons);

                        let exit_btn = danger_button("Exit")
                            .size(ButtonSize::Small)
                            .build(buttons);

                        buttons.commands().entity(exit_btn).insert(ExitButton);
                    });
                });
        });
}

fn create_sidebar(parent: &mut ChildSpawnerCommands) {
    PanelBuilder::new()
        .style(PanelStyle::Bordered)
        .width(Val::Px(200.0))
        .padding(UiRect::all(Val::Px(15.0)))
        .build_with_children(parent, |sidebar| {
            label("Navigation")
                .style(LabelStyle::Heading)
                .margin(UiRect::bottom(Val::Px(15.0)))
                .build(sidebar);

            separator().build(sidebar);

            // Navigation buttons
            sidebar
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    margin: UiRect::top(Val::Px(15.0)),
                    ..default()
                })
                .with_children(|nav| {
                    secondary_button("Dashboard")
                        .width(Val::Percent(100.0))
                        .build(nav);

                    secondary_button("Profile")
                        .width(Val::Percent(100.0))
                        .build(nav);

                    secondary_button("Documents")
                        .width(Val::Percent(100.0))
                        .build(nav);

                    secondary_button("Settings")
                        .width(Val::Percent(100.0))
                        .build(nav);
                });

            // Progress section
            sidebar
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    margin: UiRect::top(Val::Px(30.0)),
                    ..default()
                })
                .with_children(|progress_section| {
                    label("Storage Used")
                        .style(LabelStyle::Caption)
                        .build(progress_section);

                    ProgressBarBuilder::new(0.0)
                        .with_label()
                        .animated()
                        .build(progress_section);
                });
        });
}

fn create_main_panel(parent: &mut ChildSpawnerCommands) {
    PanelBuilder::new()
        .style(PanelStyle::Default)
        .padding(UiRect::all(Val::Px(30.0)))
        .build_with_children(parent, |main| {
            label("Welcome to Kitchen Sink")
                .style(LabelStyle::Title)
                .margin(UiRect::bottom(Val::Px(20.0)))
                .build(main);

            separator()
                .style(SeparatorStyle::Solid)
                .margin(UiRect::bottom(Val::Px(20.0)))
                .build(main);

            label("This demo showcases all UI builders in one application")
                .style(LabelStyle::Body)
                .margin(UiRect::bottom(Val::Px(15.0)))
                .build(main);

            // Quick settings panel
            PanelBuilder::new()
                .style(PanelStyle::Card)
                .with_title("Quick Settings")
                .padding(UiRect::all(Val::Px(20.0)))
                .build_with_children(main, |settings| {
                    // Volume slider
                    label("Volume").style(LabelStyle::Body).build(settings);
                    SliderBuilder::new(0.0..100.0)
                        .value(75.0)
                        .with_preview(true)
                        .format(ValueFormat::Percentage)
                        .build(settings);

                    separator()
                        .style(SeparatorStyle::Invisible)
                        .margin(UiRect::vertical(Val::Px(10.0)))
                        .build(settings);

                    // Brightness slider
                    label("Brightness").style(LabelStyle::Body).build(settings);
                    SliderBuilder::new(0.0..100.0)
                        .value(80.0)
                        .with_preview(true)
                        .format(ValueFormat::Integer)
                        .build(settings);
                });

            // Action buttons
            main.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                margin: UiRect::top(Val::Px(30.0)),
                ..default()
            })
            .with_children(|buttons| {
                primary_button("Start")
                    .size(ButtonSize::Large)
                    .build(buttons);

                secondary_button("Pause")
                    .size(ButtonSize::Large)
                    .build(buttons);

                danger_button("Reset")
                    .size(ButtonSize::Large)
                    .build(buttons);
            });
        });
}

fn create_right_panel(parent: &mut ChildSpawnerCommands) {
    PanelBuilder::new()
        .style(PanelStyle::Elevated)
        .width(Val::Px(250.0))
        .padding(UiRect::all(Val::Px(15.0)))
        .build_with_children(parent, |panel| {
            label("Status")
                .style(LabelStyle::Heading)
                .margin(UiRect::bottom(Val::Px(15.0)))
                .build(panel);

            separator().build(panel);

            // Status items
            panel
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(12.0),
                    margin: UiRect::top(Val::Px(15.0)),
                    ..default()
                })
                .with_children(|status| {
                    create_status_item(status, "Connection", "Online", LabelStyle::Success);
                    create_status_item(status, "Updates", "3 Available", LabelStyle::Warning);
                    create_status_item(status, "Memory", "4.2 GB", LabelStyle::Body);
                    create_status_item(status, "CPU", "45%", LabelStyle::Body);
                });

            // Notification section
            panel
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    margin: UiRect::top(Val::Px(30.0)),
                    ..default()
                })
                .with_children(|notifications| {
                    label("Recent Activity")
                        .style(LabelStyle::Caption)
                        .build(notifications);

                    PanelBuilder::new()
                        .style(PanelStyle::Dark)
                        .padding(UiRect::all(Val::Px(10.0)))
                        .build_with_children(notifications, |notif| {
                            label("File uploaded successfully")
                                .style(LabelStyle::Caption)
                                .build(notif);
                        });

                    PanelBuilder::new()
                        .style(PanelStyle::Dark)
                        .padding(UiRect::all(Val::Px(10.0)))
                        .build_with_children(notifications, |notif| {
                            label("Settings saved")
                                .style(LabelStyle::Caption)
                                .build(notif);
                        });
                });
        });
}

fn create_footer(parent: &mut ChildSpawnerCommands) {
    PanelBuilder::new()
        .style(PanelStyle::Dark)
        .height(Val::Px(40.0))
        .padding(UiRect::horizontal(Val::Px(20.0)))
        .build_with_children(parent, |footer| {
            footer
                .spawn(Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|row| {
                    label("2025 Bevy UI Builders")
                        .style(LabelStyle::Muted)
                        .font_size(12.0)
                        .build(row);

                    label("Version 0.1.0")
                        .style(LabelStyle::Muted)
                        .font_size(12.0)
                        .build(row);
                });
        });
}

fn create_status_item(
    parent: &mut ChildSpawnerCommands,
    label_text: &str,
    value: &str,
    value_style: LabelStyle,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        })
        .with_children(|row| {
            label(label_text)
                .style(LabelStyle::Muted)
                .build(row);

            label(value)
                .style(value_style)
                .build(row);
        });
}

fn update_progress(
    mut query: Query<&mut ProgressBar>,
    time: Res<Time>,
) {
    for mut progress in &mut query {
        progress.value += time.delta_secs() * 0.1;
        if progress.value > 1.0 {
            progress.value = 0.0;
        }
    }
}

#[derive(Component)]
struct ExitButton;

fn handle_buttons(
    interaction_query: Query<(&Interaction, Option<&ExitButton>), (Changed<Interaction>, With<Button>)>,
    mut exit_events: EventWriter<AppExit>,
) {
    for (interaction, exit_marker) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if exit_marker.is_some() {
                info!("Exit button clicked - closing application");
                exit_events.write(AppExit::Success);
            } else {
                info!("Button clicked!");
            }
        }
    }
}