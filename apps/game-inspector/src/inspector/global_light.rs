//! Global light controls for inspector scenes.

use bevy::{
    feathers::{
        controls::{
            FeathersCheckbox, FeathersMenu, FeathersMenuButton, FeathersMenuItem,
            FeathersMenuPopup, FeathersSlider,
        },
        font_styles::InheritableFont,
        theme::{ThemeBackgroundColor, ThemedText},
        tokens,
    },
    input_focus::tab_navigation::TabGroup,
    prelude::*,
    ui::Checked,
    ui_widgets::{
        checkbox_self_update, slider_self_update, Activate, SliderPrecision, SliderStep,
        SliderValue, ValueChange,
    },
};
use game_core::plugins::{global_lighting::GlobalLightControls, inspector::InspectorSceneState};

use super::{consts::PANEL_FONT_SIZE, despawn_ui::despawn_ui};

#[derive(Component, Clone, Default)]
struct GlobalLightUi;

#[derive(Component, Clone, Default)]
struct ShadowMapSizeCaption;

#[derive(Component, Copy, Clone, Default)]
struct GlobalLightSlider {
    setting: GlobalLightSliderSetting,
}

#[derive(Component, Copy, Clone, Default)]
struct ShadowsEnabledCheckbox;

#[derive(Component, Copy, Clone, Default)]
struct SoftShadowEnabledCheckbox;

#[derive(Copy, Clone, Default)]
enum GlobalLightSliderSetting {
    #[default]
    AmbientBrightness,
    SunIlluminance,
    SunElevation,
    SunAzimuth,
    ShadowDepthBias,
    ShadowNormalBias,
    CascadeMinimumDistance,
    CascadeFirstFarBound,
    CascadeMaximumDistance,
    CascadeOverlapProportion,
    SoftShadowSize,
}

impl GlobalLightSliderSetting {
    const fn value(self, controls: &GlobalLightControls) -> f32 {
        match self {
            Self::AmbientBrightness => controls.ambient_brightness,
            Self::SunIlluminance => controls.sun_illuminance,
            Self::SunElevation => controls.sun_elevation_degrees,
            Self::SunAzimuth => controls.sun_azimuth_degrees,
            Self::ShadowDepthBias => controls.shadow_depth_bias,
            Self::ShadowNormalBias => controls.shadow_normal_bias,
            Self::CascadeMinimumDistance => controls.cascade_minimum_distance,
            Self::CascadeFirstFarBound => controls.cascade_first_far_bound,
            Self::CascadeMaximumDistance => controls.cascade_maximum_distance,
            Self::CascadeOverlapProportion => controls.cascade_overlap_proportion,
            Self::SoftShadowSize => controls.soft_shadow_size,
        }
    }

    fn set(self, controls: &mut GlobalLightControls, value: f32) {
        match self {
            Self::AmbientBrightness => controls.ambient_brightness = value,
            Self::SunIlluminance => controls.sun_illuminance = value,
            Self::SunElevation => controls.sun_elevation_degrees = value,
            Self::SunAzimuth => controls.sun_azimuth_degrees = value,
            Self::ShadowDepthBias => controls.shadow_depth_bias = value,
            Self::ShadowNormalBias => controls.shadow_normal_bias = value,
            Self::CascadeMinimumDistance => controls.cascade_minimum_distance = value,
            Self::CascadeFirstFarBound => controls.cascade_first_far_bound = value,
            Self::CascadeMaximumDistance => controls.cascade_maximum_distance = value,
            Self::CascadeOverlapProportion => {
                controls.cascade_overlap_proportion = value.clamp(0.0, 0.95);
            }
            Self::SoftShadowSize => {
                controls.soft_shadow_size = value.max(0.0);
            }
        }

        controls.normalize_shadow_constraints();
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorSceneState::Simple),
        global_light_ui.spawn(),
    )
    .add_systems(
        Update,
        (
            sync_global_light_sliders,
            sync_shadows_enabled_checkbox,
            sync_soft_shadow_enabled_checkbox,
            sync_shadow_map_size_caption,
        ),
    )
    .add_systems(OnExit(InspectorSceneState::Simple), despawn_ui::<GlobalLightUi>);
}

fn global_light_ui() -> impl SceneList {
    bsn_list![global_light_panel()]
}

fn global_light_panel() -> impl Scene {
    bsn! {
        (
            Name::new("Global Light UI")
            GlobalLightUi
            Node {
                position_type: PositionType::Absolute,
                bottom: px(12),
                left: px(12),
                min_width: px(330),
                max_height: px(600),
                padding: px(8),
                border: px(1),
                border_radius: px(6),
                flex_direction: FlexDirection::Column,
                row_gap: px(4),
                overflow: Overflow::scroll_y(),
            }
            TabGroup
            Pickable::IGNORE
            ThemeBackgroundColor(tokens::MENU_BG)
            InheritableFont {
                font_size: PANEL_FONT_SIZE,
            }
            Children [
                (Text("Global Light") ThemedText),
                light_slider(GlobalLightSliderSetting::AmbientBrightness, "Ambient", 0.0, 1000.0, 5.0, 1),
                light_slider(GlobalLightSliderSetting::SunIlluminance, "Sun lx", 0.0, 30000.0, 250.0, 0),
                light_slider(GlobalLightSliderSetting::SunElevation, "Elevation", -180.0, 180.0, 1.0, 1),
                light_slider(GlobalLightSliderSetting::SunAzimuth, "Azimuth", -180.0, 180.0, 1.0, 1),
                shadows_checkbox(),
                light_slider(GlobalLightSliderSetting::ShadowDepthBias, "Depth bias", 0.0, 0.2, 0.001, 4),
                light_slider(GlobalLightSliderSetting::ShadowNormalBias, "Normal bias", 0.0, 2.0, 0.01, 3),
                soft_shadow_checkbox(),
                light_slider(GlobalLightSliderSetting::SoftShadowSize, "PCSS radius", 0.0, 50.0, 0.5, 1),
                light_slider(GlobalLightSliderSetting::CascadeMinimumDistance, "Min dist", 0.0, 5.0, 0.1, 2),
                light_slider(GlobalLightSliderSetting::CascadeFirstFarBound, "First far", 1.0, 150.0, 1.0, 1),
                light_slider(GlobalLightSliderSetting::CascadeMaximumDistance, "Max dist", 1.0, 200.0, 1.0, 1),
                light_slider(GlobalLightSliderSetting::CascadeOverlapProportion, "Overlap", 0.0, 0.95, 0.01, 2),
                shadow_map_size_menu(),
            ]
        )
    }
}

fn light_slider(
    setting: GlobalLightSliderSetting,
    label: &'static str,
    min: f32,
    max: f32,
    step: f32,
    precision: i32,
) -> impl Scene {
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
                    width: px(80),
                }
            ),
            (
                @FeathersSlider {
                    @min: min,
                    @max: max,
                    @value: min,
                }
                template_value(GlobalLightSlider { setting })
                SliderStep(step)
                SliderPrecision(precision)
                on(slider_self_update)
                on(handle_light_slider_change)
            )
        ]
    }
}

fn shadows_checkbox() -> impl Scene {
    bsn! {
        (
            @FeathersCheckbox {
                @caption: bsn! { Text("Shadows") ThemedText }
            }
            InheritableFont {
                font_size: PANEL_FONT_SIZE,
            }
            ShadowsEnabledCheckbox
            Checked
            on(checkbox_self_update)
            on(handle_shadows_enabled_change)
        )
    }
}

fn soft_shadow_checkbox() -> impl Scene {
    bsn! {
        (
            @FeathersCheckbox {
                @caption: bsn! { Text("PCSS") ThemedText }
            }
            InheritableFont {
                font_size: PANEL_FONT_SIZE,
            }
            SoftShadowEnabledCheckbox
            on(checkbox_self_update)
            on(handle_soft_shadow_enabled_change)
        )
    }
}

fn shadow_map_size_menu() -> impl Scene {
    bsn! {
        (
            @FeathersMenu
            Children [
                (
                    @FeathersMenuButton {
                        @caption: bsn! {
                            Text("2048")
                            ThemedText
                            ShadowMapSizeCaption
                        }
                    }
                    AccessibleLabel("Shadow map size selector")
                    Node {
                        width: percent(100),
                    }
                ),
                (
                    @FeathersMenuPopup
                    Children [
                        shadow_map_size_item(1024),
                        shadow_map_size_item(2048),
                        shadow_map_size_item(4096),
                        shadow_map_size_item(8192),
                    ]
                )
            ]
        )
    }
}

fn shadow_map_size_item(size: usize) -> impl Scene {
    let label = match size {
        1024 => "Shadow map 1024",
        2048 => "Shadow map 2048",
        4096 => "Shadow map 4096",
        8192 => "Shadow map 8192",
        _ => "Shadow map",
    };

    bsn! {
        (
            @FeathersMenuItem {
                @caption: bsn! { Text(label) ThemedText }
            }
            InheritableFont {
                font_size: PANEL_FONT_SIZE,
            }
            on(move |_: On<'_, '_, Activate>, mut controls: ResMut<'_, GlobalLightControls>| {
                controls.shadow_map_size = size;
            })
        )
    }
}

#[allow(clippy::needless_pass_by_value)]
fn handle_light_slider_change(
    change: On<'_, '_, ValueChange<f32>>,
    sliders: Query<'_, '_, &GlobalLightSlider>,
    mut controls: ResMut<'_, GlobalLightControls>,
) {
    let Ok(slider) = sliders.get(change.source) else {
        return;
    };

    slider.setting.set(&mut controls, change.value);
}

#[allow(clippy::needless_pass_by_value)]
fn handle_shadows_enabled_change(
    change: On<'_, '_, ValueChange<bool>>,
    checkboxes: Query<'_, '_, Entity, With<ShadowsEnabledCheckbox>>,
    mut controls: ResMut<'_, GlobalLightControls>,
) {
    if checkboxes.get(change.source).is_ok() {
        controls.shadows_enabled = change.value;
    }
}

#[allow(clippy::needless_pass_by_value)]
fn handle_soft_shadow_enabled_change(
    change: On<'_, '_, ValueChange<bool>>,
    checkboxes: Query<'_, '_, Entity, With<SoftShadowEnabledCheckbox>>,
    mut controls: ResMut<'_, GlobalLightControls>,
) {
    if checkboxes.get(change.source).is_ok() {
        if change.value {
            controls.soft_shadow_size = if controls.soft_shadow_size > 0.0 {
                controls.soft_shadow_size
            } else {
                10.0
            };
        } else {
            controls.soft_shadow_size = 0.0;
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn sync_global_light_sliders(
    mut commands: Commands<'_, '_>,
    controls: Res<'_, GlobalLightControls>,
    sliders: Query<'_, '_, (Entity, &GlobalLightSlider, &SliderValue)>,
) {
    for (entity, slider, value) in &sliders {
        let expected = slider.setting.value(&controls);
        if (value.0 - expected).abs() > 0.001 {
            commands.entity(entity).insert(SliderValue(expected));
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn sync_shadows_enabled_checkbox(
    mut commands: Commands<'_, '_>,
    controls: Res<'_, GlobalLightControls>,
    checkboxes: Query<'_, '_, (Entity, Has<Checked>), With<ShadowsEnabledCheckbox>>,
) {
    for (entity, checked) in &checkboxes {
        if controls.shadows_enabled && !checked {
            commands.entity(entity).insert(Checked);
        } else if !controls.shadows_enabled && checked {
            commands.entity(entity).remove::<Checked>();
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn sync_soft_shadow_enabled_checkbox(
    mut commands: Commands<'_, '_>,
    controls: Res<'_, GlobalLightControls>,
    checkboxes: Query<'_, '_, (Entity, Has<Checked>), With<SoftShadowEnabledCheckbox>>,
) {
    let enabled = controls.soft_shadow_size > 0.0;
    for (entity, checked) in &checkboxes {
        if enabled && !checked {
            commands.entity(entity).insert(Checked);
        } else if !enabled && checked {
            commands.entity(entity).remove::<Checked>();
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn sync_shadow_map_size_caption(
    controls: Res<'_, GlobalLightControls>,
    mut caption: Single<'_, '_, &mut Text, With<ShadowMapSizeCaption>>,
) {
    if controls.is_changed() {
        caption.0 = controls.shadow_map_size.to_string();
    }
}
