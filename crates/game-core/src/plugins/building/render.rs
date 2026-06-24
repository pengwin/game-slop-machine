use bevy::prelude::*;
use building_gen::config::BuildingConfig;
use building_gen::layout::BuildingLayout;
use building_gen::mesh::{BuildingMesh, MeshData, generate_building_mesh};
use building_gen::scene::{SceneMaterialKind, SceneMeshPart};

use super::mesh_util::convert_mesh;
use super::procedural_texture::ProceduralTextures;

pub fn spawn_building_layout(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
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
        textures,
        images,
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
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    config: &BuildingConfig,
    bmesh: &BuildingMesh,
    transform: Transform,
    name_prefix: &str,
) -> Vec<Entity> {
    let mut entities = Vec::new();
    let name = |part: &str| format!("{} {}", name_prefix, part);

    spawn_part(
        commands,
        meshes,
        materials,
        &mut entities,
        &bmesh.foundation_mesh,
        foundation_material(config, textures, images, config.seed as u32),
        transform,
        &name("Foundation"),
        None,
        config.visual_style.dirt_intensity,
    );
    spawn_part(
        commands,
        meshes,
        materials,
        &mut entities,
        &bmesh.door_hardware_mesh,
        door_hardware_material(),
        transform,
        &name("Door Hardware"),
        None,
        config.visual_style.dirt_intensity,
    );
    spawn_part(
        commands,
        meshes,
        materials,
        &mut entities,
        &bmesh.wall_mesh,
        wall_material(config, textures, images, config.seed as u32),
        transform,
        &name("Wall Faces"),
        Some(config.seed as u32),
        config.visual_style.dirt_intensity,
    );
    spawn_part(
        commands,
        meshes,
        materials,
        &mut entities,
        &bmesh.wall_top_mesh,
        wall_top_material(config, textures, images, config.seed as u32),
        transform,
        &name("Wall Top Faces"),
        Some(config.seed as u32),
        config.visual_style.dirt_intensity,
    );
    spawn_part(
        commands,
        meshes,
        materials,
        &mut entities,
        &bmesh.exterior_wall_mesh,
        exterior_wall_material(config, textures, images, config.seed as u32),
        transform,
        &name("Exterior Wall Faces"),
        Some(config.seed as u32),
        config.visual_style.dirt_intensity,
    );
    spawn_part(
        commands,
        meshes,
        materials,
        &mut entities,
        &bmesh.exterior_corner_mesh,
        exterior_corner_material(config, textures, images, config.seed as u32),
        transform,
        &name("Exterior Corner Faces"),
        Some(config.seed as u32),
        config.visual_style.dirt_intensity,
    );
    spawn_part(
        commands,
        meshes,
        materials,
        &mut entities,
        &bmesh.exterior_t_junction_mesh,
        exterior_t_junction_material(config, textures, images, config.seed as u32),
        transform,
        &name("Exterior T-Junction Faces"),
        Some(config.seed as u32),
        config.visual_style.dirt_intensity,
    );
    spawn_part(
        commands,
        meshes,
        materials,
        &mut entities,
        &bmesh.floor_mesh,
        floor_material(config, textures, images, config.seed as u32),
        transform,
        &name("Floor"),
        None,
        config.visual_style.dirt_intensity,
    );
    spawn_part(
        commands,
        meshes,
        materials,
        &mut entities,
        &bmesh.floor_grout_mesh,
        floor_grout_material(),
        transform,
        &name("Floor Grout"),
        None,
        config.visual_style.dirt_intensity,
    );

    if config.render_roof {
        spawn_part(
            commands,
            meshes,
            materials,
            &mut entities,
            &bmesh.roof_mesh,
            roof_material(config, textures, images, config.seed as u32),
            transform,
            &name("Roof"),
            None,
            config.visual_style.dirt_intensity,
        );
        spawn_part(
            commands,
            meshes,
            materials,
            &mut entities,
            &bmesh.gable_mesh,
            exterior_wall_material(config, textures, images, config.seed as u32),
            transform,
            &name("Gables"),
            Some(config.seed as u32),
            config.visual_style.dirt_intensity,
        );
    }

    spawn_part(
        commands,
        meshes,
        materials,
        &mut entities,
        &bmesh.door_mesh,
        door_material(config, textures, images, config.seed as u32),
        transform,
        &name("Doors"),
        None,
        config.visual_style.dirt_intensity,
    );
    spawn_part(
        commands,
        meshes,
        materials,
        &mut entities,
        &bmesh.opening_trim_mesh,
        opening_trim_material(config, textures, images, config.seed as u32),
        transform,
        &name("Opening Trim"),
        None,
        config.visual_style.dirt_intensity,
    );
    spawn_part(
        commands,
        meshes,
        materials,
        &mut entities,
        &bmesh.window_mesh,
        window_material(),
        transform,
        &name("Windows"),
        None,
        config.visual_style.dirt_intensity,
    );

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
    dirt_seed: Option<u32>,
    dirt_intensity: f32,
) {
    if mesh_data.is_empty() {
        return;
    }

    let mesh = if let Some(seed) = dirt_seed {
        let subdivided = super::mesh_util::subdivide_mesh_data(mesh_data, 0.5); // Subdivide to 0.5m chunks for high vertex color density
        let mut m = convert_mesh(&subdivided);
        super::mesh_util::apply_dirt_vertex_colors(&mut m, seed, dirt_intensity);
        m
    } else {
        convert_mesh(mesh_data)
    };

    entities.push(
        commands
            .spawn((
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(materials.add(material)),
                transform,
                Name::new(name.to_string()),
            ))
            .id(),
    );
}

pub fn color(rgb: [f32; 3]) -> Color {
    Color::srgb(rgb[0], rgb[1], rgb[2])
}

pub fn textured_material(
    base_color: Color,
    albedo: Handle<Image>,
    normal: Handle<Image>,
    perceptual_roughness: f32,
) -> StandardMaterial {
    StandardMaterial {
        base_color,
        base_color_texture: Some(albedo),
        normal_map_texture: Some(normal),
        perceptual_roughness,
        ..default()
    }
}

pub fn plaster_material(
    base_color: Color,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    let orm = textures.get_plaster_orm(seed, images);
    StandardMaterial {
        base_color,
        base_color_texture: Some(textures.get_plaster_albedo(seed, images)),
        normal_map_texture: Some(textures.get_plaster_normal(seed, images)),
        metallic_roughness_texture: Some(orm.clone()),
        occlusion_texture: Some(orm),
        perceptual_roughness: 1.0,
        ..default()
    }
}

pub fn wood_material(
    _base_color: Color,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(textures.get_wood_albedo(seed, images)),
        normal_map_texture: Some(textures.get_wood_normal(seed, images)),
        perceptual_roughness: 0.85,
        ..default()
    }
}

pub fn brick_material(
    base_color: Color,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    textured_material(
        base_color,
        textures.get_brick_albedo(seed, images),
        textures.get_brick_normal(seed, images),
        0.84,
    )
}

pub fn roof_tile_material(
    base_color: Color,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    textured_material(
        base_color,
        textures.get_roof_albedo(seed, images),
        textures.get_roof_normal(seed, images),
        0.78,
    )
}

pub fn stone_material(
    base_color: Color,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    textured_material(
        base_color,
        textures.get_stone_albedo(seed, images),
        textures.get_stone_normal(seed, images),
        0.95,
    )
}

pub fn road_material(
    base_color: Color,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    textured_material(
        base_color,
        textures.get_road_albedo(seed, images),
        textures.get_road_normal(seed, images),
        0.98,
    )
}

pub fn concrete_material(
    base_color: Color,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    let orm = textures.get_concrete_orm(seed, images);
    StandardMaterial {
        base_color,
        base_color_texture: Some(textures.get_concrete_albedo(seed, images)),
        normal_map_texture: Some(textures.get_concrete_normal(seed, images)),
        metallic_roughness_texture: Some(orm.clone()),
        occlusion_texture: Some(orm),
        perceptual_roughness: 1.0,
        ..default()
    }
}

pub fn floor_tile_material(
    _base_color: Color,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    let orm = textures.get_floor_orm(seed, images);
    StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(textures.get_floor_albedo(seed, images)),
        normal_map_texture: Some(textures.get_floor_normal(seed, images)),
        metallic_roughness_texture: Some(orm.clone()),
        occlusion_texture: Some(orm),
        perceptual_roughness: 0.92,
        ..default()
    }
}

fn foundation_material(
    config: &BuildingConfig,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    concrete_material(
        color(config.visual_style.foundation_color),
        textures,
        images,
        seed,
    )
}

fn wall_material(
    config: &BuildingConfig,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    plaster_material(
        color(config.visual_style.wall_color),
        textures,
        images,
        seed,
    )
}

fn wall_top_material(
    config: &BuildingConfig,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    plaster_material(
        color(config.visual_style.wall_top_color),
        textures,
        images,
        seed,
    )
}

fn exterior_wall_material(
    config: &BuildingConfig,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    plaster_material(
        color(config.visual_style.exterior_wall_color),
        textures,
        images,
        seed,
    )
}

fn exterior_corner_material(
    config: &BuildingConfig,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    plaster_material(
        color(config.visual_style.corner_color),
        textures,
        images,
        seed,
    )
}

fn exterior_t_junction_material(
    config: &BuildingConfig,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    plaster_material(
        color(config.visual_style.t_junction_color),
        textures,
        images,
        seed,
    )
}

fn floor_material(
    config: &BuildingConfig,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    floor_tile_material(
        color(config.visual_style.floor_color),
        textures,
        images,
        seed,
    )
}

fn roof_material(
    config: &BuildingConfig,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    roof_tile_material(
        color(config.visual_style.roof_color),
        textures,
        images,
        seed,
    )
}

fn floor_grout_material() -> StandardMaterial {
    StandardMaterial {
        base_color: Color::WHITE,
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 1.0,
        cull_mode: None,
        ..default()
    }
}

fn door_material(
    _config: &BuildingConfig,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(textures.get_wood_albedo(seed, images)),
        cull_mode: None,
        perceptual_roughness: 0.72,
        ..default()
    }
}

fn opening_trim_material(
    config: &BuildingConfig,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    let mut material = wood_material(
        color(config.visual_style.trim_color),
        textures,
        images,
        seed,
    );
    material.cull_mode = None;
    material
}

fn door_hardware_material() -> StandardMaterial {
    StandardMaterial {
        base_color: Color::WHITE,
        metallic: 0.25,
        perceptual_roughness: 0.46,
        cull_mode: None,
        ..default()
    }
}

fn window_material() -> StandardMaterial {
    StandardMaterial {
        base_color: Color::srgba(0.58, 0.78, 0.95, 0.58),
        alpha_mode: AlphaMode::Blend,
        cull_mode: None,
        perceptual_roughness: 0.1,
        reflectance: 0.8,
        metallic: 0.8,
        ..default()
    }
}

/// Spawns generated scene objects as Bevy entities.
pub fn spawn_furniture(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    items: &[building_gen::scene::SceneObject],
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

        if item.material_parts.is_empty() {
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
            continue;
        }

        for (part_i, part) in item.material_parts.iter().enumerate() {
            entities.push(
                commands
                    .spawn((
                        Mesh3d(meshes.add(convert_mesh(&part.mesh))),
                        MeshMaterial3d(materials.add(scene_part_material(
                            part,
                            textures,
                            images,
                            i as u32 + part_i as u32,
                        ))),
                        world_transform,
                        Name::new(format!(
                            "{} {:?} {} Part {}",
                            name_prefix, item.item_type, i, part_i
                        )),
                    ))
                    .id(),
            );
        }
    }

    entities
}

pub fn scene_part_material(
    part: &SceneMeshPart,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    let color = Color::srgb(part.color[0], part.color[1], part.color[2]);
    match part.material {
        SceneMaterialKind::Wood => wood_material(color, textures, images, seed),
        SceneMaterialKind::Fabric => StandardMaterial {
            base_color: color,
            perceptual_roughness: 0.90,
            reflectance: 0.02,
            ..default()
        },
        SceneMaterialKind::Book => StandardMaterial {
            base_color: color,
            perceptual_roughness: 0.72,
            ..default()
        },
        SceneMaterialKind::Metal => StandardMaterial {
            base_color: color,
            metallic: 0.85,
            perceptual_roughness: 0.30,
            ..default()
        },
        SceneMaterialKind::Stone => StandardMaterial {
            base_color: color,
            perceptual_roughness: 0.94,
            reflectance: 0.04,
            ..default()
        },
        SceneMaterialKind::Colored => StandardMaterial {
            base_color: color,
            perceptual_roughness: 0.86,
            ..default()
        },
    }
}
