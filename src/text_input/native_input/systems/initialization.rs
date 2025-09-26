//! Text input initialization systems

use bevy::prelude::*;
use bevy::ui::{FocusPolicy, RelativeCursorPosition};

use super::super::components::*;
use super::super::types::CursorStyle;

/// Initialize text input when spawned
pub fn init_text_input(
    trigger: Trigger<OnAdd, NativeTextInput>,
    mut commands: Commands,
) {
    let entity = trigger.target();

    // Add default components if not present (except CursorVisual which needs cursor entity)
    commands.entity(entity).try_insert((
        TextBuffer::default(),
        SelectionState::default(),
        TextInputVisual::default(),
        // CursorVisual will be added after cursor entity is created
        ScrollViewport::default(),
        UndoHistory::default(),
        TextInputSettings::default(),
        Interaction::default(),
        FocusPolicy::Block,
        RelativeCursorPosition::default(),
    ));

    // Spawn text with 3-span structure for embedded cursor
    commands.entity(entity).with_children(|parent| {
        // Spawn text entity with proper structure
        parent.spawn((
            Text::default(),  // Parent Text component
            TextInputInner,
            Name::new("TextInputInner"),
        ))
        .with_children(|text_parent| {
            // Pre-cursor text
            text_parent.spawn((
                TextSpan::new(""),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Name::new("PreCursor"),
            ));

            // Cursor character (will be "|" when visible, "" when hidden)
            text_parent.spawn((
                TextSpan::new(""),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Name::new("Cursor"),
            ));

            // Post-cursor text
            text_parent.spawn((
                TextSpan::new(""),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Name::new("PostCursor"),
            ));
        });
    });

    // Add simplified CursorVisual component (no entity reference needed)
    commands.entity(entity).insert(CursorVisual {
        cursor_entity: None,  // No separate cursor entity anymore
        visible: false,  // Start with cursor not visible until focused
        blink_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        style: CursorStyle::Line,  // Keep for future use
        selection_entities: Vec::new(),
    });
}