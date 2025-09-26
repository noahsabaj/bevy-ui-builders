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

    // Spawn text and cursor as direct children
    let mut cursor_entity_holder = None;

    commands.entity(entity).with_children(|parent| {
        // Spawn text entity with proper structure
        parent.spawn((
            Text::default(),  // Parent Text component
            TextInputInner,
            Name::new("TextInputInner"),
        ))
        .with_children(|text_parent| {
            // The actual text content goes in a TextSpan child
            text_parent.spawn((
                TextSpan::new(""),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });

        // Spawn cursor entity
        let cursor_entity = parent.spawn((
            Node {
                width: Val::Px(2.0),
                height: Val::Px(20.0),  // Fixed height for now
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),  // Start with padding offset
                top: Val::Px(8.0),   // Center vertically (assuming ~36px input height)
                ..default()
            },
            BackgroundColor(Color::WHITE),
            Visibility::Hidden,  // Start hidden until focused
            TextInputCursor {
                input_entity: entity,
            },
            Name::new("TextInputCursor"),
        )).id();

        cursor_entity_holder = Some(cursor_entity);
    });

    // Add the CursorVisual component with the cursor entity reference
    if let Some(cursor_entity) = cursor_entity_holder {
        commands.entity(entity).insert(CursorVisual {
            cursor_entity: Some(cursor_entity),
            visible: false,  // Start with cursor not visible until focused
            blink_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            style: CursorStyle::Line,
            selection_entities: Vec::new(),
        });
    } else {
        // Fallback if cursor entity wasn't created for some reason
        commands.entity(entity).insert(CursorVisual::default());
    }
}