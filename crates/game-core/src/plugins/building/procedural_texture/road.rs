use super::builders::{build_albedo, build_normal, build_orm};
use super::noise::fbm;
use bevy::prelude::*;

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

pub fn road_orm(seed: u32) -> Image {
    build_orm(
        |u, v| {
            let h = road_height(seed, u, v);
            (0.54 + h * 0.36).clamp(0.0, 1.0)
        },
        |u, v| {
            let h = road_height(seed, u, v);
            let dust = fbm(64 ^ seed, 38.0, 2, u, v);
            (0.94 - h * 0.10 + dust * 0.04).clamp(0.78, 0.99)
        },
        |_, _| 0.0,
    )
}
