use super::config::TradeDistrictConfig;
use super::layout::RoadSegment;
use crate::geometry::Vec2;

/// Generates the spoke-and-ring road network for the trade district.
///
/// Returns: hexagonal road around square, radial roads (spokes), and outer ring roads.
pub fn generate_road_network(config: &TradeDistrictConfig) -> Vec<RoadSegment> {
    let mut roads = Vec::new();
    let center = Vec2::ZERO;
    let sq = config.town_square_radius;

    // Hexagonal road surrounding the town square
    let hex_radius = sq + config.road_width;
    let hex_segments = generate_hex_ring(hex_radius, config.road_width, center);
    roads.extend(hex_segments);

    // Compute how far the outermost ring extends
    let outer_radius = sq + config.ring_spacing * config.ring_count as f32;

    // Radial roads: evenly spaced spokes from hex road edge to outer radius
    for i in 0..config.radial_count {
        let angle = (i as f32) * std::f32::consts::TAU / (config.radial_count as f32);
        let dir = Vec2::new(angle.cos(), angle.sin());
        let inner = Vec2::new(center.x + hex_radius * dir.x, center.y + hex_radius * dir.y);
        let outer = Vec2::new(
            center.x + outer_radius * dir.x,
            center.y + outer_radius * dir.y,
        );
        roads.push(RoadSegment {
            start: inner,
            end: outer,
            width: config.road_width,
        });
    }

    // Outer ring roads: concentric hexagonal rings connecting radials
    for ring_idx in 1..=config.ring_count {
        let radius = sq + config.ring_spacing * ring_idx as f32;

        for radial_idx in 0..config.radial_count {
            let next_idx = (radial_idx + 1) % config.radial_count;
            let angle_a =
                (radial_idx as f32) * std::f32::consts::TAU / (config.radial_count as f32);
            let angle_b = (next_idx as f32) * std::f32::consts::TAU / (config.radial_count as f32);

            let a = Vec2::new(
                center.x + radius * angle_a.cos(),
                center.y + radius * angle_a.sin(),
            );
            let b = Vec2::new(
                center.x + radius * angle_b.cos(),
                center.y + radius * angle_b.sin(),
            );

            roads.push(RoadSegment {
                start: a,
                end: b,
                width: config.road_width,
            });
        }
    }

    roads
}

/// Returns only the hex road segments surrounding the town square.
pub fn hex_ring_segments(config: &TradeDistrictConfig) -> Vec<RoadSegment> {
    let hex_radius = config.town_square_radius + config.road_width;
    generate_hex_ring(hex_radius, config.road_width, Vec2::ZERO)
}

/// Returns the hex road and outer ring road segments for connector road computation.
pub fn all_ring_segments(config: &TradeDistrictConfig) -> Vec<RoadSegment> {
    let center = Vec2::ZERO;
    let sq = config.town_square_radius;
    let hex_radius = sq + config.road_width;
    let mut segments = generate_hex_ring(hex_radius, config.road_width, center);

    for ring_idx in 1..=config.ring_count {
        let radius = sq + config.ring_spacing * ring_idx as f32;
        for radial_idx in 0..config.radial_count {
            let next_idx = (radial_idx + 1) % config.radial_count;
            let angle_a =
                (radial_idx as f32) * std::f32::consts::TAU / (config.radial_count as f32);
            let angle_b = (next_idx as f32) * std::f32::consts::TAU / (config.radial_count as f32);

            let a = Vec2::new(
                center.x + radius * angle_a.cos(),
                center.y + radius * angle_a.sin(),
            );
            let b = Vec2::new(
                center.x + radius * angle_b.cos(),
                center.y + radius * angle_b.sin(),
            );

            segments.push(RoadSegment {
                start: a,
                end: b,
                width: config.road_width,
            });
        }
    }

    segments
}

/// Generates 6 segments forming a hexagonal ring at the given radius.
fn generate_hex_ring(radius: f32, width: f32, center: Vec2) -> Vec<RoadSegment> {
    let mut segments = Vec::new();
    let sides = 6;
    for i in 0..sides {
        let angle_a = (i as f32) * std::f32::consts::TAU / (sides as f32);
        let angle_b = ((i + 1) as f32) * std::f32::consts::TAU / (sides as f32);
        let a = Vec2::new(
            center.x + radius * angle_a.cos(),
            center.y + radius * angle_a.sin(),
        );
        let b = Vec2::new(
            center.x + radius * angle_b.cos(),
            center.y + radius * angle_b.sin(),
        );
        segments.push(RoadSegment {
            start: a,
            end: b,
            width,
        });
    }
    segments
}

/// Projects a point onto a line segment and returns the closest point.
pub fn closest_point_on_segment(a: Vec2, b: Vec2, p: Vec2) -> Vec2 {
    let ab = b - a;
    let ap = p - a;
    let ab_len_sq = ab.x * ab.x + ab.y * ab.y;
    if ab_len_sq < f32::EPSILON {
        return a;
    }
    let t = ((ap.x * ab.x + ap.y * ab.y) / ab_len_sq).clamp(0.0, 1.0);
    Vec2::new(a.x + ab.x * t, a.y + ab.y * t)
}

/// Finds the closest point on any road segment to the given position.
pub fn nearest_road_point(roads: &[RoadSegment], pos: Vec2) -> Option<Vec2> {
    let mut best_dist = f32::MAX;
    let mut best_point = None;

    for road in roads {
        let point = closest_point_on_segment(road.start, road.end, pos);
        let dist = pos.distance_to(point);
        if dist < best_dist {
            best_dist = dist;
            best_point = Some(point);
        }
    }

    best_point
}

/// Computes a connector road from a lot entrance toward the town square.
pub fn connector_road_from_entrance(
    entrance: Vec2,
    direction_to_center: Vec2,
    roads: &[RoadSegment],
    road_width: f32,
) -> Option<RoadSegment> {
    let target = nearest_road_intersection_along_ray(roads, entrance, direction_to_center)?;
    if entrance.distance_to(target) < 0.5 {
        return None;
    }

    Some(RoadSegment {
        start: entrance,
        end: target,
        width: road_width * 0.7,
    })
}

pub fn nearest_road_intersection_along_ray(
    roads: &[RoadSegment],
    origin: Vec2,
    direction: Vec2,
) -> Option<Vec2> {
    let dir_len = direction.length();
    if dir_len < f32::EPSILON {
        return None;
    }
    let dir = direction / dir_len;
    let mut best_t = f32::MAX;
    let mut best_point = None;

    for road in roads {
        let Some((t, u)) = ray_segment_intersection(origin, dir, road.start, road.end) else {
            continue;
        };
        if t > 0.01 && (0.0..=1.0).contains(&u) && t < best_t {
            best_t = t;
            best_point = Some(origin + dir * t);
        }
    }

    best_point
}

fn ray_segment_intersection(origin: Vec2, dir: Vec2, a: Vec2, b: Vec2) -> Option<(f32, f32)> {
    let segment = b - a;
    let denom = cross(dir, segment);
    if denom.abs() < f32::EPSILON {
        return None;
    }

    let to_segment = a - origin;
    let t = cross(to_segment, segment) / denom;
    let u = cross(to_segment, dir) / denom;
    Some((t, u))
}

fn cross(a: Vec2, b: Vec2) -> f32 {
    a.x * b.y - a.y * b.x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_road_network_non_empty() {
        let config = TradeDistrictConfig::default();
        let roads = generate_road_network(&config);
        assert!(!roads.is_empty());
    }

    #[test]
    fn test_radial_count() {
        let config = TradeDistrictConfig::default();
        let roads = generate_road_network(&config);
        // 6 hex + 6 radials + 2 rings * 6 segments = 6 + 6 + 12 = 24
        assert_eq!(roads.len(), 6 + 6 + 2 * 6);
    }

    #[test]
    fn test_ring_segments_count() {
        let config = TradeDistrictConfig::default();
        let segments = all_ring_segments(&config);
        // 6 hex + 2 rings * 6 = 6 + 12 = 18
        assert_eq!(segments.len(), 6 + 2 * 6);
    }

    #[test]
    fn test_hex_ring_vertices() {
        let segments = generate_hex_ring(10.0, 1.5, Vec2::ZERO);
        assert_eq!(segments.len(), 6);
        // First vertex should be at (10, 0)
        assert!((segments[0].start.x - 10.0).abs() < 0.01);
        assert!((segments[0].start.y - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_closest_point_on_segment() {
        let a = Vec2::new(0.0, 0.0);
        let b = Vec2::new(10.0, 0.0);

        let p = Vec2::new(5.0, 3.0);
        let closest = closest_point_on_segment(a, b, p);
        assert!((closest.x - 5.0).abs() < 0.01);
        assert!((closest.y - 0.0).abs() < 0.01);

        let p = Vec2::new(-2.0, 0.0);
        let closest = closest_point_on_segment(a, b, p);
        assert!((closest.x - 0.0).abs() < 0.01);

        let p = Vec2::new(15.0, 0.0);
        let closest = closest_point_on_segment(a, b, p);
        assert!((closest.x - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_nearest_road_intersection_along_ray() {
        let roads = vec![
            RoadSegment {
                start: Vec2::new(-5.0, 0.0),
                end: Vec2::new(5.0, 0.0),
                width: 1.0,
            },
            RoadSegment {
                start: Vec2::new(-5.0, 3.0),
                end: Vec2::new(5.0, 3.0),
                width: 1.0,
            },
        ];

        let hit =
            nearest_road_intersection_along_ray(&roads, Vec2::new(1.0, 6.0), Vec2::new(0.0, -1.0))
                .unwrap();

        assert!((hit.x - 1.0).abs() < 0.01);
        assert!((hit.y - 3.0).abs() < 0.01);
    }
}
