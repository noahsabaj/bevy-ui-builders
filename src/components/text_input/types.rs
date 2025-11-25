//! Text input component types and markers

use bevy::prelude::*;
use bevy::color::Alpha;

use crate::theme::UiTheme;

/// Defines input validation and filtering rules
#[derive(Component, Clone, Debug)]
pub struct TextInputFilter {
    /// The type of filter to apply
    pub filter_type: InputFilter,
    /// Optional maximum length of the input
    pub max_length: Option<usize>,
    /// Optional transformation to apply to the input
    pub transform: InputTransform,
}

/// Types of input filtering
#[derive(Clone, Debug)]
pub enum InputFilter {
    /// No filter
    None,
    /// Only allow numeric characters (0-9)
    Numeric,
    /// Allow integer numbers (0-9, -)
    Integer,
    /// Allow decimal numbers (0-9, -, .)
    Decimal,
    /// Allow alphabetic characters (a-z, A-Z)
    Alphabetic,
    /// Allow alphanumeric characters (a-z, A-Z, 0-9)
    Alphanumeric,
    /// Only allow hexadecimal characters (0-9, a-f, A-F)
    Hexadecimal,
    /// Custom regex pattern
    Regex(String),
    /// Custom filter function
    Custom(fn(&str) -> bool),
}

impl PartialEq for InputFilter {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::None, Self::None) => true,
            (Self::Numeric, Self::Numeric) => true,
            (Self::Integer, Self::Integer) => true,
            (Self::Decimal, Self::Decimal) => true,
            (Self::Alphabetic, Self::Alphabetic) => true,
            (Self::Alphanumeric, Self::Alphanumeric) => true,
            (Self::Hexadecimal, Self::Hexadecimal) => true,
            (Self::Regex(s1), Self::Regex(s2)) => s1 == s2,
            (Self::Custom(f1), Self::Custom(f2)) => {
                // Function pointers can't be reliably compared, so we assume they are different
                // unless they are literally the same pointer address (which is what standard PartialEq does but warns about)
                // To suppress the warning and provide safe behavior, we'll just return false for different functions
                // or true if they are the same address, but cast to usize to avoid the warning.
                (*f1 as usize) == (*f2 as usize)
            },
            _ => false,
        }
    }
}

/// Text transformation options
#[derive(Clone, Debug, PartialEq)]
pub enum InputTransform {
    /// No transformation
    None,
    /// Convert to uppercase
    Uppercase,
    /// Convert to lowercase
    Lowercase,
    /// Capitalize first letter of each word
    Capitalize,
}

impl Default for TextInputFilter {
    fn default() -> Self {
        Self {
            filter_type: InputFilter::None,
            max_length: None,
            transform: InputTransform::None,
        }
    }
}

impl InputFilter {
    /// Check if a character is valid for this filter
    pub fn is_valid_char(&self, ch: char, current_text: &str) -> bool {
        match self {
            InputFilter::None => true,
            InputFilter::Numeric => ch.is_ascii_digit(),
            InputFilter::Integer => ch.is_ascii_digit() || (ch == '-' && current_text.is_empty()),
            InputFilter::Decimal => {
                ch.is_ascii_digit()
                    || (ch == '.' && !current_text.contains('.'))
                    || (ch == '-' && current_text.is_empty())
            }
            InputFilter::Alphabetic => ch.is_alphabetic(),
            InputFilter::Alphanumeric => ch.is_alphanumeric(),
            InputFilter::Hexadecimal => ch.is_ascii_hexdigit(),
            InputFilter::Regex(_pattern) => {
                // For regex, we'd need to check the entire string
                // This is a simplified check
                true // Will be validated in the full string check
            }
            InputFilter::Custom(validator) => {
                // Test if adding this character would be valid
                let mut test_string = current_text.to_string();
                test_string.push(ch);
                validator(&test_string)
            }
        }
    }

    /// Validate an entire string
    pub fn is_valid_string(&self, text: &str) -> bool {
        match self {
            InputFilter::None => true,
            InputFilter::Numeric => text.chars().all(|c| c.is_ascii_digit()),
            InputFilter::Integer => {
                if text.is_empty() {
                    return true;
                }
                let mut chars = text.chars();
                if let Some(first) = chars.next() {
                    if first != '-' && !first.is_ascii_digit() {
                        return false;
                    }
                }
                chars.all(|c| c.is_ascii_digit())
            }
            InputFilter::Decimal => {
                if text.is_empty() {
                    return true;
                }
                let mut has_decimal = false;
                let chars = text.chars().enumerate();

                for (i, ch) in chars {
                    if ch == '-' && i != 0 {
                        return false;
                    } else if ch == '.' {
                        if has_decimal {
                            return false;
                        }
                        has_decimal = true;
                    } else if !ch.is_ascii_digit() && ch != '-' {
                        return false;
                    }
                }
                true
            }
            InputFilter::Alphabetic => text.chars().all(|c| c.is_alphabetic()),
            InputFilter::Alphanumeric => text.chars().all(|c| c.is_alphanumeric()),
            InputFilter::Hexadecimal => text.chars().all(|c| c.is_ascii_hexdigit()),
            InputFilter::Regex(_pattern) => {
                // Would need regex crate for full support
                true // Simplified for now
            }
            InputFilter::Custom(validator) => validator(text),
        }
    }

    /// Filter out invalid characters from a string
    pub fn filter_string(&self, text: &str) -> String {
        let mut result = String::new();
        for ch in text.chars() {
            if self.is_valid_char(ch, &result) {
                result.push(ch);
            }
        }
        result
    }
}

/// Component that marks a clear button and tracks which text input it clears
#[derive(Component)]
pub struct ClearButtonTarget(pub Entity);

/// Defines how a text input participates in focus management
#[derive(Component, Clone, Debug)]
pub enum TextInputFocus {
    /// This input doesn't affect other inputs when focused
    Independent,
    /// This input is part of an exclusive group - only one in the group can be focused
    ExclusiveGroup(FocusGroupId),
}

/// Identifies groups of text inputs where only one can be focused at a time
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FocusGroupId {
    /// World configuration screen (Name and Seed inputs)
    WorldConfig,
    /// Save game dialog
    SaveDialog,
    /// Mod browser search
    ModBrowser,
    /// Custom group for extensions
    Custom(u32),
}

// Default colors (dark theme) for when no theme is provided
pub(crate) mod defaults {
    use bevy::prelude::Color;

    pub const BACKGROUND: Color = Color::srgb(0.15, 0.15, 0.18);
    pub const BACKGROUND_HOVER: Color = Color::srgb(0.18, 0.18, 0.22);
    pub const BORDER: Color = Color::srgb(0.3, 0.3, 0.3);
    pub const BORDER_FOCUS: Color = Color::srgb(0.25, 0.46, 0.86);
    pub const TEXT_PRIMARY: Color = Color::srgb(0.95, 0.95, 0.95);
    pub const TEXT_MUTED: Color = Color::srgb(0.5, 0.5, 0.5);
    pub const PRIMARY: Color = Color::srgb(0.25, 0.46, 0.86);
}

/// Resolved text input colors from theme
#[derive(Clone)]
pub struct TextInputColors {
    /// Background color
    pub background: Color,
    /// Background color on hover
    pub background_hover: Color,
    /// Border color
    pub border: Color,
    /// Border color when focused
    pub border_focus: Color,
    /// Primary text color
    pub text: Color,
    /// Placeholder text color
    pub placeholder: Color,
    /// Selection highlight color
    pub selection: Color,
}

impl TextInputColors {
    /// Resolve colors from theme
    pub fn from_theme(theme: &UiTheme) -> Self {
        Self {
            background: theme.colors.surface.tertiary,
            background_hover: theme.colors.surface.secondary,
            border: theme.colors.border.default,
            border_focus: theme.colors.border.focus,
            text: theme.colors.text.primary,
            placeholder: theme.colors.text.muted,
            selection: theme.colors.primary.base.with_alpha(0.3),
        }
    }

    /// Default colors (no theme)
    pub fn default_colors() -> Self {
        Self {
            background: defaults::BACKGROUND,
            background_hover: defaults::BACKGROUND_HOVER,
            border: defaults::BORDER,
            border_focus: defaults::BORDER_FOCUS,
            text: defaults::TEXT_PRIMARY,
            placeholder: defaults::TEXT_MUTED,
            selection: defaults::PRIMARY.with_alpha(0.3),
        }
    }
}