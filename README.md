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
use bevy_ui_builders::*;

ButtonBuilder::new("Click Me")
    .style(ButtonStyle::Primary)
    .build(parent);
```

**That's it!** Hover effects, styling, and interaction handling included.

## What's New

### Major Additions
- **ScrollViewBuilder** (v0.1.4) - Responsive scrollable containers with viewport-based sizing
  - Use `Val::Vh()` and `Val::Vw()` for responsive layouts
  - Mouse wheel scrolling, visual scrollbars, auto-scroll to focused inputs
- **Native Text Input** (v0.1.4) - Full text editing implementation
  - Cursor rendering, text selection, clipboard ops (Ctrl+C/V/X)
  - Undo/Redo (Ctrl+Z/Shift+Z), password masking
- **Dialog Custom Markers** (v0.1.7) - Add your own components to dialog buttons
  - `build_with_buttons()` returns button entities
  - `build_and_mark()` helper for single marker
- **Enhanced Builder Methods** (v0.1.5-v0.1.6)
  - ButtonBuilder: `margin()`, `height()`, `enabled()`, `build_in()`
  - SliderBuilder: `with_format()`, `with_marker()`, `build_in()`
  - PanelBuilder: `border_color()`, `flex_grow()`, `flex_shrink()`

### Upgrade Guide
All v0.1.x releases maintain full backward compatibility - just update to the latest version!

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

// Styled button with all options
ButtonBuilder::new("Submit")
    .style(ButtonStyle::Primary)  // Primary, Secondary, Success, Danger, Warning, Ghost
    .size(ButtonSize::Large)       // Small, Medium, Large, XLarge
    .with_marker(SubmitButton)     // Add your own marker component
    .margin(UiRect::all(Val::Px(10.0)))  // Custom margins
    .height(Val::Px(50.0))         // Custom height
    .enabled(true)                 // Enable/disable state
    .build(parent);                // Or use .build_in(parent) alias

// Convenience functions
primary_button("Save").build(parent);
danger_button("Delete").build(parent);
ghost_button("Cancel").build(parent);
```

### 2. DialogBuilder - Modal Dialogs with Overlays

```rust
// Basic dialog
DialogBuilder::new(DialogType::Custom)
    .title("Delete Item?")
    .body("This action cannot be undone.")
    .danger_button("Delete")
    .cancel_button("Cancel")
    .dismissible(false)  // Can't click outside to close
    .z_index(1000)      // Layer above other UI
    .build(commands);

// NEW: Get button entities for custom markers (v0.1.7)
let (dialog, buttons) = DialogBuilder::new(DialogType::Custom)
    .title("Confirm")
    .confirm_button("Yes")
    .cancel_button("No")
    .build_with_buttons(commands);

// Add your own markers to dialog buttons
if let Some(btn) = buttons.get(&DialogButtonMarker::Confirm) {
    commands.entity(*btn).insert(MyCustomMarker);
}

// Helper for single marker
DialogBuilder::new(DialogType::Custom)
    .danger_button("Delete")
    .build_and_mark(commands, DialogButtonMarker::Confirm, DeleteMarker);

// Preset dialogs - static methods
DialogBuilder::error(commands, "File not found!");
DialogBuilder::unsaved_changes(commands);
DialogBuilder::confirm(commands, "Delete?", "This cannot be undone.");
```

### 3. TextInputBuilder - Native Text Editing with Full Features

```rust
// Full-featured text input with native implementation (v0.1.4+)
TextInputBuilder::new()
    .with_placeholder("Enter email...")
    .with_filter(InputFilter::Alphanumeric)  // Allow letters and numbers
    .with_max_length(100)
    .with_focus_group(FocusGroupId::LoginForm)  // Tab navigation
    .with_clear_button()                 // X button to clear
    .with_marker(EmailInput)
    .build(parent);

// Password input with masking
text_input()
    .with_placeholder("Password")
    .password()                          // Masks input with bullets
    .build(parent);

// Numeric input
text_input()
    .numeric_only()      // Only allows 0-9
    .with_value("0")
    .build(parent);
```

**Native Features (v0.1.4+)**:
- Full cursor rendering with blinking
- Text selection (keyboard Shift+arrows & mouse drag)
- Clipboard operations (Ctrl+C/V/X)
- Undo/Redo support (Ctrl+Z/Shift+Z)
- Proper Tab navigation between inputs
- Auto-scroll when text overflows

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

// Custom format with new methods
slider(75.0, 0.0..=100.0)
    .with_label(ValueFormat::Custom(|v| format!("{:.0}°C", v)))
    .with_format(ValueFormat::Integer)  // Alternative to format() method
    .with_marker(TemperatureSlider)     // Add custom marker component
    .build_in(parent);                   // Or use build() alias
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

### 7. ScrollViewBuilder - Responsive Scrollable Containers

```rust
// Basic scrollable container with viewport-based sizing
ScrollViewBuilder::new()
    .max_height(Val::Vh(80.0))  // 80% of viewport height
    .padding_vw(2.0)             // 2% viewport width padding
    .gap(Val::Vh(2.0))           // 2% viewport height gap between children
    .auto_scroll(true)           // Auto-scroll to focused elements
    .build_with_children(parent, |scroll| {
        // Add content that might overflow
        for i in 0..100 {
            LabelBuilder::new(format!("Item {}", i)).build(scroll);
        }
    });

// Horizontal scrolling
scroll_view()
    .direction(ScrollDirection::Horizontal)
    .max_width(Val::Vw(90.0))    // 90% viewport width
    .build(parent);
```

Features:
- Viewport-relative sizing (Val::Vh/Vw) for responsive design
- Mouse wheel scrolling support
- Visual scrollbars with auto-hide
- Auto-scroll to focused text inputs
- Smooth scroll animations

### 8. PanelBuilder - Flexible Container Panels

```rust
// Card panel with all options
PanelBuilder::new()
    .style(PanelStyle::Card)
    .width(Val::Px(400.0))
    .padding(UiRect::all(Val::Px(24.0)))
    .border_color(Color::srgb(0.3, 0.3, 0.3))  // Custom border
    .flex_grow(1.0)                      // Flexbox growth
    .flex_shrink(0.0)                    // Flexbox shrink
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

### 9. LabelBuilder - Consistent Text Labels

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

### 10. SeparatorBuilder - Visual Dividers

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

## Cleanup

Generic cleanup system:

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
    commands.spawn(Camera2d);

    // Use ScrollView for responsive layout (v0.1.4+)
    commands
        .spawn((Node::default(), LoginForm))
        .with_children(|parent| {
            ScrollViewBuilder::new()
                .max_height(Val::Vh(90.0))  // Responsive to viewport
                .padding_vw(2.0)            // Viewport-based padding
                .build_with_children(parent, |scroll| {
                    PanelBuilder::new()
                        .style(PanelStyle::Card)
                        .width(Val::Px(400.0))
                        .padding(UiRect::all(Val::Px(32.0)))
                        .with_title("Login")
                        .build_with_children(scroll, |panel| {
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
                                .password()  // Mask password (v0.1.4+)
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
                                .margin(UiRect::bottom(Val::Px(10.0)))
                                .build(panel);

                            // Show new dialog capabilities
                            ButtonBuilder::new("Forgot Password?")
                                .style(ButtonStyle::Ghost)
                                .build(panel);
                        });
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
bevy-ui-builders = { version = "0.1.7", default-features = false, features = ["button", "dialog"] }
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
- `cleanup` - Generic cleanup systems

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

Created by [Noah Sabaj](https://github.com/noahsabaj), the creator of [bevy-plugin-builder](https://github.com/noahsabaj/bevy-plugin-builder), and [bevy-test-suite](https://github.com/noahsabaj/bevy-test-suite).

---

## Bottom Line

```rust
// From 45 lines to 3. That's the power of builders.
ButtonBuilder::new("Play Game")
    .style(ButtonStyle::Primary)
    .build(parent);
```