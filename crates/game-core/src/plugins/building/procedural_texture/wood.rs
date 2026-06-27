use super::builders::{build_albedo, build_normal, build_orm};
use super::noise::{cell_noise, fbm, speckle};
use bevy::prelude::*;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct WoodHeightParams {
    pub warp_freq: f64,
    pub warp_oct: usize,
    pub warp_u_scale: f32,
    pub warp_v_scale: f32,
    pub slow_wander_freq: f64,
    pub slow_wander_oct: usize,
    pub slow_wander_u_scale: f32,
    pub slow_wander_v_scale: f32,
    pub slow_wander_amp: f32,
    pub broad_freq: f32,
    pub broad_warp_amp: f32,
    pub broad_exp: f32,
    pub fine_freq: f32,
    pub fine_warp_amp: f32,
    pub fine_exp: f32,
    pub extra_fine_freq: f32,
    pub extra_fine_warp_amp: f32,
    pub extra_fine_exp: f32,
    pub grain_freq: f64,
    pub grain_oct: usize,
    pub grain_u_scale: f32,
    pub grain_v_scale: f32,
    pub grain_warp_scale: f32,
    pub knot_freq: f64,
    pub knot_oct: usize,
    pub knot_u_scale: f32,
    pub knot_v_scale: f32,
    pub knot_exp: f32,
    pub pores_scale: f32,
    pub pores_threshold: f32,
    pub pores_u_scale: f32,
    pub pores_v_scale: f32,
    pub amp_broad: f32,
    pub amp_fine: f32,
    pub amp_extra_fine: f32,
    pub amp_grain: f32,
    pub amp_knot: f32,
    pub amp_pores: f32,
}

impl Hash for WoodHeightParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.warp_freq.to_bits().hash(state);
        self.warp_oct.hash(state);
        self.warp_u_scale.to_bits().hash(state);
        self.warp_v_scale.to_bits().hash(state);
        self.slow_wander_freq.to_bits().hash(state);
        self.slow_wander_oct.hash(state);
        self.slow_wander_u_scale.to_bits().hash(state);
        self.slow_wander_v_scale.to_bits().hash(state);
        self.slow_wander_amp.to_bits().hash(state);
        self.broad_freq.to_bits().hash(state);
        self.broad_warp_amp.to_bits().hash(state);
        self.broad_exp.to_bits().hash(state);
        self.fine_freq.to_bits().hash(state);
        self.fine_warp_amp.to_bits().hash(state);
        self.fine_exp.to_bits().hash(state);
        self.extra_fine_freq.to_bits().hash(state);
        self.extra_fine_warp_amp.to_bits().hash(state);
        self.extra_fine_exp.to_bits().hash(state);
        self.grain_freq.to_bits().hash(state);
        self.grain_oct.hash(state);
        self.grain_u_scale.to_bits().hash(state);
        self.grain_v_scale.to_bits().hash(state);
        self.grain_warp_scale.to_bits().hash(state);
        self.knot_freq.to_bits().hash(state);
        self.knot_oct.hash(state);
        self.knot_u_scale.to_bits().hash(state);
        self.knot_v_scale.to_bits().hash(state);
        self.knot_exp.to_bits().hash(state);
        self.pores_scale.to_bits().hash(state);
        self.pores_threshold.to_bits().hash(state);
        self.pores_u_scale.to_bits().hash(state);
        self.pores_v_scale.to_bits().hash(state);
        self.amp_broad.to_bits().hash(state);
        self.amp_fine.to_bits().hash(state);
        self.amp_extra_fine.to_bits().hash(state);
        self.amp_grain.to_bits().hash(state);
        self.amp_knot.to_bits().hash(state);
        self.amp_pores.to_bits().hash(state);
    }
}

impl Default for WoodHeightParams {
    fn default() -> Self {
        Self {
            warp_freq: 4.0,
            warp_oct: 3,
            warp_u_scale: 0.55,
            warp_v_scale: 1.3,
            slow_wander_freq: 2.0,
            slow_wander_oct: 2,
            slow_wander_u_scale: 0.45,
            slow_wander_v_scale: 0.95,
            slow_wander_amp: 1.8,
            broad_freq: 14.0,
            broad_warp_amp: 2.0,
            broad_exp: 1.65,
            fine_freq: 46.0,
            fine_warp_amp: 3.5,
            fine_exp: 2.7,
            extra_fine_freq: 122.0,
            extra_fine_warp_amp: 7.5,
            extra_fine_exp: 4.0,
            grain_freq: 8.0,
            grain_oct: 4,
            grain_u_scale: 2.6,
            grain_v_scale: 0.60,
            grain_warp_scale: 0.25,
            knot_freq: 3.7,
            knot_oct: 3,
            knot_u_scale: 1.5,
            knot_v_scale: 1.15,
            knot_exp: 5.0,
            pores_scale: 180.0,
            pores_threshold: 0.84,
            pores_u_scale: 0.35,
            pores_v_scale: 2.4,
            amp_broad: 0.30,
            amp_fine: 0.20,
            amp_extra_fine: 0.10,
            amp_grain: 0.21,
            amp_knot: 0.15,
            amp_pores: 0.08,
        }
    }
}

#[derive(Clone, Debug)]
pub struct WoodAlbedoParams {
    pub base_color: [f32; 3],
    pub shade_pores_scale: f32,
    pub shade_pores_threshold: f32,
    pub shade_pores_u_scale: f32,
    pub shade_pores_v_scale: f32,
    pub shade_min: f32,
    pub tint_bands: f32,
    pub tint_base_r: f32,
    pub tint_base_g: f32,
    pub tint_base_b: f32,
    pub tint_var_amp: f32,
}

impl Hash for WoodAlbedoParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_color[0].to_bits().hash(state);
        self.base_color[1].to_bits().hash(state);
        self.base_color[2].to_bits().hash(state);
        self.shade_pores_scale.to_bits().hash(state);
        self.shade_pores_threshold.to_bits().hash(state);
        self.shade_pores_u_scale.to_bits().hash(state);
        self.shade_pores_v_scale.to_bits().hash(state);
        self.shade_min.to_bits().hash(state);
        self.tint_bands.to_bits().hash(state);
        self.tint_base_r.to_bits().hash(state);
        self.tint_base_g.to_bits().hash(state);
        self.tint_base_b.to_bits().hash(state);
        self.tint_var_amp.to_bits().hash(state);
    }
}

impl Default for WoodAlbedoParams {
    fn default() -> Self {
        Self {
            base_color: [0.86, 0.58, 0.30],
            shade_pores_scale: 190.0,
            shade_pores_threshold: 0.86,
            shade_pores_u_scale: 0.45,
            shade_pores_v_scale: 2.2,
            shade_min: 0.30,
            tint_bands: 3.0,
            tint_base_r: 0.96,
            tint_base_g: 0.82,
            tint_base_b: 0.60,
            tint_var_amp: 0.08,
        }
    }
}

#[derive(Clone, Debug)]
pub struct WoodNormalParams {
    pub strength: f32,
}

impl Hash for WoodNormalParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.strength.to_bits().hash(state);
    }
}

impl Default for WoodNormalParams {
    fn default() -> Self {
        Self { strength: 2.1 }
    }
}

#[derive(Clone, Debug)]
pub struct WoodOrmParams {
    pub ao_base: f32,
    pub ao_height_amp: f32,
    pub rough_base: f32,
    pub rough_height_amp: f32,
    pub rough_pores_scale: f32,
    pub rough_pores_threshold: f32,
    pub rough_pores_u_scale: f32,
    pub rough_pores_v_scale: f32,
    pub rough_pores_amp: f32,
    pub rough_min: f32,
    pub rough_max: f32,
}

impl Hash for WoodOrmParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ao_base.to_bits().hash(state);
        self.ao_height_amp.to_bits().hash(state);
        self.rough_base.to_bits().hash(state);
        self.rough_height_amp.to_bits().hash(state);
        self.rough_pores_scale.to_bits().hash(state);
        self.rough_pores_threshold.to_bits().hash(state);
        self.rough_pores_u_scale.to_bits().hash(state);
        self.rough_pores_v_scale.to_bits().hash(state);
        self.rough_pores_amp.to_bits().hash(state);
        self.rough_min.to_bits().hash(state);
        self.rough_max.to_bits().hash(state);
    }
}

impl Default for WoodOrmParams {
    fn default() -> Self {
        Self {
            ao_base: 0.74,
            ao_height_amp: 0.18,
            rough_base: 0.82,
            rough_height_amp: -0.10,
            rough_pores_scale: 150.0,
            rough_pores_threshold: 0.82,
            rough_pores_u_scale: 0.4,
            rough_pores_v_scale: 2.0,
            rough_pores_amp: 0.08,
            rough_min: 0.56,
            rough_max: 0.95,
        }
    }
}

#[derive(Clone, Debug)]
pub struct WoodParams {
    pub seed: u32,
    pub version: u32,
    pub height: WoodHeightParams,
    pub albedo: WoodAlbedoParams,
    pub normal: WoodNormalParams,
    pub orm: WoodOrmParams,
}

impl Hash for WoodParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.seed.hash(state);
        self.version.hash(state);
        self.height.hash(state);
        self.albedo.hash(state);
        self.normal.hash(state);
        self.orm.hash(state);
    }
}

impl Default for WoodParams {
    fn default() -> Self {
        Self {
            seed: 0,
            version: 1,
            height: WoodHeightParams::default(),
            albedo: WoodAlbedoParams::default(),
            normal: WoodNormalParams::default(),
            orm: WoodOrmParams::default(),
        }
    }
}

pub fn wood_height(params: &WoodParams, u: f32, v: f32) -> f32 {
    let h = &params.height;
    let warp = fbm(
        22 ^ params.seed,
        h.warp_freq,
        h.warp_oct,
        u * h.warp_u_scale,
        v * h.warp_v_scale,
    );
    let slow_wander = fbm(
        24 ^ params.seed,
        h.slow_wander_freq,
        h.slow_wander_oct,
        u * h.slow_wander_u_scale,
        v * h.slow_wander_v_scale,
    ) * h.slow_wander_amp;
    let broad = ((u * h.broad_freq + warp * h.broad_warp_amp + slow_wander).sin() * 0.5 + 0.5)
        .powf(h.broad_exp);
    let fine = ((u * h.fine_freq + warp * h.fine_warp_amp).sin() * 0.5 + 0.5).powf(h.fine_exp);
    let extra_fine = ((u * h.extra_fine_freq + warp * h.extra_fine_warp_amp).sin() * 0.5 + 0.5)
        .powf(h.extra_fine_exp);
    let grain = fbm(
        21 ^ params.seed,
        h.grain_freq,
        h.grain_oct,
        u * h.grain_u_scale + warp * h.grain_warp_scale,
        v * h.grain_v_scale,
    );
    let knot = fbm(
        25 ^ params.seed,
        h.knot_freq,
        h.knot_oct,
        u * h.knot_u_scale + warp,
        v * h.knot_v_scale,
    )
    .powf(h.knot_exp);
    let pores = speckle(
        26 ^ params.seed,
        h.pores_scale,
        h.pores_threshold,
        u * h.pores_u_scale,
        v * h.pores_v_scale,
    ) * h.amp_pores;
    broad * h.amp_broad
        + fine * h.amp_fine
        + extra_fine * h.amp_extra_fine
        + grain * h.amp_grain
        + knot * h.amp_knot
        + pores
}

pub fn wood_albedo(params: &WoodParams) -> Image {
    let seed = params.seed;
    let a = &params.albedo;
    build_albedo(
        a.base_color,
        |u, v| {
            let h = wood_height(params, u, v);
            let pores = speckle(
                27 ^ seed,
                a.shade_pores_scale,
                a.shade_pores_threshold,
                u * a.shade_pores_u_scale,
                v * a.shade_pores_v_scale,
            );
            (0.66 + h * 0.42 - pores * 0.08).max(a.shade_min)
        },
        |_, v| {
            let band_id = (v * a.tint_bands).floor() as i32;
            let n = cell_noise(23 ^ seed, 0, band_id);
            [
                a.tint_base_r + n * a.tint_var_amp,
                a.tint_base_g + n * a.tint_var_amp * 0.875,
                a.tint_base_b + n * a.tint_var_amp * 0.75,
            ]
        },
    )
}

pub fn wood_normal(params: &WoodParams) -> Image {
    build_normal(|u, v| wood_height(params, u, v), params.normal.strength)
}

pub fn wood_orm(params: &WoodParams) -> Image {
    let seed = params.seed;
    let o = &params.orm;
    build_orm(
        |u, v| {
            let h = wood_height(params, u, v);
            (o.ao_base + h * o.ao_height_amp).clamp(0.0, 1.0)
        },
        |u, v| {
            let h = wood_height(params, u, v);
            let pores = speckle(
                28 ^ seed,
                o.rough_pores_scale,
                o.rough_pores_threshold,
                u * o.rough_pores_u_scale,
                v * o.rough_pores_v_scale,
            );
            (o.rough_base + h * o.rough_height_amp + pores * o.rough_pores_amp)
                .clamp(o.rough_min, o.rough_max)
        },
        |_, _| 0.0,
    )
}
