use super::super::layout::{Lot, RoadSegment};
use crate::geometry::Vec2;

pub fn lots_overlap(a: &Lot, b: &Lot, gap: f32) -> bool {
    let a_corners = lot_corners(a);
    let b_corners = lot_corners(b);
    let axes = [
        normalized(a_corners[1] - a_corners[0]),
        normalized(a_corners[3] - a_corners[0]),
        normalized(b_corners[1] - b_corners[0]),
        normalized(b_corners[3] - b_corners[0]),
    ];

    for axis in axes {
        let (a_min, a_max) = project_polygon(&a_corners, axis);
        let (b_min, b_max) = project_polygon(&b_corners, axis);
        if a_max + gap < b_min || b_max + gap < a_min {
            return false;
        }
    }

    true
}

fn normalized(v: Vec2) -> Vec2 {
    let length = v.length();
    if length <= f32::EPSILON {
        Vec2::ZERO
    } else {
        v / length
    }
}

fn project_polygon(corners: &[Vec2; 4], axis: Vec2) -> (f32, f32) {
    let mut min = dot(corners[0], axis);
    let mut max = min;

    for corner in corners.iter().skip(1) {
        let projection = dot(*corner, axis);
        min = min.min(projection);
        max = max.max(projection);
    }

    (min, max)
}

fn dot(a: Vec2, b: Vec2) -> f32 {
    a.x * b.x + a.y * b.y
}

pub fn lot_overlaps_roads(lot: &Lot, roads: &[RoadSegment], road_width: f32) -> bool {
    let corners = lot_corners(lot);
    let edges = [
        (corners[0], corners[1]),
        (corners[1], corners[2]),
        (corners[2], corners[3]),
        (corners[3], corners[0]),
    ];
    let half_road = road_width / 2.0;

    for road in roads {
        for &(edge_a, edge_b) in &edges {
            if segments_intersect_with_width(road.start, road.end, edge_a, edge_b, half_road) {
                return true;
            }
        }

        let mid = Vec2::new(
            (road.start.x + road.end.x) / 2.0,
            (road.start.y + road.end.y) / 2.0,
        );
        if point_in_rect(mid, &corners) {
            return true;
        }
    }

    false
}

#[cfg(test)]
pub fn road_crosses_lot_interior(road: &RoadSegment, lot: &Lot) -> bool {
    let corners = lot_corners(lot);
    let samples = 12;

    for i in 1..samples {
        let t = i as f32 / samples as f32;
        let p = road.start + (road.end - road.start) * t;
        if point_in_rect(p, &corners) {
            return true;
        }
    }

    false
}

fn lot_corners(lot: &Lot) -> [Vec2; 4] {
    let hw = lot.width / 2.0;
    let hd = lot.depth / 2.0;
    let cos_r = lot.rotation.cos();
    let sin_r = lot.rotation.sin();

    let dx = cos_r;
    let dz = sin_r;
    let wx = -sin_r;
    let wz = cos_r;

    [
        Vec2::new(
            lot.position.x + (-dx * hd - wx * hw),
            lot.position.y + (-dz * hd - wz * hw),
        ),
        Vec2::new(
            lot.position.x + (-dx * hd + wx * hw),
            lot.position.y + (-dz * hd + wz * hw),
        ),
        Vec2::new(
            lot.position.x + (dx * hd + wx * hw),
            lot.position.y + (dz * hd + wz * hw),
        ),
        Vec2::new(
            lot.position.x + (dx * hd - wx * hw),
            lot.position.y + (dz * hd - wz * hw),
        ),
    ]
}

fn segments_intersect_with_width(a1: Vec2, a2: Vec2, b1: Vec2, b2: Vec2, width: f32) -> bool {
    if segments_intersect(a1, a2, b1, b2) {
        return true;
    }
    if point_to_segment_dist(a1, b1, b2) < width || point_to_segment_dist(a2, b1, b2) < width {
        return true;
    }
    if point_to_segment_dist(b1, a1, a2) < width || point_to_segment_dist(b2, a1, a2) < width {
        return true;
    }
    false
}

fn point_to_segment_dist(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let closest = super::super::road::closest_point_on_segment(a, b, p);
    p.distance_to(closest)
}

fn segments_intersect(a1: Vec2, a2: Vec2, b1: Vec2, b2: Vec2) -> bool {
    let d1 = direction(a1, a2, b1);
    let d2 = direction(a1, a2, b2);
    let d3 = direction(b1, b2, a1);
    let d4 = direction(b1, b2, a2);

    if ((d1 > 0.0 && d2 < 0.0) || (d1 < 0.0 && d2 > 0.0))
        && ((d3 > 0.0 && d4 < 0.0) || (d3 < 0.0 && d4 > 0.0))
    {
        return true;
    }

    if d1.abs() < f32::EPSILON && on_segment(a1, a2, b1) {
        return true;
    }
    if d2.abs() < f32::EPSILON && on_segment(a1, a2, b2) {
        return true;
    }
    if d3.abs() < f32::EPSILON && on_segment(b1, b2, a1) {
        return true;
    }
    if d4.abs() < f32::EPSILON && on_segment(b1, b2, a2) {
        return true;
    }

    false
}

fn direction(a: Vec2, b: Vec2, c: Vec2) -> f32 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn on_segment(a: Vec2, b: Vec2, p: Vec2) -> bool {
    p.x >= a.x.min(b.x) - f32::EPSILON
        && p.x <= a.x.max(b.x) + f32::EPSILON
        && p.y >= a.y.min(b.y) - f32::EPSILON
        && p.y <= a.y.max(b.y) + f32::EPSILON
}

fn point_in_rect(p: Vec2, corners: &[Vec2; 4]) -> bool {
    for i in 0..4 {
        let a = corners[i];
        let b = corners[(i + 1) % 4];
        if direction(a, b, p) < -f32::EPSILON {
            return false;
        }
    }
    true
}
