# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2025-09-24

### Added
- Integrated bevy-plugin-builder for cleaner plugin definitions
- Smooth animation system for button hover and click interactions
- ButtonAnimationState component for tracking transition states
- Language guidelines in CLAUDE.md for consistent documentation tone
- CHANGELOG.md for tracking project changes
- **Bevy 0.16 Relationships System**: Complete implementation of custom UI relationships
- **Custom UI Relationships Module** (`src/relationships/mod.rs`) with:
  - BelongsToDialog/DialogElements for dialog management
  - SliderPart/SliderParts for slider components
  - BelongsToForm/FormFields for form validation
  - InButtonGroup/ButtonGroupMembers for radio button groups
  - PanelContent/PanelContents for panel organization
  - TextInputPart/TextInputParts for text input components
  - ProgressBarPart/ProgressBarParts for progress bars
- **GitHub Actions CI/CD**: Complete automation setup with Trusted Publishing
- Comprehensive documentation for Bevy 0.16 relationships in CLAUDE.md

### Changed
- All plugins now use define_plugin! macro from bevy-plugin-builder
- Button hover effects are now smoother with lerp interpolation
- Default hover scale reduced from 1.02 to 1.015 for subtler effect
- Press scale adjusted to 0.98 of hover scale for less jarring animation
- Documentation language updated to be purely utilitarian
- Moved ButtonStyle and ButtonSize to dedicated button_styles.rs module
- styles/mod.rs refactored to pure gateway pattern
- **DialogBuilder now uses relationships**: Utilizes BelongsToDialog for automatic cleanup
- **All despawn() calls changed to despawn_recursive()**: Prevents memory leaks from orphaned children

### Fixed
- All example compilation errors resolved
- Bevy 0.16 API compatibility issues fixed
- Borrow checker issues in examples
- Node struct field compatibility (row_gap/column_gap instead of gap)
- **Critical bug #1**: Removed flawed cleanup_orphaned_ui function that would delete all root UI
- **Critical bugs #2-4**: Fixed dialog system memory leaks (3 instances)
- **Critical bugs #5-6**: Fixed cleanup system memory leaks (2 instances)
- **Relationship compilation errors**: Fixed private field requirements for RelationshipTarget

## [0.1.0] - 2025-09-24

### Added
- Initial release with 9 UI builders
- ButtonBuilder for styled, interactive buttons
- SliderBuilder for value sliders with drag interaction
- FormBuilder for complete forms with validation
- DialogBuilder for modal dialogs with overlays
- TextInputBuilder for text inputs with filtering
- ProgressBarBuilder for progress indicators
- LabelBuilder for consistent text labels
- PanelBuilder for container panels
- SeparatorBuilder for visual dividers
- Comprehensive examples demonstrating all builders
- Gateway architecture for clean module boundaries