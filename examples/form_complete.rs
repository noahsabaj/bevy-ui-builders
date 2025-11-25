//! Complete Form Example - Demonstrates FormBuilder with all field types
//!
//! Run with: cargo run --example form_complete --features form

use bevy::prelude::*;
use bevy_ui_builders::prelude::*;
use bevy_ui_builders::*;
use bevy_ui_builders::components::form::FormLayout;

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

    // Root container - using UiContainer to prevent B0004 warnings
    commands
        .spawn(UiContainer::fullscreen())
        .with_children(|root| {
            // Wrap everything in a ScrollView
            ScrollViewBuilder::new()
                .width(Val::Percent(100.0))
                .height(Val::Percent(100.0))
                .scrollbar_visibility(bevy_ui_builders::components::scroll_view::ScrollbarVisibility::AutoHide { timeout_secs: 2.0 })
                .build_with_children(root, |scroll_content| {
                    // Forms container
                    scroll_content.spawn((
                        UiContainer::row()
                            .padding_all(Val::Px(40.0))
                            .justify(JustifyContent::SpaceEvenly)
                            .align(AlignItems::Start)
                            .gap(Val::Px(40.0))
                            .build(),
                    )).with_children(|parent| {
                        // Login Form
                        create_login_form(parent);

                        // Registration Form
                        create_registration_form(parent);

                        // Settings Form
                        create_settings_form(parent);
                    });
                });
        });
}

fn create_login_form(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn(UiContainer::column().build())
        .with_children(|container| {
            // Title
            container.spawn((
                Text::new("Login Form"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Form
            FormBuilder::new("login_form")
                .title("Sign In")
                .text_field("username", "Username")
                .required()
                .placeholder("Enter username")
                .password_field("password", "Password")
                .required()
                .placeholder("Enter password")
                .checkbox_field("remember", "Remember me")
                .submit_text("Login")
                .cancel_text("Cancel")
                .width(Val::Px(350.0))
                .build(container);
        });
}

fn create_registration_form(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn(UiContainer::column().build())
        .with_children(|container| {
            // Title
            container.spawn((
                Text::new("Registration Form"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Form
            FormBuilder::new("registration_form")
                .title("Create Account")
                .text_field("first_name", "First Name")
                .required()
                .validate(ValidationRule::MinLength(2))
                .text_field("last_name", "Last Name")
                .required()
                .validate(ValidationRule::MinLength(2))
                .email_field("email", "Email Address")
                .required()
                .password_field("password", "Password")
                .required()
                .validate(ValidationRule::MinLength(8))
                .help_text("Must be at least 8 characters")
                .password_field("confirm_password", "Confirm Password")
                .required()
                .dropdown_field("country", "Country", vec![
                    "United States".to_string(),
                    "Canada".to_string(),
                    "United Kingdom".to_string(),
                    "Australia".to_string(),
                    "Other".to_string(),
                ])
                .checkbox_field("terms", "I agree to the terms and conditions")
                .required()
                .submit_text("Register")
                .cancel_text("Back")
                .width(Val::Px(400.0))
                .build(container);
        });
}

fn create_settings_form(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn(UiContainer::column().build())
        .with_children(|container| {
            // Title
            container.spawn((
                Text::new("Settings Form"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Form
            FormBuilder::new("settings_form")
                .title("Preferences")
                .dropdown_field("theme", "Theme", vec![
                    "Dark".to_string(),
                    "Light".to_string(),
                    "Auto".to_string(),
                ])
                .slider_field("volume", "Volume", 0.0, 100.0)
                .slider_field("brightness", "Brightness", 0.0, 100.0)
                .number_field("font_size", "Font Size", Some(8.0), Some(24.0))
                .checkbox_field("notifications", "Enable notifications")
                .checkbox_field("sound", "Enable sound")
                .checkbox_field("auto_save", "Auto-save")
                .text_field("display_name", "Display Name")
                .placeholder("Optional display name")
                .submit_text("Save Settings")
                .layout(FormLayout::Vertical)
                .width(Val::Px(350.0))
                .build(container);
        });
}