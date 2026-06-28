//! Despawn utilities for inspector UI panels.

use bevy::prelude::*;

/// Despawns all UI entities matching the given marker component.
pub fn despawn_ui<M: Component>(
    mut commands: Commands<'_, '_>,
    ui: Query<'_, '_, Entity, With<M>>,
) {
    for entity in &ui {
        commands.entity(entity).despawn_children().despawn();
    }
}
