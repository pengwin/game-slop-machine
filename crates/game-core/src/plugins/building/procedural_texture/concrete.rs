use super::builders::{build_albedo, build_normal, build_orm};
use super::noise::fbm;
use bevy::prelude::*;

pub fn concrete_height(seed: u32, u: f32, v: f32) -> f32 {
    let broad = fbm(71 ^ seed, 8.0, 4, u, v);
    let fine = fbm(72 ^ seed, 35.0, 2, u, v);
    let formwork = ((v * 28.0).sin() * 0.5 + 0.5) * 0.12;
    (broad * 0.65 + fine * 0.20 + formwork).clamp(0.0, 1.0)
}

pub fn concrete_albedo(seed: u32) -> Image {
    build_albedo(
        [0.57, 0.57, 0.55],
        |u, v| {
            let h = concrete_height(seed, u, v);
            let broad_shade = fbm(80 ^ seed, 3.5, 4, u, v);
            let pitting = fbm(81 ^ seed, 60.0, 1, u, v);
            let crack = fbm(82 ^ seed, 14.0, 3, u * 1.4, v * 0.8);
            let crack_line = ((crack - 0.52).abs() * 18.0).clamp(0.0, 1.0);

            let base = 0.70 + h * 0.40;
            let shade = base + broad_shade * 0.12 - pitting * 0.08 - (1.0 - crack_line) * 0.18;
            shade.clamp(0.35, 1.15)
        },
        |u, v| {
            let stain = fbm(83 ^ seed, 4.0, 3, u + 0.11, v - 0.07);
            let mineral = fbm(84 ^ seed, 7.0, 2, u - 0.19, v + 0.23);
            let age = fbm(85 ^ seed, 12.0, 2, u + 0.31, v - 0.15);

            [
                0.98 + stain * 0.025 + mineral * 0.015 - age * 0.018,
                0.98 + stain * 0.025 + mineral * 0.010 - age * 0.018,
                0.98 + stain * 0.020 - mineral * 0.005 - age * 0.012,
            ]
        },
    )
}

pub fn concrete_normal(seed: u32) -> Image {
    build_normal(|u, v| concrete_height(seed, u, v), 1.8)
}

pub fn concrete_orm(seed: u32) -> Image {
    build_orm(
        |u, v| {
            let h = concrete_height(seed, u, v);
            0.55 + h * 0.45
        },
        |u, v| {
            let h = concrete_height(seed, u, v);
            let pitting = fbm(81 ^ seed, 60.0, 1, u, v);
            (0.92 - h * 0.10 + pitting * 0.04).clamp(0.75, 0.98)
        },
        |_, _| 0.0,
    )
}
