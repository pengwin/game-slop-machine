mod channels;
mod layers;
mod maps;
mod stage;

use crate::{GeneratedTexture, TextureColorSpace, TextureSize};
pub use crate::material::PbrTextureSet as ConcreteTextureSet;

use super::params::ConcreteParams;
use channels::{build_albedo, build_normal, build_orm};
use layers::{
    build_tileable_tone, build_formwork_marks, build_efflorescence, compose_height,
    draw_aggregate, draw_exposed_aggregate, draw_hairline_cracks, draw_stains, draw_voids,
};
use maps::WorkingMaps;

pub use stage::ConcreteGenerationStage;

/// Generates all concrete PBR texture channels.
#[must_use]
pub fn generate_concrete_set(params: &ConcreteParams, size: TextureSize) -> ConcreteTextureSet {
    generate_concrete_set_with_progress(params, size, |_| {})
}

/// Generates all concrete PBR texture channels and reports completed stages.
pub fn generate_concrete_set_with_progress(
    params: &ConcreteParams,
    size: TextureSize,
    on_stage_finished: impl FnMut(ConcreteGenerationStage),
) -> ConcreteTextureSet {
    let Some(texture_set) = generate_concrete_set_with_progress_and_cancellation(
        params,
        size,
        on_stage_finished,
        || false,
    ) else {
        unreachable!(
            "concrete generation cannot be cancelled when cancellation predicate is false"
        );
    };
    texture_set
}

/// Generates all concrete PBR texture channels, reports completed stages, and can stop early.
pub fn generate_concrete_set_with_progress_and_cancellation(
    params: &ConcreteParams,
    size: TextureSize,
    mut on_stage_finished: impl FnMut(ConcreteGenerationStage),
    should_cancel: impl Fn() -> bool,
) -> Option<ConcreteTextureSet> {
    let mut maps = WorkingMaps::new(size);

    if !build_tileable_tone(params, &mut maps, &should_cancel) {
        return None;
    }
    on_stage_finished(ConcreteGenerationStage::Tone);

    if !draw_aggregate(params, &mut maps, &should_cancel) {
        return None;
    }
    on_stage_finished(ConcreteGenerationStage::Aggregate);

    if !draw_voids(params, &mut maps, &should_cancel) {
        return None;
    }
    on_stage_finished(ConcreteGenerationStage::Voids);

    if !draw_exposed_aggregate(params, &mut maps, &should_cancel) {
        return None;
    }
    on_stage_finished(ConcreteGenerationStage::ExposedAggregate);

    if !draw_stains(params, &mut maps, &should_cancel) {
        return None;
    }
    on_stage_finished(ConcreteGenerationStage::Stains);

    if !build_formwork_marks(params, &mut maps, &should_cancel) {
        return None;
    }
    on_stage_finished(ConcreteGenerationStage::Formwork);

    if !draw_hairline_cracks(params, &mut maps, &should_cancel) {
        return None;
    }
    on_stage_finished(ConcreteGenerationStage::Cracks);

    if !build_efflorescence(params, &mut maps, &should_cancel) {
        return None;
    }
    on_stage_finished(ConcreteGenerationStage::Efflorescence);

    if !compose_height(params, &mut maps, &should_cancel) {
        return None;
    }
    on_stage_finished(ConcreteGenerationStage::Height);

    if should_cancel() {
        return None;
    }
    let albedo = GeneratedTexture {
        width: size.width,
        height: size.height,
        data: build_albedo(params, &maps),
        color_space: TextureColorSpace::Srgb,
    };
    on_stage_finished(ConcreteGenerationStage::Albedo);

    if should_cancel() {
        return None;
    }
    let normal = GeneratedTexture {
        width: size.width,
        height: size.height,
        data: build_normal(params, &maps),
        color_space: TextureColorSpace::Linear,
    };
    on_stage_finished(ConcreteGenerationStage::Normal);

    if should_cancel() {
        return None;
    }
    let orm = GeneratedTexture {
        width: size.width,
        height: size.height,
        data: build_orm(params, &maps),
        color_space: TextureColorSpace::Linear,
    };
    on_stage_finished(ConcreteGenerationStage::Orm);

    Some(ConcreteTextureSet {
        albedo,
        normal,
        orm,
    })
}
