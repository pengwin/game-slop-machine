use bevy::prelude::*;

use super::box_motion::SimpleSceneBoxMotion;

pub const PLANE_SIZE: f32 = 12.0;
pub const BOX_SIZE: Vec3 = Vec3::splat(1.5);

pub struct SimpleSceneBox {
    pub name: &'static str,
    pub color: Color,
    pub position: Vec3,
    pub motion_phase: f32,
}

impl SimpleSceneBox {
    pub fn scene(self) -> impl Scene {
        bsn!(
            Name::new(self.name)
            Mesh3d(asset_value(Cuboid::from_size(BOX_SIZE)))
            MeshMaterial3d::<StandardMaterial>(asset_value(box_material(self.color)))
            Transform::from_translation(self.position)
            template_value(SimpleSceneBoxMotion {
                base_y: self.position.y,
                phase: self.motion_phase,
            })
        )
    }
}

pub const fn boxes() -> [SimpleSceneBox; 4] {
    [
        SimpleSceneBox {
            name: "Red Box",
            color: Color::srgb(0.90, 0.18, 0.16),
            position: Vec3::new(-2.0, 0.75, -2.0),
            motion_phase: 0.0,
        },
        SimpleSceneBox {
            name: "Green Box",
            color: Color::srgb(0.18, 0.75, 0.30),
            position: Vec3::new(2.0, 0.75, -2.0),
            motion_phase: 0.7,
        },
        SimpleSceneBox {
            name: "Blue Box",
            color: Color::srgb(0.18, 0.36, 0.95),
            position: Vec3::new(-2.0, 0.75, 2.0),
            motion_phase: 1.4,
        },
        SimpleSceneBox {
            name: "Gold Box",
            color: Color::srgb(0.95, 0.68, 0.12),
            position: Vec3::new(2.0, 0.75, 2.0),
            motion_phase: 2.1,
        },
    ]
}

fn box_material(color: Color) -> StandardMaterial {
    StandardMaterial {
        base_color: color,
        perceptual_roughness: 0.65,
        ..default()
    }
}
