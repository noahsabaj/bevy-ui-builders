//! Form module - PURE GATEWAY (exports only)

// Private submodules
mod builder;
mod field;
mod types;

// Public exports
pub use builder::FormBuilder;
pub use types::{
    FieldType,
    ValidationRule,
    FormField,
    FormData,
    FormRoot,
    FormFieldMarker,
    FormSubmitButton,
    FormSubmitEvent,
    FormLayout,
    ValidationTrigger,
};