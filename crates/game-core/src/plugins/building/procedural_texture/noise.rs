use noise::{Fbm, MultiFractal, NoiseFn, Perlin};

pub fn fbm(seed: u32, frequency: f64, octaves: usize, u: f32, v: f32) -> f32 {
    let noise = Fbm::<Perlin>::new(seed)
        .set_frequency(frequency)
        .set_octaves(octaves);
    (noise.get([u as f64, v as f64]) as f32 * 0.5 + 0.5).clamp(0.0, 1.0)
}

pub fn global_dirt_color(_seed: u32, position: [f32; 3], _normal: [f32; 3], intensity: f32) -> [f32; 4] {
    let [_, y, _] = position;

    let bottom_dirt = (1.0 - (y * 0.78)).clamp(0.0, 1.0);

    let total_dirt = (bottom_dirt * 0.9).clamp(0.0, 1.0);

    let r = 1.0 - total_dirt * 0.35 * intensity;
    let g = 1.0 - total_dirt * 0.45 * intensity;
    let b = 1.0 - total_dirt * 0.55 * intensity;

    [r, g, b, 1.0]
}

pub fn cell_noise(seed: u32, ix: i32, iy: i32) -> f32 {
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

pub fn mortar_mask(coord: f32, mortar: f32) -> f32 {
    let f = coord.fract();
    if f < mortar || f > 1.0 - mortar {
        0.0
    } else {
        1.0
    }
}
