use super::brick::BrickParams;
use super::builders::{
    RENDER_TEXTURE_SIZE, TEXTURE_SIZE, create_placeholder, flat_normal, flat_orm,
};
use super::concrete::ConcreteParams;
use super::floor::FloorParams;
use super::plaster::PlasterParams;
use super::road::RoadParams;
use super::roof::RoofParams;
use super::stone::StoneParams;
use super::wood::WoodParams;
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
    let plaster = PlasterParams {
        seed: 7,
        ..default()
    };
    let wood = WoodParams {
        seed: 7,
        ..default()
    };
    let brick = BrickParams {
        seed: 7,
        ..default()
    };
    let roof = RoofParams {
        seed: 7,
        ..default()
    };
    let stone = StoneParams {
        seed: 7,
        ..default()
    };
    let road = RoadParams {
        seed: 7,
        ..default()
    };
    let concrete = ConcreteParams {
        seed: 7,
        ..default()
    };
    let floor = FloorParams {
        seed: 7,
        ..default()
    };

    for (image, is_normal) in [
        (super::plaster::plaster_albedo(&plaster), false),
        (super::plaster::plaster_preview_albedo(&plaster), false),
        (super::plaster::plaster_normal(&plaster), true),
        (super::plaster::plaster_orm(&plaster), true),
        (super::wood::wood_albedo(&wood), false),
        (super::wood::wood_normal(&wood), true),
        (super::wood::wood_orm(&wood), true),
        (super::brick::brick_albedo(&brick), false),
        (super::brick::brick_normal(&brick), true),
        (super::brick::brick_orm(&brick), true),
        (super::roof::roof_albedo(&roof), false),
        (super::roof::roof_normal(&roof), true),
        (super::roof::roof_orm(&roof), true),
        (super::stone::stone_albedo(&stone), false),
        (super::stone::stone_normal(&stone), true),
        (super::stone::stone_orm(&stone), true),
        (super::road::road_albedo(&road), false),
        (super::road::road_normal(&road), true),
        (super::road::road_orm(&road), true),
        (super::concrete::concrete_albedo(&concrete), false),
        (super::concrete::concrete_normal(&concrete), true),
        (super::concrete::concrete_orm(&concrete), true),
        (super::floor::floor_albedo(&floor), false),
        (super::floor::floor_normal(&floor), true),
        (super::floor::floor_orm(&floor), true),
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
    let plaster = PlasterParams {
        seed: 11,
        ..default()
    };
    let wood = WoodParams {
        seed: 11,
        ..default()
    };
    let brick = BrickParams {
        seed: 11,
        ..default()
    };
    let roof = RoofParams {
        seed: 11,
        ..default()
    };
    let stone = StoneParams {
        seed: 11,
        ..default()
    };
    let road = RoadParams {
        seed: 11,
        ..default()
    };
    let concrete = ConcreteParams {
        seed: 11,
        ..default()
    };
    let floor = FloorParams {
        seed: 11,
        ..default()
    };

    for image in [
        super::plaster::plaster_albedo(&plaster),
        super::plaster::plaster_preview_albedo(&plaster),
        super::wood::wood_albedo(&wood),
        super::brick::brick_albedo(&brick),
        super::roof::roof_albedo(&roof),
        super::stone::stone_albedo(&stone),
        super::road::road_albedo(&road),
        super::concrete::concrete_albedo(&concrete),
        super::floor::floor_albedo(&floor),
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
    let wood5 = WoodParams {
        seed: 5,
        ..default()
    };
    let plaster5 = PlasterParams {
        seed: 5,
        ..default()
    };
    let floor5 = FloorParams {
        seed: 5,
        ..default()
    };

    assert_eq!(
        image_bytes(&super::wood::wood_albedo(&wood5)),
        image_bytes(&super::wood::wood_albedo(&wood5))
    );
    assert_eq!(
        image_bytes(&super::plaster::plaster_normal(&plaster5)),
        image_bytes(&super::plaster::plaster_normal(&plaster5))
    );
    assert_eq!(
        image_bytes(&super::floor::floor_albedo(&floor5)),
        image_bytes(&super::floor::floor_albedo(&floor5))
    );
}
