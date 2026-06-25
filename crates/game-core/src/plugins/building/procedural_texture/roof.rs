use super::builders::{build_albedo, build_normal, build_orm};
use super::noise::{cell_noise, fbm, mortar_mask};
use bevy::prelude::*;

pub fn roof_height(seed: u32, u: f32, v: f32) -> f32 {
    let courses = v * 8.0;
    let columns = u * 6.0
        + if courses.floor() as i32 % 2 == 0 {
            0.5
        } else {
            0.0
        };
    let gap = mortar_mask(courses, 0.055) * mortar_mask(columns, 0.045);
    let curved = ((courses.fract() * std::f32::consts::PI).sin() * 0.32 + 0.58).max(0.0);
    gap * curved + fbm(41 ^ seed, 44.0, 2, u, v) * 0.08
}

pub fn roof_albedo(seed: u32) -> Image {
    build_albedo(
        [0.55, 0.25, 0.14],
        |u, v| 0.68 + roof_height(seed, u, v) * 0.33,
        |u, v| {
            let n = cell_noise(
                42 ^ seed,
                (u * 6.0).floor() as i32,
                (v * 8.0).floor() as i32,
            );
            [0.86 + n * 0.20, 0.82 + n * 0.10, 0.78 + n * 0.06]
        },
    )
}

pub fn roof_normal(seed: u32) -> Image {
    build_normal(|u, v| roof_height(seed, u, v), 4.4)
}

pub fn roof_orm(seed: u32) -> Image {
    build_orm(
        |u, v| {
            let h = roof_height(seed, u, v);
            (0.70 + h * 0.22).clamp(0.0, 1.0)
        },
        |u, v| {
            let h = roof_height(seed, u, v);
            let dust = fbm(43 ^ seed, 18.0, 2, u, v);
            (0.82 - h * 0.10 + dust * 0.10).clamp(0.58, 0.95)
        },
        |_, _| 0.0,
    )
}
