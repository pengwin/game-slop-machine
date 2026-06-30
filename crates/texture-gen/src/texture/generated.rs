/// Generated texture color interpretation.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TextureColorSpace {
    /// Color data should be sampled as sRGB.
    Srgb,
    /// Data should be sampled linearly.
    Linear,
}

/// One generated RGBA texture.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GeneratedTexture {
    /// Texture width in pixels.
    pub width: u32,
    /// Texture height in pixels.
    pub height: u32,
    /// RGBA8 pixel data.
    pub data: Vec<u8>,
    /// Color interpretation for the data.
    pub color_space: TextureColorSpace,
}
