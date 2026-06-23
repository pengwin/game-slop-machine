use bevy::prelude::*;

use super::builders::{build_albedo, build_normal, build_orm};
use super::noise::{cell_noise, fbm};

const TILE_SCALE: f32 = 2.0;
const GROUT: f32 = 0.032;

fn tile_cell(u: f32, v: f32) -> (i32, i32, f32, f32) {
    let tu = u * TILE_SCALE;
    let tv = v * TILE_SCALE;
    (tu.floor() as i32, tv.floor() as i32, tu.fract(), tv.fract())
}

fn grout_mask(local_u: f32, local_v: f32) -> f32 {
    let edge = local_u.min(1.0 - local_u).min(local_v).min(1.0 - local_v);
    (edge / GROUT).clamp(0.0, 1.0)
}

pub fn floor_height(seed: u32, u: f32, v: f32) -> f32 {
    let (ix, iy, lu, lv) = tile_cell(u, v);
    let grout = grout_mask(lu, lv);
    let tile_variation = cell_noise(31 ^ seed, ix, iy) * 0.08;
    let broad = fbm(32 ^ seed, 4.0, 4, u, v) * 0.22;
    let chips = fbm(33 ^ seed, 34.0, 2, u + tile_variation, v - tile_variation) * 0.07;
    let grout_drop = (1.0 - grout) * 0.14;

    (0.55 + tile_variation + broad + chips - grout_drop).clamp(0.0, 1.0)
}

pub fn floor_albedo(seed: u32) -> Image {
    build_albedo(
        [0.64, 0.58, 0.48],
        |u, v| {
            let (ix, iy, lu, lv) = tile_cell(u, v);
            let grout = grout_mask(lu, lv);
            let tile = cell_noise(34 ^ seed, ix, iy);
            let cloudy = fbm(35 ^ seed, 8.0, 4, u + tile * 0.03, v - tile * 0.02);
            let dirt = fbm(36 ^ seed, 2.5, 4, u - 0.17, v + 0.09);
            let wear = fbm(37 ^ seed, 18.0, 2, u + 0.23, v - 0.13);
            let edge_dirt = (1.0 - grout).powf(2.6);

            (0.90 + tile * 0.055 + cloudy * 0.14 - dirt * 0.075 - edge_dirt * 0.075 + wear * 0.035)
                .clamp(0.66, 1.10)
        },
        |u, v| {
            let (ix, iy, lu, lv) = tile_cell(u, v);
            let grout = grout_mask(lu, lv);
            let tile = cell_noise(38 ^ seed, ix, iy);
            let stain = fbm(39 ^ seed, 5.0, 3, u + 0.41, v - 0.29);
            let warm = 0.96 + tile * 0.045 - stain * 0.035;
            let grout_tint = 0.89 + grout * 0.11;

            [
                warm * grout_tint,
                (0.94 + tile * 0.07 - stain * 0.03) * grout_tint,
                (0.96 + tile * 0.04 - stain * 0.02) * grout_tint,
            ]
        },
    )
}

pub fn floor_normal(seed: u32) -> Image {
    build_normal(|u, v| floor_height(seed, u, v), 1.65)
}

pub fn floor_orm(seed: u32) -> Image {
    build_orm(
        |u, v| {
            let h = floor_height(seed, u, v);
            (0.58 + h * 0.38).clamp(0.0, 1.0)
        },
        |u, v| {
            let (_ix, _iy, lu, lv) = tile_cell(u, v);
            let grout = grout_mask(lu, lv);
            let dirt = fbm(40 ^ seed, 7.0, 3, u, v);
            (0.80 + dirt * 0.14 + (1.0 - grout) * 0.035).clamp(0.68, 0.98)
        },
        |_, _| 0.0,
    )
}
