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
    BorderColor::all(Color::BLACK),
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

## What's New in v0.2.0

### Bevy 0.17 Migration
- Updated to **Bevy 0.17** with full compatibility
- Auto-includes `DefaultPickingPlugins` for Interaction components
- Breaking changes handled internally

### New Builders (3 Added)
- **CheckboxBuilder** - Interactive checkboxes with toggle states
  - Styles: Primary, Success, Danger, Default
  - Configurable size and label positioning
- **DropdownBuilder** - Dropdown select menus
  - Click-outside-to-dismiss functionality
  - Proper z-index layering (fully opaque overlay)
- **NumberInputBuilder** - Validated number inputs
  - Auto-validates against min/max range
  - Shows red border when out of bounds

### Universal Validation System (NEW)
- Composable validation rules work with ANY input (not just forms)
- `TextInputBuilder.with_validation(Vec<ValidationRule>)` method
- Visual feedback: red border on error, default border when valid
- ValidationRule types: Required, MinLength, MaxLength, Range, Email, Pattern, Custom
- `ValidationState` and `Validated` ECS components

### Bug Fixes
- Fixed slider cursor offset bug
- Fixed button hover/press visual feedback
- Fixed scrollbar drag unfocusing when mouse exits window

### Complete Builder Count
Now includes **13 builders**: Button, Dialog, TextInput, Form, Slider, ProgressBar, Panel, Label, Separator, ScrollView, Checkbox, NumberInput, Dropdown

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
bevy = "0.17"
bevy-ui-builders = "0.2"
```

Add the plugin:

```rust
use bevy::prelude::*;
use bevy_ui_builders::UiBuilderPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiBuilderPlugin)  // Auto-includes DefaultPickingPlugins
        .run();
}
```

In your code, import the prelude for convenient access to all builders:

```rust
use bevy_ui_builders::prelude::*;

fn build_ui(mut commands: Commands) {
    commands.spawn(Camera2d);

    // All builders and types available
    ButtonBuilder::new("Click Me")
        .style(ButtonStyle::Primary)
        .build(&mut commands.spawn(Node::default()).commands());
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
// Full-featured text input with native implementation
TextInputBuilder::new()
    .with_placeholder("Enter email...")
    .with_filter(InputFilter::Alphanumeric)  // Allow letters and numbers
    .with_max_length(100)
    .with_width(Val::Px(300.0))          // Set width
    .with_height(Val::Px(40.0))          // Set height
    .with_focus_group(FocusGroupId::LoginForm)  // Tab navigation
    .with_clear_button()                 // X button to clear
    .with_marker(EmailInput)
    .build(parent);

// Text input with validation (v0.2.0+)
TextInputBuilder::new()
    .with_placeholder("Enter email")
    .with_validation(vec![
        ValidationRule::Required,
        ValidationRule::Email,
        ValidationRule::MinLength(5),
    ])
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

**Native Features**:
- Full cursor rendering with blinking
- Text selection (keyboard Shift+arrows & mouse drag)
- Clipboard operations (Ctrl+C/V/X)
- Undo/Redo support (Ctrl+Z/Shift+Z)
- Proper Tab navigation between inputs
- Auto-scroll when text overflows
- **Validation support with visual feedback** (v0.2.0+)

### 4. FormBuilder - Complete Forms with Validation

```rust
// Simple login form
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
    .build(parent);

// Complex registration with multiple field types
FormBuilder::new("registration_form")
    .title("Create Account")
    .text_field("first_name", "First Name")
    .required()
    .validate(ValidationRule::MinLength(2))
    .email_field("email", "Email Address")
    .required()
    .password_field("password", "Password")
    .required()
    .validate(ValidationRule::MinLength(8))
    .help_text("Must be at least 8 characters")
    .dropdown_field("country", "Country", vec![
        "United States".to_string(),
        "Canada".to_string(),
        "Other".to_string(),
    ])
    .checkbox_field("terms", "I agree to the terms")
    .required()
    .submit_text("Register")
    .width(Val::Px(400.0))
    .build(parent);
```

**Key API methods:**
- `FormBuilder::new(id: &str)` - Constructor takes form ID
- `.text_field(name, label)` - Text input field
- `.email_field(name, label)` - Email input with validation
- `.password_field(name, label)` - Password input (hidden text)
- `.checkbox_field(name, label)` - Checkbox field
- `.dropdown_field(name, label, options)` - Dropdown selection
- `.slider_field(name, label, min, max)` - Numeric slider
- `.number_field(name, label, min, max)` - Number input with range
- `.required()` - Mark previous field as required (chainable)
- `.validate(rule)` - Add validation rule to previous field (chainable)
- `.placeholder(text)` - Set placeholder for previous field (chainable)
- `.help_text(text)` - Add help text below previous field (chainable)
- `.submit_text(text)` - Set submit button text
- `.cancel_text(text)` - Set cancel button text
- `.width(Val)` - Set form width
- `.layout(FormLayout)` - Set layout (Vertical, Horizontal, Grid)

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
    .width(Val::Percent(100.0))   // Set width
    .height(Val::Percent(100.0))  // Set height
    .max_height(Val::Vh(80.0))    // 80% of viewport height
    .padding_vw(2.0)              // 2% viewport width padding
    .padding_vh(1.0)              // 1% viewport height padding
    .gap(Val::Vh(2.0))            // 2% viewport height gap between children
    .margin(UiRect::all(Val::Px(10.0)))  // Margin around scroll view
    .background_color(Color::srgb(0.1, 0.1, 0.1))  // Background color
    .auto_scroll(true)            // Auto-scroll to focused elements
    .scrollbar_visibility(ScrollbarVisibility::AutoHide { timeout_secs: 2.0 })
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

// Simple build without children (add content later)
let scroll_entity = ScrollViewBuilder::new()
    .width(Val::Px(400.0))
    .height(Val::Px(600.0))
    .build(parent);
```

**Features:**
- Viewport-relative sizing (Val::Vh/Vw) for responsive design
- Mouse wheel scrolling support
- Visual scrollbars with auto-hide timeout or always visible
- Auto-scroll to focused text inputs
- Smooth scroll animations
- Draggable scrollbar thumb
- Keyboard scrolling support

**Key methods:**
- `.width(Val)` - Set container width
- `.height(Val)` - Set container height
- `.padding_vh(f32)` - Viewport height padding
- `.padding_vw(f32)` - Viewport width padding
- `.margin(UiRect)` - Set margins
- `.background_color(Color)` - Background color
- `.scrollbar_visibility(ScrollbarVisibility)` - Always, AutoHide, or Hidden
- `.direction(ScrollDirection)` - Vertical (default) or Horizontal
- `.auto_scroll(bool)` - Auto-scroll to focused inputs

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

### 11. CheckboxBuilder - Interactive Checkboxes

```rust
// Simple checkbox
CheckboxBuilder::new()
    .checked(false)
    .with_label("Remember me")
    .build(parent);

// Styled checkbox with custom size
CheckboxBuilder::new()
    .checked(true)
    .style(CheckboxStyle::Primary)  // Primary, Success, Danger, Default
    .with_label("I agree to the terms")
    .label_on_right(true)  // true = right (default), false = left
    .size(24.0)  // Checkbox size in pixels (default: 20.0)
    .build(parent);
```

**Features:**
- Visual states: unchecked (empty box) and checked (blue fill with white "X")
- Styles: Default, Primary, Success, Danger
- Configurable checkbox size
- Label positioning (left or right)
- Click to toggle state

### 12. NumberInputBuilder - Validated Number Inputs

```rust
// Basic number input
NumberInputBuilder::new()
    .min(0.0)
    .max(100.0)
    .default_value(50.0)
    .build(parent);

// Number input with validation
NumberInputBuilder::new()
    .min(8.0)
    .max(24.0)
    .step(0.5)
    .default_value(16.0)
    .width(Val::Px(200.0))
    .with_placeholder("Enter font size")
    .build(parent);
```

**Features:**
- Automatic range validation (shows red border if out of range)
- Accepts decimal values
- Configurable min/max bounds
- Step size for increment/decrement
- Integrates with universal validation system
- Shows hint text: "Range: min-max"

### 13. DropdownBuilder - Dropdown Select Menus

```rust
// Simple dropdown
DropdownBuilder::new(vec![
    "Option 1".to_string(),
    "Option 2".to_string(),
    "Option 3".to_string(),
])
.build(parent);

// Dropdown with initial selection
DropdownBuilder::new(vec![
    "Light".to_string(),
    "Dark".to_string(),
    "Auto".to_string(),
])
.placeholder("Select theme")
.selected_index(Some(2))  // "Auto" pre-selected
.width(Val::Px(250.0))
.build(parent);
```

**Features:**
- Click-to-open menu overlay
- Click outside to dismiss
- Hover highlights options
- Displays selected value in button
- Fully opaque menu with proper z-index layering
- Only one option highlighted at a time

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

**Note:** In Bevy 0.16+, `despawn()` is automatically recursive - it will despawn all children. The deprecated `despawn_recursive()` is no longer needed.

### Relationship Auto-Cleanup

UI components use Bevy's relationship system with `linked_spawn` for automatic cleanup:

```rust
// When you close a dialog, all related elements (buttons, overlays) are automatically cleaned up
DialogBuilder::new(DialogType::Confirmation)
    .title("Delete?")
    .build(commands);

// Relationship components like BelongsToDialog with linked_spawn handle cleanup
// No manual cleanup needed!
```

Related entities are automatically despawned when the parent is despawned, thanks to the `linked_spawn` attribute on relationship targets.

## Complete Example

Showcasing multiple builders working together:

```rust
use bevy::prelude::*;
use bevy_ui_builders::prelude::*;

#[derive(Component)]
struct SettingsForm;

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2d);

    // ScrollView with responsive layout
    commands.spawn(Node::default()).with_children(|parent| {
        ScrollViewBuilder::new()
            .width(Val::Percent(100.0))
            .height(Val::Percent(100.0))
            .scrollbar_visibility(ScrollbarVisibility::AutoHide { timeout_secs: 2.0 })
            .build_with_children(parent, |scroll| {
                // Panel container
                PanelBuilder::new()
                    .style(PanelStyle::Card)
                    .width(Val::Px(450.0))
                    .padding(UiRect::all(Val::Px(32.0)))
                    .with_title("Settings")
                    .build_with_children(scroll, |panel| {
                        // Dropdown for theme selection
                        LabelBuilder::new("Theme")
                            .style(LabelStyle::Body)
                            .margin(UiRect::bottom(Val::Px(8.0)))
                            .build(panel);

                        DropdownBuilder::new(vec![
                            "Dark".to_string(),
                            "Light".to_string(),
                            "Auto".to_string(),
                        ])
                        .placeholder("Select theme")
                        .width(Val::Percent(100.0))
                        .build(panel);

                        SeparatorBuilder::new()
                            .style(SeparatorStyle::Invisible)
                            .margin(UiRect::vertical(Val::Px(16.0)))
                            .build(panel);

                        // Number input with validation
                        LabelBuilder::new("Font Size (8-24)")
                            .style(LabelStyle::Body)
                            .margin(UiRect::bottom(Val::Px(8.0)))
                            .build(panel);

                        NumberInputBuilder::new()
                            .min(8.0)
                            .max(24.0)
                            .default_value(16.0)
                            .width(Val::Percent(100.0))
                            .build(panel);

                        SeparatorBuilder::new()
                            .style(SeparatorStyle::Invisible)
                            .margin(UiRect::vertical(Val::Px(16.0)))
                            .build(panel);

                        // Email with validation
                        LabelBuilder::new("Email")
                            .style(LabelStyle::Body)
                            .margin(UiRect::bottom(Val::Px(8.0)))
                            .build(panel);

                        TextInputBuilder::new()
                            .with_placeholder("Enter email...")
                            .with_validation(vec![
                                ValidationRule::Required,
                                ValidationRule::Email,
                            ])
                            .width(Val::Percent(100.0))
                            .build(panel);

                        SeparatorBuilder::new()
                            .margin(UiRect::vertical(Val::Px(16.0)))
                            .build(panel);

                        // Checkboxes
                        CheckboxBuilder::new()
                            .checked(true)
                            .style(CheckboxStyle::Primary)
                            .with_label("Enable notifications")
                            .build(panel);

                        CheckboxBuilder::new()
                            .checked(false)
                            .with_label("Auto-save settings")
                            .build(panel);

                        SeparatorBuilder::new()
                            .style(SeparatorStyle::Invisible)
                            .margin(UiRect::vertical(Val::Px(24.0)))
                            .build(panel);

                        // Slider for volume
                        LabelBuilder::new("Volume")
                            .style(LabelStyle::Body)
                            .margin(UiRect::bottom(Val::Px(8.0)))
                            .build(panel);

                        SliderBuilder::new(0.0..=100.0)
                            .value(75.0)
                            .width(Val::Percent(100.0))
                            .with_label(ValueFormat::Percentage)
                            .build(panel);

                        SeparatorBuilder::new()
                            .style(SeparatorStyle::Invisible)
                            .margin(UiRect::vertical(Val::Px(24.0)))
                            .build(panel);

                        // Progress bar
                        ProgressBarBuilder::new(0.65)
                            .style(ProgressBarStyle::Thick)
                            .with_label()
                            .build(panel);

                        SeparatorBuilder::new()
                            .style(SeparatorStyle::Invisible)
                            .margin(UiRect::vertical(Val::Px(24.0)))
                            .build(panel);

                        // Action buttons
                        ButtonBuilder::new("Save Settings")
                            .style(ButtonStyle::Primary)
                            .size(ButtonSize::Large)
                            .width(Val::Percent(100.0))
                            .margin(UiRect::bottom(Val::Px(10.0)))
                            .build(panel);

                        ButtonBuilder::new("Reset to Defaults")
                            .style(ButtonStyle::Secondary)
                            .width(Val::Percent(100.0))
                            .build(panel);
                    });
            });
    });
}
```

This example demonstrates:
- **ScrollView** with auto-hide scrollbar
- **Panel** with Card style
- **Dropdown** for theme selection
- **NumberInput** with range validation (8-24)
- **TextInput** with email validation
- **Checkboxes** for boolean settings
- **Slider** with percentage formatting
- **ProgressBar** with label
- **Labels** and **Separators** for layout
- **Buttons** for actions

All 13 builders working together!

## Universal Validation System

The crate provides a composable validation system that works across all input types (not just forms).

### ValidationRule Types

```rust
use bevy_ui_builders::prelude::*;

// Available validation rules
ValidationRule::Required                    // Field cannot be empty
ValidationRule::MinLength(usize)            // Minimum character length
ValidationRule::MaxLength(usize)            // Maximum character length
ValidationRule::Range { min: f32, max: f32 } // Numeric range (for numbers)
ValidationRule::Email                       // Basic email format validation
ValidationRule::Pattern(String)             // Simple pattern matching
ValidationRule::Custom(fn(&str) -> Result<(), String>) // Custom validator
```

### Adding Validation to Inputs

```rust
// Validate any TextInput
TextInputBuilder::new()
    .with_placeholder("Enter email")
    .with_validation(vec![
        ValidationRule::Required,
        ValidationRule::Email,
        ValidationRule::MinLength(5),
    ])
    .build(parent);

// NumberInputBuilder has automatic validation
NumberInputBuilder::new()
    .min(8.0)
    .max(24.0)  // Automatically creates ValidationRule::Range { min: 8, max: 24 }
    .build(parent);

// Custom validation function
TextInputBuilder::new()
    .with_validation(vec![
        ValidationRule::Custom(|value| {
            if value.contains("admin") {
                Ok(())
            } else {
                Err("Must contain 'admin'".to_string())
            }
        })
    ])
    .build(parent);
```

### Visual Feedback

When validation fails:
- Input border turns **red** (`BORDER_ERROR` color)
- `ValidationState` component stores error message
- `Validated` component holds validation rules

When validation passes:
- Input border returns to **default** color
- `ValidationState.is_valid = true`

### How It Works

The validation system uses Bevy ECS components:

```rust
#[derive(Component)]
pub struct Validated {
    pub rules: Vec<ValidationRule>,
}

#[derive(Component)]
pub struct ValidationState {
    pub is_valid: bool,
    pub error_message: Option<String>,
}
```

A system monitors `Changed<TextBuffer>` and runs all validation rules:
- Checks each rule in order
- Stops at first error
- Updates `ValidationState`
- Changes border color

### Integration with Forms

FormBuilder fields automatically use validation:

```rust
FormBuilder::new("my_form")
    .text_field("username", "Username")
    .required()  // Adds ValidationRule::Required
    .validate(ValidationRule::MinLength(3))  // Adds custom rule
    .email_field("email", "Email")
    .required()  // Adds Required + Email rules
    .build(parent);
```

## Feature Flags

Control which builders are included:

```toml
# Include everything (default)
bevy-ui-builders = "0.2"

# Or pick specific builders
bevy-ui-builders = { version = "0.2", default-features = false, features = ["button", "dialog"] }
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
- `checkbox` - CheckboxBuilder
- `number_input` - NumberInputBuilder (depends on text_input)
- `dropdown` - DropdownBuilder
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
| 0.2             | 0.17 |

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