//! # Bevy UI Builders
//!
//! Declarative UI builders for Bevy.
//!
//! ## Example
//!
//! ```ignore
//! use bevy_ui_builders::*;
//!
//! ButtonBuilder::new("Click Me")
//!     .style(ButtonStyle::Primary)
//!     .build(commands);
//! ```

#![warn(missing_docs)]

// Re-export Bevy UI prelude for convenience
pub use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

// Core modules
mod styles;
mod systems;
mod utils;
pub mod relationships;
pub mod validation;
pub mod traits;
pub mod theme;
pub mod animation;

// Components module (contains all UI component builders)
pub mod components;

// Layout helpers
pub mod layout;

// Game UI module (feature-gated)
#[cfg(feature = "game_ui")]
pub mod game_ui;

// Public exports - Styles
pub use styles::{ButtonStyle, ButtonSize, colors, dimensions};

// Public exports - Theme
pub use theme::{
    UiTheme, ThemeColors, ColorScale, ThemePlugin, ThemeChanged,
    ThemeSpacing, ThemeTypography, ThemeBorders, ThemeAnimation,
    SurfaceColors, TextColors, BorderColors,
    // Semantic variant system
    SemanticVariant, ResolvedColors, resolve_colors,
};

// Public exports - Animation
pub use animation::{
    UiAnimation, AnimationPreset, AnimationPlugin, HoverEffect,
    Transition, Direction, Easing,
    AnimationCategory, DisableAutoAnimation,
};

// Public exports - Systems
pub use systems::cleanup::{despawn_entities, despawn_ui_entities};

// Public exports - Relationships
pub use relationships::{
    BelongsToDialog, DialogElements,
    SliderPart, SliderParts,
    BelongsToForm, FormFields,
    InButtonGroup, ButtonGroupMembers,
    PanelContent, PanelContents,
    TextInputPart, TextInputParts,
    ProgressBarPart, ProgressBarParts,
    BelongsToDropdown, DropdownElements,
    UIRelationshipsPlugin,
};

// Public exports - Validation
pub use validation::{Validated, ValidationState, ValidationPlugin};

// Public exports - ScrollView (always available)
pub use components::scroll_view::{
    ScrollViewBuilder, ScrollView, ScrollConfig, ScrollDirection, ScrollViewPlugin, scroll_view
};

// Builder exports based on features
#[cfg(feature = "button")]
pub use components::button::{
    ButtonBuilder, StyledButton,
    primary_button, secondary_button, success_button, danger_button, ghost_button,
};

#[cfg(feature = "slider")]
pub use components::slider::{SliderBuilder, Slider, SliderHandle, SliderTrack, ValueFormat};

#[cfg(feature = "form")]
pub use components::form::{FormBuilder, FieldType, ValidationRule};

#[cfg(feature = "dialog")]
pub use components::dialog::{
    DialogBuilder, DialogButtonEvent, DialogType, DialogOverlay, DialogButtonMarker,
    // Standard button markers for dialog buttons
    ConfirmButton, CancelButton, SaveButton, DiscardButton,
    OkButton, YesButton, NoButton,
    // Dialog type markers
    ExitConfirmationDialog, UnsavedChangesDialog, ResolutionDialog,
    ErrorDialog, InfoDialog, WarningDialog, SuccessDialog
};

#[cfg(feature = "text_input")]
pub use components::text_input::{TextInputBuilder, InputFilter, InputTransform, FocusGroupId, text_input};

#[cfg(feature = "progress")]
pub use components::progress::{ProgressBarBuilder, ProgressBar, ProgressBarStyle, progress};

#[cfg(feature = "label")]
pub use components::label::{LabelBuilder, Label, LabelSize, label};
// Deprecated re-export for backwards compatibility
#[allow(deprecated)]
pub use components::label::LabelStyle;

#[cfg(feature = "panel")]
pub use components::panel::{PanelBuilder, Panel, PanelStyle, panel};

#[cfg(feature = "separator")]
pub use components::separator::{SeparatorBuilder, Separator, SeparatorStyle, Orientation, separator};

#[cfg(feature = "checkbox")]
pub use components::checkbox::{CheckboxBuilder, Checkbox, CheckboxState, CheckboxStyle};

#[cfg(feature = "number_input")]
pub use components::number_input::{NumberInputBuilder, NumberInput, NumberInputConfig};

#[cfg(feature = "dropdown")]
pub use components::dropdown::{DropdownBuilder, Dropdown, DropdownState, DropdownData};

#[cfg(feature = "tooltip")]
pub use components::tooltip::{
    TooltipBuilder, HasTooltip, TooltipContent, TooltipEntity, TooltipSettings, TooltipState, tooltip
};

#[cfg(feature = "tabs")]
pub use components::tabs::{
    TabViewBuilder, TabView, TabButton, TabContent, TabPosition, TabStyle,
    TabSelectedEvent, TabConfig, NoTabs, HasTabs, tabs
};

#[cfg(feature = "toast")]
pub use components::toast::{
    ToastBuilder, Toast, ToastVariant, ToastPosition, ToastQueue, ToastSettings,
    ActiveToast, ToastContainer, ToastActionEvent, DismissToastEvent,
    show_toast, show_success, show_error, show_warning
};

#[cfg(feature = "context_menu")]
pub use components::context_menu::{
    ContextMenuBuilder, SubmenuBuilder, MenuItem, ContextMenuTrigger, ContextMenu,
    ContextMenuItem, ContextMenuActionEvent, ContextMenuCheckboxEvent,
    ContextMenuSettings, OpenContextMenu, context_menu
};

// Game UI exports
#[cfg(feature = "inventory")]
pub use game_ui::inventory::{
    InventoryGridBuilder, InventoryGrid, InventorySlot, InventoryItem,
    SlotClickEvent, ItemDragStartEvent, ItemDropEvent, inventory_grid
};

#[cfg(feature = "resource_bar")]
pub use game_ui::resource_bar::{
    ResourceBarBuilder, ResourceBar, ResourceBarStyle, ResourceBarFill,
    health_bar, mana_bar, stamina_bar, experience_bar
};

#[cfg(feature = "minimap")]
pub use game_ui::minimap::{
    MinimapBuilder, Minimap, MinimapShape, MinimapRotation, minimap
};

#[cfg(feature = "dialogue")]
pub use game_ui::dialogue::{
    DialogueBoxBuilder, DialogueBox, DialogueChoice, DialogueChoiceEvent,
    DialogueTypingCompleteEvent, dialogue_box
};

/// Prelude module for convenient imports
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        despawn_ui_entities, despawn_entities,
    };

    // Theme
    pub use crate::{UiTheme, ThemeColors, ColorScale, SemanticVariant};

    // Animation
    pub use crate::{UiAnimation, AnimationPreset, HoverEffect, Transition, Easing};

    // Validation
    pub use crate::{Validated, ValidationState, ValidationRule};

    #[cfg(feature = "button")]
    pub use crate::{ButtonBuilder, ButtonStyle, ButtonSize};

    #[cfg(feature = "slider")]
    pub use crate::{SliderBuilder, ValueFormat};

    #[cfg(feature = "form")]
    pub use crate::{FormBuilder, FieldType};

    #[cfg(feature = "dialog")]
    pub use crate::{
        DialogBuilder, DialogType, DialogButtonMarker,
        // Standard button markers
        ConfirmButton, CancelButton, SaveButton, DiscardButton,
        OkButton, YesButton, NoButton,
    };

    #[cfg(feature = "text_input")]
    pub use crate::{TextInputBuilder, InputFilter};

    #[cfg(feature = "progress")]
    pub use crate::{ProgressBarBuilder, ProgressBarStyle};

    #[cfg(feature = "label")]
    pub use crate::{LabelBuilder, LabelSize};

    #[cfg(feature = "panel")]
    pub use crate::{PanelBuilder, PanelStyle};

    #[cfg(feature = "separator")]
    pub use crate::{SeparatorBuilder, Orientation};

    #[cfg(feature = "checkbox")]
    pub use crate::{CheckboxBuilder, CheckboxState, CheckboxStyle};

    #[cfg(feature = "number_input")]
    pub use crate::{NumberInputBuilder};

    #[cfg(feature = "dropdown")]
    pub use crate::{DropdownBuilder, DropdownState};

    #[cfg(feature = "tooltip")]
    pub use crate::{TooltipBuilder, tooltip};

    #[cfg(feature = "tabs")]
    pub use crate::{TabViewBuilder, TabPosition, TabStyle, TabConfig, tabs};

    #[cfg(feature = "toast")]
    pub use crate::{ToastBuilder, ToastVariant, ToastPosition, ToastQueue, show_toast, show_success, show_error, show_warning};

    #[cfg(feature = "context_menu")]
    pub use crate::{ContextMenuBuilder, MenuItem, ContextMenuActionEvent, context_menu};

    // Game UI
    #[cfg(feature = "inventory")]
    pub use crate::{InventoryGridBuilder, SlotClickEvent, inventory_grid};

    #[cfg(feature = "resource_bar")]
    pub use crate::{ResourceBarBuilder, ResourceBarStyle, health_bar, mana_bar};

    #[cfg(feature = "minimap")]
    pub use crate::{MinimapBuilder, MinimapShape, MinimapRotation, minimap};

    #[cfg(feature = "dialogue")]
    pub use crate::{DialogueBoxBuilder, DialogueChoiceEvent, dialogue_box};

    // Traits
    pub use crate::traits::{
        UiBuilder, LayoutBuilder, StyleBuilder, SizeableBuilder,
        InteractiveBuilder, ContentBuilder, TooltipPosition,
    };

    // Layout helpers
    pub use crate::layout::{
        UiContainer, UiContainerBuilder,
        RowBuilder, ColumnBuilder, SpacerBuilder,
        row, column, centered, spacer,
    };

    // Convenience functions from components
    #[cfg(feature = "button")]
    pub use crate::components::button::{
        button, primary_button, secondary_button, success_button, danger_button, ghost_button,
    };

    #[cfg(feature = "label")]
    pub use crate::components::label::{label, heading, title, secondary_text};

    #[cfg(feature = "slider")]
    pub use crate::components::slider::{slider, percentage_slider, normalized_slider};

    #[cfg(feature = "text_input")]
    pub use crate::components::text_input::text_input;
}

define_plugin!(UiBuilderPlugin {
    plugins: [ThemePlugin, AnimationPlugin, UIRelationshipsPlugin, ValidationPlugin],
    custom_init: |app: &mut App| {
        // Bevy 0.17 requires picking plugins for Interaction component updates
        // Only add if not already present (DefaultPlugins includes them)
        if !app.is_plugin_added::<bevy::picking::input::PointerInputPlugin>() {
            app.add_plugins(bevy::picking::DefaultPickingPlugins);
        }

        // ScrollView plugin (always available)
        app.add_plugins(components::scroll_view::ScrollViewPlugin);

        #[cfg(feature = "button")]
        app.add_plugins(components::button::ButtonPlugin);

        #[cfg(feature = "slider")]
        app.add_plugins(components::slider::SliderPlugin);

        #[cfg(feature = "dialog")]
        app.add_plugins(components::dialog::DialogPlugin);

        #[cfg(feature = "text_input")]
        app.add_plugins(components::text_input::TextInputPlugin);

        #[cfg(feature = "progress")]
        app.add_plugins(components::progress::ProgressBarPlugin);

        #[cfg(feature = "checkbox")]
        app.add_plugins(components::checkbox::CheckboxPlugin);

        #[cfg(feature = "dropdown")]
        app.add_plugins(components::dropdown::DropdownPlugin);

        #[cfg(feature = "tooltip")]
        app.add_plugins(components::tooltip::TooltipPlugin);

        #[cfg(feature = "tabs")]
        app.add_plugins(components::tabs::TabsPlugin);

        #[cfg(feature = "toast")]
        app.add_plugins(components::toast::ToastPlugin);

        #[cfg(feature = "context_menu")]
        app.add_plugins(components::context_menu::ContextMenuPlugin);

        // Game UI plugins
        #[cfg(feature = "inventory")]
        app.add_plugins(game_ui::inventory::InventoryPlugin);

        #[cfg(feature = "resource_bar")]
        app.add_plugins(game_ui::resource_bar::ResourceBarPlugin);

        #[cfg(feature = "minimap")]
        app.add_plugins(game_ui::minimap::MinimapPlugin);

        #[cfg(feature = "dialogue")]
        app.add_plugins(game_ui::dialogue::DialoguePlugin);
    }
});
