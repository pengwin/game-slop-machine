use super::maps::WorkingMaps;
use crate::ConcreteParams;
use crate::surface::{
    math::{distance_to_segment, smoothstep, u32_to_f32, wrapped_delta},
    rng::SmallRng,
    tile_noise::fbm_tileable,
};
use num_traits::ToPrimitive;

pub fn build_tileable_tone(
    params: &ConcreteParams,
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
            let broad = fbm_tileable(params.seed ^ 0x3100, 2, 4, u, v);
            let pozzolana = fbm_tileable(params.seed ^ 0x3101, 5, 4, u, v);
            let lime = fbm_tileable(params.seed ^ 0x3102, 11, 3, u, v);
            let grain = fbm_tileable(params.seed ^ 0x3103, 120, 2, u, v);
            let i = maps.index(x, y);

            maps.tone[i] = broad.mul_add(0.55, pozzolana * 0.45).mul_add(2.0, -1.0);
            maps.lime[i] = (broad - 0.5).mul_add(0.3, (lime * 2.0 - 1.0) * 0.7);
            maps.height[i] = (grain * 2.0 - 1.0) * params.grain_height;
        }
    }

    true
}

pub fn draw_aggregate(
    params: &ConcreteParams,
    maps: &mut WorkingMaps,
    should_cancel: &impl Fn() -> bool,
) -> bool {
    let mut rng = SmallRng::new(params.seed ^ 0xA991);

    for _ in 0..params.aggregate_count {
        if should_cancel() {
            return false;
        }

        let center_u = rng.f32();
        let center_v = rng.f32();
        let radius_u = rng.range(0.0025, 0.011);
        let radius_v = radius_u * rng.range(0.65, 1.45);
        let strength = rng.range(0.35, 1.0);
        let tint = if rng.f32() > 0.52 {
            rng.range(0.25, 1.0)
        } else {
            -rng.range(0.2, 0.85)
        };

        raster_ellipse(
            maps,
            center_u,
            center_v,
            radius_u,
            radius_v,
            |maps, index, soft| {
                let value = soft * strength;
                maps.aggregate[index] = maps.aggregate[index].max(value);
                if value > maps.aggregate_tint[index].abs() {
                    maps.aggregate_tint[index] = tint * value;
                }
            },
        );
    }

    true
}

pub fn draw_voids(
    params: &ConcreteParams,
    maps: &mut WorkingMaps,
    should_cancel: &impl Fn() -> bool,
) -> bool {
    let mut rng = SmallRng::new(params.seed ^ 0xA992);

    for _ in 0..params.void_count {
        if should_cancel() {
            return false;
        }

        let center_u = rng.f32();
        let center_v = rng.f32();
        let radius = rng.range(0.0018, 0.0095);
        let strength = rng.range(0.3, 1.0);

        raster_ellipse(
            maps,
            center_u,
            center_v,
            radius,
            radius,
            |maps, index, soft| {
                maps.void[index] = maps.void[index].max(soft * strength);
            },
        );
    }

    true
}

pub fn draw_stains(
    params: &ConcreteParams,
    maps: &mut WorkingMaps,
    should_cancel: &impl Fn() -> bool,
) -> bool {
    let mut rng = SmallRng::new(params.seed ^ 0xA993);

    for _ in 0..params.stain_count {
        if should_cancel() {
            return false;
        }

        let center_u = rng.f32();
        let center_v = rng.f32();
        let radius_u = rng.range(0.045, 0.19);
        let radius_v = rng.range(0.04, 0.17);
        let strength = rng.range(0.2, 0.95);

        raster_ellipse(
            maps,
            center_u,
            center_v,
            radius_u,
            radius_v,
            |maps, index, soft| {
                maps.stain[index] = maps.stain[index].max(soft * strength);
            },
        );
    }

    true
}

pub fn draw_hairline_cracks(
    params: &ConcreteParams,
    maps: &mut WorkingMaps,
    should_cancel: &impl Fn() -> bool,
) -> bool {
    let mut rng = SmallRng::new(params.seed ^ 0xA994);

    for _ in 0..params.crack_count {
        if should_cancel() {
            return false;
        }

        let start = [rng.range(0.1, 0.9), rng.range(0.1, 0.9)];
        let angle = rng.range(0.0, std::f32::consts::TAU);
        let length = rng.range(0.1, 0.28);
        let segments = rng.u32_range(5, 12);
        let width = rng.range(0.0012, 0.0028);
        let strength = rng.range(0.35, 0.82);
        let points = fracture_path(start, angle, length, segments, &mut rng);

        if !rasterize_polyline_crack(maps, &points, width, strength, &mut rng, should_cancel) {
            return false;
        }
    }

    true
}

pub fn compose_height(
    params: &ConcreteParams,
    maps: &mut WorkingMaps,
    should_cancel: &impl Fn() -> bool,
) -> bool {
    for i in 0..maps.height.len() {
        if i % 4096 == 0 && should_cancel() {
            return false;
        }

        maps.height[i] =
            (maps.lime[i] * params.lime_cloud_strength).mul_add(0.018, maps.height[i]);
        maps.height[i] = maps.aggregate[i].mul_add(params.aggregate_height, maps.height[i]);
        maps.height[i] = maps.void[i].mul_add(-params.void_depth, maps.height[i]);
        maps.height[i] = (maps.crack_lip[i] * params.crack_depth).mul_add(0.42, maps.height[i]);
        maps.height[i] = maps.crack[i].mul_add(-params.crack_depth, maps.height[i]);
    }

    true
}

fn raster_ellipse(
    maps: &mut WorkingMaps,
    center_u: f32,
    center_v: f32,
    radius_u: f32,
    radius_v: f32,
    mut write: impl FnMut(&mut WorkingMaps, usize, f32),
) {
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
                let index = maps.index(pixel_x, pixel_y);
                write(maps, index, soft);
            }
        }
    }
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
        lateral = (lateral + rng.range(-0.02, 0.02)) * 0.64;
        let forward_jitter = rng.range(-0.01, 0.01) * endpoint_fade;
        let base_distance = length.mul_add(t, forward_jitter);
        let lateral_distance = lateral * endpoint_fade;
        points.push([
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
        ]);
    }

    points
}

fn rasterize_polyline_crack(
    maps: &mut WorkingMaps,
    points: &[[f32; 2]],
    width: f32,
    strength: f32,
    rng: &mut SmallRng,
    should_cancel: &impl Fn() -> bool,
) -> bool {
    for segment in points.windows(2) {
        if should_cancel() {
            return false;
        }
        let a = segment[0];
        let b = segment[1];
        let segment_strength = strength * rng.range(0.72, 1.16);
        let segment_width = width * rng.range(0.76, 1.34);
        let padding = segment_width * 3.6;
        let min_u = a[0].min(b[0]) - padding;
        let max_u = a[0].max(b[0]) + padding;
        let min_v = a[1].min(b[1]) - padding;
        let max_v = a[1].max(b[1]) + padding;
        let min_x = normalized_to_pixel(min_u, maps.size.width);
        let max_x = normalized_to_pixel(max_u, maps.size.width);
        let min_y = normalized_to_pixel(min_v, maps.size.height);
        let max_y = normalized_to_pixel(max_v, maps.size.height);
        let radius_x = normalized_radius_to_pixels(max_u - min_u, maps.size.width);
        let radius_y = normalized_radius_to_pixels(max_v - min_v, maps.size.height);
        let center_x = min_x.midpoint(max_x);
        let center_y = min_y.midpoint(max_y);

        for row_offset in -radius_y..=radius_y {
            for column_offset in -radius_x..=radius_x {
                let pixel_x = wrap_pixel(center_x + column_offset, maps.size.width);
                let pixel_y = wrap_pixel(center_y + row_offset, maps.size.height);
                let p = [
                    pixel_u(pixel_x, maps.size.width),
                    pixel_u(pixel_y, maps.size.height),
                ];
                let distance = distance_to_segment(p, a, b);

                if distance < segment_width * 3.2 {
                    let core = 1.0 - smoothstep(0.0, segment_width, distance);
                    let lip = 1.0 - smoothstep(segment_width, segment_width * 3.2, distance);
                    let i = maps.index(pixel_x, pixel_y);
                    maps.crack[i] = maps.crack[i].max(core * segment_strength);
                    maps.crack_lip[i] =
                        maps.crack_lip[i].max((lip - core).max(0.0) * segment_strength);
                }
            }
        }
    }

    true
}

fn normalized_to_pixel(value: f32, size: u32) -> i32 {
    (value.fract() * u32_to_f32(size))
        .floor()
        .to_i32()
        .unwrap_or(0)
}

fn normalized_radius_to_pixels(radius: f32, size: u32) -> i32 {
    (radius.abs() * u32_to_f32(size))
        .ceil()
        .to_i32()
        .unwrap_or(0)
        .max(1)
}

fn wrap_pixel(pixel: i32, size: u32) -> u32 {
    u32::try_from(pixel.rem_euclid(i32::try_from(size).unwrap_or(1))).unwrap_or(0)
}

fn pixel_u(pixel: u32, size: u32) -> f32 {
    (u32_to_f32(pixel) + 0.5) / u32_to_f32(size)
}
