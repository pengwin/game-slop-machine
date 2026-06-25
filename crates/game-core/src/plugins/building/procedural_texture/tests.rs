use super::brick::*;
use super::builders::{
    RENDER_TEXTURE_SIZE, TEXTURE_SIZE, create_placeholder, flat_normal, flat_orm,
};
use super::concrete::*;
use super::floor::*;
use super::plaster::*;
use super::road::*;
use super::roof::*;
use super::stone::*;
use super::wood::*;
use bevy::prelude::*;
use bevy::render::render_resource::TextureFormat;

fn image_bytes(image: &Image) -> &[u8] {
    image
        .data
        .as_ref()
        .expect("procedural images keep CPU data")
}

#[test]
fn generated_images_have_expected_layout() {
    for (image, is_normal) in [
        (plaster_albedo(7), false),
        (plaster_preview_albedo(7), false),
        (plaster_normal(7), true),
        (plaster_orm(7), true),
        (wood_albedo(7), false),
        (wood_normal(7), true),
        (wood_orm(7), true),
        (brick_albedo(7), false),
        (brick_normal(7), true),
        (brick_orm(7), true),
        (roof_albedo(7), false),
        (roof_normal(7), true),
        (roof_orm(7), true),
        (stone_albedo(7), false),
        (stone_normal(7), true),
        (stone_orm(7), true),
        (road_albedo(7), false),
        (road_normal(7), true),
        (road_orm(7), true),
        (concrete_albedo(7), false),
        (concrete_normal(7), true),
        (concrete_orm(7), true),
        (floor_albedo(7), false),
        (floor_normal(7), true),
        (floor_orm(7), true),
        (flat_normal(), true),
        (flat_orm(1.0, 0.5, 0.0), true),
    ] {
        assert_eq!(image.texture_descriptor.size.width, TEXTURE_SIZE);
        assert_eq!(image.texture_descriptor.size.height, TEXTURE_SIZE);
        assert_eq!(
            image_bytes(&image).len(),
            (TEXTURE_SIZE * TEXTURE_SIZE * 4) as usize
        );
        assert_eq!(
            image.texture_descriptor.format,
            if is_normal {
                TextureFormat::Rgba8Unorm
            } else {
                TextureFormat::Rgba8UnormSrgb
            }
        );
    }
}

#[test]
fn runtime_texture_size_is_high_resolution() {
    assert_eq!(RENDER_TEXTURE_SIZE, 512);
    assert!(TEXTURE_SIZE <= RENDER_TEXTURE_SIZE);
}

#[test]
fn placeholders_match_texture_kind() {
    let albedo = create_placeholder(false);
    let normal = create_placeholder(true);

    assert!(
        image_bytes(&albedo)
            .chunks_exact(4)
            .all(|px| px == [128, 128, 128, 255])
    );
    assert_eq!(
        albedo.texture_descriptor.format,
        TextureFormat::Rgba8UnormSrgb
    );
    assert!(
        image_bytes(&normal)
            .chunks_exact(4)
            .all(|px| px == [128, 128, 255, 255])
    );
    assert_eq!(normal.texture_descriptor.format, TextureFormat::Rgba8Unorm);
}

#[test]
fn albedo_images_are_not_flat_fills() {
    for image in [
        plaster_albedo(11),
        plaster_preview_albedo(11),
        wood_albedo(11),
        brick_albedo(11),
        roof_albedo(11),
        stone_albedo(11),
        road_albedo(11),
        concrete_albedo(11),
        floor_albedo(11),
    ] {
        let bytes = image_bytes(&image);
        assert!(
            bytes.chunks_exact(4).any(|px| px != &bytes[0..4]),
            "albedo texture should contain visible variation"
        );
    }
}

#[test]
fn generation_is_deterministic_per_seed() {
    assert_eq!(image_bytes(&wood_albedo(5)), image_bytes(&wood_albedo(5)));
    assert_eq!(
        image_bytes(&plaster_normal(5)),
        image_bytes(&plaster_normal(5))
    );
    assert_eq!(image_bytes(&floor_albedo(5)), image_bytes(&floor_albedo(5)));
}
