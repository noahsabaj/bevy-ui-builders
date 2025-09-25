# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2] - 2025-09-25

### Added
- **Bevy 0.16 Relationships System**: Complete implementation of custom UI relationships
- **Custom UI Relationships Module** (`src/relationships/`) with strict gateway architecture:
  - BelongsToDialog/DialogElements for dialog management with auto-cleanup
  - SliderPart/SliderParts for slider components
  - BelongsToForm/FormFields for form validation
  - InButtonGroup/ButtonGroupMembers for radio button groups
  - PanelContent/PanelContents for panel organization
  - TextInputPart/TextInputParts for text input components
  - ProgressBarPart/ProgressBarParts for progress bars
- **GitHub Actions CI/CD**: Complete automation setup with:
  - CI pipeline (ci.yml) with test, clippy, and fmt checks
  - Release automation (release.yml) with Trusted Publishing to crates.io
  - Multi-job CI strategy testing main, minimal features, and MSRV
  - All required Bevy system dependencies configured
- Complete examples suite with 9 demonstration files

### Changed
- **Relationships module refactored** to strict gateway architecture pattern
- **README.md project structure** updated to complete 72-file listing
- **All despawn() calls updated** for Bevy 0.16 (now automatically recursive)
- Dialog systems now utilize relationships for automatic cleanup

### Fixed
- **Critical bug**: Removed flawed cleanup_orphaned_ui function that would delete all root UI entities
- **Memory leaks**: Fixed 5 instances where dialogs weren't properly cleaning up child entities
- **Documentation errors**:
  - DialogType::Confirmation → DialogType::Custom (type doesn't exist)
  - InputFilter::Email → InputFilter::Alphanumeric (filter doesn't exist)
- **Gateway architecture violations**: Refactored relationships/mod.rs from 263-line implementation to 12-line pure gateway

## [0.1.1] - 2025-09-24

### Added
- Integrated bevy-plugin-builder for cleaner plugin definitions
- Smooth animation system for button hover and click interactions
- ButtonAnimationState component for tracking transition states
- Language guidelines in CLAUDE.md for consistent documentation tone
- CHANGELOG.md for tracking project changes
- Comprehensive documentation for Bevy 0.16 API changes in CLAUDE.md

### Changed
- All plugins now use define_plugin! macro from bevy-plugin-builder
- Button hover effects are now smoother with lerp interpolation
- Default hover scale reduced from 1.02 to 1.015 for subtler effect
- Press scale adjusted to 0.98 of hover scale for less jarring animation
- Documentation language updated to be purely utilitarian
- Moved ButtonStyle and ButtonSize to dedicated button_styles.rs module
- styles/mod.rs refactored to pure gateway pattern

### Fixed
- All example compilation errors resolved
- Bevy 0.16 API compatibility issues fixed
- Borrow checker issues in examples
- Node struct field compatibility (row_gap/column_gap instead of gap)

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