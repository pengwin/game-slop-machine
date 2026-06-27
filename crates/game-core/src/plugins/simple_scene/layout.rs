use bevy::prelude::*;

pub const PLANE_SIZE: f32 = 12.0;
pub const BOX_SIZE: Vec3 = Vec3::splat(1.5);

pub struct SimpleSceneBox {
    pub name: &'static str,
    pub color: Color,
    pub position: Vec3,
}

pub const fn boxes() -> [SimpleSceneBox; 4] {
    [
        SimpleSceneBox {
            name: "Red Box",
            color: Color::srgb(0.90, 0.18, 0.16),
            position: Vec3::new(-2.0, 0.75, -2.0),
        },
        SimpleSceneBox {
            name: "Green Box",
            color: Color::srgb(0.18, 0.75, 0.30),
            position: Vec3::new(2.0, 0.75, -2.0),
        },
        SimpleSceneBox {
            name: "Blue Box",
            color: Color::srgb(0.18, 0.36, 0.95),
            position: Vec3::new(-2.0, 0.75, 2.0),
        },
        SimpleSceneBox {
            name: "Gold Box",
            color: Color::srgb(0.95, 0.68, 0.12),
            position: Vec3::new(2.0, 0.75, 2.0),
        },
    ]
}
