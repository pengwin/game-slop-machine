use super::math::{normalize3, u32_to_f32, write_rgba};
use crate::{RUNTIME_TEXTURE_SIZE, TextureSize};

/// Builds a tangent-space normal map from a height buffer using central differences.
pub fn build_normal_from_height(
    height: &[f32],
    size: TextureSize,
    normal_strength: f32,
) -> Vec<u8> {
    let mut data = vec![255; size.rgba_len()];
    let width_scale = u32_to_f32(size.width) / u32_to_f32(RUNTIME_TEXTURE_SIZE.width);
    let height_scale = u32_to_f32(size.height) / u32_to_f32(RUNTIME_TEXTURE_SIZE.height);

    let index = |x: u32, y: u32| -> usize { (y * size.width + x) as usize };

    let sample_wrapped = |x: i64, y: i64| -> f32 {
        let w = i64::from(size.width);
        let h = i64::from(size.height);
        let xx = u32::try_from(x.rem_euclid(w)).unwrap_or(0);
        let yy = u32::try_from(y.rem_euclid(h)).unwrap_or(0);
        height[index(xx, yy)]
    };

    for y in 0..size.height {
        for x in 0..size.width {
            let xi = i64::from(x);
            let yi = i64::from(y);
            let left = sample_wrapped(xi - 1, yi);
            let right = sample_wrapped(xi + 1, yi);
            let up = sample_wrapped(xi, yi - 1);
            let down = sample_wrapped(xi, yi + 1);
            let dx = (right - left) * width_scale;
            let dy = (down - up) * height_scale;
            let normal = normalize3([-dx * normal_strength, -dy * normal_strength, 1.0]);

            write_rgba(
                &mut data,
                size,
                x,
                y,
                [
                    normal[0].mul_add(0.5, 0.5),
                    normal[1].mul_add(0.5, 0.5),
                    normal[2].mul_add(0.5, 0.5),
                    1.0,
                ],
            );
        }
    }

    data
}
