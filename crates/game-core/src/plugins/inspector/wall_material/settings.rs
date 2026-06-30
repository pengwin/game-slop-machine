use bevy::{prelude::*, render::render_resource::Face};

/// Editable `StandardMaterial` settings shared by all wall material scenes.
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct WallMaterialSettings {
    /// Red tint multiplier.
    pub tint_r: f32,
    /// Green tint multiplier.
    pub tint_g: f32,
    /// Blue tint multiplier.
    pub tint_b: f32,
    /// Roughness scalar multiplied with the ORM roughness channel.
    pub perceptual_roughness: f32,
    /// Metallic scalar multiplied with the ORM metallic channel.
    pub metallic: f32,
    /// Specular intensity for the non-metal surface.
    pub reflectance: f32,
    /// Enables two-sided lighting in the PBR shader.
    pub double_sided: bool,
    /// Disables backface culling when true.
    pub cull_none: bool,
    /// Shows base color only, ignoring lighting and maps.
    pub unlit: bool,
}

impl Default for WallMaterialSettings {
    fn default() -> Self {
        Self {
            tint_r: 1.0,
            tint_g: 1.0,
            tint_b: 1.0,
            perceptual_roughness: 1.0,
            metallic: 1.0,
            reflectance: 0.5,
            double_sided: false,
            cull_none: false,
            unlit: false,
        }
    }
}

/// Applies the wall material settings to a `StandardMaterial`.
#[allow(clippy::missing_const_for_fn)]
pub fn apply_material_settings(
    material: &mut StandardMaterial,
    settings: &WallMaterialSettings,
) {
    material.base_color = Color::srgba(
        settings.tint_r.clamp(0.0, 2.0),
        settings.tint_g.clamp(0.0, 2.0),
        settings.tint_b.clamp(0.0, 2.0),
        1.0,
    );
    material.perceptual_roughness = settings.perceptual_roughness.clamp(0.0, 1.0);
    material.metallic = settings.metallic.clamp(0.0, 1.0);
    material.reflectance = settings.reflectance.clamp(0.0, 1.0);
    material.double_sided = settings.double_sided;
    material.cull_mode = if settings.cull_none {
        None
    } else {
        Some(Face::Back)
    };
    material.unlit = settings.unlit;
}
