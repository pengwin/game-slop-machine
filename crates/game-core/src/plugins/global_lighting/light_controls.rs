use bevy::prelude::*;

use super::{SceneLightingSettings, SceneShadowCascadeSettings};

/// Runtime controls for tuning the global sun and ambient light.
#[derive(Resource, Clone, PartialEq)]
pub struct GlobalLightControls {
    /// Ambient light brightness.
    pub ambient_brightness: f32,
    /// Directional sun illuminance.
    pub sun_illuminance: f32,
    /// Directional sun elevation in degrees.
    pub sun_elevation_degrees: f32,
    /// Directional sun azimuth in degrees.
    pub sun_azimuth_degrees: f32,
    /// Whether the sun casts shadow maps.
    pub shadows_enabled: bool,
    /// Directional shadow depth bias.
    pub shadow_depth_bias: f32,
    /// Directional shadow normal bias.
    pub shadow_normal_bias: f32,
    /// Number of directional shadow cascades.
    pub cascade_count: usize,
    /// Minimum distance from the camera receiving shadows.
    pub cascade_minimum_distance: f32,
    /// Far bound of the first cascade.
    pub cascade_first_far_bound: f32,
    /// Maximum distance from the camera receiving shadows.
    pub cascade_maximum_distance: f32,
    /// Proportion of overlap between shadow cascades.
    pub cascade_overlap_proportion: f32,
    /// Directional shadow map size.
    pub shadow_map_size: usize,
}

impl GlobalLightControls {
    /// Converts lighting preset values into editable controls.
    #[must_use]
    pub fn from_settings(settings: &SceneLightingSettings) -> Self {
        let (sun_elevation, sun_azimuth, _) = settings.sun_rotation.to_euler(EulerRot::XYZ);

        Self {
            ambient_brightness: settings.ambient_brightness,
            sun_illuminance: settings.sun_illuminance,
            sun_elevation_degrees: sun_elevation.to_degrees(),
            sun_azimuth_degrees: sun_azimuth.to_degrees(),
            shadows_enabled: settings.shadows_enabled,
            shadow_depth_bias: settings.shadow_depth_bias,
            shadow_normal_bias: settings.shadow_normal_bias,
            cascade_count: settings.shadow_cascades.num_cascades,
            cascade_minimum_distance: settings.shadow_cascades.minimum_distance,
            cascade_first_far_bound: settings.shadow_cascades.first_cascade_far_bound,
            cascade_maximum_distance: settings.shadow_cascades.maximum_distance,
            cascade_overlap_proportion: settings.shadow_cascades.overlap_proportion,
            shadow_map_size: 8192,
        }
    }

    /// Returns the sun rotation represented by the editable angle controls.
    #[must_use]
    pub fn sun_rotation(&self) -> Quat {
        Quat::from_euler(
            EulerRot::XYZ,
            self.sun_elevation_degrees.to_radians(),
            self.sun_azimuth_degrees.to_radians(),
            0.0,
        )
    }

    /// Converts these controls into a lighting preset.
    #[must_use]
    pub fn to_settings(&self) -> SceneLightingSettings {
        SceneLightingSettings {
            ambient_color: Color::WHITE,
            ambient_brightness: self.ambient_brightness,
            ambient_affects_lightmapped_meshes: true,
            sun_illuminance: self.sun_illuminance,
            sun_rotation: self.sun_rotation(),
            shadows_enabled: self.shadows_enabled,
            shadow_depth_bias: self.shadow_depth_bias,
            shadow_normal_bias: self.shadow_normal_bias,
            shadow_cascades: SceneShadowCascadeSettings {
                num_cascades: self.cascade_count,
                minimum_distance: self.cascade_minimum_distance,
                first_cascade_far_bound: self.cascade_first_far_bound,
                maximum_distance: self.cascade_maximum_distance,
                overlap_proportion: self.cascade_overlap_proportion,
            },
        }
    }

    /// Clamps dependent shadow settings into a Bevy-safe cascade configuration.
    pub fn normalize_shadow_constraints(&mut self) {
        self.cascade_count = self.cascade_count.clamp(1, 4);
        self.cascade_minimum_distance = self.cascade_minimum_distance.clamp(0.0, 10.0);
        self.cascade_maximum_distance = self
            .cascade_maximum_distance
            .max(self.cascade_minimum_distance + 0.01);
        self.cascade_overlap_proportion = self.cascade_overlap_proportion.clamp(0.0, 0.95);

        if self.cascade_count == 1 {
            self.cascade_first_far_bound = self.cascade_maximum_distance;
        } else {
            self.cascade_first_far_bound = self.cascade_first_far_bound.clamp(
                self.cascade_minimum_distance + 0.01,
                self.cascade_maximum_distance,
            );
        }
    }
}

impl Default for GlobalLightControls {
    fn default() -> Self {
        Self {
            ambient_brightness: 700.0,
            sun_illuminance: 7_000.0,
            sun_elevation_degrees: -75.0,
            sun_azimuth_degrees: -10.0,
            shadows_enabled: true,
            shadow_depth_bias: DirectionalLight::DEFAULT_SHADOW_DEPTH_BIAS,
            shadow_normal_bias: DirectionalLight::DEFAULT_SHADOW_NORMAL_BIAS,
            cascade_count: 1,
            cascade_minimum_distance: 0.1,
            cascade_first_far_bound: 10.0,
            cascade_maximum_distance: 150.0,
            cascade_overlap_proportion: 0.2,
            shadow_map_size: 8192,
        }
    }
}
