#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::suboptimal_flops,
    reason = "texture rasterization intentionally converts pixel coordinates and normalized floats"
)]

use crate::TextureSize;

pub fn build_albedo(
    size: TextureSize,
    base: [f32; 3],
    shade: impl Fn(f32, f32) -> f32,
    tint: impl Fn(f32, f32) -> [f32; 3],
) -> Vec<u8> {
    let mut data = vec![255; size.rgba_len()];
    for y in 0..size.height {
        for x in 0..size.width {
            let u = x as f32 / size.width as f32;
            let v = y as f32 / size.height as f32;
            let s = shade(u, v).clamp(0.0, 1.5);
            let t = tint(u, v);
            write_rgba(
                size,
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
    data
}

pub fn build_normal(size: TextureSize, height: impl Fn(f32, f32) -> f32, strength: f32) -> Vec<u8> {
    let mut data = vec![255; size.rgba_len()];
    let eps_u = 1.0 / size.width as f32;
    let eps_v = 1.0 / size.height as f32;

    for y in 0..size.height {
        for x in 0..size.width {
            let u = x as f32 / size.width as f32;
            let v = y as f32 / size.height as f32;
            let dx = height(u + eps_u, v) - height(u - eps_u, v);
            let dy = height(u, v + eps_v) - height(u, v - eps_v);
            let normal = normalize([-dx * strength, -dy * strength, 1.0]);
            write_rgba(
                size,
                &mut data,
                x,
                y,
                [
                    normal[0] * 0.5 + 0.5,
                    normal[1] * 0.5 + 0.5,
                    normal[2] * 0.5 + 0.5,
                    1.0,
                ],
            );
        }
    }
    data
}

pub fn build_orm(
    size: TextureSize,
    occlusion: impl Fn(f32, f32) -> f32,
    roughness: impl Fn(f32, f32) -> f32,
    metallic: impl Fn(f32, f32) -> f32,
) -> Vec<u8> {
    let mut data = vec![255; size.rgba_len()];
    for y in 0..size.height {
        for x in 0..size.width {
            let u = x as f32 / size.width as f32;
            let v = y as f32 / size.height as f32;
            write_rgba(
                size,
                &mut data,
                x,
                y,
                [
                    occlusion(u, v).clamp(0.0, 1.0),
                    roughness(u, v).clamp(0.0, 1.0),
                    metallic(u, v).clamp(0.0, 1.0),
                    1.0,
                ],
            );
        }
    }
    data
}

fn write_rgba(size: TextureSize, data: &mut [u8], x: u32, y: u32, rgba: [f32; 4]) {
    let i = ((y * size.width + x) * 4) as usize;
    data[i] = to_u8(rgba[0]);
    data[i + 1] = to_u8(rgba[1]);
    data[i + 2] = to_u8(rgba[2]);
    data[i + 3] = to_u8(rgba[3]);
}

fn to_u8(value: f32) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}

fn normalize(vector: [f32; 3]) -> [f32; 3] {
    let length_squared = vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2];
    if length_squared <= f32::EPSILON {
        return [0.0, 0.0, 0.0];
    }

    let inv_length = length_squared.sqrt().recip();
    [
        vector[0] * inv_length,
        vector[1] * inv_length,
        vector[2] * inv_length,
    ]
}
