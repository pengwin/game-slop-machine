use noise::{Fbm, MultiFractal, NoiseFn, Perlin};

pub fn fbm(seed: u32, frequency: f64, octaves: usize, u: f32, v: f32) -> f32 {
    let noise = Fbm::<Perlin>::new(seed)
        .set_frequency(frequency)
        .set_octaves(octaves);
    (noise.get([u as f64, v as f64]) as f32 * 0.5 + 0.5).clamp(0.0, 1.0)
}

pub fn global_dirt_color(seed: u32, position: [f32; 3], normal: [f32; 3]) -> [f32; 4] {
    let [x, y, z] = position;
    let (u, v) = if normal[1].abs() > 0.75 {
        (x, z)
    } else if normal[0].abs() > normal[2].abs() {
        (z, y)
    } else {
        (x, y)
    };

    let dirt_mask = fbm(95 ^ seed, 0.35, 4, u, v);
    let fine_stain = fbm(96 ^ seed, 2.4, 3, u + 0.17, v - 0.11);
    let dirt = (dirt_mask * 1.45 + fine_stain * 0.45 - 0.28).clamp(0.0, 1.0);

    let bottom_dirt = (1.0 - (y * 0.78)).clamp(0.0, 1.0);
    let corner_hint = (normal[1].abs() < 0.25) as i32 as f32 * fbm(97 ^ seed, 1.6, 2, x, z) * 0.18;

    let total_dirt = (dirt + bottom_dirt * 0.9 + corner_hint).clamp(0.0, 1.0);

    let r = 1.0 - total_dirt * 0.35;
    let g = 1.0 - total_dirt * 0.45;
    let b = 1.0 - total_dirt * 0.55;

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

pub fn mortar_mask(coord: f32, mortar: f32) -> f32 {
    let f = coord.fract();
    if f < mortar || f > 1.0 - mortar {
        0.0
    } else {
        1.0
    }
}
