//! Dialogue systems

use bevy::prelude::*;
use super::types::*;

/// System to handle typing effect
pub fn update_typing_effect(
    time: Res<Time>,
    mut dialogue_query: Query<&mut DialogueBox>,
    mut text_query: Query<(&DialogueText, &mut Text)>,
    mut complete_events: MessageWriter<DialogueTypingCompleteEvent>,
) {
    for (dialogue_text, mut text) in text_query.iter_mut() {
        let Ok(mut dialogue) = dialogue_query.get_mut(dialogue_text.dialogue) else {
            continue;
        };

        if !dialogue.is_typing {
            continue;
        }

        // Calculate how many characters should be revealed
        let chars_per_frame = dialogue.typing_speed * time.delta_secs();
        dialogue.chars_revealed = (dialogue.chars_revealed as f32 + chars_per_frame) as usize;

        // Update the text
        let full_text = &dialogue_text.full_text;
        if dialogue.chars_revealed >= full_text.len() {
            dialogue.chars_revealed = full_text.len();
            dialogue.is_typing = false;
            *text = Text::new(full_text.clone());

            complete_events.write(DialogueTypingCompleteEvent {
                dialogue: dialogue_text.dialogue,
            });
        } else {
            // Get character-safe slice
            let revealed: String = full_text.chars().take(dialogue.chars_revealed).collect();
            *text = Text::new(revealed);
        }
    }
}

/// System to handle clicking to skip typing
pub fn handle_skip_typing(
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut dialogue_query: Query<&mut DialogueBox>,
    mut text_query: Query<(&DialogueText, &mut Text)>,
) {
    // Skip on click or space/enter
    let should_skip = mouse.just_pressed(MouseButton::Left)
        || keyboard.just_pressed(KeyCode::Space)
        || keyboard.just_pressed(KeyCode::Enter);

    if !should_skip {
        return;
    }

    for (dialogue_text, mut text) in text_query.iter_mut() {
        let Ok(mut dialogue) = dialogue_query.get_mut(dialogue_text.dialogue) else {
            continue;
        };

        if dialogue.is_typing {
            // Skip to end
            dialogue.is_typing = false;
            dialogue.chars_revealed = dialogue_text.full_text.len();
            *text = Text::new(dialogue_text.full_text.clone());
        }
    }
}

/// System to handle choice hover effects
pub fn handle_choice_hover(
    mut choice_query: Query<(&DialogueChoice, &Interaction, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (choice, interaction, mut bg_color) in choice_query.iter_mut() {
        *bg_color = match interaction {
            Interaction::Hovered => BackgroundColor(Color::srgba(0.2, 0.2, 0.3, 0.9)),
            Interaction::Pressed => BackgroundColor(Color::srgba(0.15, 0.15, 0.25, 0.9)),
            Interaction::None => BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.8)),
        };
    }
}

/// System to handle choice clicks
pub fn handle_choice_clicks(
    choice_query: Query<(&DialogueChoice, &Interaction), Changed<Interaction>>,
    mut choice_events: MessageWriter<DialogueChoiceEvent>,
) {
    for (choice, interaction) in choice_query.iter() {
        if *interaction == Interaction::Pressed {
            choice_events.write(DialogueChoiceEvent {
                dialogue: choice.dialogue,
                index: choice.index,
                choice_id: choice.choice_id.clone(),
            });
        }
    }
}
