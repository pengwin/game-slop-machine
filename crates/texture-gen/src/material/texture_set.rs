use crate::GeneratedTexture;

/// Complete PBR texture set produced by any material generator.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PbrTextureSet {
    /// Base color texture (sRGB).
    pub albedo: GeneratedTexture,
    /// Tangent-space normal map (Linear).
    pub normal: GeneratedTexture,
    /// Occlusion, roughness, metallic packed texture (Linear).
    pub orm: GeneratedTexture,
}
