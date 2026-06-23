use super::builders::{build_albedo, build_normal};
use super::noise::{cell_noise, fbm, mortar_mask};
use bevy::prelude::*;

pub fn brick_height(seed: u32, u: f32, v: f32) -> f32 {
    let rows = v * 7.0;
    let row = rows.floor() as i32;
    let offset = if row % 2 == 0 { 0.5 } else { 0.0 };
    let cols = u * 4.0 + offset;
    let brick = mortar_mask(rows, 0.045) * mortar_mask(cols, 0.035);
    let surface = fbm(31 ^ seed, 38.0, 3, u, v) * 0.2;
    brick * (0.72 + surface)
}

pub fn brick_albedo(seed: u32) -> Image {
    build_albedo(
        [0.69, 0.37, 0.23],
        |u, v| {
            let rows = v * 7.0;
            let row = rows.floor() as i32;
            let offset = if row % 2 == 0 { 0.5 } else { 0.0 };
            let cols = u * 4.0 + offset;
            let mortar = mortar_mask(rows, 0.045) * mortar_mask(cols, 0.035);
            if mortar < 0.5 {
                0.52
            } else {
                0.78 + fbm(32 ^ seed, 30.0, 2, u, v) * 0.24
            }
        },
        |u, v| {
            let row = (v * 7.0).floor() as i32;
            let col = (u * 4.0).floor() as i32;
            let n = cell_noise(33 ^ seed, col, row);
            [0.88 + n * 0.25, 0.86 + n * 0.12, 0.82 + n * 0.10]
        },
    )
}

pub fn brick_normal(seed: u32) -> Image {
    build_normal(|u, v| brick_height(seed, u, v), 5.0)
}
