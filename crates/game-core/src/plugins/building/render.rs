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
        door_hardware_material(textures, images),
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
        floor_grout_material(textures, images),
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
        window_material(textures, images),
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
    orm: Handle<Image>,
    perceptual_roughness: f32,
) -> StandardMaterial {
    StandardMaterial {
        base_color,
        base_color_texture: Some(albedo),
        normal_map_texture: Some(normal),
        metallic_roughness_texture: Some(orm.clone()),
        occlusion_texture: Some(orm),
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
    let params = super::procedural_texture::PlasterParams {
        seed,
        ..default()
    };
    plaster_material_with_params(base_color, textures, images, &params)
}

pub fn plaster_material_with_params(
    base_color: Color,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    params: &super::procedural_texture::PlasterParams,
) -> StandardMaterial {
    let orm = textures.get_plaster_orm(params, images);
    StandardMaterial {
        base_color,
        base_color_texture: Some(textures.get_plaster_albedo(params, images)),
        normal_map_texture: Some(textures.get_plaster_normal(params, images)),
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
    let params = super::procedural_texture::WoodParams {
        seed,
        ..default()
    };
    wood_material_with_params(_base_color, textures, images, &params)
}

pub fn wood_material_with_params(
    _base_color: Color,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    params: &super::procedural_texture::WoodParams,
) -> StandardMaterial {
    let orm = textures.get_wood_orm(params, images);
    StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(textures.get_wood_albedo(params, images)),
        normal_map_texture: Some(textures.get_wood_normal(params, images)),
        metallic_roughness_texture: Some(orm.clone()),
        occlusion_texture: Some(orm),
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
    let params = super::procedural_texture::BrickParams {
        seed,
        ..default()
    };
    brick_material_with_params(base_color, textures, images, &params)
}

pub fn brick_material_with_params(
    base_color: Color,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    params: &super::procedural_texture::BrickParams,
) -> StandardMaterial {
    textured_material(
        base_color,
        textures.get_brick_albedo(params, images),
        textures.get_brick_normal(params, images),
        textures.get_brick_orm(params, images),
        0.84,
    )
}

pub fn roof_tile_material(
    base_color: Color,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    let params = super::procedural_texture::RoofParams {
        seed,
        ..default()
    };
    roof_tile_material_with_params(base_color, textures, images, &params)
}

pub fn roof_tile_material_with_params(
    base_color: Color,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    params: &super::procedural_texture::RoofParams,
) -> StandardMaterial {
    textured_material(
        base_color,
        textures.get_roof_albedo(params, images),
        textures.get_roof_normal(params, images),
        textures.get_roof_orm(params, images),
        0.78,
    )
}

pub fn stone_material(
    base_color: Color,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    let params = super::procedural_texture::StoneParams {
        seed,
        ..default()
    };
    stone_material_with_params(base_color, textures, images, &params)
}

pub fn stone_material_with_params(
    base_color: Color,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    params: &super::procedural_texture::StoneParams,
) -> StandardMaterial {
    textured_material(
        base_color,
        textures.get_stone_albedo(params, images),
        textures.get_stone_normal(params, images),
        textures.get_stone_orm(params, images),
        0.95,
    )
}

pub fn road_material(
    base_color: Color,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    let params = super::procedural_texture::RoadParams {
        seed,
        ..default()
    };
    road_material_with_params(base_color, textures, images, &params)
}

pub fn road_material_with_params(
    base_color: Color,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    params: &super::procedural_texture::RoadParams,
) -> StandardMaterial {
    textured_material(
        base_color,
        textures.get_road_albedo(params, images),
        textures.get_road_normal(params, images),
        textures.get_road_orm(params, images),
        0.98,
    )
}

pub fn concrete_material(
    base_color: Color,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    let params = super::procedural_texture::ConcreteParams {
        seed,
        ..default()
    };
    concrete_material_with_params(base_color, textures, images, &params)
}

pub fn concrete_material_with_params(
    base_color: Color,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    params: &super::procedural_texture::ConcreteParams,
) -> StandardMaterial {
    let orm = textures.get_concrete_orm(params, images);
    StandardMaterial {
        base_color,
        base_color_texture: Some(textures.get_concrete_albedo(params, images)),
        normal_map_texture: Some(textures.get_concrete_normal(params, images)),
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
    let params = super::procedural_texture::FloorParams {
        seed,
        ..default()
    };
    floor_tile_material_with_params(_base_color, textures, images, &params)
}

pub fn floor_tile_material_with_params(
    _base_color: Color,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    params: &super::procedural_texture::FloorParams,
) -> StandardMaterial {
    let orm = textures.get_floor_orm(params, images);
    StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(textures.get_floor_albedo(params, images)),
        normal_map_texture: Some(textures.get_floor_normal(params, images)),
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

fn floor_grout_material(
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
) -> StandardMaterial {
    let orm = textures.get_flat_orm("floor_grout", images, 1.0, 1.0, 0.0);
    StandardMaterial {
        base_color: Color::WHITE,
        normal_map_texture: Some(textures.get_flat_normal(images)),
        metallic_roughness_texture: Some(orm.clone()),
        occlusion_texture: Some(orm),
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
    let mut material = wood_material(Color::WHITE, textures, images, seed);
    material.cull_mode = None;
    material.perceptual_roughness = 0.72;
    material
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

fn door_hardware_material(
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
) -> StandardMaterial {
    let orm = textures.get_flat_orm("door_hardware", images, 1.0, 0.46, 0.25);
    StandardMaterial {
        base_color: Color::WHITE,
        normal_map_texture: Some(textures.get_flat_normal(images)),
        metallic_roughness_texture: Some(orm.clone()),
        occlusion_texture: Some(orm),
        metallic: 0.25,
        perceptual_roughness: 0.46,
        cull_mode: None,
        ..default()
    }
}

fn window_material(
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
) -> StandardMaterial {
    let orm = textures.get_flat_orm("window_glass", images, 1.0, 0.10, 0.8);
    StandardMaterial {
        base_color: Color::srgba(0.58, 0.78, 0.95, 0.58),
        normal_map_texture: Some(textures.get_flat_normal(images)),
        metallic_roughness_texture: Some(orm.clone()),
        occlusion_texture: Some(orm),
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
                        MeshMaterial3d(materials.add(flat_mapped_material(
                            if item.mesh.colors.is_empty() {
                                Color::srgb(item.color[0], item.color[1], item.color[2])
                            } else {
                                Color::WHITE
                            },
                            textures,
                            images,
                            "scene_fallback",
                            0.85,
                            0.0,
                            0.5,
                        ))),
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
        SceneMaterialKind::Fabric => {
            let mut material =
                flat_mapped_material(color, textures, images, "fabric", 0.90, 0.0, 0.5);
            material.reflectance = 0.02;
            material
        }
        SceneMaterialKind::Book => {
            flat_mapped_material(color, textures, images, "book", 0.72, 0.0, 0.5)
        }
        SceneMaterialKind::Metal => {
            flat_mapped_material(color, textures, images, "metal", 0.30, 0.85, 0.5)
        }
        SceneMaterialKind::Stone => {
            let mut material =
                flat_mapped_material(color, textures, images, "scene_stone", 0.94, 0.0, 0.5);
            material.reflectance = 0.04;
            material
        }
        SceneMaterialKind::Colored => {
            flat_mapped_material(color, textures, images, "colored", 0.86, 0.0, 0.5)
        }
    }
}

fn flat_mapped_material(
    base_color: Color,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    label: &str,
    perceptual_roughness: f32,
    metallic: f32,
    occlusion: f32,
) -> StandardMaterial {
    let orm = textures.get_flat_orm(label, images, occlusion, perceptual_roughness, metallic);
    StandardMaterial {
        base_color,
        normal_map_texture: Some(textures.get_flat_normal(images)),
        metallic_roughness_texture: Some(orm.clone()),
        occlusion_texture: Some(orm),
        metallic,
        perceptual_roughness,
        ..default()
    }
}
