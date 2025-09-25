//! Component types for UI relationships
//!
//! Defines all custom relationship components using Bevy 0.16's relationship system.
//! These components provide automatic cleanup, type-safe entity connections,
//! and efficient querying of related UI elements.

use bevy::prelude::*;

// ============================================================================
// Dialog Relationships
// ============================================================================

/// Marks an entity as belonging to a specific dialog.
/// When the dialog is despawned, all entities with this component
/// are automatically cleaned up thanks to linked_spawn.
#[derive(Component)]
#[relationship(relationship_target = DialogElements)]
pub struct BelongsToDialog(pub Entity);

/// Contains all entities that belong to this dialog.
/// The linked_spawn attribute ensures automatic cleanup.
#[derive(Component)]
#[relationship_target(relationship = BelongsToDialog, linked_spawn)]
pub struct DialogElements(Vec<Entity>);

impl DialogElements {
    /// Get an iterator over the dialog elements
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.0.iter()
    }
}

// ============================================================================
// Slider Relationships
// ============================================================================

/// Marks an entity as part of a slider (track, handle, label, etc.).
#[derive(Component)]
#[relationship(relationship_target = SliderParts)]
pub struct SliderPart(pub Entity);

/// Contains all parts that make up this slider.
#[derive(Component)]
#[relationship_target(relationship = SliderPart, linked_spawn)]
pub struct SliderParts(Vec<Entity>);

impl SliderParts {
    /// Get an iterator over the slider parts
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.0.iter()
    }
}

// ============================================================================
// Form Relationships
// ============================================================================

/// Marks an entity as a field belonging to a form.
#[derive(Component)]
#[relationship(relationship_target = FormFields)]
pub struct BelongsToForm(pub Entity);

/// Contains all fields that belong to this form.
#[derive(Component)]
#[relationship_target(relationship = BelongsToForm, linked_spawn)]
pub struct FormFields(Vec<Entity>);

impl FormFields {
    /// Get an iterator over the form fields
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.0.iter()
    }
}

// ============================================================================
// Button Group Relationships
// ============================================================================

/// Marks a button as part of a button group (e.g., radio buttons).
#[derive(Component)]
#[relationship(relationship_target = ButtonGroupMembers)]
pub struct InButtonGroup(pub Entity);

/// Contains all buttons in this button group.
/// Note: No linked_spawn here as button groups may outlive individual buttons.
#[derive(Component)]
#[relationship_target(relationship = InButtonGroup)]
pub struct ButtonGroupMembers(Vec<Entity>);

impl ButtonGroupMembers {
    /// Get an iterator over the button group members
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.0.iter()
    }
}

// ============================================================================
// Panel Relationships
// ============================================================================

/// Marks an entity as content within a panel.
#[derive(Component)]
#[relationship(relationship_target = PanelContents)]
pub struct PanelContent(pub Entity);

/// Contains all content entities within this panel.
#[derive(Component)]
#[relationship_target(relationship = PanelContent, linked_spawn)]
pub struct PanelContents(Vec<Entity>);

impl PanelContents {
    /// Get an iterator over the panel contents
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.0.iter()
    }
}

// ============================================================================
// Text Input Relationships
// ============================================================================

/// Marks an entity as part of a text input (cursor, selection, etc.).
#[derive(Component)]
#[relationship(relationship_target = TextInputParts)]
pub struct TextInputPart(pub Entity);

/// Contains all parts that make up this text input.
#[derive(Component)]
#[relationship_target(relationship = TextInputPart, linked_spawn)]
pub struct TextInputParts(Vec<Entity>);

impl TextInputParts {
    /// Get an iterator over the text input parts
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.0.iter()
    }
}

// ============================================================================
// Progress Bar Relationships
// ============================================================================

/// Marks an entity as part of a progress bar (fill, label, etc.).
#[derive(Component)]
#[relationship(relationship_target = ProgressBarParts)]
pub struct ProgressBarPart(pub Entity);

/// Contains all parts that make up this progress bar.
#[derive(Component)]
#[relationship_target(relationship = ProgressBarPart, linked_spawn)]
pub struct ProgressBarParts(Vec<Entity>);

impl ProgressBarParts {
    /// Get an iterator over the progress bar parts
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.0.iter()
    }
}

// ============================================================================
// Temporary Components (should be moved to appropriate modules)
// ============================================================================

/// Component marker for Slider (should be defined in slider module)
#[derive(Component)]
pub struct Slider {
    /// Current value of the slider
    pub value: f32,
    /// Minimum value
    pub min: f32,
    /// Maximum value
    pub max: f32,
}