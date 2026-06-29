#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::suboptimal_flops,
    reason = "procedural noise intentionally hashes integer cells and maps samples into f32 ranges"
)]

use noise::{Fbm, MultiFractal, NoiseFn, Perlin};

pub fn fbm(seed: u32, frequency: f64, octaves: usize, u: f32, v: f32) -> f32 {
    let noise = Fbm::<Perlin>::new(seed)
        .set_frequency(frequency)
        .set_octaves(octaves);
    (noise.get([f64::from(u), f64::from(v)]) as f32 * 0.5 + 0.5).clamp(0.0, 1.0)
}

fn cell_noise(seed: u32, ix: i32, iy: i32) -> f32 {
    let mut n = ix
        .wrapping_mul(374_761_393)
        .wrapping_add(iy.wrapping_mul(668_265_263))
        .wrapping_add(seed as i32 * 97_531);
    n = (n ^ (n >> 13)).wrapping_mul(1_274_126_177);
    ((n ^ (n >> 16)) & 0xffff) as f32 / 65_535.0
}

pub fn speckle(seed: u32, scale: f32, threshold: f32, u: f32, v: f32) -> f32 {
    let ix = (u * scale).floor() as i32;
    let iy = (v * scale).floor() as i32;
    let n = cell_noise(seed, ix, iy);
    ((n - threshold) / (1.0 - threshold).max(0.001)).clamp(0.0, 1.0)
}

pub fn hairline(seed: u32, frequency: f64, width: f32, u: f32, v: f32) -> f32 {
    let n = fbm(seed, frequency, 2, u, v);
    (1.0 - ((n - 0.5).abs() / width.max(0.001))).clamp(0.0, 1.0)
}
