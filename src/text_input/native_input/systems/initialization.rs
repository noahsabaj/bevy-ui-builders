//! Text input initialization systems

use bevy::prelude::*;
use bevy::ui::{FocusPolicy, RelativeCursorPosition};

use super::super::components::*;
use super::super::types::CursorStyle;

/// Initialize text input when spawned
pub fn init_text_input(
    trigger: Trigger<OnAdd, NativeTextInput>,
    mut commands: Commands,
    text_buffer_query: Query<(&TextBuffer, &TextInputVisual)>,
) {
    let entity = trigger.target();

    // Read initial values if they exist (set by builder)
    let (initial_content, initial_font_size, initial_color) =
        if let Ok((buffer, visual)) = text_buffer_query.get(entity) {
            (
                buffer.content.clone(),
                visual.font.font_size,
                visual.text_color,
            )
        } else {
            (String::new(), 14.0, Color::WHITE)
        };

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
            // Pre-cursor text (initialize with content from TextBuffer if available)
            text_parent.spawn((
                TextSpan::new(initial_content),
                TextFont {
                    font_size: initial_font_size,
                    ..default()
                },
                TextColor(initial_color),
                Name::new("PreCursor"),
            ));

            // Cursor character (will be "|" when visible, "" when hidden)
            text_parent.spawn((
                TextSpan::new(""),
                TextFont {
                    font_size: initial_font_size,
                    ..default()
                },
                TextColor(Color::WHITE),
                Name::new("Cursor"),
            ));

            // Post-cursor text
            text_parent.spawn((
                TextSpan::new(""),
                TextFont {
                    font_size: initial_font_size,
                    ..default()
                },
                TextColor(initial_color),
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