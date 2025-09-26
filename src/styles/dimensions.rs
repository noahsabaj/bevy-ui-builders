//! Dimension constants for consistent spacing and sizing

use bevy::prelude::Val;

// Button dimensions
pub const BUTTON_WIDTH_SMALL: f32 = 80.0;
pub const BUTTON_WIDTH_MEDIUM: f32 = 120.0;
pub const BUTTON_WIDTH_LARGE: f32 = 160.0;
pub const BUTTON_WIDTH_XLARGE: f32 = 200.0;

pub const BUTTON_HEIGHT_SMALL: f32 = 28.0;
pub const BUTTON_HEIGHT_MEDIUM: f32 = 36.0;
pub const BUTTON_HEIGHT_LARGE: f32 = 44.0;
pub const BUTTON_HEIGHT_XLARGE: f32 = 56.0;

// Font sizes
pub const FONT_SIZE_SMALL: f32 = 12.0;
pub const FONT_SIZE_MEDIUM: f32 = 14.0;
pub const FONT_SIZE_NORMAL: f32 = 14.0;  // Alias for MEDIUM
pub const FONT_SIZE_LARGE: f32 = 16.0;
pub const FONT_SIZE_XLARGE: f32 = 20.0;
pub const FONT_SIZE_HEADING: f32 = 24.0;
pub const FONT_SIZE_TITLE: f32 = 32.0;

// Spacing
pub const SPACING_TINY: f32 = 4.0;
pub const SPACING_SMALL: f32 = 8.0;
pub const SPACING_MEDIUM: f32 = 16.0;
pub const SPACING_LARGE: f32 = 24.0;
pub const SPACING_XLARGE: f32 = 32.0;

// Padding
pub const PADDING_SMALL: f32 = 8.0;
pub const PADDING_MEDIUM: f32 = 12.0;
pub const PADDING_LARGE: f32 = 16.0;
pub const PANEL_PADDING: f32 = 16.0;

// Margins
pub const MARGIN_SMALL: f32 = 8.0;
pub const MARGIN_MEDIUM: f32 = 16.0;
pub const MARGIN_LARGE: f32 = 24.0;
pub const SEPARATOR_MARGIN: f32 = 12.0;

// Border widths
pub const BORDER_WIDTH_THIN: f32 = 1.0;
pub const BORDER_WIDTH_MEDIUM: f32 = 2.0;
pub const BORDER_WIDTH_THICK: f32 = 3.0;
pub const BORDER_WIDTH: f32 = 2.0;  // Default border width

// Border radius
pub const BORDER_RADIUS_SMALL: f32 = 4.0;
pub const BORDER_RADIUS_MEDIUM: f32 = 6.0;
pub const BORDER_RADIUS_LARGE: f32 = 8.0;
pub const BORDER_RADIUS_ROUND: f32 = 999.0;

// Dialog dimensions
pub const DIALOG_WIDTH_SMALL: f32 = 400.0;
pub const DIALOG_WIDTH_MEDIUM: f32 = 600.0;
pub const DIALOG_WIDTH_LARGE: f32 = 800.0;

// Input dimensions
pub const INPUT_HEIGHT: f32 = 36.0;
pub const INPUT_WIDTH_MIN: f32 = 200.0;
pub const INPUT_WIDTH_DEFAULT: f32 = 300.0;

// Slider dimensions
pub const SLIDER_TRACK_HEIGHT: f32 = 6.0;
pub const SLIDER_HANDLE_SIZE: f32 = 20.0;
pub const SLIDER_WIDTH_DEFAULT: f32 = 200.0;

// Progress bar dimensions
pub const PROGRESS_HEIGHT: f32 = 8.0;
pub const PROGRESS_WIDTH_DEFAULT: f32 = 200.0;

// Z-index layers
pub const Z_INDEX_BASE: i32 = 0;
pub const Z_INDEX_DROPDOWN: i32 = 100;
pub const Z_INDEX_MODAL: i32 = 1000;
pub const Z_INDEX_TOOLTIP: i32 = 2000;
pub const Z_INDEX_NOTIFICATION: i32 = 3000;

// Animation durations (in seconds)
pub const ANIMATION_FAST: f32 = 0.15;
pub const ANIMATION_NORMAL: f32 = 0.3;
pub const ANIMATION_SLOW: f32 = 0.5;

// Dynamic responsive dimensions using viewport units
// These adapt to the window size automatically

// Responsive spacing using viewport height
pub const SPACING_SMALL_VH: Val = Val::Vh(1.0);   // 1% of viewport height
pub const SPACING_MEDIUM_VH: Val = Val::Vh(2.0);  // 2% of viewport height
pub const SPACING_LARGE_VH: Val = Val::Vh(3.0);   // 3% of viewport height
pub const SPACING_XLARGE_VH: Val = Val::Vh(5.0);  // 5% of viewport height

// Responsive spacing using viewport width
pub const SPACING_SMALL_VW: Val = Val::Vw(1.0);   // 1% of viewport width
pub const SPACING_MEDIUM_VW: Val = Val::Vw(2.0);  // 2% of viewport width
pub const SPACING_LARGE_VW: Val = Val::Vw(3.0);   // 3% of viewport width
pub const SPACING_XLARGE_VW: Val = Val::Vw(5.0);  // 5% of viewport width

// Responsive padding using viewport units
pub const PADDING_SMALL_VH: Val = Val::Vh(1.0);
pub const PADDING_MEDIUM_VH: Val = Val::Vh(2.0);
pub const PADDING_LARGE_VH: Val = Val::Vh(3.0);
pub const PADDING_SMALL_VW: Val = Val::Vw(1.0);
pub const PADDING_MEDIUM_VW: Val = Val::Vw(2.0);
pub const PADDING_LARGE_VW: Val = Val::Vw(3.0);

// Flexible content widths using percentages
pub const CONTENT_WIDTH_SMALL: Val = Val::Percent(30.0);
pub const CONTENT_WIDTH_MEDIUM: Val = Val::Percent(50.0);
pub const CONTENT_WIDTH_LARGE: Val = Val::Percent(80.0);
pub const CONTENT_WIDTH_FULL: Val = Val::Percent(100.0);

// Flexible content heights
pub const CONTENT_HEIGHT_AUTO: Val = Val::Auto;
pub const CONTENT_HEIGHT_HALF: Val = Val::Percent(50.0);
pub const CONTENT_HEIGHT_FULL: Val = Val::Percent(100.0);

// Maximum dimensions for containers (viewport relative)
pub const MAX_CONTENT_HEIGHT: Val = Val::Vh(90.0);  // 90% viewport height
pub const MAX_CONTENT_WIDTH: Val = Val::Vw(95.0);   // 95% viewport width
pub const MAX_DIALOG_WIDTH: Val = Val::Vw(80.0);    // 80% viewport width
pub const MAX_DIALOG_HEIGHT: Val = Val::Vh(80.0);   // 80% viewport height

// Minimum dimensions to prevent collapse
pub const MIN_INPUT_WIDTH: Val = Val::Px(150.0);
pub const MIN_BUTTON_WIDTH: Val = Val::Px(80.0);
pub const MIN_CONTENT_HEIGHT: Val = Val::Px(100.0);