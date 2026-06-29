use super::{
    maps::WorkingMaps,
    math::{normalize3, write_rgba},
};
use crate::PlasterParams;

pub(super) fn build_albedo(params: &PlasterParams, maps: &WorkingMaps) -> Vec<u8> {
    let mut data = vec![255; maps.size.rgba_len()];

    for y in 0..maps.size.height {
        for x in 0..maps.size.width {
            let i = maps.index(x, y);
            let tone = maps.tone[i] * params.tone_variation;
            let stain = maps.stain[i] * params.stain_darkening;
            let crack = maps.crack[i] * 0.22;
            let pit = maps.pit[i] * 0.08;
            let shade = (1.0 + tone - stain - crack - pit).clamp(0.45, 1.25);
            let warm = maps.tone[i].max(0.0) * 0.035;
            let cool = (-maps.tone[i]).max(0.0) * 0.025;

            write_rgba(
                &mut data,
                maps.size,
                x,
                y,
                [
                    params.base_color[0].mul_add(shade, warm),
                    params.base_color[1].mul_add(shade, warm * 0.55),
                    params.base_color[2].mul_add(shade, -cool),
                    1.0,
                ],
            );
        }
    }

    data
}

pub(super) fn build_normal(params: &PlasterParams, maps: &WorkingMaps) -> Vec<u8> {
    let mut data = vec![255; maps.size.rgba_len()];

    for y in 0..maps.size.height {
        for x in 0..maps.size.width {
            let xi = i64::from(x);
            let yi = i64::from(y);
            let left = maps.sample_height_wrapped(xi - 1, yi);
            let right = maps.sample_height_wrapped(xi + 1, yi);
            let up = maps.sample_height_wrapped(xi, yi - 1);
            let down = maps.sample_height_wrapped(xi, yi + 1);
            let dx = right - left;
            let dy = down - up;
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

pub(super) fn build_orm(params: &PlasterParams, maps: &WorkingMaps) -> Vec<u8> {
    let mut data = vec![255; maps.size.rgba_len()];

    for y in 0..maps.size.height {
        for x in 0..maps.size.width {
            let i = maps.index(x, y);
            let crack = maps.crack[i];
            let pit = maps.pit[i];
            let stain = maps.stain[i];
            let occlusion = pit
                .mul_add(-0.25, crack.mul_add(-0.35, params.ao_base))
                .clamp(0.0, 1.0);
            let roughness = crack
                .mul_add(
                    -0.03,
                    pit.mul_add(0.04, stain.mul_add(0.06, params.rough_base)),
                )
                .clamp(0.65, 1.0);

            write_rgba(&mut data, maps.size, x, y, [occlusion, roughness, 0.0, 1.0]);
        }
    }

    data
}
