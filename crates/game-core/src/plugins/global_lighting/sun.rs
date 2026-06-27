use bevy::prelude::*;

use super::{LightingPreset, SceneLightingSettings};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_global_sun).add_systems(
        Update,
        apply_lighting_preset.run_if(resource_changed::<LightingPreset>),
    );
}

/// Marker for the single global directional sun entity.
#[derive(Component, Clone, Default)]
pub struct GlobalSunLight;

fn spawn_global_sun(
    mut commands: Commands<'_, '_>,
    preset: Res<'_, LightingPreset>,
    mut ambient_light: ResMut<'_, GlobalAmbientLight>,
) {
    let lighting = preset.into_inner().settings();
    apply_ambient_lighting(&lighting, &mut ambient_light);

    commands.queue_spawn_scene(sun_scene(&lighting));
}

fn apply_lighting_preset(
    preset: Res<'_, LightingPreset>,
    mut ambient_light: ResMut<'_, GlobalAmbientLight>,
    mut sun: Query<'_, '_, (&mut DirectionalLight, &mut Transform), With<GlobalSunLight>>,
) {
    let lighting = preset.into_inner().settings();
    apply_ambient_lighting(&lighting, &mut ambient_light);

    let Ok((mut sun, mut transform)) = sun.single_mut() else {
        warn!("GlobalSunLight is missing");
        return;
    };

    sun.illuminance = lighting.sun_illuminance;
    sun.shadow_maps_enabled = lighting.shadows_enabled;
    transform.rotation = lighting.sun_rotation;
}

const fn apply_ambient_lighting(
    lighting: &SceneLightingSettings,
    ambient_light: &mut GlobalAmbientLight,
) {
    ambient_light.color = lighting.ambient_color;
    ambient_light.brightness = lighting.ambient_brightness;
    ambient_light.affects_lightmapped_meshes = lighting.ambient_affects_lightmapped_meshes;
}

fn sun_scene(lighting: &SceneLightingSettings) -> impl Scene {
    let illuminance = lighting.sun_illuminance;
    let shadows_enabled = lighting.shadows_enabled;
    let transform = Transform::from_rotation(lighting.sun_rotation);

    bsn! {
        (
            Name::new("Global Sun Light")
            GlobalSunLight
            DirectionalLight {
                illuminance: { illuminance },
                shadow_maps_enabled: { shadows_enabled },
            }
            template_value(transform)
        )
    }
}
