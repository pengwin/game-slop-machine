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
