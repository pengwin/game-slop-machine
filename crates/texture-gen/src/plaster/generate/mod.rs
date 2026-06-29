mod channels;
mod layers;
mod maps;
mod math;
mod rng;
mod stage;
mod texture_set;
mod tile_noise;

use crate::{GeneratedTexture, TextureColorSpace, TextureSize};

use super::params::PlasterParams;
use channels::{build_albedo, build_normal, build_orm};
use layers::{
    build_tileable_tone, compose_height, draw_hairline_cracks, draw_pits, draw_stain_blobs,
};
use maps::WorkingMaps;

pub use stage::PlasterGenerationStage;
pub use texture_set::PlasterTextureSet;

/// Generates all plaster PBR texture channels.
#[must_use]
pub fn generate_plaster_set(params: &PlasterParams, size: TextureSize) -> PlasterTextureSet {
    generate_plaster_set_with_progress(params, size, |_| {})
}

/// Generates all plaster PBR texture channels and reports completed stages.
pub fn generate_plaster_set_with_progress(
    params: &PlasterParams,
    size: TextureSize,
    mut on_stage_finished: impl FnMut(PlasterGenerationStage),
) -> PlasterTextureSet {
    let mut maps = WorkingMaps::new(size);

    build_tileable_tone(params, &mut maps);
    on_stage_finished(PlasterGenerationStage::Tone);

    draw_stain_blobs(params, &mut maps);
    on_stage_finished(PlasterGenerationStage::Stains);

    draw_pits(params, &mut maps);
    on_stage_finished(PlasterGenerationStage::Pits);

    draw_hairline_cracks(params, &mut maps);
    on_stage_finished(PlasterGenerationStage::Cracks);

    compose_height(params, &mut maps);
    on_stage_finished(PlasterGenerationStage::Height);

    let albedo = GeneratedTexture {
        width: size.width,
        height: size.height,
        data: build_albedo(params, &maps),
        color_space: TextureColorSpace::Srgb,
    };
    on_stage_finished(PlasterGenerationStage::Albedo);

    let normal = GeneratedTexture {
        width: size.width,
        height: size.height,
        data: build_normal(params, &maps),
        color_space: TextureColorSpace::Linear,
    };
    on_stage_finished(PlasterGenerationStage::Normal);

    let orm = GeneratedTexture {
        width: size.width,
        height: size.height,
        data: build_orm(params, &maps),
        color_space: TextureColorSpace::Linear,
    };
    on_stage_finished(PlasterGenerationStage::Orm);

    PlasterTextureSet {
        albedo,
        normal,
        orm,
    }
}
