use crate::TextureSize;

#[derive(Clone)]
pub struct WorkingMaps {
    pub size: TextureSize,
    pub height: Vec<f32>,
    pub tone: Vec<f32>,
    pub lime: Vec<f32>,
    pub aggregate: Vec<f32>,
    pub aggregate_tint: Vec<f32>,
    pub void: Vec<f32>,
    pub stain: Vec<f32>,
    pub crack: Vec<f32>,
    pub crack_lip: Vec<f32>,
    pub formwork: Vec<f32>,
    pub exposed_aggregate: Vec<f32>,
    pub efflorescence: Vec<f32>,
}

impl WorkingMaps {
    pub fn new(size: TextureSize) -> Self {
        let len = usize::try_from(size.width * size.height).unwrap_or(0);
        Self {
            size,
            height: vec![0.0; len],
            tone: vec![0.0; len],
            lime: vec![0.0; len],
            aggregate: vec![0.0; len],
            aggregate_tint: vec![0.0; len],
            void: vec![0.0; len],
            stain: vec![0.0; len],
            crack: vec![0.0; len],
            crack_lip: vec![0.0; len],
            formwork: vec![0.0; len],
            exposed_aggregate: vec![0.0; len],
            efflorescence: vec![0.0; len],
        }
    }

    pub const fn index(&self, x: u32, y: u32) -> usize {
        (y * self.size.width + x) as usize
    }
}
