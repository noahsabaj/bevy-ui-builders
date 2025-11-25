//! Mouse selection utilities

use bevy::prelude::*;
use bevy::text::TextLayoutInfo;

/// Calculate character index from normalized mouse position
pub fn calculate_char_index_from_position(
    normalized_pos: Vec2,
    text_layout: &TextLayoutInfo,
    content: &str,
) -> usize {
    if text_layout.glyphs.is_empty() {
        // No glyphs available - position at end of text
        return content.chars().count();
    }

    // Find nearest character boundary
    let mut closest_index = 0;
    let mut closest_distance = f32::MAX;

    for (i, glyph) in text_layout.glyphs.iter().enumerate() {
        let glyph_x = glyph.position.x / text_layout.size.x;
        let distance = (normalized_pos.x - glyph_x).abs();

        if distance < closest_distance {
            closest_distance = distance;
            // Check if click is past the midpoint of the glyph
            if normalized_pos.x > glyph_x {
                // Position cursor after this character
                closest_index = i + 1;
            } else {
                // Position cursor before this character
                closest_index = i;
            }
        }
    }

    closest_index.min(content.chars().count())
}