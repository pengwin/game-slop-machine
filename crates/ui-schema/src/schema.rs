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

/// Delegates a control schema to an owned target value.
pub trait DelegatedControlsSchema: Sized + 'static {
    /// Target type that owns the actual control metadata and accessors.
    type Target: ControlsSchema;

    /// Returns the delegated target value.
    fn target(&self) -> &Self::Target;

    /// Returns the delegated target value mutably.
    fn target_mut(&mut self) -> &mut Self::Target;
}

impl<T> ControlsSchema for T
where
    T: DelegatedControlsSchema,
{
    type Slider = <T::Target as ControlsSchema>::Slider;
    type Checkbox = <T::Target as ControlsSchema>::Checkbox;

    const SLIDERS: &'static [SliderControl<Self::Slider>] = T::Target::SLIDERS;
    const CHECKBOXES: &'static [CheckboxControl<Self::Checkbox>] = T::Target::CHECKBOXES;
    const CONTROLS: &'static [Control<Self::Slider, Self::Checkbox>] = T::Target::CONTROLS;

    fn slider_value(control: Self::Slider, data: &Self) -> f32 {
        T::Target::slider_value(control, data.target())
    }

    fn set_slider(control: Self::Slider, data: &mut Self, value: f32) {
        T::Target::set_slider(control, data.target_mut(), value);
    }

    fn checkbox_value(control: Self::Checkbox, data: &Self) -> bool {
        T::Target::checkbox_value(control, data.target())
    }

    fn set_checkbox(control: Self::Checkbox, data: &mut Self, value: bool) {
        T::Target::set_checkbox(control, data.target_mut(), value);
    }
}
