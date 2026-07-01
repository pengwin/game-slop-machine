//! Typed control schemas.

use crate::{CheckboxControl, Control, ControlId, SliderControl};

/// Typed schema for rendering editable controls from data.
pub trait ControlsSchema: Sized + 'static {
    /// Identifier enum for slider fields.
    type Slider: ControlId;
    /// Identifier enum for checkbox fields.
    type Checkbox: ControlId;

    /// Slider controls in display order.
    const SLIDERS: &'static [SliderControl<Self::Slider>];
    /// Checkbox controls in display order.
    const CHECKBOXES: &'static [CheckboxControl<Self::Checkbox>];
    /// All controls in struct field order.
    const CONTROLS: &'static [Control<Self::Slider, Self::Checkbox>];

    /// Reads a slider value from this data type.
    fn slider_value(control: Self::Slider, data: &Self) -> f32;

    /// Writes a slider value to this data type.
    fn set_slider(control: Self::Slider, data: &mut Self, value: f32);

    /// Reads a checkbox value from this data type.
    fn checkbox_value(control: Self::Checkbox, data: &Self) -> bool;

    /// Writes a checkbox value to this data type.
    fn set_checkbox(control: Self::Checkbox, data: &mut Self, value: bool);
}
