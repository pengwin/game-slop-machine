use super::math::{lerp, smooth01, u32_to_f32};
use num_traits::ToPrimitive;

pub(super) fn fbm_tileable(
    seed: u32,
    base_cells: u32,
    octaves: u32,
    sample_u: f32,
    sample_v: f32,
) -> f32 {
    let mut value = 0.0;
    let mut amp = 1.0;
    let mut amp_sum = 0.0;
    let mut cells = base_cells.max(1);

    for octave in 0..octaves {
        value += value_noise_tileable(
            seed ^ octave.wrapping_mul(0x9E37),
            cells,
            sample_u,
            sample_v,
        ) * amp;
        amp_sum += amp;
        amp *= 0.5;
        cells *= 2;
    }

    if amp_sum <= f32::EPSILON {
        0.0
    } else {
        value / amp_sum
    }
}

fn value_noise_tileable(seed: u32, cells: u32, sample_u: f32, sample_v: f32) -> f32 {
    let cells_f = u32_to_f32(cells);
    let sample_x = sample_u.fract() * cells_f;
    let sample_y = sample_v.fract() * cells_f;
    let left_column = sample_x.floor().to_u32().unwrap_or(0);
    let top_row = sample_y.floor().to_u32().unwrap_or(0);
    let right_column = (left_column + 1) % cells;
    let bottom_row = (top_row + 1) % cells;
    let blend_x = smooth01(sample_x - sample_x.floor());
    let blend_y = smooth01(sample_y - sample_y.floor());
    let top_left = hash01(seed, left_column % cells, top_row % cells);
    let top_right = hash01(seed, right_column, top_row % cells);
    let bottom_left = hash01(seed, left_column % cells, bottom_row);
    let bottom_right = hash01(seed, right_column, bottom_row);

    lerp(
        lerp(top_left, top_right, blend_x),
        lerp(bottom_left, bottom_right, blend_x),
        blend_y,
    )
}

fn hash01(seed: u32, x: u32, y: u32) -> f32 {
    let mut h = seed;
    h ^= x.wrapping_mul(0x27D4_EB2D);
    h ^= y.wrapping_mul(0x1656_67B1);
    h ^= h >> 15;
    h = h.wrapping_mul(0x85EB_CA6B);
    h ^= h >> 13;
    h = h.wrapping_mul(0xC2B2_AE35);
    h ^= h >> 16;
    u32_to_f32(h) / u32_to_f32(u32::MAX)
}
