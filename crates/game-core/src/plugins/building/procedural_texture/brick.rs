use super::builders::{build_albedo, build_normal, build_orm};
use super::noise::{cell_noise, fbm, mortar_mask};
use bevy::prelude::*;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct BrickHeightParams {
    pub rows: f32,
    pub cols: f32,
    pub mortar_row: f32,
    pub mortar_col: f32,
    pub surface_freq: f64,
    pub surface_oct: usize,
    pub surface_amp: f32,
    pub base_height: f32,
}

impl Hash for BrickHeightParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rows.to_bits().hash(state);
        self.cols.to_bits().hash(state);
        self.mortar_row.to_bits().hash(state);
        self.mortar_col.to_bits().hash(state);
        self.surface_freq.to_bits().hash(state);
        self.surface_oct.hash(state);
        self.surface_amp.to_bits().hash(state);
        self.base_height.to_bits().hash(state);
    }
}

impl Default for BrickHeightParams {
    fn default() -> Self {
        Self {
            rows: 7.0,
            cols: 4.0,
            mortar_row: 0.045,
            mortar_col: 0.035,
            surface_freq: 38.0,
            surface_oct: 3,
            surface_amp: 0.2,
            base_height: 0.72,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BrickAlbedoParams {
    pub base_color: [f32; 3],
    pub mortar_shade: f32,
    pub shade_freq: f64,
    pub shade_oct: usize,
    pub shade_amp: f32,
    pub tint_var_amp_r: f32,
    pub tint_var_amp_g: f32,
    pub tint_var_amp_b: f32,
    pub tint_base_r: f32,
    pub tint_base_g: f32,
    pub tint_base_b: f32,
}

impl Hash for BrickAlbedoParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_color[0].to_bits().hash(state);
        self.base_color[1].to_bits().hash(state);
        self.base_color[2].to_bits().hash(state);
        self.mortar_shade.to_bits().hash(state);
        self.shade_freq.to_bits().hash(state);
        self.shade_oct.hash(state);
        self.shade_amp.to_bits().hash(state);
        self.tint_var_amp_r.to_bits().hash(state);
        self.tint_var_amp_g.to_bits().hash(state);
        self.tint_var_amp_b.to_bits().hash(state);
        self.tint_base_r.to_bits().hash(state);
        self.tint_base_g.to_bits().hash(state);
        self.tint_base_b.to_bits().hash(state);
    }
}

impl Default for BrickAlbedoParams {
    fn default() -> Self {
        Self {
            base_color: [0.69, 0.37, 0.23],
            mortar_shade: 0.52,
            shade_freq: 30.0,
            shade_oct: 2,
            shade_amp: 0.24,
            tint_var_amp_r: 0.25,
            tint_var_amp_g: 0.12,
            tint_var_amp_b: 0.10,
            tint_base_r: 0.88,
            tint_base_g: 0.86,
            tint_base_b: 0.82,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BrickNormalParams {
    pub strength: f32,
}

impl Hash for BrickNormalParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.strength.to_bits().hash(state);
    }
}

impl Default for BrickNormalParams {
    fn default() -> Self {
        Self { strength: 5.0 }
    }
}

#[derive(Clone, Debug)]
pub struct BrickOrmParams {
    pub ao_base: f32,
    pub ao_height_amp: f32,
    pub rough_base: f32,
    pub rough_height_amp: f32,
    pub rough_grit_freq: f64,
    pub rough_grit_oct: usize,
    pub rough_grit_amp: f32,
    pub rough_min: f32,
    pub rough_max: f32,
}

impl Hash for BrickOrmParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ao_base.to_bits().hash(state);
        self.ao_height_amp.to_bits().hash(state);
        self.rough_base.to_bits().hash(state);
        self.rough_height_amp.to_bits().hash(state);
        self.rough_grit_freq.to_bits().hash(state);
        self.rough_grit_oct.hash(state);
        self.rough_grit_amp.to_bits().hash(state);
        self.rough_min.to_bits().hash(state);
        self.rough_max.to_bits().hash(state);
    }
}

impl Default for BrickOrmParams {
    fn default() -> Self {
        Self {
            ao_base: 0.66,
            ao_height_amp: 0.28,
            rough_base: 0.86,
            rough_height_amp: -0.12,
            rough_grit_freq: 55.0,
            rough_grit_oct: 2,
            rough_grit_amp: 0.08,
            rough_min: 0.62,
            rough_max: 0.96,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BrickParams {
    pub seed: u32,
    pub version: u32,
    pub height: BrickHeightParams,
    pub albedo: BrickAlbedoParams,
    pub normal: BrickNormalParams,
    pub orm: BrickOrmParams,
}

impl Hash for BrickParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.seed.hash(state);
        self.version.hash(state);
        self.height.hash(state);
        self.albedo.hash(state);
        self.normal.hash(state);
        self.orm.hash(state);
    }
}

impl Default for BrickParams {
    fn default() -> Self {
        Self {
            seed: 0,
            version: 1,
            height: BrickHeightParams::default(),
            albedo: BrickAlbedoParams::default(),
            normal: BrickNormalParams::default(),
            orm: BrickOrmParams::default(),
        }
    }
}

pub fn brick_height(params: &BrickParams, u: f32, v: f32) -> f32 {
    let h = &params.height;
    let rows = v * h.rows;
    let row = rows.floor() as i32;
    let offset = if row % 2 == 0 { 0.5 } else { 0.0 };
    let cols = u * h.cols + offset;
    let brick = mortar_mask(rows, h.mortar_row) * mortar_mask(cols, h.mortar_col);
    let surface = fbm(31 ^ params.seed, h.surface_freq, h.surface_oct, u, v) * h.surface_amp;
    brick * (h.base_height + surface)
}

pub fn brick_albedo(params: &BrickParams) -> Image {
    let seed = params.seed;
    let a = &params.albedo;
    let h = &params.height;
    build_albedo(
        a.base_color,
        |u, v| {
            let rows = v * h.rows;
            let row = rows.floor() as i32;
            let offset = if row % 2 == 0 { 0.5 } else { 0.0 };
            let cols = u * h.cols + offset;
            let mortar = mortar_mask(rows, h.mortar_row) * mortar_mask(cols, h.mortar_col);
            if mortar < 0.5 {
                a.mortar_shade
            } else {
                0.78 + fbm(32 ^ seed, a.shade_freq, a.shade_oct, u, v) * a.shade_amp
            }
        },
        |u, v| {
            let row = (v * h.rows).floor() as i32;
            let col = (u * h.cols).floor() as i32;
            let n = cell_noise(33 ^ seed, col, row);
            [
                a.tint_base_r + n * a.tint_var_amp_r,
                a.tint_base_g + n * a.tint_var_amp_g,
                a.tint_base_b + n * a.tint_var_amp_b,
            ]
        },
    )
}

pub fn brick_normal(params: &BrickParams) -> Image {
    build_normal(|u, v| brick_height(params, u, v), params.normal.strength)
}

pub fn brick_orm(params: &BrickParams) -> Image {
    let seed = params.seed;
    let o = &params.orm;
    build_orm(
        |u, v| {
            let h = brick_height(params, u, v);
            (o.ao_base + h * o.ao_height_amp).clamp(0.0, 1.0)
        },
        |u, v| {
            let h = brick_height(params, u, v);
            let grit = fbm(34 ^ seed, o.rough_grit_freq, o.rough_grit_oct, u, v);
            (o.rough_base + h * o.rough_height_amp + grit * o.rough_grit_amp)
                .clamp(o.rough_min, o.rough_max)
        },
        |_, _| 0.0,
    )
}
