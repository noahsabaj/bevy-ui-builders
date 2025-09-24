# Bevy UI Builders

[![Crates.io](https://img.shields.io/crates/v/bevy-ui-builders)](https://crates.io/crates/bevy-ui-builders)
[![Documentation](https://docs.rs/bevy-ui-builders/badge.svg)](https://docs.rs/bevy-ui-builders)
[![License](https://img.shields.io/crates/l/bevy-ui-builders)](LICENSE-MIT)

## Before using bevy-ui-builders

```rust
commands.spawn((
    Button,
    Node {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(2.0)),
        margin: UiRect::all(Val::Px(10.0)),
        padding: UiRect::all(Val::Px(10.0)),
        ..default()
    },
    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
    BorderColor(Color::BLACK),
)).with_children(|parent| {
    parent.spawn((
        Text::new("Click Me"),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
    ));
});

// Plus 30+ more lines for hover system...
```

## After using bevy-ui-builders

```rust
// AFTER: With bevy-ui-builders (3 lines!)
use bevy_ui_builders::*;

ButtonBuilder::new("Click Me")
    .style(ButtonStyle::Primary)
    .build(parent);
```

**That's it!** Hover effects, styling, and interaction handling included.

## Real-World Battle-Tested

These builders power **[Living Worlds](https://github.com/noahsabaj/livingworlds)**, a Steam game launching December 2025.

- **336+ uses** across 72 files
- **6,300+ lines** of boilerplate eliminated
- **Zero learning curve** - if you know Rust, you know this

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
bevy = "0.16"
bevy-ui-builders = "0.1"
```

Add the plugin:

```rust
use bevy::prelude::*;
use bevy_ui_builders::UiBuilderPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiBuilderPlugin)
        .run();
}
```

## Complete Builder Catalog

### 1. ButtonBuilder - Styled Interactive Buttons

```rust
// Simple button
ButtonBuilder::new("Click Me").build(parent);

// Styled button with size
ButtonBuilder::new("Submit")
    .style(ButtonStyle::Primary)  // Primary, Secondary, Success, Danger, Warning, Ghost
    .size(ButtonSize::Large)       // Small, Medium, Large, XLarge
    .with_marker(SubmitButton)     // Add your own marker component
    .build(parent);

// Convenience functions
primary_button("Save").build(parent);
danger_button("Delete").build(parent);
ghost_button("Cancel").build(parent);
```

### 2. DialogBuilder - Modal Dialogs with Overlays

```rust
// Confirmation dialog
DialogBuilder::new(DialogType::Confirmation)
    .title("Delete Item?")
    .body("This action cannot be undone.")
    .danger_button("Delete")
    .cancel_button("Cancel")
    .dismissible(false)  // Can't click outside to close
    .z_index(1000)      // Layer above other UI
    .build(commands);

// Preset dialogs
use bevy_ui_builders::dialog::presets;
presets::error(commands, "File not found!");
presets::unsaved_changes(commands);
```

### 3. TextInputBuilder - Advanced Text Inputs

```rust
// Text input with validation
TextInputBuilder::new()
    .with_placeholder("Enter email...")
    .with_filter(InputFilter::Email)     // Email validation
    .with_max_length(100)
    .with_focus_group(FocusGroupId::LoginForm)  // Focus management
    .with_clear_button()                 // X button to clear
    .with_marker(EmailInput)
    .build(parent);

// Numeric input
text_input()
    .numeric_only()      // Only allows 0-9
    .with_value("0")
    .build(parent);
```

### 4. FormBuilder - Complete Forms with Validation

```rust
FormBuilder::new()
    .title("User Registration")
    .field(FieldType::Text, "username", "Username")
    .field(FieldType::Email, "email", "Email Address")
    .field(FieldType::Password, "password", "Password")
    .field(FieldType::Checkbox, "terms", "I accept the terms")
    .validation("username", ValidationRule::Required)
    .validation("username", ValidationRule::MinLength(3))
    .validation("email", ValidationRule::Required)
    .validation("email", ValidationRule::Email)
    .submit_button("Register")
    .cancel_button("Cancel")
    .build(parent);
```

### 5. SliderBuilder - Value Sliders with Formatting

```rust
// Percentage slider
SliderBuilder::new(0.5, 0.0..=1.0)
    .width(Val::Px(200.0))
    .with_label(ValueFormat::Percentage)  // Shows "50%"
    .with_marker(VolumeSlider)
    .build(parent);

// Custom format
slider(75.0, 0.0..=100.0)
    .with_label(ValueFormat::Custom(|v| format!("{:.0}°C", v)))
    .build(parent);
```

### 6. ProgressBarBuilder - Progress Indicators

```rust
// Basic progress bar
ProgressBarBuilder::new(0.75)  // 75% complete
    .style(ProgressBarStyle::Thick)
    .with_label()  // Shows "75%"
    .animated()    // Smooth transitions
    .build(parent);

// Custom styled
progress(loading_progress)
    .fill_color(Color::srgb(0.2, 0.8, 0.2))
    .track_color(Color::srgb(0.1, 0.1, 0.1))
    .with_label_text("Loading assets...")
    .build(parent);
```

### 7. PanelBuilder - Flexible Container Panels

```rust
// Card panel with title
PanelBuilder::new()
    .style(PanelStyle::Card)
    .width(Val::Px(400.0))
    .padding(UiRect::all(Val::Px(24.0)))
    .with_title("Settings")
    .build_with_children(parent, |panel| {
        // Add any children here
        LabelBuilder::new("Audio Settings").build(panel);
        SeparatorBuilder::new().build(panel);
        // More UI...
    });

// Transparent overlay panel
panel()
    .style(PanelStyle::Transparent)
    .position_type(PositionType::Absolute)
    .build(parent);
```

### 8. LabelBuilder - Consistent Text Labels

```rust
// Styled labels
LabelBuilder::new("Welcome!")
    .style(LabelStyle::Title)    // Title, Heading, Body, Caption
    .text_align(JustifyContent::Center)
    .margin(UiRect::bottom(Val::Px(20.0)))
    .build(parent);

// Status labels
label("Error: File not found")
    .style(LabelStyle::Error)    // Error, Success, Warning, Muted
    .build(parent);
```

### 9. SeparatorBuilder - Visual Dividers

```rust
// Horizontal separator
SeparatorBuilder::new()
    .style(SeparatorStyle::Dashed)
    .margin(UiRect::vertical(Val::Px(20.0)))
    .build(parent);

// Vertical divider
separator_vertical()
    .style(SeparatorStyle::Thick)
    .length(Val::Percent(80.0))
    .build(parent);
```

## The Cleanup Revolution

Never write cleanup boilerplate again:

```rust
// Before: 15 lines of cleanup per UI system
fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// After: One line, works for ANY component!
app.add_systems(OnExit(GameState::Menu), despawn_ui_entities::<MenuRoot>);
```

## Complete Example

```rust
use bevy::prelude::*;
use bevy_ui_builders::*;

#[derive(Component)]
struct LoginForm;

#[derive(Component)]
struct SubmitButton;

fn setup_ui(mut commands: Commands) {
    // Create a login form
    commands.spawn(Camera2d);

    commands
        .spawn((Node::default(), LoginForm))
        .with_children(|parent| {
            PanelBuilder::new()
                .style(PanelStyle::Card)
                .width(Val::Px(400.0))
                .padding(UiRect::all(Val::Px(32.0)))
                .with_title("Login")
                .build_with_children(parent, |panel| {
                    // Username input
                    LabelBuilder::new("Username")
                        .style(LabelStyle::Body)
                        .margin(UiRect::bottom(Val::Px(8.0)))
                        .build(panel);

                    TextInputBuilder::new()
                        .with_placeholder("Enter username...")
                        .with_focus_group(FocusGroupId::Custom(1))
                        .build(panel);

                    // Add spacing
                    SeparatorBuilder::new()
                        .style(SeparatorStyle::Invisible)
                        .margin(UiRect::vertical(Val::Px(16.0)))
                        .build(panel);

                    // Password input
                    LabelBuilder::new("Password")
                        .style(LabelStyle::Body)
                        .margin(UiRect::bottom(Val::Px(8.0)))
                        .build(panel);

                    TextInputBuilder::new()
                        .with_placeholder("Enter password...")
                        .with_focus_group(FocusGroupId::Custom(1))
                        .build(panel);

                    // Buttons
                    SeparatorBuilder::new()
                        .style(SeparatorStyle::Invisible)
                        .margin(UiRect::vertical(Val::Px(24.0)))
                        .build(panel);

                    ButtonBuilder::new("Login")
                        .style(ButtonStyle::Primary)
                        .size(ButtonSize::Large)
                        .with_marker(SubmitButton)
                        .build(panel);
                });
        });
}
```

## Feature Flags

Control which builders are included:

```toml
# Include everything (default)
bevy-ui-builders = "0.1"

# Or pick specific builders
bevy-ui-builders = { version = "0.1", default-features = false, features = ["button", "dialog"] }
```

Available features:
- `button` - ButtonBuilder
- `dialog` - DialogBuilder
- `text_input` - TextInputBuilder
- `form` - FormBuilder
- `slider` - SliderBuilder
- `progress` - ProgressBarBuilder
- `panel` - PanelBuilder
- `label` - LabelBuilder
- `separator` - SeparatorBuilder

## Why Choose bevy-ui-builders?

### We Fill The Gap

| Crate | Problem |
|-------|---------|
| `bevy_ui_builder` | Too basic, no text inputs or forms |
| `sickle_ui` | Complex macro system, steep learning curve |
| `bevy_dioxus` | Requires React/web knowledge |
| `bevy_egui` | Immediate mode, not native Bevy |
| **bevy-ui-builders** | **Just right!** Simple, powerful, pure Bevy |

### Unique Features

- **Automatic cleanup system** - Generic `despawn_ui_entities<T>`
- **Consistent styling** - Centralized colors and dimensions
- **Rich text inputs** - Filtering, validation, focus management
- **Type-safe markers** - Add your own components to any builder
- **Gateway architecture** - Clean module boundaries
- **Battle-tested** - Used in production Steam game

## Bevy Version Support

| bevy-ui-builders | Bevy |
|-----------------|------|
| 0.1             | 0.16 |

## License

Dual-licensed under either:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

## Contributing

Contributions welcome! Please check [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, coding standards, and how to submit pull requests.

## Credits

Created by [Noah Sabaj](https://github.com/noahsabaj), the creator of [bevy-plugin-builder](https://github.com/noahsabaj/bevy-plugin-builder).

---

## The Bottom Line

```rust
// From 45 lines to 3. That's the power of builders.
ButtonBuilder::new("Play Game")
    .style(ButtonStyle::Primary)
    .build(parent);
```

## Complete Project Structure

```
bevy-ui-builders/
├── Cargo.toml              # Package manifest
├── README.md               # Public documentation
├── CLAUDE.md               # THIS FILE - AI assistant instructions
├── examples/               # Coming soon!
│   └── (examples will be added in v0.2.0)
└── src/
    ├── lib.rs              # Main library entry point
    │
    ├── button/             # ButtonBuilder module
    │   ├── mod.rs          # Gateway (exports only)
    │   ├── builder.rs      # ButtonBuilder implementation
    │   ├── plugin.rs       # ButtonPlugin
    │   ├── systems.rs      # Button interaction systems
    │   └── types.rs        # Component types (StyledButton, etc.)
    │
    ├── slider/             # SliderBuilder module
    │   ├── mod.rs          # Gateway
    │   ├── builder.rs      # SliderBuilder implementation
    │   ├── plugin.rs       # SliderPlugin
    │   ├── systems.rs      # Slider drag systems
    │   └── types.rs        # Slider, SliderHandle, SliderTrack
    │
    ├── form/               # FormBuilder module
    │   ├── mod.rs          # Gateway
    │   ├── builder.rs      # FormBuilder implementation
    │   ├── field.rs        # Form field implementations
    │   └── types.rs        # FieldType, ValidationRule
    │
    ├── dialog/             # DialogBuilder module
    │   ├── mod.rs          # Gateway
    │   ├── builder.rs      # DialogBuilder implementation
    │   ├── plugin.rs       # DialogPlugin
    │   ├── systems.rs      # Dialog interaction (ESC, overlay clicks)
    │   └── types.rs        # DialogOverlay, DialogType, button markers
    │
    ├── text_input/         # TextInputBuilder module
    │   ├── mod.rs          # Gateway
    │   ├── builder.rs      # TextInputBuilder implementation
    │   ├── plugin.rs       # TextInputPlugin
    │   ├── systems.rs      # Focus management, validation
    │   └── types.rs        # InputFilter, TextInputFocus, etc.
    │
    ├── progress/           # ProgressBarBuilder module
    │   ├── mod.rs          # Gateway
    │   ├── builder.rs      # ProgressBarBuilder implementation
    │   ├── plugin.rs       # ProgressBarPlugin
    │   ├── systems.rs      # Progress bar update systems
    │   └── types.rs        # ProgressBar, ProgressBarStyle
    │
    ├── label/              # LabelBuilder module
    │   ├── mod.rs          # Gateway
    │   ├── builder.rs      # LabelBuilder implementation
    │   └── types.rs        # Label, LabelStyle
    │
    ├── panel/              # PanelBuilder module
    │   ├── mod.rs          # Gateway
    │   ├── builder.rs      # PanelBuilder implementation
    │   └── types.rs        # Panel, PanelStyle
    │
    ├── separator/          # SeparatorBuilder module
    │   ├── mod.rs          # Gateway
    │   ├── builder.rs      # SeparatorBuilder implementation
    │   └── types.rs        # Separator, SeparatorStyle, Orientation
    │
    ├── styles/             # Centralized styling
    │   ├── mod.rs          # Gateway (exports colors & dimensions)
    │   ├── colors.rs       # Color constants (PRIMARY, SECONDARY, etc.)
    │   └── dimensions.rs   # Size constants (FONT_SIZE_*, PADDING_*, etc.)
    │
    ├── systems/            # Shared systems
    │   ├── mod.rs          # Gateway
    │   ├── cleanup.rs      # Generic despawn_entities<T> system
    │   ├── hover.rs        # HoverPlugin for button effects
    │   └── interaction.rs  # Generic interaction handling
    │
    └── utils/              # Utilities
        ├── mod.rs          # Gateway
        └── intrinsic.rs    # Intrinsic sizing utilities

Total: 48 Rust files across 13 modules
```