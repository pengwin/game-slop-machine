use super::builders::{build_albedo, build_normal, build_orm};
use super::noise::{cell_noise, fbm, mortar_mask};
use bevy::prelude::*;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct StoneHeightParams {
    pub blocks_y: f32,
    pub blocks_x: f32,
    pub offset_even: f32,
    pub mortar_x: f32,
    pub mortar_y: f32,
    pub surface_freq: f64,
    pub surface_oct: usize,
    pub surface_amp: f32,
    pub base_height: f32,
}

impl Hash for StoneHeightParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.blocks_y.to_bits().hash(state);
        self.blocks_x.to_bits().hash(state);
        self.offset_even.to_bits().hash(state);
        self.mortar_x.to_bits().hash(state);
        self.mortar_y.to_bits().hash(state);
        self.surface_freq.to_bits().hash(state);
        self.surface_oct.hash(state);
        self.surface_amp.to_bits().hash(state);
        self.base_height.to_bits().hash(state);
    }
}

impl Default for StoneHeightParams {
    fn default() -> Self {
        Self {
            blocks_y: 3.0,
            blocks_x: 5.0,
            offset_even: 0.35,
            mortar_x: 0.035,
            mortar_y: 0.055,
            surface_freq: 18.0,
            surface_oct: 4,
            surface_amp: 0.36,
            base_height: 0.55,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StoneAlbedoParams {
    pub base_color: [f32; 3],
    pub shade_mortar: f32,
    pub shade_base: f32,
    pub shade_height_amp: f32,
    pub tint_var_r: f32,
    pub tint_var_g: f32,
    pub tint_var_b: f32,
    pub tint_base_r: f32,
    pub tint_base_g: f32,
    pub tint_base_b: f32,
}

impl Hash for StoneAlbedoParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_color[0].to_bits().hash(state);
        self.base_color[1].to_bits().hash(state);
        self.base_color[2].to_bits().hash(state);
        self.shade_mortar.to_bits().hash(state);
        self.shade_base.to_bits().hash(state);
        self.shade_height_amp.to_bits().hash(state);
        self.tint_var_r.to_bits().hash(state);
        self.tint_var_g.to_bits().hash(state);
        self.tint_var_b.to_bits().hash(state);
        self.tint_base_r.to_bits().hash(state);
        self.tint_base_g.to_bits().hash(state);
        self.tint_base_b.to_bits().hash(state);
    }
}

impl Default for StoneAlbedoParams {
    fn default() -> Self {
        Self {
            base_color: [0.47, 0.46, 0.40],
            shade_mortar: 0.56,
            shade_base: 0.74,
            shade_height_amp: 0.24,
            tint_var_r: 0.12,
            tint_var_g: 0.10,
            tint_var_b: 0.08,
            tint_base_r: 0.90,
            tint_base_g: 0.90,
            tint_base_b: 0.88,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StoneNormalParams {
    pub strength: f32,
}

impl Hash for StoneNormalParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.strength.to_bits().hash(state);
    }
}

impl Default for StoneNormalParams {
    fn default() -> Self {
        Self { strength: 3.0 }
    }
}

#[derive(Clone, Debug)]
pub struct StoneOrmParams {
    pub ao_base: f32,
    pub ao_height_amp: f32,
    pub rough_base: f32,
    pub rough_height_amp: f32,
    pub rough_mineral_freq: f64,
    pub rough_mineral_oct: usize,
    pub rough_mineral_amp: f32,
    pub rough_min: f32,
    pub rough_max: f32,
}

impl Hash for StoneOrmParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ao_base.to_bits().hash(state);
        self.ao_height_amp.to_bits().hash(state);
        self.rough_base.to_bits().hash(state);
        self.rough_height_amp.to_bits().hash(state);
        self.rough_mineral_freq.to_bits().hash(state);
        self.rough_mineral_oct.hash(state);
        self.rough_mineral_amp.to_bits().hash(state);
        self.rough_min.to_bits().hash(state);
        self.rough_max.to_bits().hash(state);
    }
}

impl Default for StoneOrmParams {
    fn default() -> Self {
        Self {
            ao_base: 0.62,
            ao_height_amp: 0.30,
            rough_base: 0.90,
            rough_height_amp: -0.08,
            rough_mineral_freq: 24.0,
            rough_mineral_oct: 2,
            rough_mineral_amp: 0.06,
            rough_min: 0.70,
            rough_max: 0.98,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StoneParams {
    pub seed: u32,
    pub version: u32,
    pub height: StoneHeightParams,
    pub albedo: StoneAlbedoParams,
    pub normal: StoneNormalParams,
    pub orm: StoneOrmParams,
}

impl Hash for StoneParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.seed.hash(state);
        self.version.hash(state);
        self.height.hash(state);
        self.albedo.hash(state);
        self.normal.hash(state);
        self.orm.hash(state);
    }
}

impl Default for StoneParams {
    fn default() -> Self {
        Self {
            seed: 0,
            version: 1,
            height: StoneHeightParams::default(),
            albedo: StoneAlbedoParams::default(),
            normal: StoneNormalParams::default(),
            orm: StoneOrmParams::default(),
        }
    }
}

pub fn stone_height(params: &StoneParams, u: f32, v: f32) -> f32 {
    let h = &params.height;
    let blocks_y = v * h.blocks_y;
    let row = blocks_y.floor() as i32;
    let blocks_x = u * h.blocks_x + if row % 2 == 0 { h.offset_even } else { 0.0 };
    let joints = mortar_mask(blocks_x, h.mortar_x) * mortar_mask(blocks_y, h.mortar_y);
    joints
        * (h.base_height
            + fbm(51 ^ params.seed, h.surface_freq, h.surface_oct, u, v) * h.surface_amp)
}

pub fn stone_albedo(params: &StoneParams) -> Image {
    let seed = params.seed;
    let a = &params.albedo;
    let h = &params.height;
    build_albedo(
        a.base_color,
        |u, v| {
            let height_val = stone_height(params, u, v);
            if height_val < 0.08 {
                a.shade_mortar
            } else {
                a.shade_base + height_val * a.shade_height_amp
            }
        },
        |u, v| {
            let n = cell_noise(
                52 ^ seed,
                (u * h.blocks_x).floor() as i32,
                (v * h.blocks_y).floor() as i32,
            );
            [
                a.tint_base_r + n * a.tint_var_r,
                a.tint_base_g + n * a.tint_var_g,
                a.tint_base_b + n * a.tint_var_b,
            ]
        },
    )
}

pub fn stone_normal(params: &StoneParams) -> Image {
    build_normal(|u, v| stone_height(params, u, v), params.normal.strength)
}

pub fn stone_orm(params: &StoneParams) -> Image {
    let seed = params.seed;
    let o = &params.orm;
    build_orm(
        |u, v| {
            let h = stone_height(params, u, v);
            (o.ao_base + h * o.ao_height_amp).clamp(0.0, 1.0)
        },
        |u, v| {
            let h = stone_height(params, u, v);
            let mineral = fbm(53 ^ seed, o.rough_mineral_freq, o.rough_mineral_oct, u, v);
            (o.rough_base + h * o.rough_height_amp + mineral * o.rough_mineral_amp)
                .clamp(o.rough_min, o.rough_max)
        },
        |_, _| 0.0,
    )
}
