use bevy::asset::RenderAssetUsages;
use bevy::image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

pub const RENDER_TEXTURE_SIZE: u32 = 512;

#[cfg(test)]
pub const TEXTURE_SIZE: u32 = 128;

#[cfg(not(test))]
pub const TEXTURE_SIZE: u32 = RENDER_TEXTURE_SIZE;

pub fn create_placeholder(is_normal: bool) -> Image {
    let data = if is_normal {
        vec![128, 128, 255, 255]
    } else {
        vec![128, 128, 128, 255]
    };
    let format = if is_normal {
        TextureFormat::Rgba8Unorm
    } else {
        TextureFormat::Rgba8UnormSrgb
    };
    Image::new(
        Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        format,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    )
}

pub fn create_image(width: u32, height: u32, data: Vec<u8>, is_normal: bool) -> Image {
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

pub fn repeating_linear_sampler() -> ImageSamplerDescriptor {
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

pub fn build_albedo(
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

pub fn build_normal(height: impl Fn(f32, f32) -> f32, strength: f32) -> Image {
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

pub fn build_orm(
    occlusion: impl Fn(f32, f32) -> f32,
    roughness: impl Fn(f32, f32) -> f32,
    metallic: impl Fn(f32, f32) -> f32,
) -> Image {
    let mut data = vec![255; (TEXTURE_SIZE * TEXTURE_SIZE * 4) as usize];
    for y in 0..TEXTURE_SIZE {
        for x in 0..TEXTURE_SIZE {
            let u = x as f32 / TEXTURE_SIZE as f32;
            let v = y as f32 / TEXTURE_SIZE as f32;
            let o = occlusion(u, v).clamp(0.0, 1.0);
            let r = roughness(u, v).clamp(0.0, 1.0);
            let m = metallic(u, v).clamp(0.0, 1.0);
            write_rgba(&mut data, x, y, [o, r, m, 1.0]);
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
