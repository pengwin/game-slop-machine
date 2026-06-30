use super::TextureSize;

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
