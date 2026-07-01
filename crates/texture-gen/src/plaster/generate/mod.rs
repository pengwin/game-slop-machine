mod channels;
mod layers;
mod maps;
mod stage;

pub use crate::material::PbrTextureSet as PlasterTextureSet;
use crate::{GeneratedTexture, TextureColorSpace, TextureSize};

use super::params::PlasterParams;
use channels::{build_albedo, build_normal, build_orm};
use layers::{
    build_tileable_tone, compose_height, draw_hairline_cracks, draw_pits, draw_stain_blobs,
};
use maps::WorkingMaps;

pub use stage::PlasterGenerationStage;

/// Generates all plaster PBR texture channels.
#[must_use]
pub fn generate_plaster_set(params: &PlasterParams, size: TextureSize) -> PlasterTextureSet {
    generate_plaster_set_with_progress(params, size, |_| {})
}

/// Generates all plaster PBR texture channels and reports completed stages.
pub fn generate_plaster_set_with_progress(
    params: &PlasterParams,
    size: TextureSize,
    on_stage_finished: impl FnMut(PlasterGenerationStage),
) -> PlasterTextureSet {
    let Some(texture_set) = generate_plaster_set_with_progress_and_cancellation(
        params,
        size,
        on_stage_finished,
        || false,
    ) else {
        unreachable!("plaster generation cannot be cancelled when cancellation predicate is false");
    };
    texture_set
}

/// Generates all plaster PBR texture channels, reports completed stages, and can stop early.
pub fn generate_plaster_set_with_progress_and_cancellation(
    params: &PlasterParams,
    size: TextureSize,
    mut on_stage_finished: impl FnMut(PlasterGenerationStage),
    should_cancel: impl Fn() -> bool,
) -> Option<PlasterTextureSet> {
    let mut maps = WorkingMaps::new(size);

    if !build_tileable_tone(params, &mut maps, &should_cancel) {
        return None;
    }
    on_stage_finished(PlasterGenerationStage::Tone);

    if !draw_stain_blobs(params, &mut maps, &should_cancel) {
        return None;
    }
    on_stage_finished(PlasterGenerationStage::Stains);

    if !draw_pits(params, &mut maps, &should_cancel) {
        return None;
    }
    on_stage_finished(PlasterGenerationStage::Pits);

    if !draw_hairline_cracks(params, &mut maps, &should_cancel) {
        return None;
    }
    on_stage_finished(PlasterGenerationStage::Cracks);

    if !compose_height(params, &mut maps, &should_cancel) {
        return None;
    }
    on_stage_finished(PlasterGenerationStage::Height);

    if should_cancel() {
        return None;
    }
    let albedo = GeneratedTexture {
        width: size.width,
        height: size.height,
        data: build_albedo(params, &maps),
        color_space: TextureColorSpace::Srgb,
    };
    on_stage_finished(PlasterGenerationStage::Albedo);

    if should_cancel() {
        return None;
    }
    let normal = GeneratedTexture {
        width: size.width,
        height: size.height,
        data: build_normal(params, &maps),
        color_space: TextureColorSpace::Linear,
    };
    on_stage_finished(PlasterGenerationStage::Normal);

    if should_cancel() {
        return None;
    }
    let orm = GeneratedTexture {
        width: size.width,
        height: size.height,
        data: build_orm(params, &maps),
        color_space: TextureColorSpace::Linear,
    };
    on_stage_finished(PlasterGenerationStage::Orm);

    Some(PlasterTextureSet {
        albedo,
        normal,
        orm,
    })
}
