use super::builders::{build_albedo, build_normal, build_orm};
use super::noise::{fbm, hairline, speckle};
use bevy::prelude::*;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct PlasterHeightParams {
    pub broad_freq: f64,
    pub broad_oct: usize,
    pub broad_amp: f32,
    pub fine_freq: f64,
    pub fine_oct: usize,
    pub fine_amp: f32,
    pub patches_freq: f64,
    pub patches_oct: usize,
    pub patches_u_off: f32,
    pub patches_v_off: f32,
    pub patches_amp: f32,
    pub pits_scale: f32,
    pub pits_threshold: f32,
    pub pits_amp: f32,
    pub hair_freq: f64,
    pub hair_width: f32,
    pub hair_warp: f32,
    pub hair_amp: f32,
}

impl Hash for PlasterHeightParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.broad_freq.to_bits().hash(state);
        self.broad_oct.hash(state);
        self.broad_amp.to_bits().hash(state);
        self.fine_freq.to_bits().hash(state);
        self.fine_oct.hash(state);
        self.fine_amp.to_bits().hash(state);
        self.patches_freq.to_bits().hash(state);
        self.patches_oct.hash(state);
        self.patches_u_off.to_bits().hash(state);
        self.patches_v_off.to_bits().hash(state);
        self.patches_amp.to_bits().hash(state);
        self.pits_scale.to_bits().hash(state);
        self.pits_threshold.to_bits().hash(state);
        self.pits_amp.to_bits().hash(state);
        self.hair_freq.to_bits().hash(state);
        self.hair_width.to_bits().hash(state);
        self.hair_warp.to_bits().hash(state);
        self.hair_amp.to_bits().hash(state);
    }
}

impl Default for PlasterHeightParams {
    fn default() -> Self {
        Self {
            broad_freq: 5.0,
            broad_oct: 5,
            broad_amp: 0.38,
            fine_freq: 38.0,
            fine_oct: 3,
            fine_amp: 0.22,
            patches_freq: 2.2,
            patches_oct: 4,
            patches_u_off: 0.19,
            patches_v_off: -0.31,
            patches_amp: 0.22,
            pits_scale: 82.0,
            pits_threshold: 0.76,
            pits_amp: 0.10,
            hair_freq: 18.0,
            hair_width: 0.018,
            hair_warp: 0.04,
            hair_amp: 0.06,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PlasterAlbedoShadeParams {
    pub broad_freq: f64,
    pub broad_oct: usize,
    pub broad_scale: f32,
    pub broad_u_off: f32,
    pub broad_v_off: f32,
    pub stains_freq: f64,
    pub stains_oct: usize,
    pub stains_warp: f32,
    pub streaks_freq: f64,
    pub streaks_oct: usize,
    pub streaks_u_scale: f32,
    pub streaks_v_scale: f32,
    pub pores_scale: f32,
    pub pores_threshold: f32,
    pub pores_warp: f32,
    pub pale_sand_scale: f32,
    pub pale_sand_threshold: f32,
    pub pale_sand_u_off: f32,
    pub pale_sand_v_off: f32,
    pub hair_freq: f64,
    pub hair_width: f32,
    pub hair_warp_scale: f32,
    pub hair_u_scale: f32,
    pub hair_v_scale: f32,
    pub vertical_exp: f32,
    pub vertical_amp: f32,
    pub shade_base: f32,
    pub shade_broad_amp: f32,
    pub shade_stains_amp: f32,
    pub shade_streaks_amp: f32,
    pub shade_pores_amp: f32,
    pub shade_pale_amp: f32,
    pub shade_hair_amp: f32,
    pub shade_min: f32,
    pub shade_max: f32,
}

impl Hash for PlasterAlbedoShadeParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.broad_freq.to_bits().hash(state);
        self.broad_oct.hash(state);
        self.broad_scale.to_bits().hash(state);
        self.broad_u_off.to_bits().hash(state);
        self.broad_v_off.to_bits().hash(state);
        self.stains_freq.to_bits().hash(state);
        self.stains_oct.hash(state);
        self.stains_warp.to_bits().hash(state);
        self.streaks_freq.to_bits().hash(state);
        self.streaks_oct.hash(state);
        self.streaks_u_scale.to_bits().hash(state);
        self.streaks_v_scale.to_bits().hash(state);
        self.pores_scale.to_bits().hash(state);
        self.pores_threshold.to_bits().hash(state);
        self.pores_warp.to_bits().hash(state);
        self.pale_sand_scale.to_bits().hash(state);
        self.pale_sand_threshold.to_bits().hash(state);
        self.pale_sand_u_off.to_bits().hash(state);
        self.pale_sand_v_off.to_bits().hash(state);
        self.hair_freq.to_bits().hash(state);
        self.hair_width.to_bits().hash(state);
        self.hair_warp_scale.to_bits().hash(state);
        self.hair_u_scale.to_bits().hash(state);
        self.hair_v_scale.to_bits().hash(state);
        self.vertical_exp.to_bits().hash(state);
        self.vertical_amp.to_bits().hash(state);
        self.shade_base.to_bits().hash(state);
        self.shade_broad_amp.to_bits().hash(state);
        self.shade_stains_amp.to_bits().hash(state);
        self.shade_streaks_amp.to_bits().hash(state);
        self.shade_pores_amp.to_bits().hash(state);
        self.shade_pale_amp.to_bits().hash(state);
        self.shade_hair_amp.to_bits().hash(state);
        self.shade_min.to_bits().hash(state);
        self.shade_max.to_bits().hash(state);
    }
}

impl Default for PlasterAlbedoShadeParams {
    fn default() -> Self {
        Self {
            broad_freq: 2.6,
            broad_oct: 5,
            broad_scale: 0.85,
            broad_u_off: 0.13,
            broad_v_off: -0.07,
            stains_freq: 7.0,
            stains_oct: 3,
            stains_warp: 0.18,
            streaks_freq: 22.0,
            streaks_oct: 2,
            streaks_u_scale: 0.35,
            streaks_v_scale: 1.8,
            pores_scale: 96.0,
            pores_threshold: 0.72,
            pores_warp: 0.03,
            pale_sand_scale: 130.0,
            pale_sand_threshold: 0.84,
            pale_sand_u_off: -0.27,
            pale_sand_v_off: 0.16,
            hair_freq: 24.0,
            hair_width: 0.022,
            hair_warp_scale: 0.08,
            hair_u_scale: 0.45,
            hair_v_scale: 1.6,
            vertical_exp: 1.4,
            vertical_amp: 0.10,
            shade_base: 0.76,
            shade_broad_amp: 0.08,
            shade_stains_amp: 0.20,
            shade_streaks_amp: 1.0,
            shade_pores_amp: 0.080,
            shade_pale_amp: 0.055,
            shade_hair_amp: 0.090,
            shade_min: 0.36,
            shade_max: 1.12,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PlasterAlbedoTintParams {
    pub age_freq: f64,
    pub age_oct: usize,
    pub age_u_off: f32,
    pub age_v_off: f32,
    pub age_r_amp: f32,
    pub age_g_amp: f32,
    pub age_b_amp: f32,
    pub base_r: f32,
    pub base_g: f32,
    pub base_b: f32,
}

impl Hash for PlasterAlbedoTintParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.age_freq.to_bits().hash(state);
        self.age_oct.hash(state);
        self.age_u_off.to_bits().hash(state);
        self.age_v_off.to_bits().hash(state);
        self.age_r_amp.to_bits().hash(state);
        self.age_g_amp.to_bits().hash(state);
        self.age_b_amp.to_bits().hash(state);
        self.base_r.to_bits().hash(state);
        self.base_g.to_bits().hash(state);
        self.base_b.to_bits().hash(state);
    }
}

impl Default for PlasterAlbedoTintParams {
    fn default() -> Self {
        Self {
            age_freq: 4.2,
            age_oct: 4,
            age_u_off: -0.23,
            age_v_off: 0.19,
            age_r_amp: 0.10,
            age_g_amp: 0.06,
            age_b_amp: 0.04,
            base_r: 0.97,
            base_g: 0.94,
            base_b: 0.86,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PlasterAlbedoParams {
    pub base_color: [f32; 3],
    pub shade: PlasterAlbedoShadeParams,
    pub tint: PlasterAlbedoTintParams,
}

impl Hash for PlasterAlbedoParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_color[0].to_bits().hash(state);
        self.base_color[1].to_bits().hash(state);
        self.base_color[2].to_bits().hash(state);
        self.shade.hash(state);
        self.tint.hash(state);
    }
}

impl Default for PlasterAlbedoParams {
    fn default() -> Self {
        Self {
            base_color: [0.95, 0.88, 0.70],
            shade: PlasterAlbedoShadeParams::default(),
            tint: PlasterAlbedoTintParams::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PlasterNormalParams {
    pub strength: f32,
}

impl Hash for PlasterNormalParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.strength.to_bits().hash(state);
    }
}

impl Default for PlasterNormalParams {
    fn default() -> Self {
        Self { strength: 0.65 }
    }
}

#[derive(Clone, Debug)]
pub struct PlasterOrmParams {
    pub ao_base: f32,
    pub ao_height_amp: f32,
    pub rough_base: f32,
    pub rough_height_amp: f32,
}

impl Hash for PlasterOrmParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ao_base.to_bits().hash(state);
        self.ao_height_amp.to_bits().hash(state);
        self.rough_base.to_bits().hash(state);
        self.rough_height_amp.to_bits().hash(state);
    }
}

impl Default for PlasterOrmParams {
    fn default() -> Self {
        Self {
            ao_base: 0.94,
            ao_height_amp: -0.04,
            rough_base: 0.98,
            rough_height_amp: -0.08,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PlasterParams {
    pub seed: u32,
    pub version: u32,
    pub height: PlasterHeightParams,
    pub albedo: PlasterAlbedoParams,
    pub normal: PlasterNormalParams,
    pub orm: PlasterOrmParams,
}

impl Hash for PlasterParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.seed.hash(state);
        self.version.hash(state);
        self.height.hash(state);
        self.albedo.hash(state);
        self.normal.hash(state);
        self.orm.hash(state);
    }
}

impl Default for PlasterParams {
    fn default() -> Self {
        Self {
            seed: 0,
            version: 1,
            height: PlasterHeightParams::default(),
            albedo: PlasterAlbedoParams::default(),
            normal: PlasterNormalParams::default(),
            orm: PlasterOrmParams::default(),
        }
    }
}

pub fn plaster_height(params: &PlasterParams, u: f32, v: f32) -> f32 {
    let h = &params.height;
    let broad = fbm(11 ^ params.seed, h.broad_freq, h.broad_oct, u, v) * h.broad_amp;
    let fine = fbm(12 ^ params.seed, h.fine_freq, h.fine_oct, u, v) * h.fine_amp;
    let patches = fbm(
        13 ^ params.seed,
        h.patches_freq,
        h.patches_oct,
        u + h.patches_u_off,
        v - h.patches_v_off,
    ) * h.patches_amp;
    let pits = speckle(18 ^ params.seed, h.pits_scale, h.pits_threshold, u, v) * h.pits_amp;
    let hair = hairline(
        19 ^ params.seed,
        h.hair_freq,
        h.hair_width,
        u + broad * h.hair_warp,
        v - broad * h.hair_warp,
    ) * h.hair_amp;
    (broad + fine + patches + pits + hair).clamp(0.0, 1.0)
}

pub fn plaster_albedo(params: &PlasterParams) -> Image {
    let seed = params.seed;
    let s = &params.albedo.shade;
    let t = &params.albedo.tint;
    build_albedo(
        params.albedo.base_color,
        |u, v| {
            let broad = fbm(
                14 ^ seed,
                s.broad_freq,
                s.broad_oct,
                u * s.broad_scale + s.broad_u_off,
                v * s.broad_scale + s.broad_v_off,
            );
            let stains = fbm(
                15 ^ seed,
                s.stains_freq,
                s.stains_oct,
                u + broad * s.stains_warp,
                v,
            );
            let streaks = fbm(
                16 ^ seed,
                s.streaks_freq,
                s.streaks_oct,
                u * s.streaks_u_scale,
                v * s.streaks_v_scale,
            );
            let pores = speckle(
                20 ^ seed,
                s.pores_scale,
                s.pores_threshold,
                u + broad * s.pores_warp,
                v,
            );
            let pale_sand = speckle(
                21 ^ seed,
                s.pale_sand_scale,
                s.pale_sand_threshold,
                u - s.pale_sand_u_off.abs(),
                v + s.pale_sand_v_off,
            );
            let hair_cracks = hairline(
                22 ^ seed,
                s.hair_freq,
                s.hair_width,
                u * s.hair_u_scale + broad * s.hair_warp_scale,
                v * s.hair_v_scale,
            );
            let vertical = (1.0 - v.fract()).powf(s.vertical_exp) * s.vertical_amp;
            let height_val = plaster_height(params, u, v);
            let base = s.shade_base + height_val * (1.0 - s.shade_base) + broad * s.shade_broad_amp
                - stains * s.shade_stains_amp
                - streaks * vertical * s.shade_streaks_amp
                - pores * s.shade_pores_amp
                + pale_sand * s.shade_pale_amp
                - hair_cracks.powf(2.0) * s.shade_hair_amp;
            base.clamp(s.shade_min, s.shade_max)
        },
        |u, v| {
            let age = fbm(
                17 ^ seed,
                t.age_freq,
                t.age_oct,
                u - t.age_u_off.abs(),
                v + t.age_v_off,
            );
            [
                t.base_r + age * t.age_r_amp,
                t.base_g + age * t.age_g_amp,
                t.base_b + age * t.age_b_amp,
            ]
        },
    )
}

pub fn plaster_preview_albedo(params: &PlasterParams) -> Image {
    let seed = params.seed;
    build_albedo(
        params.albedo.base_color,
        |u, v| {
            let broad = fbm(90 ^ seed, 2.3, 5, u * 0.82 + 0.13, v * 0.82 - 0.07);
            let cloudy = fbm(91 ^ seed, 5.4, 4, u + broad * 0.08, v - broad * 0.06);
            let fine = fbm(92 ^ seed, 24.0, 2, u, v);
            let pores = speckle(98 ^ seed, 82.0, 0.78, u, v);
            let scratches = hairline(99 ^ seed, 15.0, 0.016, u + broad * 0.03, v - broad * 0.03);

            let base_shade = 0.98 + broad * 0.055 + cloudy * 0.040 + fine * 0.014
                - pores * 0.040
                - scratches.powf(2.0) * 0.030;
            base_shade.clamp(0.78, 1.12)
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

pub fn plaster_normal(params: &PlasterParams) -> Image {
    build_normal(|u, v| plaster_height(params, u, v), params.normal.strength)
}

pub fn plaster_orm(params: &PlasterParams) -> Image {
    let o = &params.orm;
    build_orm(
        |u, v| {
            let h = plaster_height(params, u, v);
            o.ao_base + h * o.ao_height_amp
        },
        |u, v| {
            let h = plaster_height(params, u, v);
            o.rough_base + h * o.rough_height_amp
        },
        |_, _| 0.0,
    )
}
