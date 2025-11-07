//! Text input initialization systems

use bevy::prelude::*;
use bevy::ui::{FocusPolicy, RelativeCursorPosition};

use super::super::components::*;
use super::super::types::CursorStyle;

/// Initialize text input when spawned (observer for initial setup)
pub fn init_text_input(
    trigger: On<Add, NativeTextInput>,
    mut commands: Commands,
) {
    let entity = trigger.entity;

    // Add default components if not present
    // Note: TextBuffer and TextInputVisual are set by builder, don't override
    commands.entity(entity).try_insert((
        SelectionState::default(),
        ScrollViewport::default(),
        UndoHistory::default(),
        TextInputSettings::default(),
        Interaction::default(),
        FocusPolicy::Block,
        RelativeCursorPosition::default(),
    ));

    // Spawn text container with 3-span structure for embedded cursor
    // Initial content will be set by sync_initial_text_content system
    commands.entity(entity).with_children(|parent| {
        parent.spawn((
            Text::default(),
            TextInputInner,
            Name::new("TextInputInner"),
        ))
        .with_children(|text_parent| {
            // Pre-cursor text (will be populated by sync system)
            text_parent.spawn((
                TextSpan::new(""),
                TextFont::default(),
                TextColor(Color::WHITE),
                Name::new("PreCursor"),
            ));

            // Cursor character
            text_parent.spawn((
                TextSpan::new(""),
                TextFont::default(),
                TextColor(Color::WHITE),
                Name::new("Cursor"),
            ));

            // Post-cursor text
            text_parent.spawn((
                TextSpan::new(""),
                TextFont::default(),
                TextColor(Color::WHITE),
                Name::new("PostCursor"),
            ));
        });
    });

    // Add CursorVisual component
    commands.entity(entity).insert(CursorVisual {
        cursor_entity: None,
        visible: false,
        blink_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        style: CursorStyle::Line,
        selection_entities: Vec::new(),
    });
}

/// System that runs once to sync initial TextBuffer content to visual TextSpans
/// This runs after all components are guaranteed to be present
pub fn sync_initial_text_content(
    text_inputs: Query<
        (&TextBuffer, &TextInputVisual, &Children),
        (Added<CursorVisual>, With<NativeTextInput>)
    >,
    text_inner_query: Query<&Children, With<TextInputInner>>,
    mut text_span_query: Query<(&mut TextSpan, &mut TextFont, &mut TextColor)>,
) {
    for (buffer, visual, children) in &text_inputs {
        // Find TextInputInner entity
        for child in children.iter() {
            if let Ok(text_spans) = text_inner_query.get(child) {
                let text_spans_vec: Vec<Entity> = text_spans.iter().collect();

                if text_spans_vec.len() >= 3 {
                    // Initialize pre-cursor span with initial content
                    if let Ok((mut span, mut font, mut color)) = text_span_query.get_mut(text_spans_vec[0]) {
                        *span = TextSpan::new(buffer.content.clone());
                        *font = visual.font.clone();
                        color.0 = if buffer.content.is_empty() {
                            visual.placeholder_color
                        } else {
                            visual.text_color
                        };
                    }

                    // Initialize cursor span with correct font
                    if let Ok((_, mut font, _)) = text_span_query.get_mut(text_spans_vec[1]) {
                        *font = visual.font.clone();
                    }

                    // Initialize post-cursor span with correct font/color
                    if let Ok((_, mut font, mut color)) = text_span_query.get_mut(text_spans_vec[2]) {
                        *font = visual.font.clone();
                        color.0 = visual.text_color;
                    }
                }
                break;
            }
        }
    }
}