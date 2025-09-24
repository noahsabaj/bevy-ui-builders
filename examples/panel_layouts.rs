//! Panel Layouts Example - Demonstrates different panel styles and nesting
//!
//! Run with: cargo run --example panel_layouts --features panel

use bevy::prelude::*;
use bevy_ui_builders::*;
use bevy_ui_builders::panel::PanelStyle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiBuilderPlugin)
        .add_systems(Startup, setup)
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
            row_gap: Val::Px(20.0),
            ..default()
        })
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Panel Layouts Showcase"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Grid of panels
            parent
                .spawn(Node {
                    display: Display::Grid,
                    grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::flex(1.0), GridTrack::flex(1.0)],
                    grid_template_rows: vec![GridTrack::flex(1.0), GridTrack::flex(1.0)],
                    row_gap: Val::Px(15.0),
                    column_gap: Val::Px(15.0),
                    width: Val::Px(900.0),
                    height: Val::Px(500.0),
                    ..default()
                })
                .with_children(|grid| {
                    // Default panel
                    PanelBuilder::new()
                        .style(PanelStyle::Default)
                        .padding(UiRect::all(Val::Px(15.0)))
                        .build_with_children(grid, |panel| {
                            label("Default Panel")
                                .style(LabelStyle::Heading)
                                .build(panel);

                            label("Standard panel with default background")
                                .style(LabelStyle::Body)
                                .build(panel);
                        });

                    // Card panel
                    PanelBuilder::new()
                        .style(PanelStyle::Card)
                        .with_title("Card Style")
                        .padding(UiRect::all(Val::Px(20.0)))
                        .build_with_children(grid, |panel| {
                            label("Card-style panel with subtle styling")
                                .style(LabelStyle::Body)
                                .build(panel);
                        });

                    // Elevated panel
                    PanelBuilder::new()
                        .style(PanelStyle::Elevated)
                        .padding(UiRect::all(Val::Px(15.0)))
                        .build_with_children(grid, |panel| {
                            label("Elevated Panel")
                                .style(LabelStyle::Heading)
                                .build(panel);

                            label("Appears raised with shadow effect")
                                .style(LabelStyle::Caption)
                                .build(panel);
                        });

                    // Dark panel
                    PanelBuilder::new()
                        .style(PanelStyle::Dark)
                        .padding(UiRect::all(Val::Px(15.0)))
                        .build_with_children(grid, |panel| {
                            label("Dark Panel")
                                .style(LabelStyle::Heading)
                                .build(panel);

                            label("Dark background theme")
                                .style(LabelStyle::Muted)
                                .build(panel);
                        });

                    // Light panel
                    PanelBuilder::new()
                        .style(PanelStyle::Light)
                        .padding(UiRect::all(Val::Px(15.0)))
                        .build_with_children(grid, |panel| {
                            label("Light Panel")
                                .style(LabelStyle::Heading)
                                .color(Color::BLACK)
                                .build(panel);

                            label("Light background theme")
                                .style(LabelStyle::Body)
                                .color(Color::srgb(0.2, 0.2, 0.2))
                                .build(panel);
                        });

                    // Bordered panel
                    PanelBuilder::new()
                        .style(PanelStyle::Bordered)
                        .padding(UiRect::all(Val::Px(15.0)))
                        .build_with_children(grid, |panel| {
                            label("Bordered Panel")
                                .style(LabelStyle::Heading)
                                .build(panel);

                            label("Panel with visible border")
                                .style(LabelStyle::Body)
                                .build(panel);
                        });
                });

            // Nested panels example
            parent
                .spawn(Node {
                    width: Val::Px(900.0),
                    ..default()
                })
                .with_children(|container| {
                    PanelBuilder::new()
                        .style(PanelStyle::Card)
                        .with_title("Nested Panels Example")
                        .padding(UiRect::all(Val::Px(20.0)))
                        .width(Val::Percent(100.0))
                        .build_with_children(container, |outer_panel| {
                            label("This is the outer panel")
                                .style(LabelStyle::Body)
                                .margin(UiRect::bottom(Val::Px(15.0)))
                                .build(outer_panel);

                            // Nested panel 1
                            PanelBuilder::new()
                                .style(PanelStyle::Dark)
                                .padding(UiRect::all(Val::Px(15.0)))
                                .build_with_children(outer_panel, |inner| {
                                    label("Nested Dark Panel")
                                        .style(LabelStyle::Caption)
                                        .build(inner);

                                    primary_button("Button in Nested Panel")
                                        .size(ButtonSize::Small)
                                        .build(inner);
                                });

                            // Nested panel 2
                            panel()
                                .style(PanelStyle::Transparent)
                                .padding(UiRect::all(Val::Px(10.0)))
                                .build_with_children(outer_panel, |inner| {
                                    label("Transparent nested panel - no background")
                                        .style(LabelStyle::Muted)
                                        .build(inner);
                                });
                        });
                });
        });
}