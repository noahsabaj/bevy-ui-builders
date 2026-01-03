# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-01-03

### Added
- **Game UI Module**: Complete game-specific UI components (feature-gated under `game_ui`)
  - `InventoryGridBuilder`: Grid-based inventory with drag-and-drop support
  - `ResourceBarBuilder`: Health/mana/stamina bars with convenience functions (`health_bar()`, `mana_bar()`, etc.)
  - `MinimapBuilder`: Minimap display with configurable shape and rotation
  - `DialogueBoxBuilder`: RPG-style dialogue boxes with typing animation and choices
- **Button Selection States**: Complete multi-state selection system for ButtonBuilder
  - New components: `SelectableButton`, `Selected`, `Active`
  - Auto-toggle functionality for checkbox/toggle behavior
  - Radio button group support with exclusive selection using `InButtonGroup` relationship
  - Builder methods: `.selectable()`, `.selected()`, `.active()`, `.in_group()`, `.manual_toggle()`, `.selection_colors()`
  - `SelectionChanged` message events for tracking selection changes
  - Custom selection color support via `.selection_colors()`
- **WASM Support**: Clipboard operations no longer block WASM builds
  - `arboard` dependency now optional behind `clipboard` feature (enabled by default)
  - Auto-excluded on `wasm32` targets via target-specific dependency
  - Clipboard operations (Ctrl+C/V/X) become graceful no-ops when unavailable
  - Fixes #1

### Changed
- **Codebase Architecture**: Restructured from flat to nested `components/` directory
  - All UI builders now under `src/components/` with consistent module structure
  - Each component has `mod.rs`, `builder.rs`, `types.rs`, `plugin.rs`, `systems.rs`
  - Cleaner separation of concerns and easier navigation
- **Prelude Organization**: Cleaner module structure
  - Moved `pub use bevy::prelude::*` from crate root into `prelude` module
  - Users now have single clean import: `use bevy_ui_builders::prelude::*`
- **Selection Color Generation**: Improved visual distinction for selectable buttons
  - Unselected state now uses neutral gray (SECONDARY) instead of style color
  - Selected state uses the button's style color (PRIMARY, SUCCESS, etc.)
  - Active state uses emphasized darker/saturated version of style color

### Fixed
- **Button Group Exclusivity**: Fixed radio button groups not working correctly
- Replaced non-ASCII characters with ASCII equivalents for compatibility
- Updated all examples for new API

## [0.2.0] - 2025-11-04

### Breaking Changes
- **Migrated to Bevy 0.17**: This version requires Bevy 0.17 and is not compatible with earlier versions
  - Updated event system: `EventWriter`/`EventReader` → `MessageWriter`/`MessageReader`
  - Updated `BorderColor` API: `BorderColor(color)` → `BorderColor::all(color)`
  - Updated `BorderColor` field access: `border_color.0` → `BorderColor::all()` for assignments
  - Updated `RelativeCursorPosition` coordinate system: center-based `[-0.5, 0.5]` instead of corner-based `[0.0, 1.0]`

### Fixed
- **Slider cursor offset**: Fixed 50% offset issue where slider handle appeared left of cursor position
  - Bevy 0.17 changed `RelativeCursorPosition.normalized` from corner-based to center-based coordinates
  - Updated slider drag calculation to convert coordinates: `(cursor_pos.x + 0.5)`
- **Button visual feedback**: Fixed missing hover/pressed state visual changes
  - `UiBuilderPlugin` now conditionally adds `DefaultPickingPlugins` if not already present
  - Prevents duplicate plugin errors when using with `DefaultPlugins`
  - Added `Pickable::IGNORE` to all button text children to prevent them from blocking pointer events
  - This fixes the issue where hovering over button text would not trigger hover/press visual feedback
  - Bevy 0.17 requires picking system to be active for `Interaction` component updates

### Changed
- Bumped bevy dependency from 0.16 to 0.17
- Added `bevy_picking` and `bevy_log` features to enable required functionality
- Updated all internal usage of event system APIs to use Messages
- Updated all BorderColor instantiations to use the new `::all()` constructor
- Updated version to 0.2.0 to reflect breaking API changes

### Technical Notes
- The migration maintains full feature parity with v0.1.10
- All existing builder patterns continue to work unchanged from the user's perspective
- Internal implementation updated to comply with Bevy 0.17 APIs
- `UiBuilderPlugin` now includes `DefaultPickingPlugins` automatically

## [0.1.10] - 2025-09-30

### Fixed
- **TextInputBuilder initial value rendering (proper fix)**: Fixed observer timing issue where querying components during `OnAdd` could fail, causing initial values to not render. Split initialization into two phases:
  1. Observer spawns child TextSpan entities with empty content
  2. New `sync_initial_text_content` system (triggered by `Added<CursorVisual>`) syncs TextBuffer content to TextSpans once all components are guaranteed to be present
- This ensures `.with_value("text")` always displays the initial value, and programmatic updates to `TextBuffer` are properly rendered by the existing `render_text` system

### Technical Details
The v0.1.9 fix attempted to query TextBuffer in the observer, but Bevy 0.16 observers may not reliably access other components in the spawn bundle during `OnAdd` execution. v0.1.10 uses a deferred initialization pattern that waits for all components to be present before syncing initial content.

## [0.1.9] - 2025-09-30

### Fixed
- **TextInputBuilder initial value rendering (incomplete fix)**: Attempted to fix initial value rendering by querying TextBuffer in the observer, but this was unreliable due to observer timing issues. See v0.1.10 for the proper fix.

## [0.1.8] - 2025-09-27

### Documentation
- **Comprehensive README update**: Brought documentation up to date with all features through v0.1.7
  - Added complete ScrollViewBuilder documentation with examples
  - Fixed incorrect DialogBuilder preset syntax (no `presets::` module exists)
  - Documented new methods added in v0.1.5-v0.1.7:
    - ButtonBuilder: `margin()`, `height()`, `enabled()`, `build_in()`
    - SliderBuilder: `with_format()`, `with_marker()`, `build_in()`
    - PanelBuilder: `border_color()`, `flex_grow()`, `flex_shrink()`
    - DialogBuilder: `build_with_buttons()`, `build_and_mark()`
  - Highlighted native text input features (cursor, selection, clipboard, undo/redo)
  - Added "What's New" section showcasing recent additions
  - Updated all version references from "0.1" to appropriate versions
  - Added missing `cleanup` feature to features list
  - Updated Complete Example to use ScrollView and password masking
  - Fixed Bevy version support table

### Changed
- README now accurately reflects all current APIs and features
- Documentation examples tested against actual codebase

## [0.1.7] - 2025-09-27

### Added
- **DialogBuilder custom marker support**: Complete solution for adding custom components to dialog buttons
  - `build_with_buttons()` - Returns `(Entity, HashMap<DialogButtonMarker, Entity>)` for post-creation modification
  - `build_and_mark()` - Helper method to add a single custom marker to a specific button
  - Standard button markers now exported from top-level module for direct use:
    - `ConfirmButton`, `CancelButton`, `SaveButton`, `DiscardButton`
    - `OkButton`, `YesButton`, `NoButton`
  - Dialog type markers exported for type-specific handling:
    - `ExitConfirmationDialog`, `UnsavedChangesDialog`, `ResolutionDialog`
    - `ErrorDialog`, `InfoDialog`, `WarningDialog`, `SuccessDialog`
  - New example `dialog_custom_markers.rs` demonstrating all usage patterns
  - `DialogButtonMarker` now derives `Clone`, `Hash`, `PartialEq`, `Eq` for HashMap usage

### Fixed
- Living Worlds integration issue where custom marker components couldn't be added to dialog buttons
- Dialog buttons can now be queried with both standard and custom markers
- Full backward compatibility maintained - existing DialogBuilder code continues to work

## [0.1.6] - 2025-09-26

### Added
- **SliderBuilder enhancements**:
  - `with_marker<T: Component>(marker: T)` - Attach custom marker components to sliders
  - `SliderBuilderWithMarker` struct for type-safe marker attachment
  - Both `build()` and `build_in()` methods supported on SliderBuilderWithMarker

### Fixed
- Sliders can now properly be identified in event handlers using marker components
- Improved type safety for slider event handling

## [0.1.5] - 2025-09-26

### Added
- **ButtonBuilder enhancements**:
  - `with_marker<T: Component>(marker: T)` - Attach custom marker components to buttons
  - `build_in(parent: &mut ChildSpawnerCommands)` - Alias for `build()` method
  - `margin(margin: UiRect)` - Set button margins
  - `height(height: Val)` - Set custom button height
  - `enabled(enabled: bool)` - Set enabled/disabled state (complement to `disabled()`)
  - `ButtonBuilderWithMarker` struct for type-safe marker attachment
- **SliderBuilder enhancements**:
  - `build_in(parent: &mut ChildSpawnerCommands)` - Alias for `build()` method
  - `with_format(format: ValueFormat)` - Alias for `format()` method
- **PanelBuilder enhancements**:
  - `border_color(color: Color)` - Set custom border color

### Fixed
- Missing API methods that were expected but not published in v0.1.4
- API consistency across all builders with `build_in()` methods

## [0.1.4] - 2025-09-26

### Added
- **ScrollView Module**: Complete scrolling solution with dynamic sizing
  - `ScrollViewBuilder` with viewport-based responsive sizing (Val::Vh/Vw)
  - `ScrollViewPlugin` with mouse wheel scrolling support
  - Visual scrollbar components with auto-hide functionality
  - Smooth scroll animations with lerp interpolation
  - Auto-scroll to focused text inputs
  - `ScrollState` component tracking scroll position and limits
- **Native Text Input System**: Full-featured text editing implementation
  - Cursor rendering with proper visibility and blinking
  - Text selection with keyboard and mouse
  - Clipboard operations (Ctrl+C/V/X)
  - Undo/Redo support (Ctrl+Z/Shift+Z)
  - Input filtering (numeric, alphabetic, alphanumeric)
  - Max length constraints
  - Clear button functionality
  - Password masking support
  - Placeholder text
- **Viewport-Relative Dimensions**: New responsive sizing constants
  - `SPACING_*_VH/VW` for viewport-based spacing
  - `PADDING_*_VH/VW` for responsive padding
  - `MAX_CONTENT_HEIGHT/WIDTH` using viewport units
  - Dynamic content sizing without hardcoded pixels
- `flex_grow` and `flex_shrink` methods to PanelBuilder

### Changed
- TextInputBuilder now uses native text input implementation
- text_input_demo updated to use ScrollViewBuilder with responsive sizing
- Tab navigation now properly respects FocusGroupId boundaries
- Cursor visibility logic improved for consistent rendering
- Focus tracking enhanced for better text input interactions

### Fixed
- Tab key navigation jumping to random inputs outside focus groups
- Cursor not visible in text inputs
- Max length enforcement in text inputs
- Focus state tracking issues
- Content cut off at window edges (now scrollable)
- Layout inconsistencies across different window sizes

### Removed
- 3 deprecated empty functions from text_input/systems.rs:
  - `handle_text_input_focus` (empty stub)
  - `handle_click_outside_unfocus` (empty stub)
  - `validate_text_input_changes` (empty stub)
- Technical debt from legacy text input implementation

## [0.1.3] - 2025-09-25

### Fixed
- Fixed failing doctests in cleanup.rs by marking examples as `no_run`
- Made doctest examples self-contained with proper GameState definitions

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