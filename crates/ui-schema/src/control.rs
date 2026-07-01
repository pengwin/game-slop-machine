//! Control metadata types.

use crate::ControlId;

/// Slider metadata for one editable field.
#[derive(Clone, Copy, Debug)]
pub struct SliderControl<Id: ControlId> {
    /// Typed identifier used by generated accessors.
    pub id: Id,
    /// Human-readable label.
    pub label: &'static str,
    /// Minimum slider value.
    pub min: f32,
    /// Maximum slider value.
    pub max: f32,
    /// Slider step.
    pub step: f32,
    /// Number of decimal places displayed by the UI widget.
    pub precision: i32,
}

/// Checkbox metadata for one editable boolean field.
#[derive(Clone, Copy, Debug)]
pub struct CheckboxControl<Id: ControlId> {
    /// Typed identifier used by generated accessors.
    pub id: Id,
    /// Human-readable label.
    pub label: &'static str,
}

/// Metadata for one editable control in struct field order.
#[derive(Clone, Copy, Debug)]
pub enum Control<SliderId: ControlId, CheckboxId: ControlId> {
    /// Slider field.
    Slider(SliderControl<SliderId>),
    /// Checkbox field.
    Checkbox(CheckboxControl<CheckboxId>),
}
