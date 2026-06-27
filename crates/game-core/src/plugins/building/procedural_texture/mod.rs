mod brick;
mod builders;
mod concrete;
mod floor;
mod noise;
mod plaster;
mod road;
mod roof;
mod stone;
#[cfg(test)]
mod tests;
mod wood;

use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
    mpsc::{Receiver, Sender, channel},
};

pub use noise::{fbm, global_dirt_color};

pub use brick::{BrickParams, brick_albedo, brick_normal, brick_orm};
pub use concrete::{ConcreteParams, concrete_albedo, concrete_normal, concrete_orm};
pub use floor::{FloorParams, floor_albedo, floor_normal, floor_orm};
pub use plaster::{
    PlasterParams, plaster_albedo, plaster_normal, plaster_orm, plaster_preview_albedo,
};
pub use road::{RoadParams, road_albedo, road_normal, road_orm};
pub use roof::{RoofParams, roof_albedo, roof_normal, roof_orm};
pub use stone::{StoneParams, stone_albedo, stone_normal, stone_orm};
pub use wood::{WoodParams, wood_albedo, wood_normal, wood_orm};

fn hash_params(params: &impl Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    params.hash(&mut hasher);
    hasher.finish()
}

#[derive(Resource)]
pub struct ProceduralTextures {
    cache: HashMap<String, Handle<Image>>,
    sender: Sender<(Handle<Image>, Image)>,
    receiver: Mutex<Receiver<(Handle<Image>, Image)>>,
    pending: Arc<AtomicUsize>,
}

fn texture_key_value(value: f32) -> i32 {
    (value.clamp(0.0, 1.0) * 1000.0).round() as i32
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
        if let Some(mut existing) = images.get_mut(&handle) {
            *existing = image;
        } else {
            let _ = images.insert(&handle, image);
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

        let is_normal = key.contains("_normal_");
        let is_orm = key.contains("_orm_");
        let placeholder = if is_orm {
            builders::flat_orm(1.0, 0.8, 0.0)
        } else {
            builders::create_placeholder(is_normal)
        };
        let handle = images.add(placeholder);
        self.cache.insert(key.to_string(), handle.clone());
        self.pending.fetch_add(1, Ordering::Relaxed);

        let sender = self.sender.clone();
        let handle_clone = handle.clone();
        let key_clone = key.to_string();
        let is_linear = is_normal || is_orm;
        AsyncComputeTaskPool::get()
            .spawn(async move {
                let path = format!("assets/generated/textures/{}.png", key_clone);
                let image = if let Ok(img) = image::open(&path) {
                    let rgba = img.to_rgba8();
                    builders::create_image(
                        builders::TEXTURE_SIZE,
                        builders::TEXTURE_SIZE,
                        rgba.into_raw(),
                        is_linear,
                    )
                } else {
                    let generated = generator();
                    if let Some(parent) = std::path::Path::new(&path).parent() {
                        let _ = std::fs::create_dir_all(parent);
                    }
                    if let Some(img) = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(
                        builders::TEXTURE_SIZE,
                        builders::TEXTURE_SIZE,
                        generated.data.clone().unwrap(),
                    ) {
                        let _ = image::DynamicImage::ImageRgba8(img).save(&path);
                    }
                    generated
                };
                let _ = sender.send((handle_clone, image));
            })
            .detach();

        handle
    }

    pub fn pending_count(&self) -> usize {
        self.pending.load(Ordering::Relaxed)
    }

    pub fn get_flat_normal(&mut self, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate("flat_normal_default", images, builders::flat_normal)
    }

    pub fn get_flat_orm(
        &mut self,
        label: &str,
        images: &mut Assets<Image>,
        occlusion: f32,
        roughness: f32,
        metallic: f32,
    ) -> Handle<Image> {
        let key = format!(
            "flat_orm_{}_{}_{}_{}",
            label,
            texture_key_value(occlusion),
            texture_key_value(roughness),
            texture_key_value(metallic)
        );
        self.get_or_generate(&key, images, move || {
            builders::flat_orm(occlusion, roughness, metallic)
        })
    }

    // --- Plaster ---

    pub fn get_plaster_albedo(
        &mut self,
        params: &PlasterParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("plaster_albedo_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || plaster::plaster_albedo(&p))
    }

    pub fn get_plaster_preview_albedo(
        &mut self,
        params: &PlasterParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("plaster_preview_albedo_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || plaster::plaster_preview_albedo(&p))
    }

    pub fn get_plaster_preview_albedo_now(
        &mut self,
        params: &PlasterParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("plaster_preview_albedo_{:016x}", h);
        if let Some(handle) = self.cache.get(&key) {
            return handle.clone();
        }
        let handle = images.add(plaster::plaster_preview_albedo(params));
        self.cache.insert(key, handle.clone());
        handle
    }

    pub fn get_plaster_normal_now(
        &mut self,
        params: &PlasterParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("plaster_normal_{:016x}", h);
        if let Some(handle) = self.cache.get(&key) {
            return handle.clone();
        }
        let handle = images.add(plaster::plaster_normal(params));
        self.cache.insert(key, handle.clone());
        handle
    }

    pub fn get_plaster_orm_now(
        &mut self,
        params: &PlasterParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("plaster_orm_{:016x}", h);
        if let Some(handle) = self.cache.get(&key) {
            return handle.clone();
        }
        let handle = images.add(plaster::plaster_orm(params));
        self.cache.insert(key, handle.clone());
        handle
    }

    pub fn get_plaster_normal(
        &mut self,
        params: &PlasterParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("plaster_normal_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || plaster::plaster_normal(&p))
    }

    pub fn get_plaster_orm(
        &mut self,
        params: &PlasterParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("plaster_orm_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || plaster::plaster_orm(&p))
    }

    // --- Wood ---

    pub fn get_wood_albedo(
        &mut self,
        params: &WoodParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("wood_albedo_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || wood::wood_albedo(&p))
    }

    pub fn get_wood_albedo_now(
        &mut self,
        params: &WoodParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("wood_albedo_{:016x}", h);
        if let Some(handle) = self.cache.get(&key) {
            return handle.clone();
        }
        let handle = images.add(wood::wood_albedo(params));
        self.cache.insert(key, handle.clone());
        handle
    }

    pub fn get_wood_normal(
        &mut self,
        params: &WoodParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("wood_normal_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || wood::wood_normal(&p))
    }

    pub fn get_wood_orm(
        &mut self,
        params: &WoodParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("wood_orm_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || wood::wood_orm(&p))
    }

    pub fn get_wood_normal_now(
        &mut self,
        params: &WoodParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("wood_normal_{:016x}", h);
        if let Some(handle) = self.cache.get(&key) {
            return handle.clone();
        }
        let handle = images.add(wood::wood_normal(params));
        self.cache.insert(key, handle.clone());
        handle
    }

    // --- Brick ---

    pub fn get_brick_albedo(
        &mut self,
        params: &BrickParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("brick_albedo_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || brick::brick_albedo(&p))
    }

    pub fn get_brick_normal(
        &mut self,
        params: &BrickParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("brick_normal_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || brick::brick_normal(&p))
    }

    pub fn get_brick_orm(
        &mut self,
        params: &BrickParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("brick_orm_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || brick::brick_orm(&p))
    }

    // --- Roof ---

    pub fn get_roof_albedo(
        &mut self,
        params: &RoofParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("roof_albedo_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || roof::roof_albedo(&p))
    }

    pub fn get_roof_normal(
        &mut self,
        params: &RoofParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("roof_normal_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || roof::roof_normal(&p))
    }

    pub fn get_roof_orm(
        &mut self,
        params: &RoofParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("roof_orm_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || roof::roof_orm(&p))
    }

    // --- Stone ---

    pub fn get_stone_albedo(
        &mut self,
        params: &StoneParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("stone_albedo_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || stone::stone_albedo(&p))
    }

    pub fn get_stone_normal(
        &mut self,
        params: &StoneParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("stone_normal_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || stone::stone_normal(&p))
    }

    pub fn get_stone_orm(
        &mut self,
        params: &StoneParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("stone_orm_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || stone::stone_orm(&p))
    }

    // --- Road ---

    pub fn get_road_albedo(
        &mut self,
        params: &RoadParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("road_albedo_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || road::road_albedo(&p))
    }

    pub fn get_road_normal(
        &mut self,
        params: &RoadParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("road_normal_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || road::road_normal(&p))
    }

    pub fn get_road_orm(
        &mut self,
        params: &RoadParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("road_orm_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || road::road_orm(&p))
    }

    // --- Concrete ---

    pub fn get_concrete_albedo(
        &mut self,
        params: &ConcreteParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("concrete_albedo_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || concrete::concrete_albedo(&p))
    }

    pub fn get_concrete_normal(
        &mut self,
        params: &ConcreteParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("concrete_normal_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || concrete::concrete_normal(&p))
    }

    pub fn get_concrete_orm(
        &mut self,
        params: &ConcreteParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("concrete_orm_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || concrete::concrete_orm(&p))
    }

    // --- Floor ---

    pub fn get_floor_albedo(
        &mut self,
        params: &FloorParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("floor_albedo_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || floor::floor_albedo(&p))
    }

    pub fn get_floor_normal(
        &mut self,
        params: &FloorParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("floor_normal_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || floor::floor_normal(&p))
    }

    pub fn get_floor_orm(
        &mut self,
        params: &FloorParams,
        images: &mut Assets<Image>,
    ) -> Handle<Image> {
        let h = hash_params(params);
        let key = format!("floor_orm_{:016x}", h);
        let p = params.clone();
        self.get_or_generate(&key, images, move || floor::floor_orm(&p))
    }
}
