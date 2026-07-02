use bevy::prelude::*;

use super::{
    resources::{MaterialGeneration, MaterialSettings},
    spec::MaterialInspectorSpec,
};
use crate::plugins::inspector::wall_material::apply_material_settings;

#[allow(clippy::needless_pass_by_value)]
pub(super) fn apply_material_settings_system<S: MaterialInspectorSpec>(
    generation: Option<Res<'_, MaterialGeneration<S>>>,
    settings: Res<'_, MaterialSettings>,
    mut materials: ResMut<'_, Assets<StandardMaterial>>,
) {
    let Some(generation) = generation.as_deref() else {
        return;
    };
    let Some(mut material) = materials.get_mut(&generation.material) else {
        return;
    };

    apply_material_settings(&mut material, &settings);
}
