use std::cell::Cell;

use crate::{TEST_TEXTURE_SIZE, TextureColorSpace};

use super::{
    PlasterGenerationStage, PlasterParams, generate_plaster_set,
    generate_plaster_set_with_progress, generate_plaster_set_with_progress_and_cancellation,
};

fn params(seed: u32) -> PlasterParams {
    PlasterParams {
        seed,
        ..Default::default()
    }
}

#[test]
fn generated_set_has_expected_layout() {
    let texture_set = generate_plaster_set(&params(7), TEST_TEXTURE_SIZE);

    for (texture, expected_space) in [
        (&texture_set.albedo, TextureColorSpace::Srgb),
        (&texture_set.normal, TextureColorSpace::Linear),
        (&texture_set.orm, TextureColorSpace::Linear),
    ] {
        assert_eq!(texture.width, TEST_TEXTURE_SIZE.width);
        assert_eq!(texture.height, TEST_TEXTURE_SIZE.height);
        assert_eq!(texture.data.len(), TEST_TEXTURE_SIZE.rgba_len());
        assert_eq!(texture.color_space, expected_space);
    }
}

#[test]
fn generated_channels_are_not_flat() {
    let texture_set = generate_plaster_set(&params(11), TEST_TEXTURE_SIZE);

    for (name, data) in [
        ("albedo", texture_set.albedo.data.as_slice()),
        ("normal", texture_set.normal.data.as_slice()),
        ("orm", texture_set.orm.data.as_slice()),
    ] {
        let first = &data[0..4];
        assert!(
            data.chunks_exact(4).any(|pixel| pixel != first),
            "{name} texture should contain visible variation"
        );
    }
}

#[test]
fn generation_is_deterministic_per_seed() {
    let first = generate_plaster_set(&params(13), TEST_TEXTURE_SIZE);
    let second = generate_plaster_set(&params(13), TEST_TEXTURE_SIZE);

    assert_eq!(first, second);
}

#[test]
fn different_seeds_change_output() {
    let first = generate_plaster_set(&params(17), TEST_TEXTURE_SIZE);
    let second = generate_plaster_set(&params(18), TEST_TEXTURE_SIZE);

    assert_ne!(first.albedo.data, second.albedo.data);
    assert_ne!(first.normal.data, second.normal.data);
    assert_ne!(first.orm.data, second.orm.data);
}

#[test]
fn orm_metallic_channel_is_zero() {
    let texture_set = generate_plaster_set(&params(23), TEST_TEXTURE_SIZE);

    for pixel in texture_set.orm.data.chunks_exact(4) {
        assert_eq!(pixel[2], 0);
    }
}

#[test]
fn staged_generation_reports_expected_order() {
    let mut stages = Vec::new();
    let _ = generate_plaster_set_with_progress(&params(29), TEST_TEXTURE_SIZE, |stage| {
        stages.push(stage);
    });

    assert_eq!(
        stages,
        [
            PlasterGenerationStage::Tone,
            PlasterGenerationStage::Stains,
            PlasterGenerationStage::Pits,
            PlasterGenerationStage::Cracks,
            PlasterGenerationStage::Height,
            PlasterGenerationStage::Albedo,
            PlasterGenerationStage::Normal,
            PlasterGenerationStage::Orm,
        ]
    );
}

#[test]
fn cancellable_generation_stops_after_requested_stage() {
    let cancelled = Cell::new(false);
    let mut stages = Vec::new();
    let texture_set = generate_plaster_set_with_progress_and_cancellation(
        &params(31),
        TEST_TEXTURE_SIZE,
        |stage| {
            stages.push(stage);
            cancelled.set(true);
        },
        || cancelled.get(),
    );

    assert!(texture_set.is_none());
    assert_eq!(stages, [PlasterGenerationStage::Tone]);
}
