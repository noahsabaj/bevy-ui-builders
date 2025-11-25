//! Tab view systems

use bevy::prelude::*;
use crate::theme::UiTheme;
use super::types::*;

/// Default colors for tabs (dark theme fallback)
mod defaults {
    use bevy::prelude::Color;
    pub const PRIMARY: Color = Color::srgb(0.25, 0.46, 0.86);
    pub const TEXT_ON_PRIMARY: Color = Color::WHITE;
    pub const TEXT_PRIMARY: Color = Color::srgb(0.95, 0.95, 0.95);
    pub const GHOST_HOVER: Color = Color::srgba(1.0, 1.0, 1.0, 0.05);
    pub const GHOST_PRESSED: Color = Color::srgba(1.0, 1.0, 1.0, 0.1);
}

/// Resolved tab colors from theme
struct ResolvedTabColors {
    primary: Color,
    text_on_primary: Color,
    text_primary: Color,
    ghost_hover: Color,
    ghost_pressed: Color,
}

impl ResolvedTabColors {
    fn from_theme(theme: &UiTheme) -> Self {
        Self {
            primary: theme.colors.primary.base,
            text_on_primary: theme.colors.primary.on_color,
            text_primary: theme.colors.text.primary,
            ghost_hover: theme.colors.ghost.hover,
            ghost_pressed: theme.colors.ghost.pressed,
        }
    }

    fn default_colors() -> Self {
        Self {
            primary: defaults::PRIMARY,
            text_on_primary: defaults::TEXT_ON_PRIMARY,
            text_primary: defaults::TEXT_PRIMARY,
            ghost_hover: defaults::GHOST_HOVER,
            ghost_pressed: defaults::GHOST_PRESSED,
        }
    }
}

/// System to handle tab button clicks
pub fn handle_tab_clicks(
    mut tab_view_query: Query<&mut TabView>,
    tab_button_query: Query<(&TabButton, &Interaction), Changed<Interaction>>,
    mut events: MessageWriter<TabSelectedEvent>,
) {
    for (tab_button, interaction) in tab_button_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if let Ok(mut tab_view) = tab_view_query.get_mut(tab_button.tab_view) {
            let previous_index = tab_view.active_tab;

            if previous_index != tab_button.index {
                tab_view.active_tab = tab_button.index;

                events.write(TabSelectedEvent {
                    tab_view: tab_button.tab_view,
                    index: tab_button.index,
                    previous_index,
                });
            }
        }
    }
}

/// System to update tab button visuals when active tab changes
pub fn update_tab_button_visuals(
    tab_view_query: Query<(Entity, &TabView), Changed<TabView>>,
    mut tab_button_query: Query<(&TabButton, &mut BackgroundColor, &Children)>,
    mut text_query: Query<&mut TextColor>,
    theme: Option<Res<UiTheme>>,
) {
    let colors = if let Some(ref theme) = theme {
        ResolvedTabColors::from_theme(theme)
    } else {
        ResolvedTabColors::default_colors()
    };

    for (tab_view_entity, tab_view) in tab_view_query.iter() {
        for (tab_button, mut bg_color, children) in tab_button_query.iter_mut() {
            if tab_button.tab_view != tab_view_entity {
                continue;
            }

            let is_active = tab_button.index == tab_view.active_tab;

            *bg_color = if is_active {
                BackgroundColor(colors.primary)
            } else {
                BackgroundColor(Color::NONE)
            };

            // Update text color
            for child in children.iter() {
                if let Ok(mut text_color) = text_query.get_mut(child) {
                    *text_color = if is_active {
                        TextColor(colors.text_on_primary)
                    } else {
                        TextColor(colors.text_primary)
                    };
                }
            }
        }
    }
}

/// System to show/hide tab content based on active tab
pub fn update_tab_content_visibility(
    tab_view_query: Query<(Entity, &TabView), Changed<TabView>>,
    mut tab_content_query: Query<(&TabContent, &mut Node)>,
) {
    for (tab_view_entity, tab_view) in tab_view_query.iter() {
        for (tab_content, mut node) in tab_content_query.iter_mut() {
            if tab_content.tab_view != tab_view_entity {
                continue;
            }

            let is_active = tab_content.index == tab_view.active_tab;
            node.display = if is_active { Display::Flex } else { Display::None };
        }
    }
}

/// System to handle tab button hover effects
pub fn handle_tab_hover(
    tab_view_query: Query<&TabView>,
    mut tab_button_query: Query<(&TabButton, &Interaction, &mut BackgroundColor), Changed<Interaction>>,
    theme: Option<Res<UiTheme>>,
) {
    let colors = if let Some(ref theme) = theme {
        ResolvedTabColors::from_theme(theme)
    } else {
        ResolvedTabColors::default_colors()
    };

    for (tab_button, interaction, mut bg_color) in tab_button_query.iter_mut() {
        let Ok(tab_view) = tab_view_query.get(tab_button.tab_view) else {
            continue;
        };

        let is_active = tab_button.index == tab_view.active_tab;

        if is_active {
            // Active tab keeps primary color
            continue;
        }

        *bg_color = match interaction {
            Interaction::Hovered => BackgroundColor(colors.ghost_hover),
            Interaction::Pressed => BackgroundColor(colors.ghost_pressed),
            Interaction::None => BackgroundColor(Color::NONE),
        };
    }
}
