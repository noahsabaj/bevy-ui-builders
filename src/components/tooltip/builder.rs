//! TooltipBuilder implementation

use bevy::prelude::*;
use std::time::Duration;
use crate::traits::{UiBuilder, LayoutBuilder, BuilderBase, TooltipPosition};
use super::types::*;

/// Builder for creating tooltips
///
/// Tooltips can be attached to any UI element to provide additional context.
///
/// # Examples
///
/// ```ignore
/// use bevy_ui_builders::prelude::*;
///
/// // Simple tooltip attached to a button
/// ButtonBuilder::new("Hover me")
///     .tooltip("Click to submit the form")
///     .build(parent);
///
/// // Rich tooltip with title and description
/// TooltipBuilder::rich("Settings", "Configure application preferences")
///     .position(TooltipPosition::Right)
///     .build_for(parent, target_entity);
/// ```
pub struct TooltipBuilder {
    content: TooltipContent,
    position: TooltipPosition,
    delay: Option<Duration>,
    max_width: Option<f32>,
    base: BuilderBase,
}

impl TooltipBuilder {
    /// Create a simple text tooltip
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            content: TooltipContent::Simple(text.into()),
            position: TooltipPosition::Top,
            delay: None,
            max_width: None,
            base: BuilderBase::new(),
        }
    }

    /// Create a rich tooltip with title and optional description
    pub fn rich(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            content: TooltipContent::Rich {
                title: title.into(),
                description: Some(description.into()),
            },
            position: TooltipPosition::Top,
            delay: None,
            max_width: None,
            base: BuilderBase::new(),
        }
    }

    /// Create a rich tooltip with title only
    pub fn titled(title: impl Into<String>) -> Self {
        Self {
            content: TooltipContent::Rich {
                title: title.into(),
                description: None,
            },
            position: TooltipPosition::Top,
            delay: None,
            max_width: None,
            base: BuilderBase::new(),
        }
    }

    /// Set the tooltip position
    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the delay before showing the tooltip
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
    }

    /// Set the delay in milliseconds
    pub fn delay_ms(mut self, ms: u64) -> Self {
        self.delay = Some(Duration::from_millis(ms));
        self
    }

    /// Set the maximum width
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = Some(width);
        self
    }

    /// Build and attach the tooltip to an existing entity
    pub fn build_for(self, commands: &mut Commands, target: Entity) {
        let delay = self.delay.unwrap_or(Duration::from_millis(500));
        let max_width = self.max_width.unwrap_or(300.0);

        commands.entity(target).insert((
            HasTooltip {
                content: self.content,
                position: self.position,
                delay,
                max_width,
            },
            TooltipState::default(),
        ));
    }

    /// Get the tooltip components to manually attach
    pub fn into_components(self) -> (HasTooltip, TooltipState) {
        let delay = self.delay.unwrap_or(Duration::from_millis(500));
        let max_width = self.max_width.unwrap_or(300.0);

        (
            HasTooltip {
                content: self.content,
                position: self.position,
                delay,
                max_width,
            },
            TooltipState::default(),
        )
    }
}

impl UiBuilder for TooltipBuilder {
    fn build(self, parent: &mut ChildSpawnerCommands) -> Entity {
        // When built standalone, create a placeholder entity with the tooltip
        let delay = self.delay.unwrap_or(Duration::from_millis(500));
        let max_width = self.max_width.unwrap_or(300.0);

        let entity = parent
            .spawn((
                Node::default(),
                HasTooltip {
                    content: self.content,
                    position: self.position,
                    delay,
                    max_width,
                },
                TooltipState::default(),
            ))
            .id();

        entity
    }

    fn insert(mut self, bundle: impl Bundle + Clone) -> Self {
        self.base.hooks.push(Box::new(move |cmds| {
            cmds.insert(bundle.clone());
        }));
        self
    }

    fn id(mut self, id: Entity) -> Self {
        self.base.entity = Some(id);
        self
    }
}

impl LayoutBuilder for TooltipBuilder {}

/// Convenience function to create a tooltip builder
pub fn tooltip(text: impl Into<String>) -> TooltipBuilder {
    TooltipBuilder::new(text)
}
