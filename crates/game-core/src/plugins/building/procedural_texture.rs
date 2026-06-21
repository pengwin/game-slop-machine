use bevy::asset::RenderAssetUsages;
use bevy::image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::tasks::AsyncComputeTaskPool;
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};
use std::collections::HashMap;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
    mpsc::{Receiver, Sender, channel},
};

const TEXTURE_SIZE: u32 = 256;

#[derive(Resource)]
pub struct ProceduralTextures {
    cache: HashMap<String, Handle<Image>>,
    sender: Sender<(Handle<Image>, Image)>,
    receiver: Mutex<Receiver<(Handle<Image>, Image)>>,
    pending: Arc<AtomicUsize>,
}

impl Default for ProceduralTextures {
    fn default() -> Self {
        let (sender, receiver) = channel();
        Self {
            cache: HashMap::new(),
            sender,
            receiver: Mutex::new(receiver),
            pending: Arc::new(AtomicUsize::new(0)),
        }
    }
}

pub fn update_procedural_textures(
    textures: Res<ProceduralTextures>,
    mut images: ResMut<Assets<Image>>,
) {
    let receiver = textures.receiver.lock().unwrap();
    while let Ok((handle, image)) = receiver.try_recv() {
        if images.insert(&handle, image).is_err() {
            warn!("Generated procedural texture for a dropped handle");
        }
        textures.pending.fetch_sub(1, Ordering::Relaxed);
        info!("Async procedural texture generated and updated");
    }
}

impl ProceduralTextures {
    fn get_or_generate(
        &mut self,
        key: &str,
        images: &mut Assets<Image>,
        generator: impl FnOnce() -> Image + Send + 'static,
    ) -> Handle<Image> {
        if let Some(handle) = self.cache.get(key) {
            return handle.clone();
        }

        info!("Queueing async generation for texture: {}", key);

        let placeholder = create_placeholder(key.contains("_normal_"));
        let handle = images.add(placeholder);
        self.cache.insert(key.to_string(), handle.clone());
        self.pending.fetch_add(1, Ordering::Relaxed);

        let sender = self.sender.clone();
        let handle_clone = handle.clone();
        AsyncComputeTaskPool::get()
            .spawn(async move {
                let image = generator();
                let _ = sender.send((handle_clone, image));
            })
            .detach();

        handle
    }

    pub fn pending_count(&self) -> usize {
        self.pending.load(Ordering::Relaxed)
    }

    pub fn get_plaster_albedo(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("plaster_albedo_{}", seed), images, move || {
            plaster_albedo(seed)
        })
    }

    pub fn get_plaster_preview_albedo(
        &mut self,
        seed: u32,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        self.get_or_generate(
            &format!("plaster_preview_albedo_{}", seed),
            images,
            move || plaster_preview_albedo(seed),
        )
    }

    pub fn get_plaster_preview_albedo_now(
        &mut self,
        seed: u32,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let key = format!("plaster_preview_albedo_{}", seed);
        if let Some(handle) = self.cache.get(&key) {
            return handle.clone();
        }

        let handle = images.add(plaster_preview_albedo(seed));
        self.cache.insert(key, handle.clone());
        handle
    }

    pub fn get_plaster_normal_now(
        &mut self,
        seed: u32,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let key = format!("plaster_normal_{}", seed);
        if let Some(handle) = self.cache.get(&key) {
            return handle.clone();
        }

        let handle = images.add(plaster_normal(seed));
        self.cache.insert(key, handle.clone());
        handle
    }

    pub fn get_plaster_orm_now(
        &mut self,
        seed: u32,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let key = format!("plaster_orm_{}", seed);
        if let Some(handle) = self.cache.get(&key) {
            return handle.clone();
        }

        let handle = images.add(plaster_orm(seed));
        self.cache.insert(key, handle.clone());
        handle
    }

    pub fn get_plaster_normal(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("plaster_normal_{}", seed), images, move || {
            plaster_normal(seed)
        })
    }

    pub fn get_plaster_orm(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("plaster_orm_{}", seed), images, move || {
            plaster_orm(seed)
        })
    }

    pub fn get_wood_albedo(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("wood_albedo_{}", seed), images, move || {
            wood_albedo(seed)
        })
    }

    pub fn get_wood_normal(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("wood_normal_{}", seed), images, move || {
            wood_normal(seed)
        })
    }

    pub fn get_brick_albedo(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("brick_albedo_{}", seed), images, move || {
            brick_albedo(seed)
        })
    }

    pub fn get_brick_normal(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("brick_normal_{}", seed), images, move || {
            brick_normal(seed)
        })
    }

    pub fn get_roof_albedo(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("roof_albedo_{}", seed), images, move || {
            roof_albedo(seed)
        })
    }

    pub fn get_roof_normal(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("roof_normal_{}", seed), images, move || {
            roof_normal(seed)
        })
    }

    pub fn get_stone_albedo(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("stone_albedo_{}", seed), images, move || {
            stone_albedo(seed)
        })
    }

    pub fn get_stone_normal(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("stone_normal_{}", seed), images, move || {
            stone_normal(seed)
        })
    }

    pub fn get_road_albedo(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("road_albedo_{}", seed), images, move || {
            road_albedo(seed)
        })
    }

    pub fn get_road_normal(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("road_normal_{}", seed), images, move || {
            road_normal(seed)
        })
    }
}

fn create_placeholder(is_normal: bool) -> Image {
    let data = if is_normal {
        vec![128, 128, 255, 255]
    } else {
        vec![128, 128, 128, 255]
    };
    let format = if is_normal {
        TextureFormat::Rgba8Unorm
    } else {
        TextureFormat::Rgba8UnormSrgb
    };
    Image::new(
        Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        format,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    )
}

fn create_image(width: u32, height: u32, data: Vec<u8>, is_normal: bool) -> Image {
    let mut image = Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        if is_normal {
            TextureFormat::Rgba8Unorm
        } else {
            TextureFormat::Rgba8UnormSrgb
        },
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    image.sampler = ImageSampler::Descriptor(repeating_linear_sampler());
    image
}

fn repeating_linear_sampler() -> ImageSamplerDescriptor {
    ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        ..default()
    }
}

fn build_albedo(
    base: [f32; 3],
    shade: impl Fn(f32, f32) -> f32,
    tint: impl Fn(f32, f32) -> [f32; 3],
) -> Image {
    let mut data = vec![255; (TEXTURE_SIZE * TEXTURE_SIZE * 4) as usize];
    for y in 0..TEXTURE_SIZE {
        for x in 0..TEXTURE_SIZE {
            let u = x as f32 / TEXTURE_SIZE as f32;
            let v = y as f32 / TEXTURE_SIZE as f32;
            let s = shade(u, v).clamp(0.0, 1.5);
            let t = tint(u, v);
            write_rgba(
                &mut data,
                x,
                y,
                [
                    (base[0] * t[0] * s).clamp(0.0, 1.0),
                    (base[1] * t[1] * s).clamp(0.0, 1.0),
                    (base[2] * t[2] * s).clamp(0.0, 1.0),
                    1.0,
                ],
            );
        }
    }
    create_image(TEXTURE_SIZE, TEXTURE_SIZE, data, false)
}

fn build_normal(height: impl Fn(f32, f32) -> f32, strength: f32) -> Image {
    let mut data = vec![255; (TEXTURE_SIZE * TEXTURE_SIZE * 4) as usize];
    let eps = 1.0 / TEXTURE_SIZE as f32;

    for y in 0..TEXTURE_SIZE {
        for x in 0..TEXTURE_SIZE {
            let u = x as f32 / TEXTURE_SIZE as f32;
            let v = y as f32 / TEXTURE_SIZE as f32;
            let dx = height(u + eps, v) - height(u - eps, v);
            let dy = height(u, v + eps) - height(u, v - eps);
            let n = Vec3::new(-dx * strength, -dy * strength, 1.0).normalize_or_zero();
            write_rgba(
                &mut data,
                x,
                y,
                [n.x * 0.5 + 0.5, n.y * 0.5 + 0.5, n.z * 0.5 + 0.5, 1.0],
            );
        }
    }
    create_image(TEXTURE_SIZE, TEXTURE_SIZE, data, true)
}

fn build_orm(
    occlusion: impl Fn(f32, f32) -> f32,
    roughness: impl Fn(f32, f32) -> f32,
    metallic: impl Fn(f32, f32) -> f32,
) -> Image {
    let mut data = vec![255; (TEXTURE_SIZE * TEXTURE_SIZE * 4) as usize];
    for y in 0..TEXTURE_SIZE {
        for x in 0..TEXTURE_SIZE {
            let u = x as f32 / TEXTURE_SIZE as f32;
            let v = y as f32 / TEXTURE_SIZE as f32;
            let o = occlusion(u, v).clamp(0.0, 1.0);
            let r = roughness(u, v).clamp(0.0, 1.0);
            let m = metallic(u, v).clamp(0.0, 1.0);
            write_rgba(&mut data, x, y, [o, r, m, 1.0]);
        }
    }
    // ORM map shouldn't be sRGB, use linear. Same as normal map flag = true for create_image.
    create_image(TEXTURE_SIZE, TEXTURE_SIZE, data, true)
}

fn write_rgba(data: &mut [u8], x: u32, y: u32, rgba: [f32; 4]) {
    let i = ((y * TEXTURE_SIZE + x) * 4) as usize;
    data[i] = to_u8(rgba[0]);
    data[i + 1] = to_u8(rgba[1]);
    data[i + 2] = to_u8(rgba[2]);
    data[i + 3] = to_u8(rgba[3]);
}

fn to_u8(v: f32) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0).round() as u8
}

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
    
    // Large dirt patches across the wall (frequency 0.25)
    let dirt_mask = fbm(95 ^ seed, 0.25, 3, u, v);
    let dirt = (dirt_mask * 1.6 - 0.2).clamp(0.0, 1.0);
    
    // Vertical gradient: dirty near the floor (y=0)
    let bottom_dirt = (1.0 - (y * 0.6)).clamp(0.0, 1.0);
    
    // Total dirt amount
    let total_dirt = (dirt + bottom_dirt * 0.7).clamp(0.0, 1.0);
    
    // Shift color: lower RGB and push hue towards warm brown-grey
    let r = 1.0 - total_dirt * 0.35;
    let g = 1.0 - total_dirt * 0.45;
    let b = 1.0 - total_dirt * 0.55;
    
    [r, g, b, 1.0]
}

fn cell_noise(seed: u32, ix: i32, iy: i32) -> f32 {
    let mut n = ix
        .wrapping_mul(374_761_393)
        .wrapping_add(iy.wrapping_mul(668_265_263))
        .wrapping_add(seed as i32 * 97_531);
    n = (n ^ (n >> 13)).wrapping_mul(1_274_126_177);
    ((n ^ (n >> 16)) & 0xffff) as f32 / 65_535.0
}

fn mortar_mask(coord: f32, mortar: f32) -> f32 {
    let f = coord.fract();
    if f < mortar || f > 1.0 - mortar {
        0.0
    } else {
        1.0
    }
}

fn plaster_height(seed: u32, u: f32, v: f32) -> f32 {
    fbm(11 ^ seed, 18.0, 5, u, v) * 0.7 + fbm(12 ^ seed, 58.0, 2, u, v) * 0.3
}

fn plaster_albedo(seed: u32) -> Image {
    build_albedo(
        [0.95, 0.88, 0.70], // Light yellow/sand color
        |u, v| {
            let base = 0.82 + plaster_height(seed, u, v) * 0.20;
            base.clamp(0.4, 1.1)
        },
        |_, _| [1.0, 1.0, 1.0],
    )
}

fn plaster_preview_albedo(seed: u32) -> Image {
    build_albedo(
        [0.95, 0.88, 0.70], // Light yellow/sand color
        |u, v| {
            let broad = fbm(90 ^ seed, 2.3, 5, u * 0.82 + 0.13, v * 0.82 - 0.07);
            let cloudy = fbm(91 ^ seed, 6.5, 4, u + broad * 0.10, v - broad * 0.08);
            let fine = fbm(92 ^ seed, 30.0, 2, u, v);
            
            let base_shade = 0.90 + broad * 0.10 + cloudy * 0.07 + fine * 0.020;
            base_shade.clamp(0.6, 1.1)
        },
        |u, v| {
            let stain = fbm(93 ^ seed, 3.4, 4, u + 0.17, v - 0.11);
            let age = fbm(94 ^ seed, 12.0, 2, u - 0.23, v + 0.19);
            
            [
                0.96 + stain * 0.030 + age * 0.010,
                0.96 + stain * 0.026 + age * 0.008,
                0.92 + stain * 0.020,
            ]
        },
    )
}

fn plaster_normal(seed: u32) -> Image {
    build_normal(|u, v| plaster_height(seed, u, v), 1.4)
}

fn plaster_orm(seed: u32) -> Image {
    build_orm(
        |u, v| {
            // AO: deeper areas are darker
            let h = plaster_height(seed, u, v);
            0.6 + h * 0.4
        },
        |u, v| {
            // Roughness: higher areas are slightly smoother, deeper areas are rougher
            let h = plaster_height(seed, u, v);
            0.98 - h * 0.15
        },
        |_, _| 0.0, // Metallic: 0
    )
}

fn wood_height(seed: u32, u: f32, v: f32) -> f32 {
    let plank = (v * 5.0).fract();
    let seam = if !(0.035..=0.965).contains(&plank) {
        -0.55
    } else {
        0.0
    };
    let grain = fbm(21 ^ seed, 22.0, 4, u * 3.5, v * 0.55);
    let fine = ((u * 72.0 + fbm(22 ^ seed, 9.0, 2, u, v) * 3.0).sin() * 0.5 + 0.5) * 0.22;
    seam + grain * 0.58 + fine
}

fn wood_albedo(seed: u32) -> Image {
    build_albedo(
        [0.64, 0.43, 0.24],
        |u, v| {
            let h = wood_height(seed, u, v);
            (0.70 + h * 0.30).max(0.34)
        },
        |_, v| {
            let plank_id = (v * 5.0).floor() as i32;
            let n = cell_noise(23 ^ seed, 0, plank_id);
            [0.92 + n * 0.18, 0.90 + n * 0.10, 0.86 + n * 0.08]
        },
    )
}

fn wood_normal(seed: u32) -> Image {
    build_normal(|u, v| wood_height(seed, u, v), 2.1)
}

fn brick_height(seed: u32, u: f32, v: f32) -> f32 {
    let rows = v * 7.0;
    let row = rows.floor() as i32;
    let offset = if row % 2 == 0 { 0.5 } else { 0.0 };
    let cols = u * 4.0 + offset;
    let brick = mortar_mask(rows, 0.045) * mortar_mask(cols, 0.035);
    let surface = fbm(31 ^ seed, 38.0, 3, u, v) * 0.2;
    brick * (0.72 + surface)
}

fn brick_albedo(seed: u32) -> Image {
    build_albedo(
        [0.69, 0.37, 0.23],
        |u, v| {
            let rows = v * 7.0;
            let row = rows.floor() as i32;
            let offset = if row % 2 == 0 { 0.5 } else { 0.0 };
            let cols = u * 4.0 + offset;
            let mortar = mortar_mask(rows, 0.045) * mortar_mask(cols, 0.035);
            if mortar < 0.5 {
                0.52
            } else {
                0.78 + fbm(32 ^ seed, 30.0, 2, u, v) * 0.24
            }
        },
        |u, v| {
            let row = (v * 7.0).floor() as i32;
            let col = (u * 4.0).floor() as i32;
            let n = cell_noise(33 ^ seed, col, row);
            [0.88 + n * 0.25, 0.86 + n * 0.12, 0.82 + n * 0.10]
        },
    )
}

fn brick_normal(seed: u32) -> Image {
    build_normal(|u, v| brick_height(seed, u, v), 5.0)
}

fn roof_height(seed: u32, u: f32, v: f32) -> f32 {
    let courses = v * 8.0;
    let columns = u * 6.0
        + if courses.floor() as i32 % 2 == 0 {
            0.5
        } else {
            0.0
        };
    let gap = mortar_mask(courses, 0.055) * mortar_mask(columns, 0.045);
    let curved = ((courses.fract() * std::f32::consts::PI).sin() * 0.32 + 0.58).max(0.0);
    gap * curved + fbm(41 ^ seed, 44.0, 2, u, v) * 0.08
}

fn roof_albedo(seed: u32) -> Image {
    build_albedo(
        [0.55, 0.25, 0.14],
        |u, v| 0.68 + roof_height(seed, u, v) * 0.33,
        |u, v| {
            let n = cell_noise(
                42 ^ seed,
                (u * 6.0).floor() as i32,
                (v * 8.0).floor() as i32,
            );
            [0.86 + n * 0.20, 0.82 + n * 0.10, 0.78 + n * 0.06]
        },
    )
}

fn roof_normal(seed: u32) -> Image {
    build_normal(|u, v| roof_height(seed, u, v), 4.4)
}

fn stone_height(seed: u32, u: f32, v: f32) -> f32 {
    let blocks_y = v * 3.0;
    let row = blocks_y.floor() as i32;
    let blocks_x = u * 5.0 + if row % 2 == 0 { 0.35 } else { 0.0 };
    let joints = mortar_mask(blocks_x, 0.035) * mortar_mask(blocks_y, 0.055);
    joints * (0.55 + fbm(51 ^ seed, 18.0, 4, u, v) * 0.36)
}

fn stone_albedo(seed: u32) -> Image {
    build_albedo(
        [0.47, 0.46, 0.40],
        |u, v| {
            let h = stone_height(seed, u, v);
            if h < 0.08 { 0.56 } else { 0.74 + h * 0.24 }
        },
        |u, v| {
            let n = cell_noise(
                52 ^ seed,
                (u * 5.0).floor() as i32,
                (v * 3.0).floor() as i32,
            );
            [0.90 + n * 0.12, 0.90 + n * 0.10, 0.88 + n * 0.08]
        },
    )
}

fn stone_normal(seed: u32) -> Image {
    build_normal(|u, v| stone_height(seed, u, v), 3.0)
}

fn road_height(seed: u32, u: f32, v: f32) -> f32 {
    fbm(61 ^ seed, 24.0, 4, u, v) * 0.65 + fbm(62 ^ seed, 95.0, 2, u, v) * 0.35
}

fn road_albedo(seed: u32) -> Image {
    build_albedo(
        [0.42, 0.34, 0.25],
        |u, v| 0.62 + road_height(seed, u, v) * 0.34,
        |u, v| {
            let pebble = fbm(63 ^ seed, 120.0, 1, u, v);
            [
                0.92 + pebble * 0.16,
                0.90 + pebble * 0.12,
                0.86 + pebble * 0.08,
            ]
        },
    )
}

fn road_normal(seed: u32) -> Image {
    build_normal(|u, v| road_height(seed, u, v), 1.8)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn image_bytes(image: &Image) -> &[u8] {
        image
            .data
            .as_ref()
            .expect("procedural images keep CPU data")
    }

    #[test]
    fn generated_images_have_expected_layout() {
        for (image, is_normal) in [
            (plaster_albedo(7), false),
            (plaster_preview_albedo(7), false),
            (plaster_normal(7), true),
            (wood_albedo(7), false),
            (wood_normal(7), true),
            (brick_albedo(7), false),
            (brick_normal(7), true),
            (roof_albedo(7), false),
            (roof_normal(7), true),
            (stone_albedo(7), false),
            (stone_normal(7), true),
            (road_albedo(7), false),
            (road_normal(7), true),
        ] {
            assert_eq!(image.texture_descriptor.size.width, TEXTURE_SIZE);
            assert_eq!(image.texture_descriptor.size.height, TEXTURE_SIZE);
            assert_eq!(
                image_bytes(&image).len(),
                (TEXTURE_SIZE * TEXTURE_SIZE * 4) as usize
            );
            assert_eq!(
                image.texture_descriptor.format,
                if is_normal {
                    TextureFormat::Rgba8Unorm
                } else {
                    TextureFormat::Rgba8UnormSrgb
                }
            );
        }
    }

    #[test]
    fn placeholders_match_texture_kind() {
        let albedo = create_placeholder(false);
        let normal = create_placeholder(true);

        assert_eq!(image_bytes(&albedo), &[128, 128, 128, 255]);
        assert_eq!(
            albedo.texture_descriptor.format,
            TextureFormat::Rgba8UnormSrgb
        );
        assert_eq!(image_bytes(&normal), &[128, 128, 255, 255]);
        assert_eq!(normal.texture_descriptor.format, TextureFormat::Rgba8Unorm);
    }

    #[test]
    fn albedo_images_are_not_flat_fills() {
        for image in [
            plaster_albedo(11),
            plaster_preview_albedo(11),
            wood_albedo(11),
            brick_albedo(11),
            roof_albedo(11),
            stone_albedo(11),
            road_albedo(11),
        ] {
            let bytes = image_bytes(&image);
            assert!(
                bytes.chunks_exact(4).any(|px| px != &bytes[0..4]),
                "albedo texture should contain visible variation"
            );
        }
    }

    #[test]
    fn generation_is_deterministic_per_seed() {
        assert_eq!(image_bytes(&wood_albedo(5)), image_bytes(&wood_albedo(5)));
        assert_eq!(
            image_bytes(&plaster_normal(5)),
            image_bytes(&plaster_normal(5))
        );
    }
}
