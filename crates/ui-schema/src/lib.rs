//! UI-agnostic control schema types.

mod control;
mod id;
mod schema;

pub use control::{CheckboxControl, Control, SliderControl};
pub use id::{ControlId, NoControl};
pub use schema::{ControlsSchema, DelegatedControlsSchema};
