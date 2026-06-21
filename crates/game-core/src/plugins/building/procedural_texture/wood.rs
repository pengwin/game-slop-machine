use bevy::prelude::*;
use super::builders::{build_albedo, build_normal};
use super::noise::{cell_noise, fbm};

pub fn wood_height(seed: u32, u: f32, v: f32) -> f32 {
    let plank = (v * 5.0).fract();
    let seam = if !(0.035..=0.965).contains(&plank) {
        -0.55
    } else {
        0.0
    };
    let grain = fbm(21 ^ seed, 22.0, 4, u * 3.5, v * 0.55);
    let fine = ((u * 72.0 + fbm(22 ^ seed, 9.0, 2, u, v) * 3.0).sin() * 0.5 + 0.5) * 0.22;
    seam + grain * 0.58 + fine
}

pub fn wood_albedo(seed: u32) -> Image {
    build_albedo(
        [0.64, 0.43, 0.24],
        |u, v| {
            let h = wood_height(seed, u, v);
            (0.70 + h * 0.30).max(0.34)
        },
        |_, v| {
            let plank_id = (v * 5.0).floor() as i32;
            let n = cell_noise(23 ^ seed, 0, plank_id);
            [0.92 + n * 0.18, 0.90 + n * 0.10, 0.86 + n * 0.08]
        },
    )
}

pub fn wood_normal(seed: u32) -> Image {
    build_normal(|u, v| wood_height(seed, u, v), 2.1)
}
