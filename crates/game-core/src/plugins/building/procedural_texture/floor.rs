use bevy::prelude::*;

use super::builders::{build_albedo, build_normal, build_orm};
use super::noise::{cell_noise, fbm, hairline, speckle};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct FloorTileParams {
    pub scale: f32,
    pub grout: f32,
    pub grout_width_freq: f64,
    pub grout_width_oct: usize,
    pub grout_width_u_off: f32,
    pub grout_width_v_off: f32,
    pub grout_width_min: f32,
    pub grout_width_range: f32,
    pub grout_ragged_freq: f64,
    pub grout_ragged_oct: usize,
    pub grout_ragged_u_off: f32,
    pub grout_ragged_v_off: f32,
    pub grout_ragged_scale: f32,
}

impl Hash for FloorTileParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.scale.to_bits().hash(state);
        self.grout.to_bits().hash(state);
        self.grout_width_freq.to_bits().hash(state);
        self.grout_width_oct.hash(state);
        self.grout_width_u_off.to_bits().hash(state);
        self.grout_width_v_off.to_bits().hash(state);
        self.grout_width_min.to_bits().hash(state);
        self.grout_width_range.to_bits().hash(state);
        self.grout_ragged_freq.to_bits().hash(state);
        self.grout_ragged_oct.hash(state);
        self.grout_ragged_u_off.to_bits().hash(state);
        self.grout_ragged_v_off.to_bits().hash(state);
        self.grout_ragged_scale.to_bits().hash(state);
    }
}

impl Default for FloorTileParams {
    fn default() -> Self {
        Self {
            scale: 4.0,
            grout: 0.020,
            grout_width_freq: 34.0,
            grout_width_oct: 2,
            grout_width_u_off: 0.13,
            grout_width_v_off: -0.21,
            grout_width_min: 0.68,
            grout_width_range: 0.50,
            grout_ragged_freq: 78.0,
            grout_ragged_oct: 2,
            grout_ragged_u_off: -0.07,
            grout_ragged_v_off: 0.19,
            grout_ragged_scale: 0.28,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FloorHeightParams {
    pub tile_var_amp: f32,
    pub broad_freq: f64,
    pub broad_oct: usize,
    pub broad_amp: f32,
    pub chips_freq: f64,
    pub chips_oct: usize,
    pub chips_amp: f32,
    pub pitted_scale: f32,
    pub pitted_threshold: f32,
    pub pitted_amp: f32,
    pub scratch_freq: f64,
    pub scratch_width: f32,
    pub scratch_amp: f32,
    pub grout_drop_amp: f32,
    pub base_height: f32,
}

impl Hash for FloorHeightParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tile_var_amp.to_bits().hash(state);
        self.broad_freq.to_bits().hash(state);
        self.broad_oct.hash(state);
        self.broad_amp.to_bits().hash(state);
        self.chips_freq.to_bits().hash(state);
        self.chips_oct.hash(state);
        self.chips_amp.to_bits().hash(state);
        self.pitted_scale.to_bits().hash(state);
        self.pitted_threshold.to_bits().hash(state);
        self.pitted_amp.to_bits().hash(state);
        self.scratch_freq.to_bits().hash(state);
        self.scratch_width.to_bits().hash(state);
        self.scratch_amp.to_bits().hash(state);
        self.grout_drop_amp.to_bits().hash(state);
        self.base_height.to_bits().hash(state);
    }
}

impl Default for FloorHeightParams {
    fn default() -> Self {
        Self {
            tile_var_amp: 0.045,
            broad_freq: 4.0,
            broad_oct: 4,
            broad_amp: 0.22,
            chips_freq: 34.0,
            chips_oct: 2,
            chips_amp: 0.07,
            pitted_scale: 78.0,
            pitted_threshold: 0.74,
            pitted_amp: 0.16,
            scratch_freq: 16.0,
            scratch_width: 0.030,
            scratch_amp: 0.14,
            grout_drop_amp: 0.075,
            base_height: 0.55,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FloorAlbedoParams {
    pub base_color: [f32; 3],
    pub cloudy_freq: f64,
    pub cloudy_oct: usize,
    pub cloudy_u_off: f32,
    pub cloudy_v_off: f32,
    pub cloudy_amp: f32,
    pub dirt_freq: f64,
    pub dirt_oct: usize,
    pub dirt_u_off: f32,
    pub dirt_v_off: f32,
    pub dirt_amp: f32,
    pub wear_freq: f64,
    pub wear_oct: usize,
    pub wear_u_off: f32,
    pub wear_v_off: f32,
    pub wear_amp: f32,
    pub edge_dirt_exp: f32,
    pub edge_dirt_amp: f32,
    pub seam_grit_scale: f32,
    pub seam_grit_threshold: f32,
    pub seam_grit_u_off: f32,
    pub seam_grit_v_off: f32,
    pub seam_grit_amp: f32,
    pub seam_grit_exp: f32,
    pub pores_scale: f32,
    pub pores_threshold: f32,
    pub pores_u_off: f32,
    pub pores_v_off: f32,
    pub pores_amp: f32,
    pub pale_scale: f32,
    pub pale_threshold: f32,
    pub pale_u_off: f32,
    pub pale_v_off: f32,
    pub pale_amp: f32,
    pub crack_freq: f64,
    pub crack_width: f32,
    pub crack_u_off: f32,
    pub crack_v_off: f32,
    pub crack_amp: f32,
    pub scuff_freq: f64,
    pub scuff_oct: usize,
    pub scuff_u_off: f32,
    pub scuff_v_off: f32,
    pub scuff_amp: f32,
    pub shade_base: f32,
    pub shade_tile_amp: f32,
    pub shade_cloudy_amp: f32,
    pub shade_dirt_amp: f32,
    pub shade_wear_amp: f32,
    pub shade_pores_amp: f32,
    pub shade_pale_amp: f32,
    pub shade_seam_amp: f32,
    pub shade_crack_amp: f32,
    pub shade_scuff_amp: f32,
    pub shade_min: f32,
    pub shade_max: f32,
    pub tint_stain_freq: f64,
    pub tint_stain_oct: usize,
    pub tint_stain_u_off: f32,
    pub tint_stain_v_off: f32,
    pub tint_stain_amp: f32,
    pub tint_warm_base: f32,
    pub tint_warm_tile_amp: f32,
    pub tint_warm_stain_amp: f32,
    pub tint_g_base: f32,
    pub tint_g_tile_amp: f32,
    pub tint_g_stain_amp: f32,
    pub tint_b_base: f32,
    pub tint_b_tile_amp: f32,
    pub tint_b_stain_amp: f32,
}

impl Hash for FloorAlbedoParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_color[0].to_bits().hash(state);
        self.base_color[1].to_bits().hash(state);
        self.base_color[2].to_bits().hash(state);
        self.cloudy_freq.to_bits().hash(state);
        self.cloudy_oct.hash(state);
        self.cloudy_u_off.to_bits().hash(state);
        self.cloudy_v_off.to_bits().hash(state);
        self.cloudy_amp.to_bits().hash(state);
        self.dirt_freq.to_bits().hash(state);
        self.dirt_oct.hash(state);
        self.dirt_u_off.to_bits().hash(state);
        self.dirt_v_off.to_bits().hash(state);
        self.dirt_amp.to_bits().hash(state);
        self.wear_freq.to_bits().hash(state);
        self.wear_oct.hash(state);
        self.wear_u_off.to_bits().hash(state);
        self.wear_v_off.to_bits().hash(state);
        self.wear_amp.to_bits().hash(state);
        self.edge_dirt_exp.to_bits().hash(state);
        self.edge_dirt_amp.to_bits().hash(state);
        self.seam_grit_scale.to_bits().hash(state);
        self.seam_grit_threshold.to_bits().hash(state);
        self.seam_grit_u_off.to_bits().hash(state);
        self.seam_grit_v_off.to_bits().hash(state);
        self.seam_grit_amp.to_bits().hash(state);
        self.seam_grit_exp.to_bits().hash(state);
        self.pores_scale.to_bits().hash(state);
        self.pores_threshold.to_bits().hash(state);
        self.pores_u_off.to_bits().hash(state);
        self.pores_v_off.to_bits().hash(state);
        self.pores_amp.to_bits().hash(state);
        self.pale_scale.to_bits().hash(state);
        self.pale_threshold.to_bits().hash(state);
        self.pale_u_off.to_bits().hash(state);
        self.pale_v_off.to_bits().hash(state);
        self.pale_amp.to_bits().hash(state);
        self.crack_freq.to_bits().hash(state);
        self.crack_width.to_bits().hash(state);
        self.crack_u_off.to_bits().hash(state);
        self.crack_v_off.to_bits().hash(state);
        self.crack_amp.to_bits().hash(state);
        self.scuff_freq.to_bits().hash(state);
        self.scuff_oct.hash(state);
        self.scuff_u_off.to_bits().hash(state);
        self.scuff_v_off.to_bits().hash(state);
        self.scuff_amp.to_bits().hash(state);
        self.shade_base.to_bits().hash(state);
        self.shade_tile_amp.to_bits().hash(state);
        self.shade_cloudy_amp.to_bits().hash(state);
        self.shade_dirt_amp.to_bits().hash(state);
        self.shade_wear_amp.to_bits().hash(state);
        self.shade_pores_amp.to_bits().hash(state);
        self.shade_pale_amp.to_bits().hash(state);
        self.shade_seam_amp.to_bits().hash(state);
        self.shade_crack_amp.to_bits().hash(state);
        self.shade_scuff_amp.to_bits().hash(state);
        self.shade_min.to_bits().hash(state);
        self.shade_max.to_bits().hash(state);
        self.tint_stain_freq.to_bits().hash(state);
        self.tint_stain_oct.hash(state);
        self.tint_stain_u_off.to_bits().hash(state);
        self.tint_stain_v_off.to_bits().hash(state);
        self.tint_stain_amp.to_bits().hash(state);
        self.tint_warm_base.to_bits().hash(state);
        self.tint_warm_tile_amp.to_bits().hash(state);
        self.tint_warm_stain_amp.to_bits().hash(state);
        self.tint_g_base.to_bits().hash(state);
        self.tint_g_tile_amp.to_bits().hash(state);
        self.tint_g_stain_amp.to_bits().hash(state);
        self.tint_b_base.to_bits().hash(state);
        self.tint_b_tile_amp.to_bits().hash(state);
        self.tint_b_stain_amp.to_bits().hash(state);
    }
}

impl Default for FloorAlbedoParams {
    fn default() -> Self {
        Self {
            base_color: [0.64, 0.58, 0.48],
            cloudy_freq: 8.0,
            cloudy_oct: 4,
            cloudy_u_off: 0.03,
            cloudy_v_off: -0.02,
            cloudy_amp: 0.14,
            dirt_freq: 2.5,
            dirt_oct: 4,
            dirt_u_off: -0.17,
            dirt_v_off: 0.09,
            dirt_amp: 0.075,
            wear_freq: 18.0,
            wear_oct: 2,
            wear_u_off: 0.23,
            wear_v_off: -0.13,
            wear_amp: 0.035,
            edge_dirt_exp: 2.25,
            edge_dirt_amp: 0.040,
            seam_grit_scale: 138.0,
            seam_grit_threshold: 0.76,
            seam_grit_u_off: 0.09,
            seam_grit_v_off: -0.06,
            seam_grit_amp: 0.035,
            seam_grit_exp: 1.4,
            pores_scale: 92.0,
            pores_threshold: 0.72,
            pores_u_off: 0.07,
            pores_v_off: -0.05,
            pores_amp: 0.12,
            pale_scale: 118.0,
            pale_threshold: 0.82,
            pale_u_off: -0.11,
            pale_v_off: 0.17,
            pale_amp: 0.070,
            crack_freq: 11.0,
            crack_width: 0.030,
            crack_u_off: 0.18,
            crack_v_off: -0.13,
            crack_amp: 0.15,
            scuff_freq: 24.0,
            scuff_oct: 2,
            scuff_u_off: 0.21,
            scuff_v_off: -0.17,
            scuff_amp: 0.055,
            shade_base: 0.90,
            shade_tile_amp: 0.035,
            shade_cloudy_amp: 1.0,
            shade_dirt_amp: 1.0,
            shade_wear_amp: 1.0,
            shade_pores_amp: 1.0,
            shade_pale_amp: 1.0,
            shade_seam_amp: 1.0,
            shade_crack_amp: 1.0,
            shade_scuff_amp: 1.0,
            shade_min: 0.66,
            shade_max: 1.10,
            tint_stain_freq: 5.0,
            tint_stain_oct: 3,
            tint_stain_u_off: 0.41,
            tint_stain_v_off: -0.29,
            tint_stain_amp: 0.035,
            tint_warm_base: 0.96,
            tint_warm_tile_amp: 0.030,
            tint_warm_stain_amp: 0.035,
            tint_g_base: 0.94,
            tint_g_tile_amp: 0.045,
            tint_g_stain_amp: 0.03,
            tint_b_base: 0.96,
            tint_b_tile_amp: 0.030,
            tint_b_stain_amp: 0.02,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FloorNormalParams {
    pub strength: f32,
}

impl Hash for FloorNormalParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.strength.to_bits().hash(state);
    }
}

impl Default for FloorNormalParams {
    fn default() -> Self {
        Self { strength: 2.15 }
    }
}

#[derive(Clone, Debug)]
pub struct FloorOrmParams {
    pub ao_base: f32,
    pub ao_height_amp: f32,
    pub rough_base: f32,
    pub rough_dirt_freq: f64,
    pub rough_dirt_oct: usize,
    pub rough_dirt_amp: f32,
    pub rough_grout_amp: f32,
    pub rough_seam_scale: f32,
    pub rough_seam_threshold: f32,
    pub rough_seam_u_off: f32,
    pub rough_seam_v_off: f32,
    pub rough_seam_amp: f32,
    pub rough_min: f32,
    pub rough_max: f32,
}

impl Hash for FloorOrmParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ao_base.to_bits().hash(state);
        self.ao_height_amp.to_bits().hash(state);
        self.rough_base.to_bits().hash(state);
        self.rough_dirt_freq.to_bits().hash(state);
        self.rough_dirt_oct.hash(state);
        self.rough_dirt_amp.to_bits().hash(state);
        self.rough_grout_amp.to_bits().hash(state);
        self.rough_seam_scale.to_bits().hash(state);
        self.rough_seam_threshold.to_bits().hash(state);
        self.rough_seam_u_off.to_bits().hash(state);
        self.rough_seam_v_off.to_bits().hash(state);
        self.rough_seam_amp.to_bits().hash(state);
        self.rough_min.to_bits().hash(state);
        self.rough_max.to_bits().hash(state);
    }
}

impl Default for FloorOrmParams {
    fn default() -> Self {
        Self {
            ao_base: 0.58,
            ao_height_amp: 0.38,
            rough_base: 0.80,
            rough_dirt_freq: 7.0,
            rough_dirt_oct: 3,
            rough_dirt_amp: 0.14,
            rough_grout_amp: 0.020,
            rough_seam_scale: 112.0,
            rough_seam_threshold: 0.80,
            rough_seam_u_off: 0.31,
            rough_seam_v_off: -0.27,
            rough_seam_amp: 0.035,
            rough_min: 0.68,
            rough_max: 0.98,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FloorParams {
    pub seed: u32,
    pub version: u32,
    pub tile: FloorTileParams,
    pub height: FloorHeightParams,
    pub albedo: FloorAlbedoParams,
    pub normal: FloorNormalParams,
    pub orm: FloorOrmParams,
}

impl Hash for FloorParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.seed.hash(state);
        self.version.hash(state);
        self.tile.hash(state);
        self.height.hash(state);
        self.albedo.hash(state);
        self.normal.hash(state);
        self.orm.hash(state);
    }
}

impl Default for FloorParams {
    fn default() -> Self {
        Self {
            seed: 0,
            version: 1,
            tile: FloorTileParams::default(),
            height: FloorHeightParams::default(),
            albedo: FloorAlbedoParams::default(),
            normal: FloorNormalParams::default(),
            orm: FloorOrmParams::default(),
        }
    }
}

fn tile_cell(params: &FloorParams, u: f32, v: f32) -> (i32, i32, f32, f32) {
    let tu = u * params.tile.scale;
    let tv = v * params.tile.scale;
    (tu.floor() as i32, tv.floor() as i32, tu.fract(), tv.fract())
}

fn clean_grout_mask(params: &FloorParams, local_u: f32, local_v: f32) -> f32 {
    let edge = local_u
        .min(1.0 - local_u)
        .min(local_v)
        .min(1.0 - local_v);
    (edge / params.tile.grout).clamp(0.0, 1.0)
}

fn grout_mask(
    params: &FloorParams,
    seed: u32,
    u: f32,
    v: f32,
    local_u: f32,
    local_v: f32,
) -> f32 {
    let t = &params.tile;
    let edge = local_u
        .min(1.0 - local_u)
        .min(local_v)
        .min(1.0 - local_v);
    let width_noise = fbm(
        47 ^ seed,
        t.grout_width_freq,
        t.grout_width_oct,
        u + t.grout_width_u_off,
        v - t.grout_width_v_off.abs(),
    );
    let width = t.grout * (t.grout_width_min + width_noise * t.grout_width_range);
    let ragged = fbm(
        48 ^ seed,
        t.grout_ragged_freq,
        t.grout_ragged_oct,
        u - t.grout_ragged_u_off.abs(),
        v + t.grout_ragged_v_off,
    ) - 0.5;
    ((edge + ragged * t.grout * t.grout_ragged_scale) / width).clamp(0.0, 1.0)
}

pub fn floor_height(params: &FloorParams, u: f32, v: f32) -> f32 {
    let seed = params.seed;
    let h = &params.height;
    let (ix, iy, lu, lv) = tile_cell(params, u, v);
    let grout = grout_mask(params, seed, u, v, lu, lv);
    let tile_variation = cell_noise(31 ^ seed, ix, iy) * h.tile_var_amp;
    let broad = fbm(32 ^ seed, h.broad_freq, h.broad_oct, u, v) * h.broad_amp;
    let chips = fbm(
        33 ^ seed,
        h.chips_freq,
        h.chips_oct,
        u + tile_variation,
        v - tile_variation,
    ) * h.chips_amp;
    let pitted = speckle(41 ^ seed, h.pitted_scale, h.pitted_threshold, u, v) * h.pitted_amp;
    let scratched = hairline(
        42 ^ seed,
        h.scratch_freq,
        h.scratch_width,
        u + tile_variation,
        v - tile_variation,
    ) * h.scratch_amp;
    let grout_drop = (1.0 - grout) * h.grout_drop_amp;

    (h.base_height + tile_variation + broad + chips + pitted + scratched - grout_drop).clamp(0.0, 1.0)
}

pub fn floor_albedo(params: &FloorParams) -> Image {
    let seed = params.seed;
    let a = &params.albedo;
    build_albedo(
        a.base_color,
        |u, v| {
            let (ix, iy, lu, lv) = tile_cell(params, u, v);
            let grout = grout_mask(params, seed, u, v, lu, lv);
            let tile = cell_noise(34 ^ seed, ix, iy);
            let cloudy = fbm(
                35 ^ seed,
                a.cloudy_freq,
                a.cloudy_oct,
                u + tile * a.cloudy_u_off,
                v - tile * a.cloudy_v_off.abs(),
            );
            let dirt = fbm(
                36 ^ seed,
                a.dirt_freq,
                a.dirt_oct,
                u - a.dirt_u_off.abs(),
                v + a.dirt_v_off,
            );
            let wear = fbm(
                37 ^ seed,
                a.wear_freq,
                a.wear_oct,
                u + a.wear_u_off,
                v - a.wear_v_off.abs(),
            );
            let edge_dirt = (1.0 - grout).powf(a.edge_dirt_exp);
            let clean = clean_grout_mask(params, lu, lv);
            let seam_grit = speckle(
                49 ^ seed,
                a.seam_grit_scale,
                a.seam_grit_threshold,
                u + tile * a.seam_grit_u_off,
                v - tile * a.seam_grit_v_off.abs(),
            ) * (1.0 - clean).powf(a.seam_grit_exp);
            let pores = speckle(
                43 ^ seed,
                a.pores_scale,
                a.pores_threshold,
                u + tile * a.pores_u_off,
                v - tile * a.pores_v_off.abs(),
            );
            let pale_grit = speckle(
                44 ^ seed,
                a.pale_scale,
                a.pale_threshold,
                u - a.pale_u_off.abs(),
                v + a.pale_v_off,
            );
            let crack = hairline(
                45 ^ seed,
                a.crack_freq,
                a.crack_width,
                u + tile * a.crack_u_off,
                v - tile * a.crack_v_off.abs(),
            );
            let scuff = fbm(
                46 ^ seed,
                a.scuff_freq,
                a.scuff_oct,
                u + tile * a.scuff_u_off,
                v - tile * a.scuff_v_off.abs(),
            );

            (a.shade_base
                + tile * a.shade_tile_amp
                + cloudy * a.shade_cloudy_amp * a.cloudy_amp
                - dirt * a.shade_dirt_amp * a.dirt_amp
                - edge_dirt * a.edge_dirt_amp
                + wear * a.shade_wear_amp * a.wear_amp
                - pores * a.shade_pores_amp * a.pores_amp
                + pale_grit * a.shade_pale_amp * a.pale_amp
                - seam_grit * a.shade_seam_amp * a.seam_grit_amp
                - crack.powf(2.0) * a.shade_crack_amp * a.crack_amp
                + (scuff - 0.5) * a.shade_scuff_amp * a.scuff_amp)
                .clamp(a.shade_min, a.shade_max)
        },
        |u, v| {
            let (ix, iy, lu, lv) = tile_cell(params, u, v);
            let grout = grout_mask(params, seed, u, v, lu, lv);
            let tile = cell_noise(38 ^ seed, ix, iy);
            let stain = fbm(
                39 ^ seed,
                a.tint_stain_freq,
                a.tint_stain_oct,
                u + a.tint_stain_u_off,
                v - a.tint_stain_v_off.abs(),
            );
            let seam_grit = speckle(
                50 ^ seed,
                a.seam_grit_scale,
                a.seam_grit_threshold,
                u - a.pale_u_off.abs(),
                v + a.pale_v_off,
            ) * (1.0 - clean_grout_mask(params, lu, lv));
            let warm = a.tint_warm_base + tile * a.tint_warm_tile_amp
                - stain * a.tint_warm_stain_amp;
            let grout_tint = 0.94 + grout * 0.06 - seam_grit * 0.020;

            [
                warm * grout_tint,
                (a.tint_g_base + tile * a.tint_g_tile_amp - stain * a.tint_g_stain_amp)
                    * grout_tint,
                (a.tint_b_base + tile * a.tint_b_tile_amp - stain * a.tint_b_stain_amp)
                    * grout_tint,
            ]
        },
    )
}

pub fn floor_normal(params: &FloorParams) -> Image {
    build_normal(|u, v| floor_height(params, u, v), params.normal.strength)
}

pub fn floor_orm(params: &FloorParams) -> Image {
    let seed = params.seed;
    let o = &params.orm;
    build_orm(
        |u, v| {
            let h = floor_height(params, u, v);
            (o.ao_base + h * o.ao_height_amp).clamp(0.0, 1.0)
        },
        |u, v| {
            let (_ix, _iy, lu, lv) = tile_cell(params, u, v);
            let grout = grout_mask(params, seed, u, v, lu, lv);
            let dirt = fbm(40 ^ seed, o.rough_dirt_freq, o.rough_dirt_oct, u, v);
            let seam_grit = speckle(
                51 ^ seed,
                o.rough_seam_scale,
                o.rough_seam_threshold,
                u + o.rough_seam_u_off,
                v - o.rough_seam_v_off.abs(),
            ) * (1.0 - clean_grout_mask(params, lu, lv));
            (o.rough_base
                + dirt * o.rough_dirt_amp
                + (1.0 - grout) * o.rough_grout_amp
                + seam_grit * o.rough_seam_amp)
                .clamp(o.rough_min, o.rough_max)
        },
        |_, _| 0.0,
    )
}
