use super::builders::{build_albedo, build_normal, build_orm};
use super::noise::fbm;
use bevy::prelude::*;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct ConcreteHeightParams {
    pub broad_freq: f64,
    pub broad_oct: usize,
    pub fine_freq: f64,
    pub fine_oct: usize,
    pub formwork_freq: f32,
    pub formwork_amp: f32,
    pub amp_broad: f32,
    pub amp_fine: f32,
}

impl Hash for ConcreteHeightParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.broad_freq.to_bits().hash(state);
        self.broad_oct.hash(state);
        self.fine_freq.to_bits().hash(state);
        self.fine_oct.hash(state);
        self.formwork_freq.to_bits().hash(state);
        self.formwork_amp.to_bits().hash(state);
        self.amp_broad.to_bits().hash(state);
        self.amp_fine.to_bits().hash(state);
    }
}

impl Default for ConcreteHeightParams {
    fn default() -> Self {
        Self {
            broad_freq: 8.0,
            broad_oct: 4,
            fine_freq: 35.0,
            fine_oct: 2,
            formwork_freq: 28.0,
            formwork_amp: 0.12,
            amp_broad: 0.65,
            amp_fine: 0.20,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ConcreteAlbedoParams {
    pub base_color: [f32; 3],
    pub broad_shade_freq: f64,
    pub broad_shade_oct: usize,
    pub broad_shade_amp: f32,
    pub pitting_freq: f64,
    pub pitting_oct: usize,
    pub pitting_amp: f32,
    pub crack_freq: f64,
    pub crack_oct: usize,
    pub crack_u_scale: f32,
    pub crack_v_scale: f32,
    pub crack_threshold: f32,
    pub crack_scale: f32,
    pub shade_base: f32,
    pub shade_height_amp: f32,
    pub shade_crack_amp: f32,
    pub shade_min: f32,
    pub shade_max: f32,
    pub stain_freq: f64,
    pub stain_oct: usize,
    pub stain_u_off: f32,
    pub stain_v_off: f32,
    pub stain_r_amp: f32,
    pub stain_g_amp: f32,
    pub stain_b_amp: f32,
    pub mineral_freq: f64,
    pub mineral_oct: usize,
    pub mineral_u_off: f32,
    pub mineral_v_off: f32,
    pub mineral_r_amp: f32,
    pub mineral_g_amp: f32,
    pub mineral_b_amp: f32,
    pub age_freq: f64,
    pub age_oct: usize,
    pub age_u_off: f32,
    pub age_v_off: f32,
    pub age_r_amp: f32,
    pub age_g_amp: f32,
    pub age_b_amp: f32,
}

impl Hash for ConcreteAlbedoParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_color[0].to_bits().hash(state);
        self.base_color[1].to_bits().hash(state);
        self.base_color[2].to_bits().hash(state);
        self.broad_shade_freq.to_bits().hash(state);
        self.broad_shade_oct.hash(state);
        self.broad_shade_amp.to_bits().hash(state);
        self.pitting_freq.to_bits().hash(state);
        self.pitting_oct.hash(state);
        self.pitting_amp.to_bits().hash(state);
        self.crack_freq.to_bits().hash(state);
        self.crack_oct.hash(state);
        self.crack_u_scale.to_bits().hash(state);
        self.crack_v_scale.to_bits().hash(state);
        self.crack_threshold.to_bits().hash(state);
        self.crack_scale.to_bits().hash(state);
        self.shade_base.to_bits().hash(state);
        self.shade_height_amp.to_bits().hash(state);
        self.shade_crack_amp.to_bits().hash(state);
        self.shade_min.to_bits().hash(state);
        self.shade_max.to_bits().hash(state);
        self.stain_freq.to_bits().hash(state);
        self.stain_oct.hash(state);
        self.stain_u_off.to_bits().hash(state);
        self.stain_v_off.to_bits().hash(state);
        self.stain_r_amp.to_bits().hash(state);
        self.stain_g_amp.to_bits().hash(state);
        self.stain_b_amp.to_bits().hash(state);
        self.mineral_freq.to_bits().hash(state);
        self.mineral_oct.hash(state);
        self.mineral_u_off.to_bits().hash(state);
        self.mineral_v_off.to_bits().hash(state);
        self.mineral_r_amp.to_bits().hash(state);
        self.mineral_g_amp.to_bits().hash(state);
        self.mineral_b_amp.to_bits().hash(state);
        self.age_freq.to_bits().hash(state);
        self.age_oct.hash(state);
        self.age_u_off.to_bits().hash(state);
        self.age_v_off.to_bits().hash(state);
        self.age_r_amp.to_bits().hash(state);
        self.age_g_amp.to_bits().hash(state);
        self.age_b_amp.to_bits().hash(state);
    }
}

impl Default for ConcreteAlbedoParams {
    fn default() -> Self {
        Self {
            base_color: [0.66, 0.65, 0.61],
            broad_shade_freq: 3.5,
            broad_shade_oct: 4,
            broad_shade_amp: 0.10,
            pitting_freq: 60.0,
            pitting_oct: 1,
            pitting_amp: 0.06,
            crack_freq: 14.0,
            crack_oct: 3,
            crack_u_scale: 1.4,
            crack_v_scale: 0.8,
            crack_threshold: 0.52,
            crack_scale: 18.0,
            shade_base: 0.82,
            shade_height_amp: 0.28,
            shade_crack_amp: 0.10,
            shade_min: 0.50,
            shade_max: 1.16,
            stain_freq: 4.0,
            stain_oct: 3,
            stain_u_off: 0.11,
            stain_v_off: -0.07,
            stain_r_amp: 0.025,
            stain_g_amp: 0.025,
            stain_b_amp: 0.020,
            mineral_freq: 7.0,
            mineral_oct: 2,
            mineral_u_off: -0.19,
            mineral_v_off: 0.23,
            mineral_r_amp: 0.015,
            mineral_g_amp: 0.010,
            mineral_b_amp: -0.005,
            age_freq: 12.0,
            age_oct: 2,
            age_u_off: 0.31,
            age_v_off: -0.15,
            age_r_amp: -0.018,
            age_g_amp: -0.018,
            age_b_amp: -0.012,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ConcreteNormalParams {
    pub strength: f32,
}

impl Hash for ConcreteNormalParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.strength.to_bits().hash(state);
    }
}

impl Default for ConcreteNormalParams {
    fn default() -> Self {
        Self { strength: 1.8 }
    }
}

#[derive(Clone, Debug)]
pub struct ConcreteOrmParams {
    pub ao_base: f32,
    pub ao_height_amp: f32,
    pub rough_base: f32,
    pub rough_height_amp: f32,
    pub rough_pitting_freq: f64,
    pub rough_pitting_oct: usize,
    pub rough_pitting_amp: f32,
    pub rough_min: f32,
    pub rough_max: f32,
}

impl Hash for ConcreteOrmParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ao_base.to_bits().hash(state);
        self.ao_height_amp.to_bits().hash(state);
        self.rough_base.to_bits().hash(state);
        self.rough_height_amp.to_bits().hash(state);
        self.rough_pitting_freq.to_bits().hash(state);
        self.rough_pitting_oct.hash(state);
        self.rough_pitting_amp.to_bits().hash(state);
        self.rough_min.to_bits().hash(state);
        self.rough_max.to_bits().hash(state);
    }
}

impl Default for ConcreteOrmParams {
    fn default() -> Self {
        Self {
            ao_base: 0.55,
            ao_height_amp: 0.45,
            rough_base: 0.92,
            rough_height_amp: -0.10,
            rough_pitting_freq: 60.0,
            rough_pitting_oct: 1,
            rough_pitting_amp: 0.04,
            rough_min: 0.75,
            rough_max: 0.98,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ConcreteParams {
    pub seed: u32,
    pub version: u32,
    pub height: ConcreteHeightParams,
    pub albedo: ConcreteAlbedoParams,
    pub normal: ConcreteNormalParams,
    pub orm: ConcreteOrmParams,
}

impl Hash for ConcreteParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.seed.hash(state);
        self.version.hash(state);
        self.height.hash(state);
        self.albedo.hash(state);
        self.normal.hash(state);
        self.orm.hash(state);
    }
}

impl Default for ConcreteParams {
    fn default() -> Self {
        Self {
            seed: 0,
            version: 1,
            height: ConcreteHeightParams::default(),
            albedo: ConcreteAlbedoParams::default(),
            normal: ConcreteNormalParams::default(),
            orm: ConcreteOrmParams::default(),
        }
    }
}

pub fn concrete_height(params: &ConcreteParams, u: f32, v: f32) -> f32 {
    let h = &params.height;
    let broad = fbm(71 ^ params.seed, h.broad_freq, h.broad_oct, u, v);
    let fine = fbm(72 ^ params.seed, h.fine_freq, h.fine_oct, u, v);
    let formwork = ((v * h.formwork_freq).sin() * 0.5 + 0.5) * h.formwork_amp;
    (broad * h.amp_broad + fine * h.amp_fine + formwork).clamp(0.0, 1.0)
}

pub fn concrete_albedo(params: &ConcreteParams) -> Image {
    let seed = params.seed;
    let a = &params.albedo;
    build_albedo(
        a.base_color,
        |u, v| {
            let h = concrete_height(params, u, v);
            let broad_shade = fbm(80 ^ seed, a.broad_shade_freq, a.broad_shade_oct, u, v);
            let pitting = fbm(81 ^ seed, a.pitting_freq, a.pitting_oct, u, v);
            let crack = fbm(
                82 ^ seed,
                a.crack_freq,
                a.crack_oct,
                u * a.crack_u_scale,
                v * a.crack_v_scale,
            );
            let crack_line = ((crack - a.crack_threshold).abs() * a.crack_scale).clamp(0.0, 1.0);

            let base = a.shade_base + h * a.shade_height_amp;
            let shade = base + broad_shade * a.broad_shade_amp
                - pitting * a.pitting_amp
                - (1.0 - crack_line) * a.shade_crack_amp;
            shade.clamp(a.shade_min, a.shade_max)
        },
        |u, v| {
            let stain = fbm(
                83 ^ seed,
                a.stain_freq,
                a.stain_oct,
                u + a.stain_u_off,
                v - a.stain_v_off.abs(),
            );
            let mineral = fbm(
                84 ^ seed,
                a.mineral_freq,
                a.mineral_oct,
                u - a.mineral_u_off.abs(),
                v + a.mineral_v_off,
            );
            let age = fbm(
                85 ^ seed,
                a.age_freq,
                a.age_oct,
                u + a.age_u_off,
                v - a.age_v_off.abs(),
            );

            [
                0.98 + stain * a.stain_r_amp + mineral * a.mineral_r_amp + age * a.age_r_amp,
                0.98 + stain * a.stain_g_amp + mineral * a.mineral_g_amp + age * a.age_g_amp,
                0.98 + stain * a.stain_b_amp + mineral * a.mineral_b_amp + age * a.age_b_amp,
            ]
        },
    )
}

pub fn concrete_normal(params: &ConcreteParams) -> Image {
    build_normal(|u, v| concrete_height(params, u, v), params.normal.strength)
}

pub fn concrete_orm(params: &ConcreteParams) -> Image {
    let seed = params.seed;
    let o = &params.orm;
    build_orm(
        |u, v| {
            let h = concrete_height(params, u, v);
            o.ao_base + h * o.ao_height_amp
        },
        |u, v| {
            let h = concrete_height(params, u, v);
            let pitting = fbm(81 ^ seed, o.rough_pitting_freq, o.rough_pitting_oct, u, v);
            (o.rough_base + h * o.rough_height_amp + pitting * o.rough_pitting_amp)
                .clamp(o.rough_min, o.rough_max)
        },
        |_, _| 0.0,
    )
}
