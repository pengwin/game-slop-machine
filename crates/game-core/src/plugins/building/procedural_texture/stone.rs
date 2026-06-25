use super::builders::{build_albedo, build_normal, build_orm};
use super::noise::{cell_noise, fbm, mortar_mask};
use bevy::prelude::*;

pub fn stone_height(seed: u32, u: f32, v: f32) -> f32 {
    let blocks_y = v * 3.0;
    let row = blocks_y.floor() as i32;
    let blocks_x = u * 5.0 + if row % 2 == 0 { 0.35 } else { 0.0 };
    let joints = mortar_mask(blocks_x, 0.035) * mortar_mask(blocks_y, 0.055);
    joints * (0.55 + fbm(51 ^ seed, 18.0, 4, u, v) * 0.36)
}

pub fn stone_albedo(seed: u32) -> Image {
    build_albedo(
        [0.47, 0.46, 0.40],
        |u, v| {
            let h = stone_height(seed, u, v);
            if h < 0.08 { 0.56 } else { 0.74 + h * 0.24 }
        },
        |u, v| {
            let n = cell_noise(
                52 ^ seed,
                (u * 5.0).floor() as i32,
                (v * 3.0).floor() as i32,
            );
            [0.90 + n * 0.12, 0.90 + n * 0.10, 0.88 + n * 0.08]
        },
    )
}

pub fn stone_normal(seed: u32) -> Image {
    build_normal(|u, v| stone_height(seed, u, v), 3.0)
}

pub fn stone_orm(seed: u32) -> Image {
    build_orm(
        |u, v| {
            let h = stone_height(seed, u, v);
            (0.62 + h * 0.30).clamp(0.0, 1.0)
        },
        |u, v| {
            let h = stone_height(seed, u, v);
            let mineral = fbm(53 ^ seed, 24.0, 2, u, v);
            (0.90 - h * 0.08 + mineral * 0.06).clamp(0.70, 0.98)
        },
        |_, _| 0.0,
    )
}
