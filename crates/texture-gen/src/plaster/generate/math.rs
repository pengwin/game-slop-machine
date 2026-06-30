use crate::TextureSize;
use num_traits::ToPrimitive;

pub fn u32_to_f32(value: u32) -> f32 {
    value.to_f32().unwrap_or(0.0)
}

pub fn write_rgba(data: &mut [u8], size: TextureSize, x: u32, y: u32, rgba: [f32; 4]) {
    let i = ((y * size.width + x) * 4) as usize;
    data[i] = to_u8(rgba[0]);
    data[i + 1] = to_u8(rgba[1]);
    data[i + 2] = to_u8(rgba[2]);
    data[i + 3] = to_u8(rgba[3]);
}

fn to_u8(value: f32) -> u8 {
    value
        .clamp(0.0, 1.0)
        .mul_add(255.0, 0.0)
        .round()
        .to_u8()
        .unwrap_or(0)
}

pub fn smooth01(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * 2.0f32.mul_add(-t, 3.0)
}

pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0).max(0.00001)).clamp(0.0, 1.0);
    smooth01(t)
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (b - a).mul_add(t, a)
}

pub fn wrapped_delta(delta: f32) -> f32 {
    if delta > 0.5 {
        delta - 1.0
    } else if delta < -0.5 {
        delta + 1.0
    } else {
        delta
    }
}

pub fn normalize3(v: [f32; 3]) -> [f32; 3] {
    let len = v[2].mul_add(v[2], v[1].mul_add(v[1], v[0] * v[0])).sqrt();
    if len <= f32::EPSILON {
        [0.0, 0.0, 1.0]
    } else {
        [v[0] / len, v[1] / len, v[2] / len]
    }
}

pub fn distance_to_segment(p: [f32; 2], a: [f32; 2], b: [f32; 2]) -> f32 {
    let ab = [b[0] - a[0], b[1] - a[1]];
    let ap = [p[0] - a[0], p[1] - a[1]];
    let ab_len2 = ab[1].mul_add(ab[1], ab[0] * ab[0]);

    if ab_len2 <= f32::EPSILON {
        let dx = p[0] - a[0];
        let dy = p[1] - a[1];
        return dx.hypot(dy);
    }

    let t = (ap[1].mul_add(ab[1], ap[0] * ab[0]) / ab_len2).clamp(0.0, 1.0);
    let closest = [ab[0].mul_add(t, a[0]), ab[1].mul_add(t, a[1])];
    let dx = p[0] - closest[0];
    let dy = p[1] - closest[1];
    dx.hypot(dy)
}
