use super::maps::WorkingMaps;
use crate::surface::math::write_rgba;
use crate::surface::normal::build_normal_from_height;
use crate::ConcreteParams;

pub fn build_albedo(params: &ConcreteParams, maps: &WorkingMaps) -> Vec<u8> {
    let mut data = vec![255; maps.size.rgba_len()];

    for y in 0..maps.size.height {
        for x in 0..maps.size.width {
            let i = maps.index(x, y);
            let tone = maps.tone[i] * params.tone_variation;
            let lime = maps.lime[i] * params.lime_cloud_strength;
            let aggregate = maps.aggregate[i];
            let aggregate_tint = maps.aggregate_tint[i] * params.aggregate_contrast;
            let exposed = maps.exposed_aggregate[i];
            let exposed_tint = maps.aggregate_tint[i] * 0.6;
            let stain = maps.stain[i] * params.stain_darkening;
            let void = maps.void[i] * 0.12;
            let crack = maps.crack[i] * 0.18;
            let formwork = maps.formwork[i] * params.formwork_strength * 0.15;
            let efflo = maps.efflorescence[i] * params.efflorescence_strength;
            let shade =
                (lime.mul_add(0.65, 1.0 + tone) - stain - void - crack - formwork).clamp(0.42, 1.32);
            let warm_aggregate = aggregate_tint.max(0.0) * aggregate;
            let cool_aggregate = (-aggregate_tint).max(0.0) * aggregate;
            let warm_exposed = exposed_tint.max(0.0) * exposed;
            let cool_exposed = (-exposed_tint).max(0.0) * exposed;
            let lime_rub = lime.max(0.0) * 0.055;
            let pozzolana = (-lime).max(0.0) * 0.045;

            let r = params.base_color[0].mul_add(
                shade,
                lime_rub + warm_aggregate * 0.22 + warm_exposed * 0.35,
            );
            let g = params.base_color[1].mul_add(
                shade,
                lime_rub * 0.8 + warm_aggregate * 0.11 + warm_exposed * 0.18,
            );
            let b = params.base_color[2].mul_add(
                shade,
                cool_aggregate * 0.14 + cool_exposed * 0.22 - pozzolana,
            );

            write_rgba(
                &mut data,
                maps.size,
                x,
                y,
                [
                    (1.0 - r).mul_add(efflo, r),
                    (1.0 - g).mul_add(efflo, g),
                    (1.0 - b).mul_add(efflo, b),
                    1.0,
                ],
            );
        }
    }

    data
}

pub fn build_normal(params: &ConcreteParams, maps: &WorkingMaps) -> Vec<u8> {
    build_normal_from_height(&maps.height, maps.size, params.normal_strength)
}

pub fn build_orm(params: &ConcreteParams, maps: &WorkingMaps) -> Vec<u8> {
    let mut data = vec![255; maps.size.rgba_len()];

    for y in 0..maps.size.height {
        for x in 0..maps.size.width {
            let i = maps.index(x, y);
            let occlusion = maps.void[i]
                .mul_add(
                    -0.28,
                    maps.crack[i].mul_add(
                        -0.31,
                        maps.exposed_aggregate[i].mul_add(
                            -0.04,
                            maps.formwork[i].mul_add(
                                -0.06,
                                maps.aggregate[i].mul_add(-0.025, params.ao_base),
                            ),
                        ),
                    ),
                )
                .clamp(0.0, 1.0);
            let roughness = maps.efflorescence[i]
                .mul_add(
                    -0.08,
                    maps.stain[i].mul_add(
                        0.05,
                        maps.void[i].mul_add(
                            0.04,
                            maps.exposed_aggregate[i].mul_add(
                                0.05,
                                maps.aggregate[i].mul_add(0.035, params.rough_base),
                            ),
                        ),
                    ),
                )
                .clamp(0.62, 1.0);

            write_rgba(&mut data, maps.size, x, y, [occlusion, roughness, 0.0, 1.0]);
        }
    }

    data
}
