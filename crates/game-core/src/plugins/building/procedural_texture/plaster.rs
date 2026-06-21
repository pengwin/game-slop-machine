use bevy::prelude::*;
use super::builders::{build_albedo, build_normal, build_orm};
use super::noise::fbm;

pub fn plaster_height(seed: u32, u: f32, v: f32) -> f32 {
    fbm(11 ^ seed, 18.0, 5, u, v) * 0.7 + fbm(12 ^ seed, 58.0, 2, u, v) * 0.3
}

pub fn plaster_albedo(seed: u32) -> Image {
    build_albedo(
        [0.95, 0.88, 0.70],
        |u, v| {
            let base = 0.82 + plaster_height(seed, u, v) * 0.20;
            base.clamp(0.4, 1.1)
        },
        |_, _| [1.0, 1.0, 1.0],
    )
}

pub fn plaster_preview_albedo(seed: u32) -> Image {
    build_albedo(
        [0.95, 0.88, 0.70],
        |u, v| {
            let broad = fbm(90 ^ seed, 2.3, 5, u * 0.82 + 0.13, v * 0.82 - 0.07);
            let cloudy = fbm(91 ^ seed, 6.5, 4, u + broad * 0.10, v - broad * 0.08);
            let fine = fbm(92 ^ seed, 30.0, 2, u, v);

            let base_shade = 0.90 + broad * 0.10 + cloudy * 0.07 + fine * 0.020;
            base_shade.clamp(0.6, 1.1)
        },
        |u, v| {
            let stain = fbm(93 ^ seed, 3.4, 4, u + 0.17, v - 0.11);
            let age = fbm(94 ^ seed, 12.0, 2, u - 0.23, v + 0.19);

            [
                0.96 + stain * 0.030 + age * 0.010,
                0.96 + stain * 0.026 + age * 0.008,
                0.92 + stain * 0.020,
            ]
        },
    )
}

pub fn plaster_normal(seed: u32) -> Image {
    build_normal(|u, v| plaster_height(seed, u, v), 1.4)
}

pub fn plaster_orm(seed: u32) -> Image {
    build_orm(
        |u, v| {
            let h = plaster_height(seed, u, v);
            0.6 + h * 0.4
        },
        |u, v| {
            let h = plaster_height(seed, u, v);
            0.98 - h * 0.15
        },
        |_, _| 0.0,
    )
}
