use super::builders::{build_albedo, build_normal, build_orm};
use super::noise::fbm;
use bevy::prelude::*;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct RoadHeightParams {
    pub broad_freq: f64,
    pub broad_oct: usize,
    pub broad_amp: f32,
    pub fine_freq: f64,
    pub fine_oct: usize,
    pub fine_amp: f32,
}

impl Hash for RoadHeightParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.broad_freq.to_bits().hash(state);
        self.broad_oct.hash(state);
        self.broad_amp.to_bits().hash(state);
        self.fine_freq.to_bits().hash(state);
        self.fine_oct.hash(state);
        self.fine_amp.to_bits().hash(state);
    }
}

impl Default for RoadHeightParams {
    fn default() -> Self {
        Self {
            broad_freq: 24.0,
            broad_oct: 4,
            broad_amp: 0.65,
            fine_freq: 95.0,
            fine_oct: 2,
            fine_amp: 0.35,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RoadAlbedoParams {
    pub base_color: [f32; 3],
    pub shade_base: f32,
    pub shade_height_amp: f32,
    pub pebble_freq: f64,
    pub pebble_oct: usize,
    pub pebble_var_r: f32,
    pub pebble_var_g: f32,
    pub pebble_var_b: f32,
    pub tint_base_r: f32,
    pub tint_base_g: f32,
    pub tint_base_b: f32,
}

impl Hash for RoadAlbedoParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_color[0].to_bits().hash(state);
        self.base_color[1].to_bits().hash(state);
        self.base_color[2].to_bits().hash(state);
        self.shade_base.to_bits().hash(state);
        self.shade_height_amp.to_bits().hash(state);
        self.pebble_freq.to_bits().hash(state);
        self.pebble_oct.hash(state);
        self.pebble_var_r.to_bits().hash(state);
        self.pebble_var_g.to_bits().hash(state);
        self.pebble_var_b.to_bits().hash(state);
        self.tint_base_r.to_bits().hash(state);
        self.tint_base_g.to_bits().hash(state);
        self.tint_base_b.to_bits().hash(state);
    }
}

impl Default for RoadAlbedoParams {
    fn default() -> Self {
        Self {
            base_color: [0.42, 0.34, 0.25],
            shade_base: 0.62,
            shade_height_amp: 0.34,
            pebble_freq: 120.0,
            pebble_oct: 1,
            pebble_var_r: 0.16,
            pebble_var_g: 0.12,
            pebble_var_b: 0.08,
            tint_base_r: 0.92,
            tint_base_g: 0.90,
            tint_base_b: 0.86,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RoadNormalParams {
    pub strength: f32,
}

impl Hash for RoadNormalParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.strength.to_bits().hash(state);
    }
}

impl Default for RoadNormalParams {
    fn default() -> Self {
        Self { strength: 1.8 }
    }
}

#[derive(Clone, Debug)]
pub struct RoadOrmParams {
    pub ao_base: f32,
    pub ao_height_amp: f32,
    pub rough_base: f32,
    pub rough_height_amp: f32,
    pub rough_dust_freq: f64,
    pub rough_dust_oct: usize,
    pub rough_dust_amp: f32,
    pub rough_min: f32,
    pub rough_max: f32,
}

impl Hash for RoadOrmParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ao_base.to_bits().hash(state);
        self.ao_height_amp.to_bits().hash(state);
        self.rough_base.to_bits().hash(state);
        self.rough_height_amp.to_bits().hash(state);
        self.rough_dust_freq.to_bits().hash(state);
        self.rough_dust_oct.hash(state);
        self.rough_dust_amp.to_bits().hash(state);
        self.rough_min.to_bits().hash(state);
        self.rough_max.to_bits().hash(state);
    }
}

impl Default for RoadOrmParams {
    fn default() -> Self {
        Self {
            ao_base: 0.54,
            ao_height_amp: 0.36,
            rough_base: 0.94,
            rough_height_amp: -0.10,
            rough_dust_freq: 38.0,
            rough_dust_oct: 2,
            rough_dust_amp: 0.04,
            rough_min: 0.78,
            rough_max: 0.99,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RoadParams {
    pub seed: u32,
    pub version: u32,
    pub height: RoadHeightParams,
    pub albedo: RoadAlbedoParams,
    pub normal: RoadNormalParams,
    pub orm: RoadOrmParams,
}

impl Hash for RoadParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.seed.hash(state);
        self.version.hash(state);
        self.height.hash(state);
        self.albedo.hash(state);
        self.normal.hash(state);
        self.orm.hash(state);
    }
}

impl Default for RoadParams {
    fn default() -> Self {
        Self {
            seed: 0,
            version: 1,
            height: RoadHeightParams::default(),
            albedo: RoadAlbedoParams::default(),
            normal: RoadNormalParams::default(),
            orm: RoadOrmParams::default(),
        }
    }
}

pub fn road_height(params: &RoadParams, u: f32, v: f32) -> f32 {
    let h = &params.height;
    fbm(61 ^ params.seed, h.broad_freq, h.broad_oct, u, v) * h.broad_amp
        + fbm(62 ^ params.seed, h.fine_freq, h.fine_oct, u, v) * h.fine_amp
}

pub fn road_albedo(params: &RoadParams) -> Image {
    let seed = params.seed;
    let a = &params.albedo;
    build_albedo(
        a.base_color,
        |u, v| a.shade_base + road_height(params, u, v) * a.shade_height_amp,
        |u, v| {
            let pebble = fbm(63 ^ seed, a.pebble_freq, a.pebble_oct, u, v);
            [
                a.tint_base_r + pebble * a.pebble_var_r,
                a.tint_base_g + pebble * a.pebble_var_g,
                a.tint_base_b + pebble * a.pebble_var_b,
            ]
        },
    )
}

pub fn road_normal(params: &RoadParams) -> Image {
    build_normal(|u, v| road_height(params, u, v), params.normal.strength)
}

pub fn road_orm(params: &RoadParams) -> Image {
    let seed = params.seed;
    let o = &params.orm;
    build_orm(
        |u, v| {
            let h = road_height(params, u, v);
            (o.ao_base + h * o.ao_height_amp).clamp(0.0, 1.0)
        },
        |u, v| {
            let h = road_height(params, u, v);
            let dust = fbm(64 ^ seed, o.rough_dust_freq, o.rough_dust_oct, u, v);
            (o.rough_base + h * o.rough_height_amp + dust * o.rough_dust_amp)
                .clamp(o.rough_min, o.rough_max)
        },
        |_, _| 0.0,
    )
}
