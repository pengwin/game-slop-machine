use crate::{TEST_TEXTURE_SIZE, TextureColorSpace};

use super::{PlasterParams, TextureKind, generate_plaster_channel};

fn params(seed: u32) -> PlasterParams {
    PlasterParams {
        seed,
        ..Default::default()
    }
}

#[test]
fn generated_channels_have_expected_layout() {
    for (kind, expected_space) in [
        (TextureKind::Albedo, TextureColorSpace::Srgb),
        (TextureKind::Normal, TextureColorSpace::Linear),
        (TextureKind::Orm, TextureColorSpace::Linear),
    ] {
        let texture = generate_plaster_channel(&params(7), kind, TEST_TEXTURE_SIZE);

        assert_eq!(texture.width, TEST_TEXTURE_SIZE.width);
        assert_eq!(texture.height, TEST_TEXTURE_SIZE.height);
        assert_eq!(texture.data.len(), TEST_TEXTURE_SIZE.rgba_len());
        assert_eq!(texture.color_space, expected_space);
    }
}

#[test]
fn albedo_is_not_flat() {
    let texture = generate_plaster_channel(&params(11), TextureKind::Albedo, TEST_TEXTURE_SIZE);
    let first = &texture.data[0..4];

    assert!(
        texture.data.chunks_exact(4).any(|pixel| pixel != first),
        "albedo texture should contain visible variation"
    );
}

#[test]
fn generation_is_deterministic_per_seed() {
    let first = generate_plaster_channel(&params(13), TextureKind::Normal, TEST_TEXTURE_SIZE);
    let second = generate_plaster_channel(&params(13), TextureKind::Normal, TEST_TEXTURE_SIZE);

    assert_eq!(first.data, second.data);
}

#[test]
fn different_seeds_change_output() {
    let first = generate_plaster_channel(&params(17), TextureKind::Orm, TEST_TEXTURE_SIZE);
    let second = generate_plaster_channel(&params(18), TextureKind::Orm, TEST_TEXTURE_SIZE);

    assert_ne!(first.data, second.data);
}
