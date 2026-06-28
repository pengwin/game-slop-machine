//! Camera effects controls for inspector scenes.

use bevy::{
    feathers::{
        controls::FeathersCheckbox,
        theme::{ThemeBackgroundColor, ThemedText},
        tokens,
    },
    input_focus::tab_navigation::TabGroup,
    prelude::*,
    ui::Checked,
    ui_widgets::{checkbox_self_update, ValueChange},
};
use game_core::plugins::{global_camera::CameraEffects, inspector::InspectorSceneState};

#[derive(Component, Clone, Default)]
struct CameraEffectsUi;

#[derive(Component, Copy, Clone, Default)]
struct CameraEffectCheckbox {
    effect: CameraEffect,
}

#[derive(Copy, Clone, Default)]
enum CameraEffect {
    #[default]
    MsaaOff,
    Hdr,
    TonemappingAcesFitted,
    DepthPrepass,
    NormalPrepass,
    MotionVectorPrepass,
    ScreenSpaceAmbientOcclusion,
    TemporalJitter,
    TemporalAntiAliasing,
}

impl CameraEffect {
    const fn enabled(self, effects: &CameraEffects) -> bool {
        match self {
            Self::MsaaOff => effects.msaa_off,
            Self::Hdr => effects.hdr,
            Self::TonemappingAcesFitted => effects.tonemapping_aces_fitted,
            Self::DepthPrepass => effects.depth_prepass,
            Self::NormalPrepass => effects.normal_prepass,
            Self::MotionVectorPrepass => effects.motion_vector_prepass,
            Self::ScreenSpaceAmbientOcclusion => effects.screen_space_ambient_occlusion,
            Self::TemporalJitter => effects.temporal_jitter,
            Self::TemporalAntiAliasing => effects.temporal_anti_aliasing,
        }
    }

    const fn set(self, effects: &mut CameraEffects, value: bool) {
        match self {
            Self::MsaaOff => effects.msaa_off = value,
            Self::Hdr => effects.hdr = value,
            Self::TonemappingAcesFitted => effects.tonemapping_aces_fitted = value,
            Self::DepthPrepass => effects.depth_prepass = value,
            Self::NormalPrepass => effects.normal_prepass = value,
            Self::MotionVectorPrepass => effects.motion_vector_prepass = value,
            Self::ScreenSpaceAmbientOcclusion => effects.screen_space_ambient_occlusion = value,
            Self::TemporalJitter => effects.temporal_jitter = value,
            Self::TemporalAntiAliasing => effects.temporal_anti_aliasing = value,
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorSceneState::Simple),
        camera_effects_ui.spawn(),
    )
    .add_systems(Update, sync_camera_effect_checkboxes)
    .add_systems(
        OnExit(InspectorSceneState::Simple),
        despawn_camera_effects_ui,
    );
}

fn camera_effects_ui() -> impl SceneList {
    bsn_list![camera_effects_panel()]
}

fn camera_effects_panel() -> impl Scene {
    bsn! {
        (
            Name::new("Camera Effects UI")
            CameraEffectsUi
            Node {
                position_type: PositionType::Absolute,
                top: px(112),
                left: px(12),
                min_width: px(330),
                padding: px(10),
                border: px(1),
                border_radius: px(6),
                flex_direction: FlexDirection::Column,
                row_gap: px(8),
            }
            TabGroup
            Pickable::IGNORE
            ThemeBackgroundColor(tokens::MENU_BG)
            Children [
                (Text("Camera Effects") ThemedText),
                effect_checkbox(CameraEffect::MsaaOff, "Msaa::Off"),
                effect_checkbox(CameraEffect::Hdr, "Hdr"),
                effect_checkbox(CameraEffect::TonemappingAcesFitted, "Tonemapping::AcesFitted"),
                effect_checkbox(CameraEffect::DepthPrepass, "DepthPrepass"),
                effect_checkbox(CameraEffect::NormalPrepass, "NormalPrepass"),
                effect_checkbox(CameraEffect::MotionVectorPrepass, "MotionVectorPrepass"),
                effect_checkbox(CameraEffect::ScreenSpaceAmbientOcclusion, "ScreenSpaceAmbientOcclusion"),
                effect_checkbox(CameraEffect::TemporalJitter, "TemporalJitter"),
                effect_checkbox(CameraEffect::TemporalAntiAliasing, "TemporalAntiAliasing"),
            ]
        )
    }
}

fn effect_checkbox(effect: CameraEffect, label: &'static str) -> impl Scene {
    bsn! {
        (
            @FeathersCheckbox {
                @caption: bsn! { Text(label) ThemedText }
            }
            template_value(CameraEffectCheckbox { effect })
            Checked
            on(checkbox_self_update)
            on(handle_camera_effect_change)
        )
    }
}

#[allow(clippy::needless_pass_by_value)]
fn handle_camera_effect_change(
    change: On<'_, '_, ValueChange<bool>>,
    checkboxes: Query<'_, '_, &CameraEffectCheckbox>,
    mut effects: ResMut<'_, CameraEffects>,
) {
    let Ok(checkbox) = checkboxes.get(change.source) else {
        return;
    };

    checkbox.effect.set(&mut effects, change.value);
}

#[allow(clippy::needless_pass_by_value)]
fn sync_camera_effect_checkboxes(
    mut commands: Commands<'_, '_>,
    effects: Res<'_, CameraEffects>,
    checkboxes: Query<'_, '_, (Entity, &CameraEffectCheckbox, Has<Checked>)>,
) {
    for (entity, checkbox, checked) in &checkboxes {
        let enabled = checkbox.effect.enabled(&effects);
        if enabled && !checked {
            commands.entity(entity).insert(Checked);
        } else if !enabled && checked {
            commands.entity(entity).remove::<Checked>();
        }
    }
}

fn despawn_camera_effects_ui(
    mut commands: Commands<'_, '_>,
    ui: Query<'_, '_, Entity, With<CameraEffectsUi>>,
) {
    for entity in &ui {
        commands.entity(entity).despawn_children().despawn();
    }
}
