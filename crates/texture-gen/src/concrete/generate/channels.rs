use super::maps::WorkingMaps;
use crate::surface::math::{normalize3, u32_to_f32, write_rgba};
use crate::{ConcreteParams, RUNTIME_TEXTURE_SIZE};

pub fn build_albedo(params: &ConcreteParams, maps: &WorkingMaps) -> Vec<u8> {
    let mut data = vec![255; maps.size.rgba_len()];

    for y in 0..maps.size.height {
        for x in 0..maps.size.width {
            let i = maps.index(x, y);
            let tone = maps.tone[i] * params.tone_variation;
            let lime = maps.lime[i] * params.lime_cloud_strength;
            let aggregate = maps.aggregate[i];
            let aggregate_tint = maps.aggregate_tint[i] * params.aggregate_contrast;
            let stain = maps.stain[i] * params.stain_darkening;
            let void = maps.void[i] * 0.12;
            let crack = maps.crack[i] * 0.18;
            let shade = (lime.mul_add(0.65, 1.0 + tone) - stain - void - crack).clamp(0.42, 1.32);
            let warm_aggregate = aggregate_tint.max(0.0) * aggregate;
            let cool_aggregate = (-aggregate_tint).max(0.0) * aggregate;
            let lime_rub = lime.max(0.0) * 0.055;
            let pozzolana = (-lime).max(0.0) * 0.045;

            write_rgba(
                &mut data,
                maps.size,
                x,
                y,
                [
                    params.base_color[0].mul_add(shade, lime_rub + warm_aggregate * 0.22),
                    params.base_color[1].mul_add(shade, lime_rub * 0.8 + warm_aggregate * 0.11),
                    params.base_color[2].mul_add(shade, cool_aggregate * 0.14 - pozzolana),
                    1.0,
                ],
            );
        }
    }

    data
}

pub fn build_normal(params: &ConcreteParams, maps: &WorkingMaps) -> Vec<u8> {
    let mut data = vec![255; maps.size.rgba_len()];
    let width_scale = u32_to_f32(maps.size.width) / u32_to_f32(RUNTIME_TEXTURE_SIZE.width);
    let height_scale = u32_to_f32(maps.size.height) / u32_to_f32(RUNTIME_TEXTURE_SIZE.height);

    for y in 0..maps.size.height {
        for x in 0..maps.size.width {
            let xi = i64::from(x);
            let yi = i64::from(y);
            let left = maps.sample_height_wrapped(xi - 1, yi);
            let right = maps.sample_height_wrapped(xi + 1, yi);
            let up = maps.sample_height_wrapped(xi, yi - 1);
            let down = maps.sample_height_wrapped(xi, yi + 1);
            let dx = (right - left) * width_scale;
            let dy = (down - up) * height_scale;
            let normal = normalize3([
                -dx * params.normal_strength,
                -dy * params.normal_strength,
                1.0,
            ]);

            write_rgba(
                &mut data,
                maps.size,
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

pub fn build_orm(params: &ConcreteParams, maps: &WorkingMaps) -> Vec<u8> {
    let mut data = vec![255; maps.size.rgba_len()];

    for y in 0..maps.size.height {
        for x in 0..maps.size.width {
            let i = maps.index(x, y);
            let occlusion = maps.void[i]
                .mul_add(
                    -0.28,
                    maps.crack[i].mul_add(-0.31, maps.aggregate[i].mul_add(-0.025, params.ao_base)),
                )
                .clamp(0.0, 1.0);
            let roughness = maps.stain[i]
                .mul_add(
                    0.05,
                    maps.void[i].mul_add(0.04, maps.aggregate[i].mul_add(0.035, params.rough_base)),
                )
                .clamp(0.62, 1.0);

            write_rgba(&mut data, maps.size, x, y, [occlusion, roughness, 0.0, 1.0]);
        }
    }

    data
}
