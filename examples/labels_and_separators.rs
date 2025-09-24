//! Labels and Separators Example - Demonstrates text styles and dividers
//!
//! Run with: cargo run --example labels_and_separators --features "label separator"

use bevy::prelude::*;
use bevy_ui_builders::*;
use bevy_ui_builders::label::LabelStyle;
use bevy_ui_builders::separator::{SeparatorStyle, Orientation};

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
            padding: UiRect::all(Val::Px(40.0)),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            column_gap: Val::Px(40.0),
            ..default()
        })
        .with_children(|parent| {
            // Labels section
            create_labels_section(parent);

            // Vertical separator
            SeparatorBuilder::new()
                .orientation(Orientation::Vertical)
                .style(SeparatorStyle::Solid)
                .build(parent);

            // Separators section
            create_separators_section(parent);
        });
}

fn create_labels_section(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        })
        .with_children(|column| {
            // Section title
            LabelBuilder::new("Label Styles")
                .style(LabelStyle::Title)
                .margin(UiRect::bottom(Val::Px(20.0)))
                .build(column);

            // All label styles
            LabelBuilder::new("Title Style - Large prominent text")
                .style(LabelStyle::Title)
                .build(column);

            LabelBuilder::new("Heading Style - Section headers")
                .style(LabelStyle::Heading)
                .build(column);

            LabelBuilder::new("Body Style - Normal content text")
                .style(LabelStyle::Body)
                .build(column);

            LabelBuilder::new("Caption Style - Small descriptive text")
                .style(LabelStyle::Caption)
                .build(column);

            LabelBuilder::new("Muted Style - De-emphasized text")
                .style(LabelStyle::Muted)
                .build(column);

            separator().margin(UiRect::vertical(Val::Px(10.0))).build(column);

            LabelBuilder::new("Error Style - Error messages")
                .style(LabelStyle::Error)
                .build(column);

            LabelBuilder::new("Success Style - Success messages")
                .style(LabelStyle::Success)
                .build(column);

            LabelBuilder::new("Warning Style - Warning messages")
                .style(LabelStyle::Warning)
                .build(column);

            separator().margin(UiRect::vertical(Val::Px(10.0))).build(column);

            // Custom styled labels
            label("Custom Color Label")
                .color(Color::srgb(0.5, 0.7, 1.0))
                .font_size(18.0)
                .build(column);

            label("Custom Margin Label")
                .style(LabelStyle::Body)
                .margin(UiRect::left(Val::Px(30.0)))
                .build(column);
        });
}

fn create_separators_section(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(25.0),
            width: Val::Px(300.0),
            ..default()
        })
        .with_children(|column| {
            // Section title
            label("Separator Styles")
                .style(LabelStyle::Title)
                .margin(UiRect::bottom(Val::Px(20.0)))
                .build(column);

            // Solid separator
            create_separator_demo(column, "Solid", SeparatorStyle::Solid);

            // Dashed separator
            create_separator_demo(column, "Dashed", SeparatorStyle::Dashed);

            // Dotted separator
            create_separator_demo(column, "Dotted", SeparatorStyle::Dotted);

            // Thick separator
            create_separator_demo(column, "Thick", SeparatorStyle::Thick);

            // Thin separator
            create_separator_demo(column, "Thin", SeparatorStyle::Thin);

            // Invisible separator (spacing only)
            label("Invisible (Above)").style(LabelStyle::Caption).build(column);

            SeparatorBuilder::new()
                .style(SeparatorStyle::Invisible)
                .margin(UiRect::vertical(Val::Px(20.0)))
                .build(column);

            label("Invisible (Below)").style(LabelStyle::Caption).build(column);

            // Custom margins
            label("Custom Margins").style(LabelStyle::Body).build(column);

            SeparatorBuilder::new()
                .style(SeparatorStyle::Solid)
                .margin(UiRect::horizontal(Val::Px(50.0)))
                .build(column);

            // Horizontal row with vertical separators
            column
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(15.0),
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                })
                .with_children(|row| {
                    label("Item 1").build(row);

                    SeparatorBuilder::new()
                        .orientation(Orientation::Vertical)
                        .style(SeparatorStyle::Solid)
                        .build(row);

                    label("Item 2").build(row);

                    SeparatorBuilder::new()
                        .orientation(Orientation::Vertical)
                        .style(SeparatorStyle::Dashed)
                        .build(row);

                    label("Item 3").build(row);
                });
        });
}

fn create_separator_demo(parent: &mut ChildSpawnerCommands, name: &str, style: SeparatorStyle) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            ..default()
        })
        .with_children(|section| {
            label(name).style(LabelStyle::Caption).build(section);

            SeparatorBuilder::new()
                .style(style)
                .build(section);
        });
}