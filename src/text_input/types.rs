//! Text input component types and markers

use bevy::prelude::*;

/// Defines input validation and filtering rules
#[derive(Component, Clone, Debug)]
pub struct TextInputFilter {
    pub filter_type: InputFilter,
    pub max_length: Option<usize>,
    pub transform: InputTransform,
}

/// Types of input filtering
#[derive(Clone, Debug, PartialEq)]
pub enum InputFilter {
    /// Allow any characters (default)
    None,
    /// Only allow numeric characters (0-9)
    Numeric,
    /// Only allow integers (0-9, optional negative sign)
    Integer,
    /// Only allow decimal numbers (0-9, '.', optional negative)
    Decimal,
    /// Only allow alphabetic characters (a-z, A-Z)
    Alphabetic,
    /// Only allow alphanumeric characters (a-z, A-Z, 0-9)
    Alphanumeric,
    /// Only allow hexadecimal characters (0-9, a-f, A-F)
    Hexadecimal,
    /// Custom regex pattern
    Regex(String),
    /// Custom validation function
    Custom(fn(&str) -> bool),
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