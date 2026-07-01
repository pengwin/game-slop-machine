//! Shared data-driven inspector controls.

use std::marker::PhantomData;

use bevy::{
    ecs::component::Mutable,
    feathers::{
        controls::{FeathersCheckbox, FeathersSlider},
        font_styles::InheritableFont,
        theme::ThemedText,
    },
    prelude::*,
    ui::Checked,
    ui_widgets::{
        SliderPrecision, SliderStep, SliderValue, ValueChange, checkbox_self_update,
        slider_self_update,
    },
};
use ui_schema::{CheckboxControl, Control, ControlsSchema, SliderControl};

use super::consts::PANEL_FONT_SIZE;

/// Component attached to generated slider widgets.
#[derive(Component)]
pub struct SchemaSlider<T: ControlsSchema> {
    control: T::Slider,
    marker: PhantomData<fn() -> T>,
}

impl<T: ControlsSchema> Default for SchemaSlider<T> {
    fn default() -> Self {
        Self {
            control: T::Slider::default(),
            marker: PhantomData,
        }
    }
}

impl<T: ControlsSchema> Clone for SchemaSlider<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ControlsSchema> Copy for SchemaSlider<T> {}

/// Component attached to generated checkbox widgets.
#[derive(Component)]
pub struct SchemaCheckbox<T: ControlsSchema> {
    control: T::Checkbox,
    marker: PhantomData<fn() -> T>,
}

impl<T: ControlsSchema> Default for SchemaCheckbox<T> {
    fn default() -> Self {
        Self {
            control: T::Checkbox::default(),
            marker: PhantomData,
        }
    }
}

impl<T: ControlsSchema> Clone for SchemaCheckbox<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ControlsSchema> Copy for SchemaCheckbox<T> {}

/// Builds data-driven slider rows for a resource schema.
pub fn control_rows<T>(label_width: f32) -> Vec<Box<dyn SceneList>>
where
    T: Resource<Mutability = Mutable> + ControlsSchema,
{
    T::CONTROLS
        .iter()
        .map(|control| match *control {
            Control::Slider(control) => {
                Box::new(bsn_list![slider_row::<T>(control, label_width)]) as Box<dyn SceneList>
            }
            Control::Checkbox(control) => {
                Box::new(bsn_list![checkbox_row::<T>(control)]) as Box<dyn SceneList>
            }
        })
        .collect()
}

fn slider_row<T>(control: SliderControl<T::Slider>, label_width: f32) -> impl Scene
where
    T: Resource<Mutability = Mutable> + ControlsSchema,
{
    let handler_control = control.id;
    let label = control.label;
    let min = control.min;
    let max = control.max;
    let step = control.step;
    let precision = control.precision;

    bsn! {
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: px(4),
        }
        InheritableFont {
            font_size: PANEL_FONT_SIZE,
        }
        Children [
            (
                Text(label)
                ThemedText
                Node {
                    width: px(label_width),
                }
            ),
            (
                @FeathersSlider {
                    @min: min,
                    @max: max,
                    @value: min,
                }
                template_value(SchemaSlider::<T> {
                    control: control.id,
                    marker: PhantomData,
                })
                SliderStep(step)
                SliderPrecision(precision)
                on(slider_self_update)
                on(
                    move |
                        change: On<'_, '_, ValueChange<f32>>,
                    mut resource: ResMut<'_, T>,
                | {
                        T::set_slider(handler_control, &mut *resource, change.value);
                    }
                )
            )
        ]
    }
}

fn checkbox_row<T>(control: CheckboxControl<T::Checkbox>) -> impl Scene
where
    T: Resource<Mutability = Mutable> + ControlsSchema,
{
    let handler_control = control.id;
    let label = control.label;

    bsn! {
        (
            @FeathersCheckbox {
                @caption: bsn! { Text(label) ThemedText }
            }
            InheritableFont {
                font_size: PANEL_FONT_SIZE,
            }
            template_value(SchemaCheckbox::<T> {
                control: control.id,
                marker: PhantomData,
            })
            on(checkbox_self_update)
            on(
                move |
                    change: On<'_, '_, ValueChange<bool>>,
                    mut resource: ResMut<'_, T>,
                | {
                    T::set_checkbox(handler_control, &mut *resource, change.value);
                }
            )
        )
    }
}

/// Syncs generated slider widgets from their backing resource.
#[allow(clippy::needless_pass_by_value)]
pub fn sync_schema_sliders<T>(
    mut commands: Commands<'_, '_>,
    resource: Res<'_, T>,
    sliders: Query<'_, '_, (Entity, &SchemaSlider<T>, &SliderValue)>,
) where
    T: Resource + ControlsSchema,
{
    for (entity, slider, value) in &sliders {
        let expected = T::slider_value(slider.control, &resource);
        if (value.0 - expected).abs() > 0.001 {
            commands.entity(entity).insert(SliderValue(expected));
        }
    }
}

/// Syncs generated checkbox widgets from their backing resource.
#[allow(clippy::needless_pass_by_value)]
pub fn sync_schema_checkboxes<T>(
    mut commands: Commands<'_, '_>,
    resource: Res<'_, T>,
    checkboxes: Query<'_, '_, (Entity, &SchemaCheckbox<T>, Has<Checked>)>,
) where
    T: Resource + ControlsSchema,
{
    for (entity, checkbox, checked) in &checkboxes {
        let expected = T::checkbox_value(checkbox.control, &resource);
        if expected && !checked {
            commands.entity(entity).insert(Checked);
        } else if !expected && checked {
            commands.entity(entity).remove::<Checked>();
        }
    }
}
