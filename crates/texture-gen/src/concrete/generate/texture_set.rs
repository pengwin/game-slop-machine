use crate::GeneratedTexture;

/// Complete concrete PBR texture set.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConcreteTextureSet {
    /// Base color texture.
    pub albedo: GeneratedTexture,
    /// Tangent-space normal map.
    pub normal: GeneratedTexture,
    /// Occlusion, roughness, metallic packed texture.
    pub orm: GeneratedTexture,
}
