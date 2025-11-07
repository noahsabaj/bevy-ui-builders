//! Text Input Demo - Demonstrates native text input features
//!
//! Run with: cargo run --example text_input_demo --features text_input
//!
//! Features showcased:
//! - Text selection (Ctrl+A, Shift+arrows)
//! - Clipboard operations (Ctrl+C, Ctrl+V, Ctrl+X)
//! - Undo/Redo (Ctrl+Z, Ctrl+Shift+Z)
//! - Input filtering (numeric, alphabetic, alphanumeric)
//! - Max length constraints
//! - Clear buttons
//! - Focus groups with Tab navigation
//! - Password masking
//! - Placeholder text

use bevy::prelude::*;
use bevy_ui_builders::*;
use bevy_ui_builders::text_input::{InputFilter, FocusGroupId, InputTransform};
use bevy_ui_builders::scroll_view::{ScrollViewBuilder, ScrollDirection};
use bevy_ui_builders::dimensions::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiBuilderPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, log_text_changes)
        .run();
}

// Custom marker components for inputs
#[derive(Component)]
struct UsernameInput;

#[derive(Component)]
struct EmailInput;

#[derive(Component)]
struct PasswordInput;

#[derive(Component)]
struct PhoneInput;

#[derive(Component)]
struct NotesInput;

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);

    // Root node with full viewport
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        })
        .with_children(|parent| {
            // Scrollable container for all content
            ScrollViewBuilder::new()
                .max_height(Val::Vh(100.0))  // Full viewport height
                .padding_vh(3.0)              // 3% viewport padding
                .gap(Val::Vh(3.0))            // 3% viewport gap between elements
                .build_with_children(parent, |scroll_container| {
            // Title
            scroll_container.spawn((
                Text::new("Native Text Input Demo"),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Two column layout with responsive sizing
            scroll_container
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Vw(3.0),  // 3% viewport width gap
                    flex_wrap: FlexWrap::Wrap,  // Allow wrapping on small screens
                    ..default()
                })
                .with_children(|columns| {
                    // Left column - Basic features
                    columns
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Vh(2.5),  // 2.5% viewport height
                            min_width: Val::Px(350.0),  // Minimum width
                            flex_basis: Val::Percent(45.0),  // Take 45% of available width
                            flex_grow: 1.0,
                            flex_shrink: 1.0,
                            ..default()
                        })
                        .with_children(|container| {
                            // Section title
                            container.spawn((
                                Text::new("Basic Features"),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            ));

                            // Basic text input
                            create_input_section(container, "Standard Input", |section| {
                                TextInputBuilder::new()
                                    .with_placeholder("Type anything here...")
                                    .build(section);
                            });

                            // Input with value and clear button
                            create_input_section(container, "With Clear Button", |section| {
                                TextInputBuilder::new()
                                    .with_value("Click × to clear")
                                    .with_clear_button()
                                    .build(section);
                            });

                            // Password input (would need mask_char support)
                            create_input_section(container, "Password Field", |section| {
                                TextInputBuilder::new()
                                    .with_placeholder("Enter password...")
                                    .with_marker(PasswordInput)
                                    .build(section);
                                // Note: mask_char would be set in TextInputVisual
                            });

                            // Numeric only input
                            create_input_section(container, "Numbers Only", |section| {
                                TextInputBuilder::new()
                                    .with_placeholder("0-9 only...")
                                    .numeric_only()
                                    .build(section);
                            });

                            // Integer input (with negative)
                            create_input_section(container, "Integer Input", |section| {
                                TextInputBuilder::new()
                                    .with_placeholder("Allows negative...")
                                    .integer_only()
                                    .with_value("-42")
                                    .build(section);
                            });

                            // Decimal input
                            create_input_section(container, "Decimal Numbers", |section| {
                                TextInputBuilder::new()
                                    .with_placeholder("3.14159...")
                                    .decimal_only()
                                    .build(section);
                            });

                            // Alphabetic only
                            create_input_section(container, "Letters Only", |section| {
                                TextInputBuilder::new()
                                    .with_placeholder("A-Z, a-z only...")
                                    .alphabetic_only()
                                    .build(section);
                            });

                            // Alphanumeric
                            create_input_section(container, "Alphanumeric", |section| {
                                TextInputBuilder::new()
                                    .with_placeholder("Letters and numbers...")
                                    .alphanumeric_only()
                                    .build(section);
                            });
                        });

                    // Right column - Advanced features
                    columns
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Vh(2.5),  // 2.5% viewport height
                            min_width: Val::Px(350.0),  // Minimum width
                            flex_basis: Val::Percent(45.0),  // Take 45% of available width
                            flex_grow: 1.0,
                            flex_shrink: 1.0,
                            ..default()
                        })
                        .with_children(|container| {
                            // Section title
                            container.spawn((
                                Text::new("Advanced Features"),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            ));

                            // Max length constraint
                            create_input_section(container, "Max Length (10 chars)", |section| {
                                TextInputBuilder::new()
                                    .with_placeholder("Max 10 characters...")
                                    .with_max_length(10)
                                    .with_clear_button()
                                    .build(section);
                            });

                            // Text transformation - uppercase
                            create_input_section(container, "Auto Uppercase", |section| {
                                TextInputBuilder::new()
                                    .with_placeholder("Converts to UPPERCASE...")
                                    .with_transform(InputTransform::Uppercase)
                                    .build(section);
                            });

                            // Text transformation - lowercase
                            create_input_section(container, "Auto Lowercase", |section| {
                                TextInputBuilder::new()
                                    .with_placeholder("converts to lowercase...")
                                    .with_transform(InputTransform::Lowercase)
                                    .with_value("HELLO WORLD")
                                    .build(section);
                            });

                            // Username with marker
                            create_input_section(container, "Username (with validation)", |section| {
                                TextInputBuilder::new()
                                    .with_placeholder("Enter username...")
                                    .alphanumeric_only()
                                    .with_max_length(20)
                                    .with_transform(InputTransform::Lowercase)
                                    .with_marker(UsernameInput)
                                    .build(section);
                            });

                            // Email with multiple markers
                            create_input_section(container, "Email Address", |section| {
                                TextInputBuilder::new()
                                    .with_placeholder("user@example.com")
                                    .with_marker(EmailInput)
                                    .and_marker(RequiredField)
                                    .build(section);
                            });

                            // Phone number
                            create_input_section(container, "Phone Number", |section| {
                                TextInputBuilder::new()
                                    .with_placeholder("555-1234")
                                    .with_filter(InputFilter::Custom(|s| {
                                        s.chars().all(|c| c.is_numeric() || c == '-' || c == ' ')
                                    }))
                                    .with_marker(PhoneInput)
                                    .build(section);
                            });

                            // Inactive/Read-only input
                            create_input_section(container, "Read-Only Field", |section| {
                                TextInputBuilder::new()
                                    .with_value("This field is read-only")
                                    .inactive()
                                    .build(section);
                            });
                        });
                });

            // Focus group demonstration
            scroll_container
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Vh(1.5),  // 1.5% viewport height
                    padding: UiRect::all(Val::Vh(2.0)),  // 2% viewport height
                    border: UiRect::all(Val::Px(1.0)),
                    margin: UiRect::top(Val::Vh(2.0)),  // 2% viewport height
                    width: Val::Percent(90.0),  // 90% of container width
                    ..default()
                })
                .with_child((
                    BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.8)),
                    BorderColor::all(Color::srgba(0.4, 0.4, 0.5, 0.5)),
                ))
                .with_children(|section| {
                    // Title
                    section.spawn((
                        Text::new("Focus Group (Tab Navigation)"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    // Form fields in focus group
                    section
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(15.0),
                            ..default()
                        })
                        .with_children(|form| {
                            // First name
                            create_input_section(form, "First Name", |input_section| {
                                TextInputBuilder::new()
                                    .with_placeholder("John")
                                    .with_focus_group(FocusGroupId::Custom(1))
                                    .build(input_section);
                            });

                            // Last name
                            create_input_section(form, "Last Name", |input_section| {
                                TextInputBuilder::new()
                                    .with_placeholder("Doe")
                                    .with_focus_group(FocusGroupId::Custom(1))
                                    .build(input_section);
                            });

                            // Age
                            create_input_section(form, "Age", |input_section| {
                                TextInputBuilder::new()
                                    .with_placeholder("25")
                                    .numeric_only()
                                    .with_max_length(3)
                                    .with_focus_group(FocusGroupId::Custom(1))
                                    .build(input_section);
                            });
                        });
                });

            // Keyboard shortcuts reference
            scroll_container
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Vh(0.8),  // 0.8% viewport height
                    padding: UiRect::all(Val::Vh(1.5)),  // 1.5% viewport height
                    border: UiRect::all(Val::Px(1.0)),
                    margin: UiRect::top(Val::Vh(2.0)),  // 2% viewport height
                    ..default()
                })
                .with_child((
                    BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.8)),
                    BorderColor::all(Color::srgba(0.2, 0.2, 0.2, 0.5)),
                ))
                .with_children(|shortcuts| {
                    shortcuts.spawn((
                        Text::new("Keyboard Shortcuts:"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));

                    let shortcut_list = [
                        ("Ctrl+A", "Select all text"),
                        ("Ctrl+C", "Copy selected text"),
                        ("Ctrl+V", "Paste from clipboard"),
                        ("Ctrl+X", "Cut selected text"),
                        ("Ctrl+Z", "Undo last change"),
                        ("Ctrl+Shift+Z", "Redo last undo"),
                        ("Shift+Arrow", "Extend selection"),
                        ("Ctrl+Arrow", "Jump by word"),
                        ("Home/End", "Jump to line start/end"),
                        ("Tab", "Next field (in focus group)"),
                        ("Shift+Tab", "Previous field (in focus group)"),
                        ("Escape", "Clear selection / Unfocus"),
                        ("Enter", "Submit (single-line inputs)"),
                    ];

                    for (key, description) in shortcut_list {
                        shortcuts
                            .spawn(Node {
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(10.0),
                                ..default()
                            })
                            .with_children(|row| {
                                // Key
                                row.spawn((
                                    Text::new(key),
                                    TextFont {
                                        font_size: 13.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.6, 0.8, 1.0)),
                                ));
                                // Description
                                row.spawn((
                                    Text::new(description),
                                    TextFont {
                                        font_size: 13.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.7, 0.7, 0.7)),
                                ));
                            });
                    }
                });
            }); // End of scrollable container
        }); // End of root node
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
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));

            // Input
            content(section);
        });
}

// Marker component for required fields
#[derive(Component)]
struct RequiredField;

// System to log text changes for demonstration
fn log_text_changes(
    username_query: Query<&bevy_ui_builders::text_input::native_input::TextBuffer, (With<UsernameInput>, Changed<bevy_ui_builders::text_input::native_input::TextBuffer>)>,
    email_query: Query<&bevy_ui_builders::text_input::native_input::TextBuffer, (With<EmailInput>, Changed<bevy_ui_builders::text_input::native_input::TextBuffer>)>,
) {
    for buffer in &username_query {
        info!("Username changed: {}", buffer.content);
    }
    for buffer in &email_query {
        info!("Email changed: {}", buffer.content);
    }
}