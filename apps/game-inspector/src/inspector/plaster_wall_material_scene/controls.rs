//! Parameter controls for plaster wall material generation.

use bevy::{
    feathers::{
        controls::{FeathersButton, FeathersSlider},
        font_styles::InheritableFont,
        theme::{ThemeBackgroundColor, ThemedText},
        tokens,
    },
    input_focus::tab_navigation::TabGroup,
    prelude::*,
    ui_widgets::{
        Activate, SliderPrecision, SliderStep, SliderValue, ValueChange, slider_self_update,
    },
};
use game_core::plugins::inspector::{
    InspectorSceneState, PlasterWallGenerationRequest, PlasterWallMaterialControls,
};

use super::super::{consts::PANEL_FONT_SIZE, despawn_ui::despawn_ui};

#[derive(Component, Clone, Default)]
struct PlasterWallControlsUi;

#[derive(Component, Clone, Default)]
struct PlasterWallSlider {
    setting: PlasterWallSliderSetting,
}

#[derive(Clone, Default)]
enum PlasterWallSliderSetting {
    #[default]
    Seed,
    NormalStrength,
    BroadAmp,
    FineAmp,
    PatchesAmp,
    PitsAmp,
    HairAmp,
    RoughBase,
    AoBase,
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    reason = "seed is edited through a float slider and clamped to a small integer range"
)]
impl PlasterWallSliderSetting {
    const fn value(&self, controls: &PlasterWallMaterialControls) -> f32 {
        match self {
            Self::Seed => controls.params.seed as f32,
            Self::NormalStrength => controls.params.normal.strength,
            Self::BroadAmp => controls.params.height.broad_amp,
            Self::FineAmp => controls.params.height.fine_amp,
            Self::PatchesAmp => controls.params.height.patches_amp,
            Self::PitsAmp => controls.params.height.pits_amp,
            Self::HairAmp => controls.params.height.hair_amp,
            Self::RoughBase => controls.params.orm.rough_base,
            Self::AoBase => controls.params.orm.ao_base,
        }
    }

    #[allow(
        clippy::missing_const_for_fn,
        reason = "kept non-const to match other UI setting mutators"
    )]
    fn set(&self, controls: &mut PlasterWallMaterialControls, value: f32) {
        match self {
            Self::Seed => controls.params.seed = value.round().clamp(0.0, 9999.0) as u32,
            Self::NormalStrength => controls.params.normal.strength = value.clamp(0.0, 2.0),
            Self::BroadAmp => controls.params.height.broad_amp = value.clamp(0.0, 1.0),
            Self::FineAmp => controls.params.height.fine_amp = value.clamp(0.0, 1.0),
            Self::PatchesAmp => controls.params.height.patches_amp = value.clamp(0.0, 1.0),
            Self::PitsAmp => controls.params.height.pits_amp = value.clamp(0.0, 0.5),
            Self::HairAmp => controls.params.height.hair_amp = value.clamp(0.0, 0.5),
            Self::RoughBase => controls.params.orm.rough_base = value.clamp(0.0, 1.0),
            Self::AoBase => controls.params.orm.ao_base = value.clamp(0.0, 1.0),
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorSceneState::PlasterWallMaterial),
        plaster_wall_controls_ui.spawn(),
    )
    .add_systems(
        Update,
        sync_plaster_wall_sliders.run_if(in_state(InspectorSceneState::PlasterWallMaterial)),
    )
    .add_systems(
        OnExit(InspectorSceneState::PlasterWallMaterial),
        despawn_ui::<PlasterWallControlsUi>,
    );
}

fn plaster_wall_controls_ui() -> impl SceneList {
    bsn_list![controls_panel()]
}

fn controls_panel() -> impl Scene {
    bsn! {
        (
            Name::new("Plaster Wall Controls UI")
            PlasterWallControlsUi
            Node {
                position_type: PositionType::Absolute,
                bottom: px(12),
                left: px(12),
                min_width: px(330),
                max_height: px(620),
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
                (Text("Plaster Params") ThemedText),
                plaster_slider(PlasterWallSliderSetting::Seed, "Seed", 0.0, 9999.0, 1.0, 0),
                plaster_slider(PlasterWallSliderSetting::NormalStrength, "Normal", 0.0, 2.0, 0.01, 2),
                plaster_slider(PlasterWallSliderSetting::BroadAmp, "Broad amp", 0.0, 1.0, 0.01, 2),
                plaster_slider(PlasterWallSliderSetting::FineAmp, "Fine amp", 0.0, 1.0, 0.01, 2),
                plaster_slider(PlasterWallSliderSetting::PatchesAmp, "Patch amp", 0.0, 1.0, 0.01, 2),
                plaster_slider(PlasterWallSliderSetting::PitsAmp, "Pits amp", 0.0, 0.5, 0.01, 2),
                plaster_slider(PlasterWallSliderSetting::HairAmp, "Hair amp", 0.0, 0.5, 0.01, 2),
                plaster_slider(PlasterWallSliderSetting::RoughBase, "Rough base", 0.0, 1.0, 0.01, 2),
                plaster_slider(PlasterWallSliderSetting::AoBase, "AO base", 0.0, 1.0, 0.01, 2),
                command_buttons(),
            ]
        )
    }
}

fn plaster_slider(
    setting: PlasterWallSliderSetting,
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
                template_value(PlasterWallSlider { setting })
                SliderStep(step)
                SliderPrecision(precision)
                on(slider_self_update)
                on(
                    move |
                        change: On<'_, '_, ValueChange<f32>>,
                        mut controls: ResMut<'_, PlasterWallMaterialControls>,
                    | {
                        handler_setting.set(&mut controls, change.value);
                    }
                )
            )
        ]
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
    controls: Res<'_, PlasterWallMaterialControls>,
) {
    commands.insert_resource(PlasterWallGenerationRequest {
        params: controls.params.clone(),
    });
}

#[allow(clippy::needless_pass_by_value)]
fn handle_reset(
    _: On<'_, '_, Activate>,
    mut commands: Commands<'_, '_>,
    mut controls: ResMut<'_, PlasterWallMaterialControls>,
) {
    *controls = PlasterWallMaterialControls::default();
    commands.insert_resource(PlasterWallGenerationRequest {
        params: controls.params.clone(),
    });
}

#[allow(clippy::needless_pass_by_value)]
fn sync_plaster_wall_sliders(
    mut commands: Commands<'_, '_>,
    controls: Res<'_, PlasterWallMaterialControls>,
    sliders: Query<'_, '_, (Entity, &PlasterWallSlider, &SliderValue)>,
) {
    for (entity, slider, value) in &sliders {
        let expected = slider.setting.value(&controls);
        if (value.0 - expected).abs() > 0.001 {
            commands.entity(entity).insert(SliderValue(expected));
        }
    }
}
