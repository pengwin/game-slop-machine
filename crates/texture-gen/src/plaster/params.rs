use std::hash::{Hash, Hasher};

/// Height/noise parameters for plaster relief.
#[derive(Clone, Debug)]
pub struct PlasterHeightParams {
    /// Broad unevenness frequency.
    pub broad_freq: f64,
    /// Broad unevenness octaves.
    pub broad_oct: usize,
    /// Broad unevenness amplitude.
    pub broad_amp: f32,
    /// Fine grain frequency.
    pub fine_freq: f64,
    /// Fine grain octaves.
    pub fine_oct: usize,
    /// Fine grain amplitude.
    pub fine_amp: f32,
    /// Large patch frequency.
    pub patches_freq: f64,
    /// Large patch octaves.
    pub patches_oct: usize,
    /// U offset for patch noise.
    pub patches_u_off: f32,
    /// V offset for patch noise.
    pub patches_v_off: f32,
    /// Patch amplitude.
    pub patches_amp: f32,
    /// Pit cell scale.
    pub pits_scale: f32,
    /// Pit threshold.
    pub pits_threshold: f32,
    /// Pit amplitude.
    pub pits_amp: f32,
    /// Hairline crack frequency.
    pub hair_freq: f64,
    /// Hairline crack width.
    pub hair_width: f32,
    /// Hairline warp amount.
    pub hair_warp: f32,
    /// Hairline height amplitude.
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

/// Shading parameters for plaster albedo.
#[derive(Clone, Debug)]
pub struct PlasterAlbedoShadeParams {
    /// Broad shade frequency.
    pub broad_freq: f64,
    /// Broad shade octaves.
    pub broad_oct: usize,
    /// Broad shade UV scale.
    pub broad_scale: f32,
    /// Broad shade U offset.
    pub broad_u_off: f32,
    /// Broad shade V offset.
    pub broad_v_off: f32,
    /// Stain noise frequency.
    pub stains_freq: f64,
    /// Stain noise octaves.
    pub stains_oct: usize,
    /// Stain warp amount.
    pub stains_warp: f32,
    /// Vertical streak frequency.
    pub streaks_freq: f64,
    /// Vertical streak octaves.
    pub streaks_oct: usize,
    /// Vertical streak U scale.
    pub streaks_u_scale: f32,
    /// Vertical streak V scale.
    pub streaks_v_scale: f32,
    /// Pore cell scale.
    pub pores_scale: f32,
    /// Pore threshold.
    pub pores_threshold: f32,
    /// Pore warp amount.
    pub pores_warp: f32,
    /// Pale sand speckle scale.
    pub pale_sand_scale: f32,
    /// Pale sand threshold.
    pub pale_sand_threshold: f32,
    /// Pale sand U offset.
    pub pale_sand_u_off: f32,
    /// Pale sand V offset.
    pub pale_sand_v_off: f32,
    /// Hairline albedo frequency.
    pub hair_freq: f64,
    /// Hairline albedo width.
    pub hair_width: f32,
    /// Hairline warp scale.
    pub hair_warp_scale: f32,
    /// Hairline U scale.
    pub hair_u_scale: f32,
    /// Hairline V scale.
    pub hair_v_scale: f32,
    /// Vertical darkening exponent.
    pub vertical_exp: f32,
    /// Vertical darkening amplitude.
    pub vertical_amp: f32,
    /// Base shade.
    pub shade_base: f32,
    /// Broad shade amplitude.
    pub shade_broad_amp: f32,
    /// Stain shade amplitude.
    pub shade_stains_amp: f32,
    /// Streak shade amplitude.
    pub shade_streaks_amp: f32,
    /// Pore shade amplitude.
    pub shade_pores_amp: f32,
    /// Pale speckle shade amplitude.
    pub shade_pale_amp: f32,
    /// Hairline shade amplitude.
    pub shade_hair_amp: f32,
    /// Minimum shade clamp.
    pub shade_min: f32,
    /// Maximum shade clamp.
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

/// Tint parameters for plaster albedo.
#[derive(Clone, Debug)]
pub struct PlasterAlbedoTintParams {
    /// Aging noise frequency.
    pub age_freq: f64,
    /// Aging noise octaves.
    pub age_oct: usize,
    /// Aging U offset.
    pub age_u_off: f32,
    /// Aging V offset.
    pub age_v_off: f32,
    /// Red aging amplitude.
    pub age_r_amp: f32,
    /// Green aging amplitude.
    pub age_g_amp: f32,
    /// Blue aging amplitude.
    pub age_b_amp: f32,
    /// Base red tint.
    pub base_r: f32,
    /// Base green tint.
    pub base_g: f32,
    /// Base blue tint.
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

/// Albedo parameters for plaster.
#[derive(Clone, Debug)]
pub struct PlasterAlbedoParams {
    /// Base plaster color in linear-ish RGB multipliers.
    pub base_color: [f32; 3],
    /// Shade parameters.
    pub shade: PlasterAlbedoShadeParams,
    /// Tint parameters.
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

/// Normal map parameters for plaster.
#[derive(Clone, Debug)]
pub struct PlasterNormalParams {
    /// Normal strength multiplier.
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

/// ORM parameters for plaster.
#[derive(Clone, Debug)]
pub struct PlasterOrmParams {
    /// Base ambient occlusion.
    pub ao_base: f32,
    /// Height contribution to ambient occlusion.
    pub ao_height_amp: f32,
    /// Base roughness.
    pub rough_base: f32,
    /// Height contribution to roughness.
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

/// Complete plaster generation parameters.
#[derive(Clone, Debug)]
pub struct PlasterParams {
    /// Deterministic texture seed.
    pub seed: u32,
    /// Version included in cache/hash keys by callers.
    pub version: u32,
    /// Height parameters.
    pub height: PlasterHeightParams,
    /// Albedo parameters.
    pub albedo: PlasterAlbedoParams,
    /// Normal parameters.
    pub normal: PlasterNormalParams,
    /// ORM parameters.
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
