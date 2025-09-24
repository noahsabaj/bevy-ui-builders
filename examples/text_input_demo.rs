//! Text Input Demo - Demonstrates text input features, filters, and validation
//!
//! Run with: cargo run --example text_input_demo --features text_input

use bevy::prelude::*;
use bevy_ui_builders::*;
use bevy_ui_builders::text_input::{InputFilter, FocusGroupId};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiBuilderPlugin)
        .add_systems(Startup, setup)
        .run();
}

// Custom marker components for inputs
#[derive(Component)]
struct UsernameInput;

#[derive(Component)]
struct EmailInput;

#[derive(Component)]
struct PhoneInput;

#[derive(Component)]
struct AgeInput;

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);

    // Root node
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(40.0)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(30.0),
            ..default()
        })
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Text Input Demo"),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Main container
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(25.0),
                    width: Val::Px(400.0),
                    ..default()
                })
                .with_children(|container| {
                    // Basic text input
                    create_input_section(container, "Basic Input", |section| {
                        TextInputBuilder::new()
                            .with_placeholder("Enter text here...")
                            .build(section);
                    });

                    // Input with value and clear button
                    create_input_section(container, "With Clear Button", |section| {
                        TextInputBuilder::new()
                            .with_value("Pre-filled text")
                            .with_clear_button()
                            .build(section);
                    });

                    // Numeric only input
                    create_input_section(container, "Numbers Only", |section| {
                        TextInputBuilder::new()
                            .with_placeholder("Enter numbers...")
                            .numeric_only()
                            .with_value("12345")
                            .build(section);
                    });

                    // Alphanumeric only input
                    create_input_section(container, "Alphanumeric Only", |section| {
                        TextInputBuilder::new()
                            .with_placeholder("Letters and numbers only...")
                            .alphanumeric_only()
                            .build(section);
                    });

                    // Input with max length
                    create_input_section(container, "Max Length (10 chars)", |section| {
                        TextInputBuilder::new()
                            .with_placeholder("Max 10 characters...")
                            .with_max_length(10)
                            .with_clear_button()
                            .build(section);
                    });

                    // Input with custom marker component
                    create_input_section(container, "Username (with marker)", |section| {
                        TextInputBuilder::new()
                            .with_placeholder("Enter username...")
                            .alphanumeric_only()
                            .with_max_length(20)
                            .with_marker(UsernameInput)
                            .build(section);
                    });

                    // Email input with multiple markers
                    create_input_section(container, "Email (with two markers)", |section| {
                        TextInputBuilder::new()
                            .with_placeholder("user@example.com")
                            .with_filter(InputFilter::None)  // Email validation would use regex or custom function
                            .with_marker(EmailInput)
                            .and_marker(RequiredField)
                            .build(section);
                    });

                    // Focus group demonstration
                    create_focus_group_section(container);
                });

            // Instructions
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    padding: UiRect::all(Val::Px(15.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                })
                .with_child((
                    BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.5)),
                    BorderColor(Color::srgba(0.3, 0.3, 0.3, 0.5)),
                ))
                .with_children(|instructions| {
                    instructions.spawn((
                        Text::new("Tips:"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    instructions.spawn((
                        Text::new("Click on any input to focus"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));

                    instructions.spawn((
                        Text::new("Type to enter text"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));

                    instructions.spawn((
                        Text::new("Tab/Shift+Tab to navigate in focus groups"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));

                    instructions.spawn((
                        Text::new("Some inputs have validation/filters"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));
                });
        });
}

fn create_input_section<F>(parent: &mut ChildSpawnerCommands, label: &str, content: F)
where
    F: FnOnce(&mut ChildSpawnerCommands),
{
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            ..default()
        })
        .with_children(|section| {
            // Label
            section.spawn((
                Text::new(label),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));

            // Input
            content(section);
        });
}

fn create_focus_group_section(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(15.0),
            padding: UiRect::all(Val::Px(15.0)),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        })
        .with_child((
            BackgroundColor(Color::srgba(0.15, 0.15, 0.15, 0.8)),
            BorderColor(Color::srgba(0.3, 0.3, 0.3, 0.5)),
        ))
        .with_children(|section| {
            // Title
            section.spawn((
                Text::new("Focus Group Example (Tab Navigation)"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // First name input
            create_input_section(section, "First Name", |input_section| {
                TextInputBuilder::new()
                    .with_placeholder("John")
                    .with_focus_group(FocusGroupId::Custom(1))
                    .build(input_section);
            });

            // Last name input
            create_input_section(section, "Last Name", |input_section| {
                TextInputBuilder::new()
                    .with_placeholder("Doe")
                    .with_focus_group(FocusGroupId::Custom(1))
                    .build(input_section);
            });

            // Email input in same focus group
            create_input_section(section, "Email", |input_section| {
                TextInputBuilder::new()
                    .with_placeholder("john.doe@example.com")
                    .with_focus_group(FocusGroupId::Custom(1))
                    .build(input_section);
            });

            // Phone input in same focus group
            create_input_section(section, "Phone", |input_section| {
                TextInputBuilder::new()
                    .with_placeholder("555-1234")
                    .numeric_only()
                    .with_focus_group(FocusGroupId::Custom(1))
                    .with_marker(PhoneInput)
                    .build(input_section);
            });
        });
}

// Marker component for required fields
#[derive(Component)]
struct RequiredField;