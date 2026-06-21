use bevy::prelude::*;
use super::builders::{build_albedo, build_normal};
use super::noise::fbm;

pub fn road_height(seed: u32, u: f32, v: f32) -> f32 {
    fbm(61 ^ seed, 24.0, 4, u, v) * 0.65 + fbm(62 ^ seed, 95.0, 2, u, v) * 0.35
}

pub fn road_albedo(seed: u32) -> Image {
    build_albedo(
        [0.42, 0.34, 0.25],
        |u, v| 0.62 + road_height(seed, u, v) * 0.34,
        |u, v| {
            let pebble = fbm(63 ^ seed, 120.0, 1, u, v);
            [
                0.92 + pebble * 0.16,
                0.90 + pebble * 0.12,
                0.86 + pebble * 0.08,
            ]
        },
    )
}

pub fn road_normal(seed: u32) -> Image {
    build_normal(|u, v| road_height(seed, u, v), 1.8)
}
