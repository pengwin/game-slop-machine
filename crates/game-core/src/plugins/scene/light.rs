use bevy::prelude::*;

pub fn spawn_light(mut commands: Commands) {
    commands.spawn((
        Name::new("Directional Light"),
        DirectionalLight {
            illuminance: 2_000.0,
            shadow_maps_enabled: true,
            soft_shadow_size: Some(40.0),
            contact_shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -std::f32::consts::FRAC_PI_4,
            std::f32::consts::FRAC_PI_4,
            0.0,
        )),
    ));

    commands.spawn((
        Name::new("Soft Fill Light"),
        DirectionalLight {
            illuminance: 900.0,
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -std::f32::consts::FRAC_PI_4,
            -std::f32::consts::FRAC_PI_4,
            0.0,
        )),
    ));

    commands.insert_resource(GlobalAmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
        ..default()
    });
}
