//! Component-specific style configurations.
//!
//! These allow fine-grained control over individual component appearances
//! beyond what the base theme provides.

/// Component-specific style overrides.
///
/// These settings allow customization of individual component types
/// beyond the base semantic colors.
#[derive(Clone, Debug, Default)]
pub struct ComponentStyles {
    /// Button-specific styles
    pub button: ButtonComponentStyle,
    /// Text input styles
    pub text_input: TextInputComponentStyle,
    /// Slider styles
    pub slider: SliderComponentStyle,
    /// Progress bar styles
    pub progress: ProgressComponentStyle,
    /// Panel styles
    pub panel: PanelComponentStyle,
    /// Dialog styles
    pub dialog: DialogComponentStyle,
    /// Checkbox styles
    pub checkbox: CheckboxComponentStyle,
    /// Dropdown styles
    pub dropdown: DropdownComponentStyle,
}

/// Button component styles
#[derive(Clone, Debug)]
pub struct ButtonComponentStyle {
    /// Button height for small size
    pub height_sm: f32,
    /// Button height for medium size
    pub height_md: f32,
    /// Button height for large size
    pub height_lg: f32,
    /// Button height for extra large size
    pub height_xl: f32,
    /// Default border radius
    pub border_radius: f32,
    /// Default border width
    pub border_width: f32,
    /// Default hover scale effect
    pub hover_scale: f32,
}

impl Default for ButtonComponentStyle {
    fn default() -> Self {
        Self {
            height_sm: 28.0,
            height_md: 36.0,
            height_lg: 44.0,
            height_xl: 56.0,
            border_radius: 6.0,
            border_width: 2.0,
            hover_scale: 1.0, // No default scale
        }
    }
}

/// Text input component styles
#[derive(Clone, Debug)]
pub struct TextInputComponentStyle {
    /// Default input height
    pub height: f32,
    /// Minimum width
    pub min_width: f32,
    /// Default width
    pub default_width: f32,
    /// Border radius
    pub border_radius: f32,
    /// Cursor blink rate (in seconds)
    pub cursor_blink_rate: f32,
    /// Cursor width
    pub cursor_width: f32,
}

impl Default for TextInputComponentStyle {
    fn default() -> Self {
        Self {
            height: 36.0,
            min_width: 200.0,
            default_width: 300.0,
            border_radius: 6.0,
            cursor_blink_rate: 0.53,
            cursor_width: 2.0,
        }
    }
}

/// Slider component styles
#[derive(Clone, Debug)]
pub struct SliderComponentStyle {
    /// Track height
    pub track_height: f32,
    /// Handle size
    pub handle_size: f32,
    /// Default width
    pub default_width: f32,
    /// Track border radius
    pub track_radius: f32,
}

impl Default for SliderComponentStyle {
    fn default() -> Self {
        Self {
            track_height: 6.0,
            handle_size: 20.0,
            default_width: 200.0,
            track_radius: 3.0,
        }
    }
}

/// Progress bar component styles
#[derive(Clone, Debug)]
pub struct ProgressComponentStyle {
    /// Default height
    pub height: f32,
    /// Default width
    pub default_width: f32,
    /// Border radius
    pub border_radius: f32,
}

impl Default for ProgressComponentStyle {
    fn default() -> Self {
        Self {
            height: 8.0,
            default_width: 200.0,
            border_radius: 4.0,
        }
    }
}

/// Panel component styles
#[derive(Clone, Debug)]
pub struct PanelComponentStyle {
    /// Default padding
    pub padding: f32,
    /// Border radius
    pub border_radius: f32,
    /// Border width
    pub border_width: f32,
    /// Card elevation shadow offset
    pub card_elevation: f32,
}

impl Default for PanelComponentStyle {
    fn default() -> Self {
        Self {
            padding: 16.0,
            border_radius: 8.0,
            border_width: 1.0,
            card_elevation: 4.0,
        }
    }
}

/// Dialog component styles
#[derive(Clone, Debug)]
pub struct DialogComponentStyle {
    /// Small dialog width
    pub width_sm: f32,
    /// Medium dialog width
    pub width_md: f32,
    /// Large dialog width
    pub width_lg: f32,
    /// Default padding
    pub padding: f32,
    /// Border radius
    pub border_radius: f32,
    /// Backdrop opacity (0.0 to 1.0)
    pub backdrop_opacity: f32,
}

impl Default for DialogComponentStyle {
    fn default() -> Self {
        Self {
            width_sm: 400.0,
            width_md: 600.0,
            width_lg: 800.0,
            padding: 24.0,
            border_radius: 12.0,
            backdrop_opacity: 0.6,
        }
    }
}

/// Checkbox component styles
#[derive(Clone, Debug)]
pub struct CheckboxComponentStyle {
    /// Checkbox size
    pub size: f32,
    /// Border radius
    pub border_radius: f32,
    /// Border width
    pub border_width: f32,
    /// Checkmark stroke width
    pub checkmark_width: f32,
}

impl Default for CheckboxComponentStyle {
    fn default() -> Self {
        Self {
            size: 20.0,
            border_radius: 4.0,
            border_width: 2.0,
            checkmark_width: 2.0,
        }
    }
}

/// Dropdown component styles
#[derive(Clone, Debug)]
pub struct DropdownComponentStyle {
    /// Default height
    pub height: f32,
    /// Default width
    pub default_width: f32,
    /// Menu max height
    pub menu_max_height: f32,
    /// Border radius
    pub border_radius: f32,
    /// Item padding
    pub item_padding: f32,
}

impl Default for DropdownComponentStyle {
    fn default() -> Self {
        Self {
            height: 36.0,
            default_width: 200.0,
            menu_max_height: 300.0,
            border_radius: 6.0,
            item_padding: 8.0,
        }
    }
}
