use num_traits::ToPrimitive;

use crate::{GeneratedTexture, TextureColorSpace};

/// How mip levels should be generated for a texture.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MipGenerationKind {
    /// Average RGBA channels directly.
    Color,
    /// Average normal vectors and renormalize every output texel.
    Normal,
}

/// Generated RGBA texture data with a full mip chain in mip-major order.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GeneratedMipTexture {
    /// Base texture width in pixels.
    pub width: u32,
    /// Base texture height in pixels.
    pub height: u32,
    /// RGBA8 pixel data for all mip levels, from largest to smallest.
    pub data: Vec<u8>,
    /// Color interpretation for the data.
    pub color_space: TextureColorSpace,
    /// Number of mip levels stored in `data`.
    pub mip_level_count: u32,
}

/// Builds a full mip chain for a generated RGBA texture.
#[must_use]
pub fn generate_mip_chain(
    texture: &GeneratedTexture,
    mip_kind: MipGenerationKind,
) -> GeneratedMipTexture {
    let mip_level_count = mip_level_count(texture.width, texture.height);
    let mut data = texture.data.clone();
    let mut previous = texture.data.clone();
    let mut previous_width = texture.width;
    let mut previous_height = texture.height;

    while previous_width > 1 || previous_height > 1 {
        let next_width = (previous_width / 2).max(1);
        let next_height = (previous_height / 2).max(1);
        let next = downsample_mip_level(
            &previous,
            previous_width,
            previous_height,
            next_width,
            next_height,
            mip_kind,
        );
        data.extend_from_slice(&next);
        previous = next;
        previous_width = next_width;
        previous_height = next_height;
    }

    GeneratedMipTexture {
        width: texture.width,
        height: texture.height,
        data,
        color_space: texture.color_space,
        mip_level_count,
    }
}

fn mip_level_count(width: u32, height: u32) -> u32 {
    let mut levels = 1;
    let mut width = width;
    let mut height = height;

    while width > 1 || height > 1 {
        width = (width / 2).max(1);
        height = (height / 2).max(1);
        levels += 1;
    }

    levels
}

fn downsample_mip_level(
    previous: &[u8],
    previous_width: u32,
    previous_height: u32,
    next_width: u32,
    next_height: u32,
    mip_kind: MipGenerationKind,
) -> Vec<u8> {
    let mut next = vec![255; rgba_len(next_width, next_height)];

    for y in 0..next_height {
        for x in 0..next_width {
            let samples = [
                sample_rgba(previous, previous_width, previous_height, x * 2, y * 2),
                sample_rgba(previous, previous_width, previous_height, x * 2 + 1, y * 2),
                sample_rgba(previous, previous_width, previous_height, x * 2, y * 2 + 1),
                sample_rgba(
                    previous,
                    previous_width,
                    previous_height,
                    x * 2 + 1,
                    y * 2 + 1,
                ),
            ];
            let rgba = match mip_kind {
                MipGenerationKind::Color => average_rgba(samples),
                MipGenerationKind::Normal => average_normal_rgba(samples),
            };
            write_rgba_bytes(&mut next, next_width, x, y, rgba);
        }
    }

    next
}

fn sample_rgba(data: &[u8], width: u32, height: u32, x: u32, y: u32) -> [u8; 4] {
    let x = x.min(width.saturating_sub(1));
    let y = y.min(height.saturating_sub(1));
    let index = rgba_index(width, x, y);
    [
        data[index],
        data[index + 1],
        data[index + 2],
        data[index + 3],
    ]
}

fn average_rgba(samples: [[u8; 4]; 4]) -> [u8; 4] {
    let mut rgba = [0; 4];
    for channel in 0..4 {
        let sum = samples
            .iter()
            .map(|sample| u16::from(sample[channel]))
            .sum::<u16>();
        rgba[channel] = u8::try_from((sum + 2) / 4).unwrap_or(u8::MAX);
    }
    rgba
}

fn average_normal_rgba(samples: [[u8; 4]; 4]) -> [u8; 4] {
    let mut normal = [0.0; 3];
    for sample in samples {
        normal[0] += f32::from(sample[0]) / 127.5 - 1.0;
        normal[1] += f32::from(sample[1]) / 127.5 - 1.0;
        normal[2] += f32::from(sample[2]) / 127.5 - 1.0;
    }
    let normal = normalize3_or_z(normal);

    [
        normal_component_to_u8(normal[0]),
        normal_component_to_u8(normal[1]),
        normal_component_to_u8(normal[2]),
        u8::MAX,
    ]
}

fn normalize3_or_z(value: [f32; 3]) -> [f32; 3] {
    let length = value[2]
        .mul_add(value[2], value[1].mul_add(value[1], value[0] * value[0]))
        .sqrt();
    if length <= f32::EPSILON {
        [0.0, 0.0, 1.0]
    } else {
        [value[0] / length, value[1] / length, value[2] / length]
    }
}

fn normal_component_to_u8(value: f32) -> u8 {
    (value.mul_add(0.5, 0.5).clamp(0.0, 1.0) * 255.0)
        .round()
        .to_u8()
        .unwrap_or(0)
}

fn write_rgba_bytes(data: &mut [u8], width: u32, x: u32, y: u32, rgba: [u8; 4]) {
    let index = rgba_index(width, x, y);
    data[index] = rgba[0];
    data[index + 1] = rgba[1];
    data[index + 2] = rgba[2];
    data[index + 3] = rgba[3];
}

const fn rgba_index(width: u32, x: u32, y: u32) -> usize {
    ((y * width + x) * 4) as usize
}

const fn rgba_len(width: u32, height: u32) -> usize {
    (width as usize) * (height as usize) * 4
}
