//! Parameter controls for concrete wall material generation.

use bevy::{
    feathers::{
        controls::{FeathersButton, FeathersCheckbox, FeathersSlider},
        font_styles::InheritableFont,
        theme::{ThemeBackgroundColor, ThemedText},
        tokens,
    },
    input_focus::tab_navigation::TabGroup,
    prelude::*,
    ui::Checked,
    ui_widgets::{
        Activate, SliderPrecision, SliderStep, SliderValue, ValueChange, checkbox_self_update,
        slider_self_update,
    },
};
use game_core::plugins::inspector::{
    ConcreteWallDirtSettings, ConcreteWallGenerationRequest, ConcreteWallMaterialControls,
    ConcreteWallUvSettings, InspectorSceneState,
};
use num_traits::ToPrimitive;
use texture_gen::ConcreteParamsSlider;

use super::super::{consts::PANEL_FONT_SIZE, despawn_ui::despawn_ui};

#[derive(Component, Clone, Default)]
struct ConcreteWallControlsUi;

#[derive(Component, Clone, Default)]
struct ConcreteWallSlider {
    setting: ConcreteParamsSlider,
}

#[derive(Component, Clone, Default)]
struct DirtSlider {
    setting: DirtSliderSetting,
}

#[derive(Component, Clone, Default)]
struct UvSlider {
    setting: UvSliderSetting,
}

#[derive(Component, Clone, Default)]
struct PerFaceUvCheckbox;

#[derive(Clone, Default)]
enum DirtSliderSetting {
    #[default]
    FloorDirt,
    CornerDirt,
    ColorR,
    ColorG,
    ColorB,
}

#[derive(Clone, Default)]
enum UvSliderSetting {
    #[default]
    TilesPerMeter,
    FaceColumns,
    FaceRows,
}

impl DirtSliderSetting {
    const fn value(&self, settings: &ConcreteWallDirtSettings) -> f32 {
        match self {
            Self::FloorDirt => settings.floor_strength,
            Self::CornerDirt => settings.corner_strength,
            Self::ColorR => settings.color_r,
            Self::ColorG => settings.color_g,
            Self::ColorB => settings.color_b,
        }
    }

    const fn set(&self, settings: &mut ConcreteWallDirtSettings, value: f32) {
        match self {
            Self::FloorDirt => settings.floor_strength = value.clamp(0.0, 1.5),
            Self::CornerDirt => settings.corner_strength = value.clamp(0.0, 1.5),
            Self::ColorR => settings.color_r = value.clamp(0.0, 1.0),
            Self::ColorG => settings.color_g = value.clamp(0.0, 1.0),
            Self::ColorB => settings.color_b = value.clamp(0.0, 1.0),
        }
    }
}

impl UvSliderSetting {
    fn value(&self, settings: &ConcreteWallUvSettings) -> f32 {
        match self {
            Self::TilesPerMeter => settings.tiles_per_meter,
            Self::FaceColumns => settings.face_columns.to_f32().unwrap_or(1.0),
            Self::FaceRows => settings.face_rows.to_f32().unwrap_or(1.0),
        }
    }

    fn set(&self, settings: &mut ConcreteWallUvSettings, value: f32) {
        match self {
            Self::TilesPerMeter => settings.tiles_per_meter = value.clamp(0.05, 1.5),
            Self::FaceColumns => {
                settings.face_columns = value.round().clamp(1.0, 48.0).to_u32().unwrap_or(1);
            }
            Self::FaceRows => {
                settings.face_rows = value.round().clamp(1.0, 32.0).to_u32().unwrap_or(1);
            }
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorSceneState::ConcreteWallMaterial),
        concrete_wall_controls_ui.spawn(),
    )
    .add_systems(
        Update,
        (
            sync_concrete_wall_sliders,
            sync_dirt_sliders,
            sync_uv_sliders,
            sync_uv_checkboxes,
        )
            .run_if(in_state(InspectorSceneState::ConcreteWallMaterial)),
    )
    .add_systems(
        OnExit(InspectorSceneState::ConcreteWallMaterial),
        despawn_ui::<ConcreteWallControlsUi>,
    );
}

fn concrete_wall_controls_ui() -> impl SceneList {
    bsn_list![controls_panel()]
}

fn controls_panel() -> impl Scene {
    bsn! {
        (
            Name::new("Concrete Wall Controls UI")
            ConcreteWallControlsUi
            Node {
                position_type: PositionType::Absolute,
                bottom: px(12),
                left: px(12),
                min_width: px(330),
                max_height: px(980),
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
                (Text("Concrete Params") ThemedText),
                concrete_slider(ConcreteParamsSlider::Seed, "Seed"),
                concrete_slider(ConcreteParamsSlider::ToneVariation, "Tone variation"),
                concrete_slider(ConcreteParamsSlider::LimeCloudStrength, "Lime clouds"),
                concrete_slider(ConcreteParamsSlider::GrainHeight, "Grain height"),
                concrete_slider(ConcreteParamsSlider::AggregateCount, "Aggregate count"),
                concrete_slider(ConcreteParamsSlider::AggregateContrast, "Aggregate contrast"),
                concrete_slider(ConcreteParamsSlider::AggregateHeight, "Aggregate height"),
                concrete_slider(ConcreteParamsSlider::ExposedAggregateCount, "Exposed agg"),
                concrete_slider(ConcreteParamsSlider::ExposedAggregateHeight, "Exp height"),
                concrete_slider(ConcreteParamsSlider::VoidCount, "Void count"),
                concrete_slider(ConcreteParamsSlider::VoidDepth, "Void depth"),
                concrete_slider(ConcreteParamsSlider::StainCount, "Stain count"),
                concrete_slider(ConcreteParamsSlider::StainDarkening, "Stain dark"),
                concrete_slider(ConcreteParamsSlider::FormworkStrength, "Formwork"),
                concrete_slider(ConcreteParamsSlider::EfflorescenceStrength, "Efflorescence"),
                concrete_slider(ConcreteParamsSlider::CrackCount, "Crack count"),
                concrete_slider(ConcreteParamsSlider::CrackDepth, "Crack depth"),
                concrete_slider(ConcreteParamsSlider::NormalStrength, "Normal"),
                concrete_slider(ConcreteParamsSlider::RoughBase, "Rough base"),
                concrete_slider(ConcreteParamsSlider::AoBase, "AO base"),
                (Text("Dirt") ThemedText),
                dirt_slider(DirtSliderSetting::FloorDirt, "Floor dirt", 0.0, 1.5, 0.01, 2),
                dirt_slider(DirtSliderSetting::CornerDirt, "Corner dirt", 0.0, 1.5, 0.01, 2),
                dirt_slider(DirtSliderSetting::ColorR, "Dirt R", 0.0, 1.0, 0.01, 2),
                dirt_slider(DirtSliderSetting::ColorG, "Dirt G", 0.0, 1.0, 0.01, 2),
                dirt_slider(DirtSliderSetting::ColorB, "Dirt B", 0.0, 1.0, 0.01, 2),
                (Text("UV") ThemedText),
                uv_slider(UvSliderSetting::TilesPerMeter, "UV scale", 0.05, 1.5, 0.01, 2),
                uv_slider(UvSliderSetting::FaceColumns, "Columns", 1.0, 48.0, 1.0, 0),
                uv_slider(UvSliderSetting::FaceRows, "Rows", 1.0, 32.0, 1.0, 0),
                uv_checkbox(),
                command_buttons(),
            ]
        )
    }
}

fn concrete_slider(setting: ConcreteParamsSlider, label: &'static str) -> impl Scene {
    let handler_setting = setting.clone();
    let min = setting.min();
    let max = setting.max();
    let step = setting.step();
    let precision = setting.precision();

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
                template_value(ConcreteWallSlider { setting })
                SliderStep(step)
                SliderPrecision(precision)
                on(slider_self_update)
                on(
                    move |
                        change: On<'_, '_, ValueChange<f32>>,
                        mut controls: ResMut<'_, ConcreteWallMaterialControls>,
                    | {
                        handler_setting.set(&mut controls.params, change.value);
                    }
                )
            )
        ]
    }
}

fn dirt_slider(
    setting: DirtSliderSetting,
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
                template_value(DirtSlider { setting })
                SliderStep(step)
                SliderPrecision(precision)
                on(slider_self_update)
                on(
                    move |
                        change: On<'_, '_, ValueChange<f32>>,
                        mut dirt_settings: ResMut<'_, ConcreteWallDirtSettings>,
                    | {
                        handler_setting.set(&mut dirt_settings, change.value);
                    }
                )
            )
        ]
    }
}

fn uv_slider(
    setting: UvSliderSetting,
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
                template_value(UvSlider { setting })
                SliderStep(step)
                SliderPrecision(precision)
                on(slider_self_update)
                on(
                    move |
                        change: On<'_, '_, ValueChange<f32>>,
                        mut uv_settings: ResMut<'_, ConcreteWallUvSettings>,
                    | {
                        handler_setting.set(&mut uv_settings, change.value);
                    }
                )
            )
        ]
    }
}

fn uv_checkbox() -> impl Scene {
    bsn! {
        (
            @FeathersCheckbox {
                @caption: bsn! { Text("Per-face UV") ThemedText }
            }
            InheritableFont {
                font_size: PANEL_FONT_SIZE,
            }
            PerFaceUvCheckbox
            on(checkbox_self_update)
            on(
                |
                    change: On<'_, '_, ValueChange<bool>>,
                    mut uv_settings: ResMut<'_, ConcreteWallUvSettings>,
                | {
                    uv_settings.per_face_offset = change.value;
                }
            )
        )
    }
}

fn command_buttons() -> impl Scene {
    bsn! {
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            column_gap: px(6),
            padding: UiRect::top(px(4)),
        }
        Children [
            (
                @FeathersButton
                Node {
                    flex_grow: 1.0,
                    justify_content: JustifyContent::Center,
                }
                on(handle_apply)
                Children [ (Text("Apply") ThemedText) ]
            ),
            (
                @FeathersButton
                Node {
                    flex_grow: 1.0,
                    justify_content: JustifyContent::Center,
                }
                on(handle_reset)
                Children [ (Text("Reset") ThemedText) ]
            )
        ]
    }
}

#[allow(clippy::needless_pass_by_value)]
fn handle_apply(
    _: On<'_, '_, Activate>,
    mut commands: Commands<'_, '_>,
    controls: Res<'_, ConcreteWallMaterialControls>,
) {
    commands.insert_resource(ConcreteWallGenerationRequest {
        params: controls.params.clone(),
    });
}

#[allow(clippy::needless_pass_by_value)]
fn handle_reset(
    _: On<'_, '_, Activate>,
    mut commands: Commands<'_, '_>,
    mut controls: ResMut<'_, ConcreteWallMaterialControls>,
    mut dirt_settings: ResMut<'_, ConcreteWallDirtSettings>,
    mut uv_settings: ResMut<'_, ConcreteWallUvSettings>,
) {
    *controls = ConcreteWallMaterialControls::default();
    *dirt_settings = ConcreteWallDirtSettings::default();
    *uv_settings = ConcreteWallUvSettings::default();
    commands.insert_resource(ConcreteWallGenerationRequest {
        params: controls.params.clone(),
    });
}

#[allow(clippy::needless_pass_by_value)]
fn sync_concrete_wall_sliders(
    mut commands: Commands<'_, '_>,
    controls: Res<'_, ConcreteWallMaterialControls>,
    sliders: Query<'_, '_, (Entity, &ConcreteWallSlider, &SliderValue)>,
) {
    for (entity, slider, value) in &sliders {
        let expected = slider.setting.value(&controls.params);
        if (value.0 - expected).abs() > 0.001 {
            commands.entity(entity).insert(SliderValue(expected));
        }
    }
}

fn sync_uv_checkboxes(
    mut commands: Commands<'_, '_>,
    uv_settings: Res<'_, ConcreteWallUvSettings>,
    checkboxes: Query<'_, '_, (Entity, Has<Checked>), With<PerFaceUvCheckbox>>,
) {
    let uv_settings = uv_settings.into_inner();
    for (entity, checked) in &checkboxes {
        if uv_settings.per_face_offset && !checked {
            commands.entity(entity).insert(Checked);
        } else if !uv_settings.per_face_offset && checked {
            commands.entity(entity).remove::<Checked>();
        }
    }
}

fn sync_uv_sliders(
    mut commands: Commands<'_, '_>,
    uv_settings: Res<'_, ConcreteWallUvSettings>,
    sliders: Query<'_, '_, (Entity, &UvSlider, &SliderValue)>,
) {
    let uv_settings = uv_settings.into_inner();
    for (entity, slider, value) in &sliders {
        let expected = slider.setting.value(uv_settings);
        if (value.0 - expected).abs() > 0.001 {
            commands.entity(entity).insert(SliderValue(expected));
        }
    }
}

fn sync_dirt_sliders(
    mut commands: Commands<'_, '_>,
    dirt_settings: Res<'_, ConcreteWallDirtSettings>,
    sliders: Query<'_, '_, (Entity, &DirtSlider, &SliderValue)>,
) {
    let dirt_settings = dirt_settings.into_inner();
    for (entity, slider, value) in &sliders {
        let expected = slider.setting.value(dirt_settings);
        if (value.0 - expected).abs() > 0.001 {
            commands.entity(entity).insert(SliderValue(expected));
        }
    }
}
