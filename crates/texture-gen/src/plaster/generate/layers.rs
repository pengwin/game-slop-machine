use super::{
    maps::WorkingMaps,
    math::{distance_to_segment, smoothstep, u32_to_f32, wrapped_delta},
    rng::SmallRng,
    tile_noise::fbm_tileable,
};
use crate::PlasterParams;
use num_traits::ToPrimitive;

pub fn build_tileable_tone(
    params: &PlasterParams,
    maps: &mut WorkingMaps,
    should_cancel: &impl Fn() -> bool,
) -> bool {
    for y in 0..maps.size.height {
        if should_cancel() {
            return false;
        }
        for x in 0..maps.size.width {
            let u = u32_to_f32(x) / u32_to_f32(maps.size.width);
            let v = u32_to_f32(y) / u32_to_f32(maps.size.height);

            let macro_tone = fbm_tileable(params.seed ^ 0x0F, 2, 3, u, v);
            let broad = fbm_tileable(params.seed ^ 0x10, 4, 4, u, v);
            let medium = fbm_tileable(params.seed ^ 0x11, 16, 3, u, v);
            let grain = fbm_tileable(params.seed ^ 0x12, 96, 2, u, v);

            let i = maps.index(x, y);
            maps.macro_tone[i] = macro_tone * 2.0 - 1.0;
            maps.tone[i] = broad
                .mul_add(0.55, medium.mul_add(0.25, macro_tone * 0.20))
                .mul_add(2.0, -1.0);
            maps.height[i] = (grain * 2.0 - 1.0) * params.grain_height;
        }
    }

    true
}

pub fn draw_stain_blobs(
    params: &PlasterParams,
    maps: &mut WorkingMaps,
    should_cancel: &impl Fn() -> bool,
) -> bool {
    let mut rng = SmallRng::new(params.seed ^ 0xCAFE);

    for _ in 0..params.stain_count {
        if should_cancel() {
            return false;
        }

        let center_u = rng.f32();
        let center_v = rng.f32();
        let radius_u = rng.range(0.04, 0.18);
        let radius_v = rng.range(0.035, 0.16);
        let strength = rng.range(0.25, 1.0);
        let center_column = normalized_to_pixel(center_u, maps.size.width);
        let center_row = normalized_to_pixel(center_v, maps.size.height);
        let radius_columns = normalized_radius_to_pixels(radius_u, maps.size.width);
        let radius_rows = normalized_radius_to_pixels(radius_v, maps.size.height);

        for row_offset in -radius_rows..=radius_rows {
            for column_offset in -radius_columns..=radius_columns {
                let pixel_x = wrap_pixel(center_column + column_offset, maps.size.width);
                let pixel_y = wrap_pixel(center_row + row_offset, maps.size.height);
                let sample_u = pixel_u(pixel_x, maps.size.width);
                let sample_v = pixel_u(pixel_y, maps.size.height);
                let delta_u = wrapped_delta(sample_u - center_u);
                let delta_v = wrapped_delta(sample_v - center_v);
                let distance = (delta_u / radius_u).hypot(delta_v / radius_v);

                if distance < 1.0 {
                    let soft = 1.0 - smoothstep(0.0, 1.0, distance);
                    let map_index = maps.index(pixel_x, pixel_y);
                    maps.stain[map_index] = maps.stain[map_index].max(soft * strength);
                }
            }
        }
    }

    true
}

pub fn draw_pits(
    params: &PlasterParams,
    maps: &mut WorkingMaps,
    should_cancel: &impl Fn() -> bool,
) -> bool {
    let mut rng = SmallRng::new(params.seed ^ 0xBEEF);

    for _ in 0..params.pit_count {
        if should_cancel() {
            return false;
        }

        let center_u = rng.f32();
        let center_v = rng.f32();
        let radius = rng.range(0.0015, 0.006);
        let strength = rng.range(0.25, 1.0);
        let center_column = normalized_to_pixel(center_u, maps.size.width);
        let center_row = normalized_to_pixel(center_v, maps.size.height);
        let radius_pixels =
            normalized_radius_to_pixels(radius, maps.size.width.max(maps.size.height));

        for row_offset in -radius_pixels..=radius_pixels {
            for column_offset in -radius_pixels..=radius_pixels {
                let pixel_x = wrap_pixel(center_column + column_offset, maps.size.width);
                let pixel_y = wrap_pixel(center_row + row_offset, maps.size.height);
                let sample_u = pixel_u(pixel_x, maps.size.width);
                let sample_v = pixel_u(pixel_y, maps.size.height);
                let delta_u = wrapped_delta(sample_u - center_u);
                let delta_v = wrapped_delta(sample_v - center_v);
                let distance = delta_u.hypot(delta_v);

                if distance < radius {
                    let soft = 1.0 - smoothstep(0.0, radius, distance);
                    let map_index = maps.index(pixel_x, pixel_y);
                    maps.pit[map_index] = maps.pit[map_index].max(soft * strength);
                }
            }
        }
    }

    true
}

pub fn draw_hairline_cracks(
    params: &PlasterParams,
    maps: &mut WorkingMaps,
    should_cancel: &impl Fn() -> bool,
) -> bool {
    let mut rng = SmallRng::new(params.seed ^ 0x1234);

    for _ in 0..params.crack_count {
        if should_cancel() {
            return false;
        }

        let start = [rng.range(0.08, 0.92), rng.range(0.08, 0.92)];
        let angle = rng.range(0.0, std::f32::consts::TAU);
        let length = rng.range(0.14, 0.38);
        let segments = rng.u32_range(7, 15);
        let width = rng.range(0.0015, 0.0036);
        let strength = rng.range(0.48, 1.05);
        let points = fracture_path(start, angle, length, segments, &mut rng);

        if !rasterize_polyline_crack(maps, &points, width, strength, &mut rng, should_cancel) {
            return false;
        }

        let branch_roll = rng.f32();
        let branch_count = usize::from(branch_roll > 0.46) + usize::from(branch_roll > 0.78);
        for _ in 0..branch_count {
            if points.len() <= 4 {
                continue;
            }

            let branch_index =
                rng.u32_range(2, u32::try_from(points.len() - 2).unwrap_or(2)) as usize;
            let branch_start = points[branch_index];
            let side = if rng.f32() > 0.5 { 1.0 } else { -1.0 };
            let branch_angle = angle + side * rng.range(0.55, 1.35);
            let branch_len = length * rng.range(0.18, 0.42);
            let branch_segments = rng.u32_range(4, 9);
            let branch_points = fracture_path(
                branch_start,
                branch_angle,
                branch_len,
                branch_segments,
                &mut rng,
            );

            if !rasterize_polyline_crack(
                maps,
                &branch_points,
                width * rng.range(0.35, 0.62),
                strength * rng.range(0.42, 0.68),
                &mut rng,
                should_cancel,
            ) {
                return false;
            }
        }
    }

    true
}

fn fracture_path(
    start: [f32; 2],
    angle: f32,
    length: f32,
    segments: u32,
    rng: &mut SmallRng,
) -> Vec<[f32; 2]> {
    let segment_count = segments.max(2);
    let direction = [angle.cos(), angle.sin()];
    let normal = [-direction[1], direction[0]];
    let mut points = Vec::with_capacity(usize::try_from(segment_count + 1).unwrap_or(0));
    let mut lateral = 0.0;

    for segment in 0..=segment_count {
        let t = u32_to_f32(segment) / u32_to_f32(segment_count);
        let endpoint_fade = (std::f32::consts::PI * t).sin();
        lateral = (lateral + rng.range(-0.026, 0.026)) * 0.68;
        let forward_jitter = rng.range(-0.012, 0.012) * endpoint_fade;
        let base_distance = length.mul_add(t, forward_jitter);
        let lateral_distance = lateral * endpoint_fade;
        let point = [
            normal[0]
                .mul_add(
                    lateral_distance,
                    direction[0].mul_add(base_distance, start[0]),
                )
                .clamp(0.035, 0.965),
            normal[1]
                .mul_add(
                    lateral_distance,
                    direction[1].mul_add(base_distance, start[1]),
                )
                .clamp(0.035, 0.965),
        ];
        points.push(point);
    }

    points
}

pub fn compose_height(
    params: &PlasterParams,
    maps: &mut WorkingMaps,
    should_cancel: &impl Fn() -> bool,
) -> bool {
    for y in 0..maps.size.height {
        if should_cancel() {
            return false;
        }
        for x in 0..maps.size.width {
            let i = maps.index(x, y);
            maps.height[i] = maps.stain[i].mul_add(0.006, maps.height[i]);
            maps.height[i] = maps.pit[i].mul_add(-params.pit_depth, maps.height[i]);
            maps.height[i] = maps.crack_lip[i].mul_add(params.crack_depth * 0.18, maps.height[i]);
            maps.height[i] = maps.crack[i].mul_add(-params.crack_depth, maps.height[i]);
            maps.height[i] = maps.height[i].clamp(-0.25, 0.25);
        }
    }

    true
}

fn rasterize_polyline_crack(
    maps: &mut WorkingMaps,
    points: &[[f32; 2]],
    width: f32,
    strength: f32,
    rng: &mut SmallRng,
    should_cancel: &impl Fn() -> bool,
) -> bool {
    if points.len() < 2 {
        return true;
    }

    let segment_style = crack_segment_style(points.len(), rng);

    for (segment_index, segment) in points.windows(2).enumerate() {
        if should_cancel() {
            return false;
        }
        if !segment_style[segment_index].active {
            continue;
        }

        let style = segment_style[segment_index];
        let local_width = width * style.width_scale;
        let padding = width * 4.8;
        let min_x =
            normalized_to_pixel(segment[0][0].min(segment[1][0]) - padding, maps.size.width);
        let max_x =
            normalized_to_pixel(segment[0][0].max(segment[1][0]) + padding, maps.size.width);
        let min_y =
            normalized_to_pixel(segment[0][1].min(segment[1][1]) - padding, maps.size.height);
        let max_y =
            normalized_to_pixel(segment[0][1].max(segment[1][1]) + padding, maps.size.height);

        for y in clamped_pixel_range(min_y, max_y, maps.size.height) {
            let y = u32::try_from(y).unwrap_or(0);
            for x in clamped_pixel_range(min_x, max_x, maps.size.width) {
                let x = u32::try_from(x).unwrap_or(0);
                let p = [pixel_u(x, maps.size.width), pixel_u(y, maps.size.height)];
                let min_d = distance_to_segment(p, segment[0], segment[1]);
                if min_d >= width * 4.8 {
                    continue;
                }

                let i = maps.index(x, y);
                let core = 1.0 - smoothstep(0.0, local_width, min_d);
                let lip = smoothstep(local_width * 0.70, local_width * 3.6, min_d)
                    * (1.0 - smoothstep(local_width * 3.8, local_width * 4.8, min_d));

                maps.crack[i] = maps.crack[i].max(core * strength * style.strength_scale);
                maps.crack_lip[i] = maps.crack_lip[i].max(lip * strength * 0.52);
            }
        }
    }

    true
}

#[derive(Clone, Copy)]
struct CrackSegmentStyle {
    active: bool,
    width_scale: f32,
    strength_scale: f32,
}

fn crack_segment_style(segment_count: usize, rng: &mut SmallRng) -> Vec<CrackSegmentStyle> {
    let crack_segment_count = segment_count.saturating_sub(1);
    (0..crack_segment_count)
        .map(|segment_index| {
            let endpoint = segment_index == 0 || segment_index + 2 >= segment_count;
            let segment_position = u32::try_from(segment_index).unwrap_or(0);
            let segment_total = u32::try_from(crack_segment_count.max(1)).unwrap_or(1);
            let position = (u32_to_f32(segment_position) + 0.5) / u32_to_f32(segment_total);
            let taper = (std::f32::consts::PI * position).sin().sqrt();
            CrackSegmentStyle {
                active: endpoint || rng.f32() > 0.08,
                width_scale: rng.range(0.82, 1.42) * taper.mul_add(0.72, 0.28),
                strength_scale: rng.range(0.70, 1.08) * taper.mul_add(0.55, 0.45),
            }
        })
        .collect()
}

fn normalized_to_pixel(value: f32, extent: u32) -> i32 {
    (value * u32_to_f32(extent)).floor().to_i32().unwrap_or(0)
}

fn normalized_radius_to_pixels(radius: f32, extent: u32) -> i32 {
    (radius * u32_to_f32(extent))
        .ceil()
        .to_i32()
        .unwrap_or(0)
        .max(1)
}

fn wrap_pixel(value: i32, extent: u32) -> u32 {
    let extent = i32::try_from(extent).unwrap_or(1).max(1);
    u32::try_from(value.rem_euclid(extent)).unwrap_or(0)
}

fn clamped_pixel_range(min: i32, max: i32, extent: u32) -> std::ops::RangeInclusive<i32> {
    let last = i32::try_from(extent.saturating_sub(1)).unwrap_or(0);
    let start = min.clamp(0, last);
    let end = max.clamp(0, last);
    start..=end
}

fn pixel_u(pixel: u32, extent: u32) -> f32 {
    u32_to_f32(pixel) / u32_to_f32(extent)
}
