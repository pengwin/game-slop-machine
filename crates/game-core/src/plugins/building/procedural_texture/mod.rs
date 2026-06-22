mod brick;
mod builders;
mod concrete;
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
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
    mpsc::{Receiver, Sender, channel},
};

pub use noise::{fbm, global_dirt_color};

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

        let placeholder = builders::create_placeholder(key.contains("_normal_"));
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
            plaster::plaster_albedo(seed)
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
            move || plaster::plaster_preview_albedo(seed),
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

        let handle = images.add(plaster::plaster_preview_albedo(seed));
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

        let handle = images.add(plaster::plaster_normal(seed));
        self.cache.insert(key, handle.clone());
        handle
    }

    pub fn get_plaster_orm_now(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        let key = format!("plaster_orm_{}", seed);
        if let Some(handle) = self.cache.get(&key) {
            return handle.clone();
        }

        let handle = images.add(plaster::plaster_orm(seed));
        self.cache.insert(key, handle.clone());
        handle
    }

    pub fn get_plaster_normal(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("plaster_normal_{}", seed), images, move || {
            plaster::plaster_normal(seed)
        })
    }

    pub fn get_plaster_orm(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("plaster_orm_{}", seed), images, move || {
            plaster::plaster_orm(seed)
        })
    }

    pub fn get_wood_albedo(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("wood_albedo_{}", seed), images, move || {
            wood::wood_albedo(seed)
        })
    }

    pub fn get_wood_normal(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("wood_normal_{}", seed), images, move || {
            wood::wood_normal(seed)
        })
    }

    pub fn get_brick_albedo(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("brick_albedo_{}", seed), images, move || {
            brick::brick_albedo(seed)
        })
    }

    pub fn get_brick_normal(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("brick_normal_{}", seed), images, move || {
            brick::brick_normal(seed)
        })
    }

    pub fn get_roof_albedo(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("roof_albedo_{}", seed), images, move || {
            roof::roof_albedo(seed)
        })
    }

    pub fn get_roof_normal(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("roof_normal_{}", seed), images, move || {
            roof::roof_normal(seed)
        })
    }

    pub fn get_stone_albedo(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("stone_albedo_{}", seed), images, move || {
            stone::stone_albedo(seed)
        })
    }

    pub fn get_stone_normal(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("stone_normal_{}", seed), images, move || {
            stone::stone_normal(seed)
        })
    }

    pub fn get_road_albedo(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("road_albedo_{}", seed), images, move || {
            road::road_albedo(seed)
        })
    }

    pub fn get_road_normal(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("road_normal_{}", seed), images, move || {
            road::road_normal(seed)
        })
    }

    pub fn get_concrete_albedo(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("concrete_albedo_{}", seed), images, move || {
            concrete::concrete_albedo(seed)
        })
    }

    pub fn get_concrete_normal(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("concrete_normal_{}", seed), images, move || {
            concrete::concrete_normal(seed)
        })
    }

    pub fn get_concrete_orm(&mut self, seed: u32, images: &mut Assets<Image>) -> Handle<Image> {
        self.get_or_generate(&format!("concrete_orm_{}", seed), images, move || {
            concrete::concrete_orm(seed)
        })
    }
}
