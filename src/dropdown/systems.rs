//! Systems for dropdown interaction

use bevy::prelude::*;
use super::types::*;
use crate::relationships::BelongsToDropdown;

/// Handle dropdown button clicks to toggle menu
pub fn handle_dropdown_button_clicks(
    mut dropdowns: Query<(&mut DropdownState, &Children), With<Dropdown>>,
    buttons: Query<(Entity, &Interaction), (With<DropdownButton>, Changed<Interaction>)>,
    mut menus: Query<&mut Node, With<DropdownMenu>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    parents: Query<&ChildOf>,
) {
    for (button_entity, interaction) in buttons.iter() {
        if *interaction == Interaction::Pressed && mouse_button.just_pressed(MouseButton::Left) {
            // Find the dropdown this button belongs to
            if let Ok(child_of) = parents.get(button_entity) {
                if let Ok((mut state, children)) = dropdowns.get_mut(child_of.parent()) {
                    // Toggle state
                    *state = match *state {
                        DropdownState::Closed => DropdownState::Open,
                        DropdownState::Open => DropdownState::Closed,
                    };

                    // Update menu visibility
                    for child in children.iter() {
                        if let Ok(mut menu_node) = menus.get_mut(child) {
                            menu_node.display = match *state {
                                DropdownState::Open => Display::Flex,
                                DropdownState::Closed => Display::None,
                            };
                        }
                    }
                }
            }
        }
    }
}

/// Handle dropdown option selection
pub fn handle_dropdown_option_clicks(
    mut dropdowns: Query<(&mut DropdownData, &mut DropdownState, &Children), With<Dropdown>>,
    options: Query<(&DropdownOption, &Interaction, &ChildOf), Changed<Interaction>>,
    buttons: Query<&Children, With<DropdownButton>>,
    mut texts: Query<&mut Text>,
    mut menus: Query<&mut Node, With<DropdownMenu>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    parents: Query<&ChildOf>,
) {
    for (option, interaction, option_child_of) in options.iter() {
        if *interaction == Interaction::Pressed && mouse_button.just_pressed(MouseButton::Left) {
            // Find the dropdown this option belongs to
            // Option -> Menu -> Dropdown
            if let Ok(menu_child_of) = parents.get(option_child_of.parent()) {
                if let Ok((mut data, mut state, dropdown_children)) = dropdowns.get_mut(menu_child_of.parent()) {
                    // Update selected index
                    data.selected_index = Some(option.index);
                    let new_text = data.display_text().to_string();

                    // Find the button and update its text (first Text child)
                    for child in dropdown_children.iter() {
                        if let Ok(button_children) = buttons.get(child) {
                            // First child of button is the selected value text
                            if let Some(first_text_entity) = button_children.iter().next() {
                                if let Ok(mut text) = texts.get_mut(first_text_entity) {
                                    **text = new_text;
                                }
                            }
                            break;
                        }
                    }

                    // Close the menu
                    *state = DropdownState::Closed;
                    for child in dropdown_children.iter() {
                        if let Ok(mut menu_node) = menus.get_mut(child) {
                            menu_node.display = Display::None;
                        }
                    }
                }
            }
        }
    }
}

/// Update all option backgrounds when dropdown selection changes
pub fn update_dropdown_selection_highlights(
    changed_dropdowns: Query<(Entity, &DropdownData), Changed<DropdownData>>,
    mut options: Query<(&DropdownOption, &mut BackgroundColor, &Interaction, &ChildOf), With<DropdownOption>>,
    parents: Query<&ChildOf>,
) {
    for (dropdown_entity, data) in changed_dropdowns.iter() {
        // Update all options belonging to this dropdown
        for (option, mut bg_color, interaction, option_child_of) in options.iter_mut() {
            // Check if this option belongs to the changed dropdown
            if let Ok(menu_child_of) = parents.get(option_child_of.parent()) {
                if menu_child_of.parent() == dropdown_entity {
                    let is_selected = Some(option.index) == data.selected_index;

                    // Update background based on interaction state and selection
                    *bg_color = match interaction {
                        Interaction::Hovered => BackgroundColor(Color::srgba(0.4, 0.6, 0.9, 0.4)),
                        Interaction::Pressed => BackgroundColor(Color::srgba(0.3, 0.5, 0.8, 0.5)),
                        Interaction::None => {
                            if is_selected {
                                BackgroundColor(Color::srgba(0.3, 0.5, 0.8, 0.3))
                            } else {
                                BackgroundColor(Color::NONE)
                            }
                        }
                    };
                }
            }
        }
    }
}

/// Close dropdown when clicking outside
pub fn close_dropdown_on_outside_click(
    mut dropdowns: Query<(Entity, &mut DropdownState, &Children), With<Dropdown>>,
    pressed_entities: Query<(Entity, &Interaction), Changed<Interaction>>,
    buttons: Query<&ChildOf, With<DropdownButton>>,
    dropdown_parts: Query<&BelongsToDropdown>,
    mut menus: Query<&mut Node, With<DropdownMenu>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    // Only process on left click
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    // Find entities that were just clicked (Interaction::Pressed)
    let clicked_entities: Vec<Entity> = pressed_entities
        .iter()
        .filter_map(|(entity, interaction)| {
            if *interaction == Interaction::Pressed {
                Some(entity)
            } else {
                None
            }
        })
        .collect();

    // If nothing was clicked, don't close
    if clicked_entities.is_empty() {
        return;
    }

    // Check each open dropdown
    for (dropdown_entity, mut state, children) in dropdowns.iter_mut() {
        // Only process if dropdown is open
        if *state != DropdownState::Open {
            continue;
        }

        // Check if any clicked entity belongs to this dropdown
        let mut clicked_inside = false;

        for &clicked_entity in &clicked_entities {
            // Check if clicked entity is the dropdown button
            if let Ok(button_child_of) = buttons.get(clicked_entity) {
                if button_child_of.parent() == dropdown_entity {
                    clicked_inside = true;
                    break;
                }
            }

            // Check if clicked entity belongs to this dropdown (menu or option)
            if let Ok(belongs_to) = dropdown_parts.get(clicked_entity) {
                if belongs_to.0 == dropdown_entity {
                    clicked_inside = true;
                    break;
                }
            }
        }

        // If clicked outside, close the dropdown
        if !clicked_inside {
            *state = DropdownState::Closed;
            for child in children.iter() {
                if let Ok(mut menu_node) = menus.get_mut(child) {
                    menu_node.display = Display::None;
                }
            }
        }
    }
}

/// Update dropdown option hover effects
pub fn update_dropdown_option_hover(
    mut options: Query<(&Interaction, &DropdownOption, &mut BackgroundColor, &ChildOf), (With<DropdownOption>, Changed<Interaction>)>,
    dropdowns: Query<&DropdownData>,
    parents: Query<&ChildOf>,
) {
    for (interaction, option, mut bg_color, option_child_of) in options.iter_mut() {
        // Find the dropdown to check if this option is selected
        if let Ok(menu_child_of) = parents.get(option_child_of.parent()) {
            if let Ok(data) = dropdowns.get(menu_child_of.parent()) {
                let is_selected = Some(option.index) == data.selected_index;

                *bg_color = match interaction {
                    Interaction::Hovered => BackgroundColor(Color::srgba(0.4, 0.6, 0.9, 0.4)),
                    Interaction::Pressed => BackgroundColor(Color::srgba(0.3, 0.5, 0.8, 0.5)),
                    Interaction::None => {
                        if is_selected {
                            BackgroundColor(Color::srgba(0.3, 0.5, 0.8, 0.3))
                        } else {
                            BackgroundColor(Color::NONE)
                        }
                    }
                };
            }
        }
    }
}
