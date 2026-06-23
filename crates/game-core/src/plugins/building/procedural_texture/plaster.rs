use super::builders::{build_albedo, build_normal, build_orm};
use super::noise::{fbm, hairline, speckle};
use bevy::prelude::*;

pub fn plaster_height(seed: u32, u: f32, v: f32) -> f32 {
    let broad = fbm(11 ^ seed, 7.0, 5, u, v) * 0.45;
    let fine = fbm(12 ^ seed, 46.0, 3, u, v) * 0.28;
    let patches = fbm(13 ^ seed, 2.2, 4, u + 0.19, v - 0.31) * 0.27;
    let pits = speckle(18 ^ seed, 82.0, 0.74, u, v) * 0.14;
    let hair = hairline(19 ^ seed, 22.0, 0.026, u * 0.55, v * 1.4) * 0.12;
    (broad + fine + patches + pits + hair).clamp(0.0, 1.0)
}

pub fn plaster_albedo(seed: u32) -> Image {
    build_albedo(
        [0.95, 0.88, 0.70],
        |u, v| {
            let broad = fbm(14 ^ seed, 2.6, 5, u * 0.85 + 0.13, v * 0.85 - 0.07);
            let stains = fbm(15 ^ seed, 10.0, 3, u + broad * 0.12, v);
            let streaks = fbm(16 ^ seed, 22.0, 2, u * 0.35, v * 1.8);
            let pores = speckle(20 ^ seed, 96.0, 0.72, u + broad * 0.03, v);
            let pale_sand = speckle(21 ^ seed, 130.0, 0.84, u - 0.27, v + 0.16);
            let hair_cracks = hairline(22 ^ seed, 24.0, 0.022, u * 0.45 + broad * 0.08, v * 1.6);
            let vertical = (1.0 - v.fract()).powf(1.4) * 0.10;
            let base = 0.76 + plaster_height(seed, u, v) * 0.24 + broad * 0.08
                - stains * 0.08
                - streaks * vertical
                - pores * 0.080
                + pale_sand * 0.055
                - hair_cracks.powf(2.0) * 0.090;
            base.clamp(0.36, 1.12)
        },
        |u, v| {
            let age = fbm(17 ^ seed, 4.2, 4, u - 0.23, v + 0.19);
            [0.97 + age * 0.04, 0.94 + age * 0.025, 0.86 + age * 0.018]
        },
    )
}

pub fn plaster_preview_albedo(seed: u32) -> Image {
    build_albedo(
        [0.95, 0.88, 0.70],
        |u, v| {
            let broad = fbm(90 ^ seed, 2.3, 5, u * 0.82 + 0.13, v * 0.82 - 0.07);
            let cloudy = fbm(91 ^ seed, 6.5, 4, u + broad * 0.10, v - broad * 0.08);
            let fine = fbm(92 ^ seed, 30.0, 2, u, v);
            let pores = speckle(98 ^ seed, 88.0, 0.74, u, v);
            let scratches = hairline(99 ^ seed, 22.0, 0.024, u * 0.5, v * 1.5);

            let base_shade = 0.90 + broad * 0.10 + cloudy * 0.07 + fine * 0.020
                - pores * 0.070
                - scratches.powf(2.0) * 0.070;
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
