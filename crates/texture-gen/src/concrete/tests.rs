use std::cell::Cell;

use super::{
    ConcreteGenerationStage, ConcreteParams, generate_concrete_set,
    generate_concrete_set_with_progress, generate_concrete_set_with_progress_and_cancellation,
};
use crate::{TEST_TEXTURE_SIZE, TextureColorSpace};

#[test]
fn concrete_set_has_expected_channels() {
    let set = generate_concrete_set(&ConcreteParams::default(), TEST_TEXTURE_SIZE);

    assert_eq!(set.albedo.width, TEST_TEXTURE_SIZE.width);
    assert_eq!(set.albedo.height, TEST_TEXTURE_SIZE.height);
    assert_eq!(set.normal.width, TEST_TEXTURE_SIZE.width);
    assert_eq!(set.orm.width, TEST_TEXTURE_SIZE.width);
    assert_eq!(set.albedo.color_space, TextureColorSpace::Srgb);
    assert_eq!(set.normal.color_space, TextureColorSpace::Linear);
    assert_eq!(set.orm.color_space, TextureColorSpace::Linear);
}

#[test]
fn concrete_channels_are_not_flat() {
    let set = generate_concrete_set(&ConcreteParams::default(), TEST_TEXTURE_SIZE);

    assert!(has_variation(&set.albedo.data));
    assert!(has_variation(&set.normal.data));
    assert!(has_variation(&set.orm.data));
}

#[test]
fn concrete_generation_is_deterministic_per_seed() {
    let params = ConcreteParams::default();
    let a = generate_concrete_set(&params, TEST_TEXTURE_SIZE);
    let b = generate_concrete_set(&params, TEST_TEXTURE_SIZE);

    assert_eq!(a, b);
}

#[test]
fn concrete_generation_changes_with_seed() {
    let mut a_params = ConcreteParams::default();
    let mut b_params = ConcreteParams::default();
    a_params.seed = 1;
    b_params.seed = 2;

    let a = generate_concrete_set(&a_params, TEST_TEXTURE_SIZE);
    let b = generate_concrete_set(&b_params, TEST_TEXTURE_SIZE);

    assert_ne!(a.albedo.data, b.albedo.data);
}

#[test]
fn concrete_orm_metallic_channel_is_zero() {
    let set = generate_concrete_set(&ConcreteParams::default(), TEST_TEXTURE_SIZE);

    for pixel in set.orm.data.chunks_exact(4) {
        assert_eq!(pixel[2], 0);
    }
}

#[test]
fn concrete_progress_reports_expected_stage_order() {
    let mut stages = Vec::new();

    generate_concrete_set_with_progress(&ConcreteParams::default(), TEST_TEXTURE_SIZE, |stage| {
        stages.push(stage);
    });

    assert_eq!(
        stages,
        [
            ConcreteGenerationStage::Tone,
            ConcreteGenerationStage::Aggregate,
            ConcreteGenerationStage::Voids,
            ConcreteGenerationStage::Stains,
            ConcreteGenerationStage::Cracks,
            ConcreteGenerationStage::Height,
            ConcreteGenerationStage::Albedo,
            ConcreteGenerationStage::Normal,
            ConcreteGenerationStage::Orm,
        ]
    );
}

#[test]
fn concrete_generation_can_cancel_before_final_set() {
    let calls = Cell::new(0);
    let result = generate_concrete_set_with_progress_and_cancellation(
        &ConcreteParams::default(),
        TEST_TEXTURE_SIZE,
        |_| {},
        || {
            calls.set(calls.get() + 1);
            calls.get() > 1
        },
    );

    assert!(result.is_none());
}

fn has_variation(data: &[u8]) -> bool {
    data.first()
        .is_some_and(|first| data.iter().any(|value| value != first))
}
