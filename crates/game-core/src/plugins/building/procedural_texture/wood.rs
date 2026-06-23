use super::builders::{build_albedo, build_normal};
use super::noise::{cell_noise, fbm};
use bevy::prelude::*;

pub fn wood_height(seed: u32, u: f32, v: f32) -> f32 {
    let warp = fbm(22 ^ seed, 4.0, 3, u * 0.55, v * 1.3);
    let slow_wander = fbm(24 ^ seed, 2.0, 2, u * 0.45, v * 0.95) * 1.8;
    let broad = ((u * 14.0 + warp * 2.0 + slow_wander).sin() * 0.5 + 0.5).powf(1.65);
    let fine = ((u * 46.0 + warp * 3.5).sin() * 0.5 + 0.5).powf(2.7);
    let grain = fbm(21 ^ seed, 8.0, 4, u * 2.6 + warp * 0.25, v * 0.60);
    let knot = fbm(25 ^ seed, 3.7, 3, u * 1.5 + warp, v * 1.15).powf(5.0);
    broad * 0.32 + fine * 0.20 + grain * 0.22 + knot * 0.16
}

pub fn wood_albedo(seed: u32) -> Image {
    build_albedo(
        [0.86, 0.58, 0.30],
        |u, v| {
            let h = wood_height(seed, u, v);
            (0.66 + h * 0.42).max(0.34)
        },
        |_, v| {
            let band_id = (v * 3.0).floor() as i32;
            let n = cell_noise(23 ^ seed, 0, band_id);
            [0.96 + n * 0.08, 0.82 + n * 0.07, 0.60 + n * 0.06]
        },
    )
}

pub fn wood_normal(seed: u32) -> Image {
    build_normal(|u, v| wood_height(seed, u, v), 2.1)
}
