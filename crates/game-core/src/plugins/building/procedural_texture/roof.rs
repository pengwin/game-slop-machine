use super::builders::{build_albedo, build_normal, build_orm};
use super::noise::{cell_noise, fbm, mortar_mask};
use bevy::prelude::*;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct RoofHeightParams {
    pub courses: f32,
    pub columns: f32,
    pub mortar_courses: f32,
    pub mortar_columns: f32,
    pub curved_amp: f32,
    pub curved_base: f32,
    pub surface_freq: f64,
    pub surface_oct: usize,
    pub surface_amp: f32,
}

impl Hash for RoofHeightParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.courses.to_bits().hash(state);
        self.columns.to_bits().hash(state);
        self.mortar_courses.to_bits().hash(state);
        self.mortar_columns.to_bits().hash(state);
        self.curved_amp.to_bits().hash(state);
        self.curved_base.to_bits().hash(state);
        self.surface_freq.to_bits().hash(state);
        self.surface_oct.hash(state);
        self.surface_amp.to_bits().hash(state);
    }
}

impl Default for RoofHeightParams {
    fn default() -> Self {
        Self {
            courses: 8.0,
            columns: 6.0,
            mortar_courses: 0.055,
            mortar_columns: 0.045,
            curved_amp: 0.32,
            curved_base: 0.58,
            surface_freq: 44.0,
            surface_oct: 2,
            surface_amp: 0.08,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RoofAlbedoParams {
    pub base_color: [f32; 3],
    pub shade_base: f32,
    pub shade_height_amp: f32,
    pub tint_var_r: f32,
    pub tint_var_g: f32,
    pub tint_var_b: f32,
    pub tint_base_r: f32,
    pub tint_base_g: f32,
    pub tint_base_b: f32,
}

impl Hash for RoofAlbedoParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_color[0].to_bits().hash(state);
        self.base_color[1].to_bits().hash(state);
        self.base_color[2].to_bits().hash(state);
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

impl Default for RoofAlbedoParams {
    fn default() -> Self {
        Self {
            base_color: [0.55, 0.25, 0.14],
            shade_base: 0.68,
            shade_height_amp: 0.33,
            tint_var_r: 0.20,
            tint_var_g: 0.10,
            tint_var_b: 0.06,
            tint_base_r: 0.86,
            tint_base_g: 0.82,
            tint_base_b: 0.78,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RoofNormalParams {
    pub strength: f32,
}

impl Hash for RoofNormalParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.strength.to_bits().hash(state);
    }
}

impl Default for RoofNormalParams {
    fn default() -> Self {
        Self { strength: 4.4 }
    }
}

#[derive(Clone, Debug)]
pub struct RoofOrmParams {
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

impl Hash for RoofOrmParams {
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

impl Default for RoofOrmParams {
    fn default() -> Self {
        Self {
            ao_base: 0.70,
            ao_height_amp: 0.22,
            rough_base: 0.82,
            rough_height_amp: -0.10,
            rough_dust_freq: 18.0,
            rough_dust_oct: 2,
            rough_dust_amp: 0.10,
            rough_min: 0.58,
            rough_max: 0.95,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RoofParams {
    pub seed: u32,
    pub version: u32,
    pub height: RoofHeightParams,
    pub albedo: RoofAlbedoParams,
    pub normal: RoofNormalParams,
    pub orm: RoofOrmParams,
}

impl Hash for RoofParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.seed.hash(state);
        self.version.hash(state);
        self.height.hash(state);
        self.albedo.hash(state);
        self.normal.hash(state);
        self.orm.hash(state);
    }
}

impl Default for RoofParams {
    fn default() -> Self {
        Self {
            seed: 0,
            version: 1,
            height: RoofHeightParams::default(),
            albedo: RoofAlbedoParams::default(),
            normal: RoofNormalParams::default(),
            orm: RoofOrmParams::default(),
        }
    }
}

pub fn roof_height(params: &RoofParams, u: f32, v: f32) -> f32 {
    let h = &params.height;
    let courses = v * h.courses;
    let columns = u * h.columns
        + if courses.floor() as i32 % 2 == 0 {
            0.5
        } else {
            0.0
        };
    let gap = mortar_mask(courses, h.mortar_courses) * mortar_mask(columns, h.mortar_columns);
    let curved =
        ((courses.fract() * std::f32::consts::PI).sin() * h.curved_amp + h.curved_base).max(0.0);
    gap * curved + fbm(41 ^ params.seed, h.surface_freq, h.surface_oct, u, v) * h.surface_amp
}

pub fn roof_albedo(params: &RoofParams) -> Image {
    let seed = params.seed;
    let a = &params.albedo;
    let h = &params.height;
    build_albedo(
        a.base_color,
        |u, v| a.shade_base + roof_height(params, u, v) * a.shade_height_amp,
        |u, v| {
            let n = cell_noise(
                42 ^ seed,
                (u * h.columns).floor() as i32,
                (v * h.courses).floor() as i32,
            );
            [
                a.tint_base_r + n * a.tint_var_r,
                a.tint_base_g + n * a.tint_var_g,
                a.tint_base_b + n * a.tint_var_b,
            ]
        },
    )
}

pub fn roof_normal(params: &RoofParams) -> Image {
    build_normal(|u, v| roof_height(params, u, v), params.normal.strength)
}

pub fn roof_orm(params: &RoofParams) -> Image {
    let seed = params.seed;
    let o = &params.orm;
    build_orm(
        |u, v| {
            let h = roof_height(params, u, v);
            (o.ao_base + h * o.ao_height_amp).clamp(0.0, 1.0)
        },
        |u, v| {
            let h = roof_height(params, u, v);
            let dust = fbm(43 ^ seed, o.rough_dust_freq, o.rough_dust_oct, u, v);
            (o.rough_base + h * o.rough_height_amp + dust * o.rough_dust_amp)
                .clamp(o.rough_min, o.rough_max)
        },
        |_, _| 0.0,
    )
}
