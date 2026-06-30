//! `StandardMaterial` controls for the concrete wall material preview.

use bevy::{
    feathers::{
        controls::{FeathersCheckbox, FeathersSlider},
        font_styles::InheritableFont,
        theme::{ThemeBackgroundColor, ThemedText},
        tokens,
    },
    input_focus::tab_navigation::TabGroup,
    prelude::*,
    ui::Checked,
    ui_widgets::{
        SliderPrecision, SliderStep, SliderValue, ValueChange, checkbox_self_update,
        slider_self_update,
    },
};
use game_core::plugins::inspector::{ConcreteWallMaterialSettings, InspectorSceneState};

use super::super::{consts::PANEL_FONT_SIZE, despawn_ui::despawn_ui};

#[derive(Component, Clone, Default)]
struct ConcreteWallMaterialSettingsUi;

#[derive(Component, Clone, Default)]
struct MaterialSettingSlider {
    setting: MaterialSliderSetting,
}

#[derive(Component, Clone, Default)]
struct MaterialSettingCheckbox {
    setting: MaterialCheckboxSetting,
}

#[derive(Clone, Default)]
enum MaterialSliderSetting {
    #[default]
    TintR,
    TintG,
    TintB,
    Roughness,
    Metallic,
    Reflectance,
}

#[derive(Clone, Default)]
enum MaterialCheckboxSetting {
    #[default]
    DoubleSided,
    CullNone,
    Unlit,
}

impl MaterialSliderSetting {
    const fn value(&self, settings: &ConcreteWallMaterialSettings) -> f32 {
        match self {
            Self::TintR => settings.tint_r,
            Self::TintG => settings.tint_g,
            Self::TintB => settings.tint_b,
            Self::Roughness => settings.perceptual_roughness,
            Self::Metallic => settings.metallic,
            Self::Reflectance => settings.reflectance,
        }
    }

    #[allow(
        clippy::missing_const_for_fn,
        reason = "kept non-const to match other UI setting mutators"
    )]
    fn set(&self, settings: &mut ConcreteWallMaterialSettings, value: f32) {
        match self {
            Self::TintR => settings.tint_r = value.clamp(0.0, 2.0),
            Self::TintG => settings.tint_g = value.clamp(0.0, 2.0),
            Self::TintB => settings.tint_b = value.clamp(0.0, 2.0),
            Self::Roughness => settings.perceptual_roughness = value.clamp(0.0, 1.0),
            Self::Metallic => settings.metallic = value.clamp(0.0, 1.0),
            Self::Reflectance => settings.reflectance = value.clamp(0.0, 1.0),
        }
    }
}

impl MaterialCheckboxSetting {
    const fn value(&self, settings: &ConcreteWallMaterialSettings) -> bool {
        match self {
            Self::DoubleSided => settings.double_sided,
            Self::CullNone => settings.cull_none,
            Self::Unlit => settings.unlit,
        }
    }

    #[allow(
        clippy::missing_const_for_fn,
        reason = "kept non-const to match other UI setting mutators"
    )]
    fn set(&self, settings: &mut ConcreteWallMaterialSettings, value: bool) {
        match self {
            Self::DoubleSided => settings.double_sided = value,
            Self::CullNone => settings.cull_none = value,
            Self::Unlit => settings.unlit = value,
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorSceneState::ConcreteWallMaterial),
        material_settings_ui.spawn(),
    )
    .add_systems(
        Update,
        (sync_material_sliders, sync_material_checkboxes)
            .run_if(in_state(InspectorSceneState::ConcreteWallMaterial)),
    )
    .add_systems(
        OnExit(InspectorSceneState::ConcreteWallMaterial),
        despawn_ui::<ConcreteWallMaterialSettingsUi>,
    );
}

fn material_settings_ui() -> impl SceneList {
    bsn_list![settings_panel()]
}

fn settings_panel() -> impl Scene {
    bsn! {
        (
            Name::new("Concrete Wall Material Settings UI")
            ConcreteWallMaterialSettingsUi
            Node {
                position_type: PositionType::Absolute,
                bottom: px(12),
                right: px(12),
                min_width: px(330),
                max_height: px(300),
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
                (Text("Material Settings") ThemedText),
                material_slider(MaterialSliderSetting::TintR, "Tint R", 0.0, 2.0, 0.01, 2),
                material_slider(MaterialSliderSetting::TintG, "Tint G", 0.0, 2.0, 0.01, 2),
                material_slider(MaterialSliderSetting::TintB, "Tint B", 0.0, 2.0, 0.01, 2),
                material_slider(MaterialSliderSetting::Roughness, "Roughness", 0.0, 1.0, 0.01, 2),
                material_slider(MaterialSliderSetting::Metallic, "Metallic", 0.0, 1.0, 0.01, 2),
                material_slider(MaterialSliderSetting::Reflectance, "Reflect", 0.0, 1.0, 0.01, 2),
                material_checkbox(MaterialCheckboxSetting::DoubleSided, "Double sided"),
                material_checkbox(MaterialCheckboxSetting::CullNone, "Cull none"),
                material_checkbox(MaterialCheckboxSetting::Unlit, "Unlit"),
            ]
        )
    }
}

fn material_slider(
    setting: MaterialSliderSetting,
    label: &'static str,
    min: f32,
    max: f32,
    step: f32,
    precision: i32,
) -> impl Scene {
    let handler_setting = setting.clone();

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
                    width: px(88),
                }
            ),
            (
                @FeathersSlider {
                    @min: min,
                    @max: max,
                    @value: min,
                }
                template_value(MaterialSettingSlider { setting })
                SliderStep(step)
                SliderPrecision(precision)
                on(slider_self_update)
                on(
                    move |
                        change: On<'_, '_, ValueChange<f32>>,
                        mut settings: ResMut<'_, ConcreteWallMaterialSettings>,
                    | {
                        handler_setting.set(&mut settings, change.value);
                    }
                )
            )
        ]
    }
}

fn material_checkbox(setting: MaterialCheckboxSetting, label: &'static str) -> impl Scene {
    let handler_setting = setting.clone();

    bsn! {
        (
            @FeathersCheckbox {
                @caption: bsn! { Text(label) ThemedText }
            }
            InheritableFont {
                font_size: PANEL_FONT_SIZE,
            }
            template_value(MaterialSettingCheckbox { setting })
            on(checkbox_self_update)
            on(
                move |
                    change: On<'_, '_, ValueChange<bool>>,
                    mut settings: ResMut<'_, ConcreteWallMaterialSettings>,
                | {
                    handler_setting.set(&mut settings, change.value);
                }
            )
        )
    }
}

#[allow(clippy::needless_pass_by_value)]
fn sync_material_sliders(
    mut commands: Commands<'_, '_>,
    settings: Res<'_, ConcreteWallMaterialSettings>,
    sliders: Query<'_, '_, (Entity, &MaterialSettingSlider, &SliderValue)>,
) {
    for (entity, slider, value) in &sliders {
        let expected = slider.setting.value(&settings);
        if (value.0 - expected).abs() > 0.001 {
            commands.entity(entity).insert(SliderValue(expected));
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn sync_material_checkboxes(
    mut commands: Commands<'_, '_>,
    settings: Res<'_, ConcreteWallMaterialSettings>,
    checkboxes: Query<'_, '_, (Entity, &MaterialSettingCheckbox, Has<Checked>)>,
) {
    for (entity, checkbox, checked) in &checkboxes {
        let expected = checkbox.setting.value(&settings);
        if expected && !checked {
            commands.entity(entity).insert(Checked);
        } else if !expected && checked {
            commands.entity(entity).remove::<Checked>();
        }
    }
}
