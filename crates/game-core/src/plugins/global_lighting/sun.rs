use bevy::{
    light::{CascadeShadowConfig, CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
};

use super::{GlobalLightControls, LightingPreset, SceneLightingSettings};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_global_sun).add_systems(
        Update,
        (
            sync_light_controls_from_preset.run_if(resource_changed::<LightingPreset>),
            apply_light_controls.run_if(resource_changed::<GlobalLightControls>),
        )
            .chain(),
    );
}

/// Marker for the single global directional sun entity.
#[derive(Component, Clone, Default)]
pub struct GlobalSunLight;

#[allow(clippy::needless_pass_by_value)]
fn spawn_global_sun(
    mut commands: Commands<'_, '_>,
    preset: Res<'_, LightingPreset>,
    mut controls: ResMut<'_, GlobalLightControls>,
    shadow_map: Res<'_, DirectionalLightShadowMap>,
    mut ambient_light: ResMut<'_, GlobalAmbientLight>,
) {
    let lighting = preset.settings();
    *controls = GlobalLightControls::from_settings(&lighting, shadow_map.size);
    controls.normalize_shadow_constraints();
    apply_ambient_lighting(&controls, &lighting, &mut ambient_light);

    commands.queue_spawn_scene(sun_scene(&controls));
}

#[allow(clippy::needless_pass_by_value)]
fn sync_light_controls_from_preset(
    preset: Res<'_, LightingPreset>,
    mut controls: ResMut<'_, GlobalLightControls>,
    shadow_map: Res<'_, DirectionalLightShadowMap>,
) {
    let lighting = preset.settings();
    *controls = GlobalLightControls::from_settings(&lighting, shadow_map.size);
    controls.normalize_shadow_constraints();
}

#[allow(clippy::needless_pass_by_value)]
fn apply_light_controls(
    controls: Res<'_, GlobalLightControls>,
    mut ambient_light: ResMut<'_, GlobalAmbientLight>,
    mut shadow_map: ResMut<'_, DirectionalLightShadowMap>,
    mut sun: Query<
        '_,
        '_,
        (
            &mut DirectionalLight,
            &mut Transform,
            &mut CascadeShadowConfig,
        ),
        With<GlobalSunLight>,
    >,
) {
    let Ok((mut sun, mut transform, mut shadow_cascades)) = sun.single_mut() else {
        warn!("GlobalSunLight is missing");
        return;
    };

    ambient_light.brightness = controls.ambient_brightness;
    sun.illuminance = controls.sun_illuminance;
    sun.shadow_maps_enabled = controls.shadows_enabled;
    sun.shadow_depth_bias = controls.shadow_depth_bias;
    sun.shadow_normal_bias = controls.shadow_normal_bias;
    transform.rotation = controls.sun_rotation();
    let mut controls = controls.clone();
    controls.normalize_shadow_constraints();

    *shadow_cascades = cascade_shadow_config(&controls);
    shadow_map.size = controls.shadow_map_size;

    info!("Applied GlobalLightControls");
}

const fn apply_ambient_lighting(
    controls: &GlobalLightControls,
    lighting: &SceneLightingSettings,
    ambient_light: &mut GlobalAmbientLight,
) {
    ambient_light.color = lighting.ambient_color;
    ambient_light.brightness = controls.ambient_brightness;
    ambient_light.affects_lightmapped_meshes = lighting.ambient_affects_lightmapped_meshes;
}

fn sun_scene(controls: &GlobalLightControls) -> impl Scene {
    let illuminance = controls.sun_illuminance;
    let shadows_enabled = controls.shadows_enabled;
    let shadow_depth_bias = controls.shadow_depth_bias;
    let shadow_normal_bias = controls.shadow_normal_bias;
    let transform = Transform::from_rotation(controls.sun_rotation());
    let shadow_cascades = cascade_shadow_config(controls);

    bsn! {
        (
            Name::new("Global Sun Light")
            GlobalSunLight
            DirectionalLight {
                illuminance: { illuminance },
                shadow_maps_enabled: { shadows_enabled },
                shadow_depth_bias: { shadow_depth_bias },
                shadow_normal_bias: { shadow_normal_bias },
            }
            template_value(shadow_cascades)
            template_value(transform)
        )
    }
}

fn cascade_shadow_config(controls: &GlobalLightControls) -> CascadeShadowConfig {
    CascadeShadowConfigBuilder {
        num_cascades: controls.cascade_count,
        minimum_distance: controls.cascade_minimum_distance,
        first_cascade_far_bound: controls.cascade_first_far_bound,
        maximum_distance: controls.cascade_maximum_distance,
        overlap_proportion: controls.cascade_overlap_proportion,
    }
    .build()
}
