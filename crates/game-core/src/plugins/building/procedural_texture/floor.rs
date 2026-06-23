use bevy::prelude::*;

use super::builders::{build_albedo, build_normal, build_orm};
use super::noise::{cell_noise, fbm, hairline, speckle};

const TILE_SCALE: f32 = 4.0;
const GROUT: f32 = 0.020;

fn tile_cell(u: f32, v: f32) -> (i32, i32, f32, f32) {
    let tu = u * TILE_SCALE;
    let tv = v * TILE_SCALE;
    (tu.floor() as i32, tv.floor() as i32, tu.fract(), tv.fract())
}

fn clean_grout_mask(local_u: f32, local_v: f32) -> f32 {
    let edge = local_u.min(1.0 - local_u).min(local_v).min(1.0 - local_v);
    (edge / GROUT).clamp(0.0, 1.0)
}

fn grout_mask(seed: u32, u: f32, v: f32, local_u: f32, local_v: f32) -> f32 {
    let edge = local_u.min(1.0 - local_u).min(local_v).min(1.0 - local_v);
    let width_noise = fbm(47 ^ seed, 34.0, 2, u + 0.13, v - 0.21);
    let width = GROUT * (0.68 + width_noise * 0.50);
    let ragged = fbm(48 ^ seed, 78.0, 2, u - 0.07, v + 0.19) - 0.5;
    ((edge + ragged * GROUT * 0.28) / width).clamp(0.0, 1.0)
}

pub fn floor_height(seed: u32, u: f32, v: f32) -> f32 {
    let (ix, iy, lu, lv) = tile_cell(u, v);
    let grout = grout_mask(seed, u, v, lu, lv);
    let tile_variation = cell_noise(31 ^ seed, ix, iy) * 0.045;
    let broad = fbm(32 ^ seed, 4.0, 4, u, v) * 0.22;
    let chips = fbm(33 ^ seed, 34.0, 2, u + tile_variation, v - tile_variation) * 0.07;
    let pitted = speckle(41 ^ seed, 78.0, 0.74, u, v) * 0.16;
    let scratched = hairline(
        42 ^ seed,
        16.0,
        0.030,
        u + tile_variation,
        v - tile_variation,
    ) * 0.14;
    let grout_drop = (1.0 - grout) * 0.075;

    (0.55 + tile_variation + broad + chips + pitted + scratched - grout_drop).clamp(0.0, 1.0)
}

pub fn floor_albedo(seed: u32) -> Image {
    build_albedo(
        [0.64, 0.58, 0.48],
        |u, v| {
            let (ix, iy, lu, lv) = tile_cell(u, v);
            let grout = grout_mask(seed, u, v, lu, lv);
            let tile = cell_noise(34 ^ seed, ix, iy);
            let cloudy = fbm(35 ^ seed, 8.0, 4, u + tile * 0.03, v - tile * 0.02);
            let dirt = fbm(36 ^ seed, 2.5, 4, u - 0.17, v + 0.09);
            let wear = fbm(37 ^ seed, 18.0, 2, u + 0.23, v - 0.13);
            let edge_dirt = (1.0 - grout).powf(2.25);
            let clean_grout = clean_grout_mask(lu, lv);
            let seam_grit = speckle(49 ^ seed, 138.0, 0.76, u + tile * 0.09, v - tile * 0.06)
                * (1.0 - clean_grout).powf(1.4);
            let pores = speckle(43 ^ seed, 92.0, 0.72, u + tile * 0.07, v - tile * 0.05);
            let pale_grit = speckle(44 ^ seed, 118.0, 0.82, u - 0.11, v + 0.17);
            let crack = hairline(45 ^ seed, 11.0, 0.030, u + tile * 0.18, v - tile * 0.13);
            let scuff = fbm(46 ^ seed, 24.0, 2, u + tile * 0.21, v - tile * 0.17);

            (0.90 + tile * 0.035 + cloudy * 0.14 - dirt * 0.075 - edge_dirt * 0.040 + wear * 0.035
                - pores * 0.12
                + pale_grit * 0.070
                - seam_grit * 0.035
                - crack.powf(2.0) * 0.15
                + (scuff - 0.5) * 0.055)
                .clamp(0.66, 1.10)
        },
        |u, v| {
            let (ix, iy, lu, lv) = tile_cell(u, v);
            let grout = grout_mask(seed, u, v, lu, lv);
            let tile = cell_noise(38 ^ seed, ix, iy);
            let stain = fbm(39 ^ seed, 5.0, 3, u + 0.41, v - 0.29);
            let seam_grit = speckle(50 ^ seed, 126.0, 0.79, u - 0.11, v + 0.23)
                * (1.0 - clean_grout_mask(lu, lv));
            let warm = 0.96 + tile * 0.030 - stain * 0.035;
            let grout_tint = 0.94 + grout * 0.06 - seam_grit * 0.020;

            [
                warm * grout_tint,
                (0.94 + tile * 0.045 - stain * 0.03) * grout_tint,
                (0.96 + tile * 0.030 - stain * 0.02) * grout_tint,
            ]
        },
    )
}

pub fn floor_normal(seed: u32) -> Image {
    build_normal(|u, v| floor_height(seed, u, v), 2.15)
}

pub fn floor_orm(seed: u32) -> Image {
    build_orm(
        |u, v| {
            let h = floor_height(seed, u, v);
            (0.58 + h * 0.38).clamp(0.0, 1.0)
        },
        |u, v| {
            let (_ix, _iy, lu, lv) = tile_cell(u, v);
            let grout = grout_mask(seed, u, v, lu, lv);
            let dirt = fbm(40 ^ seed, 7.0, 3, u, v);
            let seam_grit = speckle(51 ^ seed, 112.0, 0.80, u + 0.31, v - 0.27)
                * (1.0 - clean_grout_mask(lu, lv));
            (0.80 + dirt * 0.14 + (1.0 - grout) * 0.020 + seam_grit * 0.035).clamp(0.68, 0.98)
        },
        |_, _| 0.0,
    )
}
