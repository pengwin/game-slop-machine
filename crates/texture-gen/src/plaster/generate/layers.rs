use super::{
    maps::WorkingMaps,
    math::{distance_to_segment, normalize2, smoothstep, u32_to_f32, wrapped_delta},
    rng::SmallRng,
    tile_noise::fbm_tileable,
};
use crate::PlasterParams;

pub(super) fn build_tileable_tone(params: &PlasterParams, maps: &mut WorkingMaps) {
    for y in 0..maps.size.height {
        for x in 0..maps.size.width {
            let u = u32_to_f32(x) / u32_to_f32(maps.size.width);
            let v = u32_to_f32(y) / u32_to_f32(maps.size.height);

            let broad = fbm_tileable(params.seed ^ 0x10, 4, 4, u, v);
            let medium = fbm_tileable(params.seed ^ 0x11, 16, 3, u, v);
            let grain = fbm_tileable(params.seed ^ 0x12, 96, 2, u, v);

            let i = maps.index(x, y);
            maps.tone[i] = broad.mul_add(0.7, medium * 0.3).mul_add(2.0, -1.0);
            maps.height[i] = (grain * 2.0 - 1.0) * params.grain_height;
        }
    }
}

pub(super) fn draw_stain_blobs(params: &PlasterParams, maps: &mut WorkingMaps) {
    let mut rng = SmallRng::new(params.seed ^ 0xCAFE);

    for _ in 0..params.stain_count {
        let center_x = rng.f32();
        let center_y = rng.f32();
        let radius_x = rng.range(0.04, 0.18);
        let radius_y = rng.range(0.035, 0.16);
        let strength = rng.range(0.25, 1.0);

        for y in 0..maps.size.height {
            for x in 0..maps.size.width {
                let u = u32_to_f32(x) / u32_to_f32(maps.size.width);
                let v = u32_to_f32(y) / u32_to_f32(maps.size.height);
                let dx = wrapped_delta(u - center_x);
                let dy = wrapped_delta(v - center_y);
                let d = (dx / radius_x).hypot(dy / radius_y);

                if d < 1.0 {
                    let soft = 1.0 - smoothstep(0.0, 1.0, d);
                    let i = maps.index(x, y);
                    maps.stain[i] = maps.stain[i].max(soft * strength);
                }
            }
        }
    }
}

pub(super) fn draw_pits(params: &PlasterParams, maps: &mut WorkingMaps) {
    let mut rng = SmallRng::new(params.seed ^ 0xBEEF);

    for _ in 0..params.pit_count {
        let center_x = rng.f32();
        let center_y = rng.f32();
        let radius = rng.range(0.0015, 0.006);
        let strength = rng.range(0.25, 1.0);

        for y in 0..maps.size.height {
            for x in 0..maps.size.width {
                let u = u32_to_f32(x) / u32_to_f32(maps.size.width);
                let v = u32_to_f32(y) / u32_to_f32(maps.size.height);
                let dx = wrapped_delta(u - center_x);
                let dy = wrapped_delta(v - center_y);
                let d = dx.hypot(dy);

                if d < radius {
                    let soft = 1.0 - smoothstep(0.0, radius, d);
                    let i = maps.index(x, y);
                    maps.pit[i] = maps.pit[i].max(soft * strength);
                }
            }
        }
    }
}

pub(super) fn draw_hairline_cracks(params: &PlasterParams, maps: &mut WorkingMaps) {
    let mut rng = SmallRng::new(params.seed ^ 0x1234);

    for _ in 0..params.crack_count {
        let start = [rng.range(0.08, 0.92), rng.range(0.08, 0.92)];
        let angle = rng.range(0.0, std::f32::consts::TAU);
        let length = rng.range(0.12, 0.42);
        let segments = rng.u32_range(3, 8);
        let width = rng.range(0.0014, 0.0038);
        let strength = rng.range(0.35, 1.0);

        let mut points = Vec::with_capacity((segments + 1) as usize);
        points.push(start);

        let mut pos = start;
        let mut dir = [angle.cos(), angle.sin()];

        for _ in 0..segments {
            let step = length / u32_to_f32(segments);
            dir[0] += rng.range(-0.45, 0.45);
            dir[1] += rng.range(-0.45, 0.45);
            dir = normalize2(dir);

            pos = [
                dir[0].mul_add(step, pos[0]).clamp(0.04, 0.96),
                dir[1].mul_add(step, pos[1]).clamp(0.04, 0.96),
            ];
            points.push(pos);
        }

        rasterize_polyline_crack(maps, &points, width, strength);

        if rng.f32() > 0.55 && points.len() > 2 {
            let branch_start = points[(points.len() / 2).max(1)];
            let branch_angle = angle + rng.range(-1.3, 1.3);
            let branch_len = length * rng.range(0.18, 0.38);
            let branch_end = [
                branch_angle
                    .cos()
                    .mul_add(branch_len, branch_start[0])
                    .clamp(0.04, 0.96),
                branch_angle
                    .sin()
                    .mul_add(branch_len, branch_start[1])
                    .clamp(0.04, 0.96),
            ];
            rasterize_polyline_crack(
                maps,
                &[branch_start, branch_end],
                width * 0.75,
                strength * 0.65,
            );
        }
    }
}

pub(super) fn compose_height(params: &PlasterParams, maps: &mut WorkingMaps) {
    for y in 0..maps.size.height {
        for x in 0..maps.size.width {
            let i = maps.index(x, y);
            maps.height[i] = maps.stain[i].mul_add(0.006, maps.height[i]);
            maps.height[i] = maps.pit[i].mul_add(-params.pit_depth, maps.height[i]);
            maps.height[i] = maps.crack[i].mul_add(-params.crack_depth, maps.height[i]);
            maps.height[i] = maps.height[i].clamp(-0.25, 0.25);
        }
    }
}

fn rasterize_polyline_crack(
    maps: &mut WorkingMaps,
    points: &[[f32; 2]],
    width: f32,
    strength: f32,
) {
    if points.len() < 2 {
        return;
    }

    for y in 0..maps.size.height {
        for x in 0..maps.size.width {
            let p = [
                u32_to_f32(x) / u32_to_f32(maps.size.width),
                u32_to_f32(y) / u32_to_f32(maps.size.height),
            ];
            let mut min_d = f32::MAX;

            for segment in points.windows(2) {
                min_d = min_d.min(distance_to_segment(p, segment[0], segment[1]));
            }

            if min_d < width {
                let core = 1.0 - smoothstep(0.0, width, min_d);
                let i = maps.index(x, y);
                maps.crack[i] = maps.crack[i].max(core * strength);
            }
        }
    }
}
