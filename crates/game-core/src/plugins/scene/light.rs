use bevy::prelude::*;

pub fn spawn_light(mut commands: Commands) {
    commands.spawn((
        Name::new("Directional Light"),
        DirectionalLight {
            illuminance: 6_500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -std::f32::consts::FRAC_PI_3,
            std::f32::consts::FRAC_PI_6,
            0.0,
        )),
    ));

    commands.insert_resource(GlobalAmbientLight {
        color: Color::WHITE,
        brightness: 0.45,
        ..default()
    });
}
