use crate::TextureSize;

#[derive(Clone)]
pub struct WorkingMaps {
    pub size: TextureSize,
    pub height: Vec<f32>,
    pub tone: Vec<f32>,
    pub macro_tone: Vec<f32>,
    pub stain: Vec<f32>,
    pub crack: Vec<f32>,
    pub crack_lip: Vec<f32>,
    pub pit: Vec<f32>,
}

impl WorkingMaps {
    pub fn new(size: TextureSize) -> Self {
        let len = (size.width * size.height) as usize;
        Self {
            size,
            height: vec![0.0; len],
            tone: vec![0.0; len],
            macro_tone: vec![0.0; len],
            stain: vec![0.0; len],
            crack: vec![0.0; len],
            crack_lip: vec![0.0; len],
            pit: vec![0.0; len],
        }
    }

    pub const fn index(&self, x: u32, y: u32) -> usize {
        (y * self.size.width + x) as usize
    }
}
