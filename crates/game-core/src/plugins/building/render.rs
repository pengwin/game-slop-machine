use bevy::prelude::*;
use building_gen::config::BuildingConfig;
use building_gen::layout::BuildingLayout;
use building_gen::mesh::{generate_building_mesh, BuildingMesh, MeshData};

use super::mesh_util::convert_mesh;

pub fn spawn_building_layout(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    config: &BuildingConfig,
    layout: &BuildingLayout,
    transform: Transform,
    name_prefix: &str,
) -> Vec<Entity> {
    let bmesh = generate_building_mesh(&layout.tile_grid, config, &layout.roof);
    spawn_building_mesh(
        commands,
        meshes,
        materials,
        config,
        &bmesh,
        transform,
        name_prefix,
    )
}

pub fn spawn_building_mesh(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    config: &BuildingConfig,
    bmesh: &BuildingMesh,
    transform: Transform,
    name_prefix: &str,
) -> Vec<Entity> {
    let mut entities = Vec::new();
    let name = |part: &str| format!("{} {}", name_prefix, part);

    spawn_part(commands, meshes, materials, &mut entities, &bmesh.foundation_mesh, foundation_material(config), transform, &name("Foundation"));
    spawn_part(commands, meshes, materials, &mut entities, &bmesh.wall_mesh, wall_material(config), transform, &name("Walls"));
    spawn_part(commands, meshes, materials, &mut entities, &bmesh.wall_top_mesh, wall_top_material(config), transform, &name("Wall Top Faces"));
    spawn_part(commands, meshes, materials, &mut entities, &bmesh.exterior_wall_mesh, exterior_wall_material(config), transform, &name("Exterior Wall Faces"));
    spawn_part(commands, meshes, materials, &mut entities, &bmesh.exterior_corner_mesh, exterior_corner_material(config), transform, &name("Exterior Corner Faces"));
    spawn_part(commands, meshes, materials, &mut entities, &bmesh.exterior_t_junction_mesh, exterior_t_junction_material(config), transform, &name("Exterior T-Junction Faces"));
    spawn_part(commands, meshes, materials, &mut entities, &bmesh.floor_mesh, floor_material(config), transform, &name("Floor"));

    if config.render_roof {
        spawn_part(commands, meshes, materials, &mut entities, &bmesh.roof_mesh, roof_material(config), transform, &name("Roof"));
        spawn_part(commands, meshes, materials, &mut entities, &bmesh.gable_mesh, exterior_wall_material(config), transform, &name("Gables"));
    }

    spawn_part(commands, meshes, materials, &mut entities, &bmesh.door_mesh, door_material(config), transform, &name("Doors"));
    spawn_part(commands, meshes, materials, &mut entities, &bmesh.opening_trim_mesh, opening_trim_material(config), transform, &name("Opening Trim"));
    spawn_part(commands, meshes, materials, &mut entities, &bmesh.window_mesh, window_material(), transform, &name("Windows"));

    entities
}

#[allow(clippy::too_many_arguments)]
fn spawn_part(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    entities: &mut Vec<Entity>,
    mesh_data: &MeshData,
    material: StandardMaterial,
    transform: Transform,
    name: &str,
) {
    if mesh_data.is_empty() {
        return;
    }

    entities.push(
        commands
            .spawn((
                Mesh3d(meshes.add(convert_mesh(mesh_data))),
                MeshMaterial3d(materials.add(material)),
                transform,
                Name::new(name.to_string()),
            ))
            .id(),
    );
}

fn color(rgb: [f32; 3]) -> Color {
    Color::srgb(rgb[0], rgb[1], rgb[2])
}

fn foundation_material(config: &BuildingConfig) -> StandardMaterial {
    StandardMaterial {
        base_color: color(config.visual_style.foundation_color),
        perceptual_roughness: 0.95,
        ..default()
    }
}

fn wall_material(config: &BuildingConfig) -> StandardMaterial {
    StandardMaterial {
        base_color: color(config.visual_style.wall_color),
        ..default()
    }
}

fn wall_top_material(config: &BuildingConfig) -> StandardMaterial {
    StandardMaterial {
        base_color: color(config.visual_style.wall_top_color),
        unlit: true,
        ..default()
    }
}

fn exterior_wall_material(config: &BuildingConfig) -> StandardMaterial {
    StandardMaterial {
        base_color: color(config.visual_style.exterior_wall_color),
        ..default()
    }
}

fn exterior_corner_material(config: &BuildingConfig) -> StandardMaterial {
    StandardMaterial {
        base_color: color(config.visual_style.corner_color),
        ..default()
    }
}

fn exterior_t_junction_material(config: &BuildingConfig) -> StandardMaterial {
    StandardMaterial {
        base_color: color(config.visual_style.t_junction_color),
        ..default()
    }
}

fn floor_material(config: &BuildingConfig) -> StandardMaterial {
    StandardMaterial {
        base_color: color(config.visual_style.floor_color),
        ..default()
    }
}

fn roof_material(config: &BuildingConfig) -> StandardMaterial {
    StandardMaterial {
        base_color: color(config.visual_style.roof_color),
        ..default()
    }
}

fn door_material(config: &BuildingConfig) -> StandardMaterial {
    StandardMaterial {
        base_color: color(config.visual_style.door_color),
        cull_mode: None,
        ..default()
    }
}

fn opening_trim_material(config: &BuildingConfig) -> StandardMaterial {
    StandardMaterial {
        base_color: color(config.visual_style.trim_color),
        cull_mode: None,
        ..default()
    }
}

fn window_material() -> StandardMaterial {
    StandardMaterial {
        base_color: Color::srgba(0.45, 0.7, 1.0, 0.45),
        alpha_mode: AlphaMode::Blend,
        cull_mode: None,
        ..default()
    }
}

/// Spawns furniture items as Bevy entities.
pub fn spawn_furniture(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    items: &[building_gen::furniture::FurnitureItem],
    transform: Transform,
    name_prefix: &str,
) -> Vec<Entity> {
    let mut entities = Vec::new();

    for (i, item) in items.iter().enumerate() {
        if item.mesh.is_empty() {
            continue;
        }

        let local_transform = Transform {
            translation: Vec3::new(item.position.x, item.position.y, item.position.z),
            rotation: Quat::from_rotation_y(item.rotation),
            ..default()
        };

        let world_transform = transform * local_transform;

        entities.push(
            commands
                .spawn((
                    Mesh3d(meshes.add(convert_mesh(&item.mesh))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: if item.mesh.colors.is_empty() {
                            Color::srgb(item.color[0], item.color[1], item.color[2])
                        } else {
                            Color::WHITE
                        },
                        perceptual_roughness: 0.85,
                        ..default()
                    })),
                    world_transform,
                    Name::new(format!("{} {:?} {}", name_prefix, item.item_type, i)),
                ))
                .id(),
        );
    }

    entities
}
