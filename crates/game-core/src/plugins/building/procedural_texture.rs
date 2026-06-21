use bevy::asset::RenderAssetUsages;
use bevy::image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};

const TEXTURE_SIZE: u32 = 256;

#[derive(Resource, Clone)]
pub struct ProceduralTextures {
    pub plaster_albedo: Handle<Image>,
    pub plaster_normal: Handle<Image>,
    pub wood_albedo: Handle<Image>,
    pub wood_normal: Handle<Image>,
    pub brick_albedo: Handle<Image>,
    pub brick_normal: Handle<Image>,
    pub roof_albedo: Handle<Image>,
    pub roof_normal: Handle<Image>,
    pub stone_albedo: Handle<Image>,
    pub stone_normal: Handle<Image>,
    pub road_albedo: Handle<Image>,
    pub road_normal: Handle<Image>,
}

pub fn generate_textures(images: &mut Assets<Image>) -> ProceduralTextures {
    info!("Generating procedural textures...");
    
    info!("  -> Generating plaster textures");
    let plaster_a = plaster_albedo();
    let plaster_n = plaster_normal();
    
    info!("  -> Generating wood textures");
    let wood_a = wood_albedo();
    let wood_n = wood_normal();
    
    info!("  -> Generating brick textures");
    let brick_a = brick_albedo();
    let brick_n = brick_normal();
    
    info!("  -> Generating roof textures");
    let roof_a = roof_albedo();
    let roof_n = roof_normal();
    
    info!("  -> Generating stone textures");
    let stone_a = stone_albedo();
    let stone_n = stone_normal();
    
    info!("  -> Generating road textures");
    let road_a = road_albedo();
    let road_n = road_normal();

    info!("Procedural textures generated successfully.");

    ProceduralTextures {
        plaster_albedo: images.add(plaster_a),
        plaster_normal: images.add(plaster_n),
        wood_albedo: images.add(wood_a),
        wood_normal: images.add(wood_n),
        brick_albedo: images.add(brick_a),
        brick_normal: images.add(brick_n),
        roof_albedo: images.add(roof_a),
        roof_normal: images.add(roof_n),
        stone_albedo: images.add(stone_a),
        stone_normal: images.add(stone_n),
        road_albedo: images.add(road_a),
        road_normal: images.add(road_n),
    }
}

fn create_image(width: u32, height: u32, data: Vec<u8>, is_normal: bool) -> Image {
    let mut image = Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        if is_normal {
            TextureFormat::Rgba8Unorm
        } else {
            TextureFormat::Rgba8UnormSrgb
        },
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    image.sampler = ImageSampler::Descriptor(repeating_linear_sampler());
    image
}

fn repeating_linear_sampler() -> ImageSamplerDescriptor {
    ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        ..default()
    }
}

fn build_albedo(
    base: [f32; 3],
    shade: impl Fn(f32, f32) -> f32,
    tint: impl Fn(f32, f32) -> [f32; 3],
) -> Image {
    let mut data = vec![255; (TEXTURE_SIZE * TEXTURE_SIZE * 4) as usize];
    for y in 0..TEXTURE_SIZE {
        for x in 0..TEXTURE_SIZE {
            let u = x as f32 / TEXTURE_SIZE as f32;
            let v = y as f32 / TEXTURE_SIZE as f32;
            let s = shade(u, v).clamp(0.0, 1.5);
            let t = tint(u, v);
            write_rgba(
                &mut data,
                x,
                y,
                [
                    (base[0] * t[0] * s).clamp(0.0, 1.0),
                    (base[1] * t[1] * s).clamp(0.0, 1.0),
                    (base[2] * t[2] * s).clamp(0.0, 1.0),
                    1.0,
                ],
            );
        }
    }
    create_image(TEXTURE_SIZE, TEXTURE_SIZE, data, false)
}

fn build_normal(height: impl Fn(f32, f32) -> f32, strength: f32) -> Image {
    let mut data = vec![255; (TEXTURE_SIZE * TEXTURE_SIZE * 4) as usize];
    let eps = 1.0 / TEXTURE_SIZE as f32;

    for y in 0..TEXTURE_SIZE {
        for x in 0..TEXTURE_SIZE {
            let u = x as f32 / TEXTURE_SIZE as f32;
            let v = y as f32 / TEXTURE_SIZE as f32;
            let dx = height(u + eps, v) - height(u - eps, v);
            let dy = height(u, v + eps) - height(u, v - eps);
            let n = Vec3::new(-dx * strength, -dy * strength, 1.0).normalize_or_zero();
            write_rgba(
                &mut data,
                x,
                y,
                [n.x * 0.5 + 0.5, n.y * 0.5 + 0.5, n.z * 0.5 + 0.5, 1.0],
            );
        }
    }
    create_image(TEXTURE_SIZE, TEXTURE_SIZE, data, true)
}

fn write_rgba(data: &mut [u8], x: u32, y: u32, rgba: [f32; 4]) {
    let i = ((y * TEXTURE_SIZE + x) * 4) as usize;
    data[i] = to_u8(rgba[0]);
    data[i + 1] = to_u8(rgba[1]);
    data[i + 2] = to_u8(rgba[2]);
    data[i + 3] = to_u8(rgba[3]);
}

fn to_u8(v: f32) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0).round() as u8
}

fn fbm(seed: u32, frequency: f64, octaves: usize, u: f32, v: f32) -> f32 {
    let noise = Fbm::<Perlin>::new(seed)
        .set_frequency(frequency)
        .set_octaves(octaves);
    (noise.get([u as f64, v as f64]) as f32 * 0.5 + 0.5).clamp(0.0, 1.0)
}

fn cell_noise(seed: u32, ix: i32, iy: i32) -> f32 {
    let mut n = ix
        .wrapping_mul(374_761_393)
        .wrapping_add(iy.wrapping_mul(668_265_263))
        .wrapping_add(seed as i32 * 97_531);
    n = (n ^ (n >> 13)).wrapping_mul(1_274_126_177);
    ((n ^ (n >> 16)) & 0xffff) as f32 / 65_535.0
}

fn mortar_mask(coord: f32, mortar: f32) -> f32 {
    let f = coord.fract();
    if f < mortar || f > 1.0 - mortar {
        0.0
    } else {
        1.0
    }
}

fn plaster_height(u: f32, v: f32) -> f32 {
    fbm(11, 18.0, 5, u, v) * 0.7 + fbm(12, 58.0, 2, u, v) * 0.3
}

fn plaster_albedo() -> Image {
    build_albedo(
        [0.96, 0.95, 0.90],
        |u, v| 0.82 + plaster_height(u, v) * 0.20,
        |_, _| [1.0, 1.0, 1.0],
    )
}

fn plaster_normal() -> Image {
    build_normal(plaster_height, 1.4)
}

fn wood_height(u: f32, v: f32) -> f32 {
    let plank = (v * 5.0).fract();
    let seam = if !(0.035..=0.965).contains(&plank) {
        -0.55
    } else {
        0.0
    };
    let grain = fbm(21, 22.0, 4, u * 3.5, v * 0.55);
    let fine = ((u * 72.0 + fbm(22, 9.0, 2, u, v) * 3.0).sin() * 0.5 + 0.5) * 0.22;
    seam + grain * 0.58 + fine
}

fn wood_albedo() -> Image {
    build_albedo(
        [0.64, 0.43, 0.24],
        |u, v| {
            let h = wood_height(u, v);
            (0.70 + h * 0.30).max(0.34)
        },
        |_, v| {
            let plank_id = (v * 5.0).floor() as i32;
            let n = cell_noise(23, 0, plank_id);
            [0.92 + n * 0.18, 0.90 + n * 0.10, 0.86 + n * 0.08]
        },
    )
}

fn wood_normal() -> Image {
    build_normal(wood_height, 2.1)
}

fn brick_height(u: f32, v: f32) -> f32 {
    let rows = v * 7.0;
    let row = rows.floor() as i32;
    let offset = if row % 2 == 0 { 0.5 } else { 0.0 };
    let cols = u * 4.0 + offset;
    let brick = mortar_mask(rows, 0.045) * mortar_mask(cols, 0.035);
    let surface = fbm(31, 38.0, 3, u, v) * 0.2;
    brick * (0.72 + surface)
}

fn brick_albedo() -> Image {
    build_albedo(
        [0.69, 0.37, 0.23],
        |u, v| {
            let rows = v * 7.0;
            let row = rows.floor() as i32;
            let offset = if row % 2 == 0 { 0.5 } else { 0.0 };
            let cols = u * 4.0 + offset;
            let mortar = mortar_mask(rows, 0.045) * mortar_mask(cols, 0.035);
            if mortar < 0.5 {
                0.52
            } else {
                0.78 + fbm(32, 30.0, 2, u, v) * 0.24
            }
        },
        |u, v| {
            let row = (v * 7.0).floor() as i32;
            let col = (u * 4.0).floor() as i32;
            let n = cell_noise(33, col, row);
            [0.88 + n * 0.25, 0.86 + n * 0.12, 0.82 + n * 0.10]
        },
    )
}

fn brick_normal() -> Image {
    build_normal(brick_height, 5.0)
}

fn roof_height(u: f32, v: f32) -> f32 {
    let courses = v * 8.0;
    let columns = u * 6.0
        + if courses.floor() as i32 % 2 == 0 {
            0.5
        } else {
            0.0
        };
    let gap = mortar_mask(courses, 0.055) * mortar_mask(columns, 0.045);
    let curved = ((courses.fract() * std::f32::consts::PI).sin() * 0.32 + 0.58).max(0.0);
    gap * curved + fbm(41, 44.0, 2, u, v) * 0.08
}

fn roof_albedo() -> Image {
    build_albedo(
        [0.55, 0.25, 0.14],
        |u, v| 0.68 + roof_height(u, v) * 0.33,
        |u, v| {
            let n = cell_noise(42, (u * 6.0).floor() as i32, (v * 8.0).floor() as i32);
            [0.86 + n * 0.20, 0.82 + n * 0.10, 0.78 + n * 0.06]
        },
    )
}

fn roof_normal() -> Image {
    build_normal(roof_height, 4.4)
}

fn stone_height(u: f32, v: f32) -> f32 {
    let blocks_y = v * 3.0;
    let row = blocks_y.floor() as i32;
    let blocks_x = u * 5.0 + if row % 2 == 0 { 0.35 } else { 0.0 };
    let joints = mortar_mask(blocks_x, 0.035) * mortar_mask(blocks_y, 0.055);
    joints * (0.55 + fbm(51, 18.0, 4, u, v) * 0.36)
}

fn stone_albedo() -> Image {
    build_albedo(
        [0.47, 0.46, 0.40],
        |u, v| {
            let h = stone_height(u, v);
            if h < 0.08 {
                0.56
            } else {
                0.74 + h * 0.24
            }
        },
        |u, v| {
            let n = cell_noise(52, (u * 5.0).floor() as i32, (v * 3.0).floor() as i32);
            [0.90 + n * 0.12, 0.90 + n * 0.10, 0.88 + n * 0.08]
        },
    )
}

fn stone_normal() -> Image {
    build_normal(stone_height, 3.0)
}

fn road_height(u: f32, v: f32) -> f32 {
    fbm(61, 24.0, 4, u, v) * 0.65 + fbm(62, 95.0, 2, u, v) * 0.35
}

fn road_albedo() -> Image {
    build_albedo(
        [0.42, 0.34, 0.25],
        |u, v| 0.62 + road_height(u, v) * 0.34,
        |u, v| {
            let pebble = fbm(63, 120.0, 1, u, v);
            [
                0.92 + pebble * 0.16,
                0.90 + pebble * 0.12,
                0.86 + pebble * 0.08,
            ]
        },
    )
}

fn road_normal() -> Image {
    build_normal(road_height, 1.8)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn image_bytes(image: &Image) -> &[u8] {
        image
            .data
            .as_ref()
            .expect("procedural images keep CPU data")
    }

    #[test]
    fn generated_images_have_expected_layout() {
        for (image, is_normal) in [
            (plaster_albedo(), false),
            (plaster_normal(), true),
            (wood_albedo(), false),
            (wood_normal(), true),
            (brick_albedo(), false),
            (brick_normal(), true),
            (roof_albedo(), false),
            (roof_normal(), true),
            (stone_albedo(), false),
            (stone_normal(), true),
            (road_albedo(), false),
            (road_normal(), true),
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
    fn albedo_images_are_not_flat_fills() {
        for image in [
            plaster_albedo(),
            wood_albedo(),
            brick_albedo(),
            roof_albedo(),
            stone_albedo(),
            road_albedo(),
        ] {
            let bytes = image_bytes(&image);
            assert!(
                bytes.chunks_exact(4).any(|px| px != &bytes[0..4]),
                "albedo texture should contain visible variation"
            );
        }
    }

    #[test]
    fn generation_is_deterministic() {
        assert_eq!(image_bytes(&wood_albedo()), image_bytes(&wood_albedo()));
        assert_eq!(
            image_bytes(&plaster_normal()),
            image_bytes(&plaster_normal())
        );
    }
}
