/// Runtime texture size used by the inspector preview.
pub const RUNTIME_TEXTURE_SIZE: TextureSize = TextureSize {
    width: 512,
    height: 512,
};

/// Small texture size intended for fast unit tests.
pub const TEST_TEXTURE_SIZE: TextureSize = TextureSize {
    width: 128,
    height: 128,
};

/// Dimensions for a generated texture.
/// Note: it's ok to keep it Copy, since it's just two u32 values
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureSize {
    /// Texture width in pixels.
    pub width: u32,
    /// Texture height in pixels.
    pub height: u32,
}

impl TextureSize {
    /// Returns the total RGBA byte count for this texture.
    #[must_use]
    pub const fn rgba_len(self) -> usize {
        (self.width as usize) * (self.height as usize) * 4
    }
}

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
