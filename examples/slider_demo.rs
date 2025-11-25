//! Slider Demo - Demonstrates slider configurations and value formats
//!
//! Run with: cargo run --example slider_demo --features slider

use bevy::prelude::*;
use bevy_ui_builders::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiBuilderPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update_slider_values)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);

    // Root node with scrolling enabled
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(20.0)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            // Scrollable container - intentionally limited height to enable scrolling
            ScrollViewBuilder::new()
                .max_height(Val::Vh(85.0)) // 85% of viewport to ensure content overflows
                .width(Val::Percent(100.0))
                .padding(UiRect::all(Val::Px(20.0)))
                .gap(Val::Px(40.0))
                .scrollbar_visibility(bevy_ui_builders::components::scroll_view::ScrollbarVisibility::Always) // Always show for testing
                .build_with_children(parent, |scroll| {
                    // Title
                    scroll.spawn((
                        Text::new("Slider Demo"),
                        TextFont {
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    // Container for all sliders
                    scroll
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(30.0),
                            width: Val::Px(500.0),
                            ..default()
                        })
                        .with_children(|container| {
                    // Basic slider with percentage
                    create_slider_row(container, "Volume", |row| {
                        SliderBuilder::new(0.0..100.0)
                            .value(75.0)
                            .format(ValueFormat::Percentage)
                            .width(Val::Px(300.0))
                            .label("Master Volume")
                            .with_preview(true)
                            .build(row);
                    });

                    // Slider with integer values
                    create_slider_row(container, "Brightness", |row| {
                        SliderBuilder::new(0.0..255.0)
                            .value(180.0)
                            .step(5.0)
                            .format(ValueFormat::Integer)
                            .width(Val::Px(300.0))
                            .with_preview(true)
                            .build(row);
                    });

                    // Slider with decimal values
                    create_slider_row(container, "Speed Multiplier", |row| {
                        SliderBuilder::new(0.5..3.0)
                            .value(1.0)
                            .step(0.1)
                            .format(ValueFormat::Decimal(1))
                            .width(Val::Px(300.0))
                            .label("Game Speed")
                            .with_preview(true)
                            .build(row);
                    });

                    // Slider with custom format
                    create_slider_row(container, "Temperature", |row| {
                        SliderBuilder::new(-20.0..50.0)
                            .value(22.0)
                            .step(1.0)
                            .format(ValueFormat::Custom(|v| format!("{:.0}C", v)))
                            .width(Val::Px(300.0))
                            .with_preview(true)
                            .build(row);
                    });

                    // Slider with increment/decrement buttons
                    create_slider_row(container, "With Buttons", |row| {
                        SliderBuilder::new(0.0..10.0)
                            .value(5.0)
                            .step(1.0)
                            .format(ValueFormat::Integer)
                            .width(Val::Px(300.0))
                            .with_buttons()
                            .with_preview(true)
                            .build(row);
                    });

                    // Multiple sliders for RGB color
                    create_color_picker_section(container);

                    // Slider without preview
                    create_slider_row(container, "Hidden Value", |row| {
                        SliderBuilder::new(0.0..100.0)
                            .value(50.0)
                            .width(Val::Px(300.0))
                            .with_preview(false)
                            .build(row);
                    });
                });

                    // Value display section
                    scroll
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(10.0),
                            padding: UiRect::all(Val::Px(20.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        })
                        .with_child((
                            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.5)),
                            BorderColor::all(Color::srgba(0.3, 0.3, 0.3, 0.5)),
                        ))
                        .with_children(|display| {
                            display.spawn((
                                Text::new("Slider values are logged to console"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                            ));
                        });
                }); // Close ScrollViewBuilder
        });
}

fn create_slider_row<F>(parent: &mut ChildSpawnerCommands, label: &str, content: F)
where
    F: FnOnce(&mut ChildSpawnerCommands),
{
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            ..default()
        })
        .with_children(|column| {
            // Label
            column.spawn((
                Text::new(label),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));

            // Slider
            content(column);
        });
}

fn create_color_picker_section(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            padding: UiRect::all(Val::Px(15.0)),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        })
        .with_child((
            BackgroundColor(Color::srgba(0.15, 0.15, 0.15, 0.8)),
            BorderColor::all(Color::srgba(0.3, 0.3, 0.3, 0.5)),
        ))
        .with_children(|section| {
            // Title
            section.spawn((
                Text::new("RGB Color Picker"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Red slider
            SliderBuilder::new(0.0..255.0)
                .value(255.0)
                .format(ValueFormat::Custom(|v| format!("R: {:.0}", v)))
                .width(Val::Px(250.0))
                .step(1.0)
                .with_preview(true)
                .build(section);

            // Green slider
            SliderBuilder::new(0.0..255.0)
                .value(128.0)
                .format(ValueFormat::Custom(|v| format!("G: {:.0}", v)))
                .width(Val::Px(250.0))
                .step(1.0)
                .with_preview(true)
                .build(section);

            // Blue slider
            SliderBuilder::new(0.0..255.0)
                .value(64.0)
                .format(ValueFormat::Custom(|v| format!("B: {:.0}", v)))
                .width(Val::Px(250.0))
                .step(1.0)
                .with_preview(true)
                .build(section);

            // Color preview
            section.spawn((
                Node {
                    width: Val::Px(250.0),
                    height: Val::Px(30.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(1.0, 0.5, 0.25)),
                BorderColor::all(Color::WHITE),
            ));
        });
}

// System to monitor slider value changes
fn update_slider_values(
    slider_query: Query<&Slider, Changed<Slider>>,
) {
    for slider in &slider_query {
        info!("Slider value changed to: {:.2} (normalized: {:.2})", slider.value, slider.normalized());
    }
}