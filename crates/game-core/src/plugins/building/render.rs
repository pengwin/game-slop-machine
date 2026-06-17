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

    spawn_part(
        commands,
        meshes,
        materials,
        &mut entities,
        &bmesh.foundation_mesh,
        foundation_material(config),
        transform,
        name_prefix,
        "Foundation",
    );
    spawn_part(
        commands,
        meshes,
        materials,
        &mut entities,
        &bmesh.wall_mesh,
        wall_material(config),
        transform,
        name_prefix,
        "Walls",
    );
    spawn_part(
        commands,
        meshes,
        materials,
        &mut entities,
        &bmesh.wall_top_mesh,
        wall_top_material(config),
        transform,
        name_prefix,
        "Wall Top Faces",
    );
    spawn_part(
        commands,
        meshes,
        materials,
        &mut entities,
        &bmesh.exterior_wall_mesh,
        exterior_wall_material(config),
        transform,
        name_prefix,
        "Exterior Wall Faces",
    );
    spawn_part(
        commands,
        meshes,
        materials,
        &mut entities,
        &bmesh.exterior_corner_mesh,
        exterior_corner_material(config),
        transform,
        name_prefix,
        "Exterior Corner Faces",
    );
    spawn_part(
        commands,
        meshes,
        materials,
        &mut entities,
        &bmesh.exterior_t_junction_mesh,
        exterior_t_junction_material(config),
        transform,
        name_prefix,
        "Exterior T-Junction Faces",
    );
    spawn_part(
        commands,
        meshes,
        materials,
        &mut entities,
        &bmesh.floor_mesh,
        floor_material(config),
        transform,
        name_prefix,
        "Floor",
    );

    if config.render_roof {
        spawn_part(
            commands,
            meshes,
            materials,
            &mut entities,
            &bmesh.roof_mesh,
            roof_material(config),
            transform,
            name_prefix,
            "Roof",
        );
        spawn_part(
            commands,
            meshes,
            materials,
            &mut entities,
            &bmesh.gable_mesh,
            exterior_wall_material(config),
            transform,
            name_prefix,
            "Gables",
        );
    }

    spawn_part(
        commands,
        meshes,
        materials,
        &mut entities,
        &bmesh.door_mesh,
        door_material(config),
        transform,
        name_prefix,
        "Doors",
    );
    spawn_part(
        commands,
        meshes,
        materials,
        &mut entities,
        &bmesh.opening_trim_mesh,
        opening_trim_material(config),
        transform,
        name_prefix,
        "Opening Trim",
    );
    spawn_part(
        commands,
        meshes,
        materials,
        &mut entities,
        &bmesh.window_mesh,
        window_material(),
        transform,
        name_prefix,
        "Windows",
    );

    entities
}

fn spawn_part(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    entities: &mut Vec<Entity>,
    mesh_data: &MeshData,
    material: StandardMaterial,
    transform: Transform,
    name_prefix: &str,
    part_name: &str,
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
                Name::new(format!("{} {}", name_prefix, part_name)),
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
