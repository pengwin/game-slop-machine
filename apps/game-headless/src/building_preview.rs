use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use building_gen::config::BuildingConfig;
use building_gen::geometry::Vec2;
use building_gen::mesh::MeshData;
use building_gen::mesh::generate_building_mesh;
use building_gen::scene::{SceneObject, SceneObjectKind};
use building_gen::tile::{CardinalDir, TileGrid, TileType, WallOpening, WallShape, WallTile};
use building_gen::tile_converter::classify_wall_tiles;
use game_core::plugins::building::mesh_util::convert_mesh;
use game_core::plugins::building::procedural_texture::ProceduralTextures;
use game_core::plugins::building::render::{
    concrete_material, floor_tile_material, scene_part_material, spawn_building_mesh, wood_material,
};

pub fn spawn_building_preview(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    config: &BuildingConfig,
    fixture: &str,
) {
    let (grid, layout) = match fixture {
        "procedural" | "with-roof" | "corridor" | "picture-room" | "texture-plaster-wall" => {
            let l = building_gen::generate_layout(config);
            (l.tile_grid.clone(), Some(l))
        }
        "four-doors" => (
            build_perimeter_opening_grid(config, WallOpening::Door { render_panel: true }),
            None,
        ),
        "four-windows" => (
            build_perimeter_opening_grid(
                config,
                WallOpening::Window {
                    render_glass: config.exterior_window_render_glass,
                },
            ),
            None,
        ),
        _ => (build_two_room_grid(config), None),
    };

    // Debug: print opening tiles
    for y in 0..grid.height {
        for x in 0..grid.width {
            if let TileType::Wall(wall) = grid.get(x, y) {
                if let Some(opening) = wall.opening {
                    println!("  Opening at ({}, {}): {:?}", x, y, opening);
                }
            }
        }
    }

    // Debug: print tile grid as ASCII
    println!("Tile grid ({}x{}):", grid.width, grid.height);
    for y in (0..grid.height).rev() {
        print!("{:2} ", y);
        for x in 0..grid.width {
            print!("{}", grid.get(x, y).ascii_char());
        }
        println!();
    }
    print!("   ");
    for x in 0..grid.width {
        print!("{}", x % 10);
    }
    println!();

    let roof = building_gen::roof::generate_roof(config.footprint, config);
    let bmesh = generate_building_mesh(&grid, config, &roof);

    println!("Mesh stats:");
    println!(
        "  foundation: {} verts, {} tris",
        bmesh.foundation_mesh.vertices.len(),
        bmesh.foundation_mesh.indices.len() / 3
    );
    println!(
        "  wall:   {} verts, {} tris",
        bmesh.wall_mesh.vertices.len(),
        bmesh.wall_mesh.indices.len() / 3
    );
    println!(
        "  wall top: {} verts, {} tris",
        bmesh.wall_top_mesh.vertices.len(),
        bmesh.wall_top_mesh.indices.len() / 3
    );
    println!(
        "  exterior wall: {} verts, {} tris",
        bmesh.exterior_wall_mesh.vertices.len(),
        bmesh.exterior_wall_mesh.indices.len() / 3
    );
    println!(
        "  exterior corner: {} verts, {} tris",
        bmesh.exterior_corner_mesh.vertices.len(),
        bmesh.exterior_corner_mesh.indices.len() / 3
    );
    println!(
        "  exterior t-junction: {} verts, {} tris",
        bmesh.exterior_t_junction_mesh.vertices.len(),
        bmesh.exterior_t_junction_mesh.indices.len() / 3
    );
    println!(
        "  floor:  {} verts, {} tris",
        bmesh.floor_mesh.vertices.len(),
        bmesh.floor_mesh.indices.len() / 3
    );
    println!(
        "  floor grout: {} verts, {} tris",
        bmesh.floor_grout_mesh.vertices.len(),
        bmesh.floor_grout_mesh.indices.len() / 3
    );
    println!(
        "  roof:   {} verts, {} tris",
        bmesh.roof_mesh.vertices.len(),
        bmesh.roof_mesh.indices.len() / 3
    );
    println!(
        "  gable:  {} verts, {} tris",
        bmesh.gable_mesh.vertices.len(),
        bmesh.gable_mesh.indices.len() / 3
    );
    println!(
        "  door:   {} verts, {} tris",
        bmesh.door_mesh.vertices.len(),
        bmesh.door_mesh.indices.len() / 3
    );
    println!(
        "  opening trim: {} verts, {} tris",
        bmesh.opening_trim_mesh.vertices.len(),
        bmesh.opening_trim_mesh.indices.len() / 3
    );
    println!(
        "  window: {} verts, {} tris",
        bmesh.window_mesh.vertices.len(),
        bmesh.window_mesh.indices.len() / 3
    );

    if fixture == "procedural" {
        spawn_building_mesh(
            commands,
            meshes,
            materials,
            textures,
            images,
            config,
            &bmesh,
            Transform::default(),
            "Procedural Preview",
        );
        println!("{} test building generated", fixture);
        return;
    }

    if !bmesh.foundation_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.foundation_mesh))),
            MeshMaterial3d(materials.add(concrete_material(
                style_color(config.visual_style.foundation_color),
                textures,
                images,
                config.seed as u32,
            ))),
            Transform::default(),
            Name::new("Foundation"),
        ));
    }

    if !bmesh.wall_mesh.is_empty() {
        let wall_mesh = wall_preview_mesh(fixture, config.seed as u32, &bmesh.wall_mesh);
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&wall_mesh))),
            MeshMaterial3d(materials.add(wall_preview_material(
                fixture,
                config.visual_style.wall_color,
                textures,
                images,
                config.seed as u32,
            ))),
            NotShadowCaster,
            Transform::default(),
            Name::new("Walls"),
        ));
    }

    if !bmesh.wall_top_mesh.is_empty() {
        let wall_top_mesh = wall_preview_mesh(fixture, config.seed as u32, &bmesh.wall_top_mesh);
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&wall_top_mesh))),
            MeshMaterial3d(materials.add(wall_preview_material(
                fixture,
                config.visual_style.wall_top_color,
                textures,
                images,
                config.seed as u32,
            ))),
            NotShadowCaster,
            Transform::default(),
            Name::new("Wall Top Faces"),
        ));
    }

    if !config.render_roof {
        let bevel_mesh = building_wall_bevel_mesh(config);
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bevel_mesh))),
            MeshMaterial3d(materials.add(wall_preview_material(
                fixture,
                config.visual_style.wall_top_color,
                textures,
                images,
                config.seed as u32,
            ))),
            NotShadowCaster,
            Transform::default(),
            Name::new("Building Wall Bevels"),
        ));
    }

    if !bmesh.exterior_wall_mesh.is_empty() {
        let exterior_wall_mesh =
            wall_preview_mesh(fixture, config.seed as u32, &bmesh.exterior_wall_mesh);
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&exterior_wall_mesh))),
            MeshMaterial3d(materials.add(wall_preview_material(
                fixture,
                config.visual_style.exterior_wall_color,
                textures,
                images,
                config.seed as u32,
            ))),
            NotShadowCaster,
            Transform::default(),
            Name::new("Exterior Wall Faces"),
        ));
    }

    if !bmesh.exterior_corner_mesh.is_empty() {
        let exterior_corner_mesh =
            wall_preview_mesh(fixture, config.seed as u32, &bmesh.exterior_corner_mesh);
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&exterior_corner_mesh))),
            MeshMaterial3d(materials.add(wall_preview_material(
                fixture,
                config.visual_style.corner_color,
                textures,
                images,
                config.seed as u32,
            ))),
            NotShadowCaster,
            Transform::default(),
            Name::new("Exterior Corner Faces"),
        ));
    }

    if !bmesh.exterior_t_junction_mesh.is_empty() {
        let exterior_t_junction_mesh =
            wall_preview_mesh(fixture, config.seed as u32, &bmesh.exterior_t_junction_mesh);
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&exterior_t_junction_mesh))),
            MeshMaterial3d(materials.add(wall_preview_material(
                fixture,
                config.visual_style.t_junction_color,
                textures,
                images,
                config.seed as u32,
            ))),
            NotShadowCaster,
            Transform::default(),
            Name::new("Exterior T-Junction Faces"),
        ));
    }

    if !bmesh.floor_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.floor_mesh))),
            MeshMaterial3d(
                materials.add(floor_preview_material(fixture, config, textures, images)),
            ),
            Transform::default(),
            Name::new("Floor"),
        ));
    }

    if !bmesh.floor_grout_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.floor_grout_mesh))),
            MeshMaterial3d(materials.add({
                let mut material =
                    flat_preview_material(Color::WHITE, textures, images, "floor_grout", 1.0, 0.0);
                material.alpha_mode = AlphaMode::Blend;
                material.cull_mode = None;
                material
            })),
            Transform::default(),
            Name::new("Floor Grout"),
        ));
    }

    if !config.render_roof {
        let contact_shadow = building_contact_shadow_mesh(config);
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&contact_shadow))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba(0.08, 0.07, 0.05, 0.12),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                cull_mode: None,
                ..default()
            })),
            Transform::default(),
            Name::new("Building Contact Shadows"),
        ));
    }

    if config.render_roof && !bmesh.roof_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.roof_mesh))),
            MeshMaterial3d(materials.add(flat_preview_material(
                style_color(config.visual_style.roof_color),
                textures,
                images,
                "preview_roof",
                0.9,
                0.0,
            ))),
            Transform::default(),
            Name::new("Roof"),
        ));
    }

    if config.render_roof && !bmesh.gable_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.gable_mesh))),
            MeshMaterial3d(materials.add(flat_preview_material(
                style_color(config.visual_style.exterior_wall_color),
                textures,
                images,
                "preview_gable",
                0.9,
                0.0,
            ))),
            Transform::default(),
            Name::new("Gables"),
        ));
    }

    if !bmesh.door_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.door_mesh))),
            MeshMaterial3d(materials.add(wood_preview_material(
                fixture,
                config.visual_style.door_color,
                textures,
                images,
                config.seed as u32,
            ))),
            NotShadowCaster,
            Transform::default(),
            Name::new("Doors"),
        ));
    }

    if !bmesh.door_hardware_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.door_hardware_mesh))),
            MeshMaterial3d(materials.add({
                let mut material = flat_preview_material(
                    Color::WHITE,
                    textures,
                    images,
                    "door_hardware",
                    0.46,
                    0.25,
                );
                material.cull_mode = None;
                material
            })),
            NotShadowCaster,
            Transform::default(),
            Name::new("Door Hardware"),
        ));
    }

    if !bmesh.opening_trim_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.opening_trim_mesh))),
            MeshMaterial3d(materials.add(wood_preview_material(
                fixture,
                config.visual_style.trim_color,
                textures,
                images,
                config.seed as u32,
            ))),
            NotShadowCaster,
            Transform::default(),
            Name::new("Opening Trim"),
        ));
    }

    if !bmesh.window_mesh.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&bmesh.window_mesh))),
            MeshMaterial3d(materials.add({
                let mut material = flat_preview_material(
                    Color::srgba(0.58, 0.78, 0.95, 0.58),
                    textures,
                    images,
                    "window_glass",
                    0.24,
                    0.0,
                );
                material.alpha_mode = AlphaMode::Blend;
                material.cull_mode = None;
                material.reflectance = 0.20;
                material
            })),
            NotShadowCaster,
            Transform::default(),
            Name::new("Windows"),
        ));
    }

    println!("{} test building generated", fixture);

    if config.furniture {
        if let Some(l) = layout {
            let items = building_gen::generate_scene_objects(&l, config);
            println!("Spawning {} scene objects", items.len());
            spawn_furniture_contact_shadows(commands, meshes, materials, &items);
            for item in items {
                spawn_preview_scene_object(
                    commands, meshes, materials, textures, images, &item, 1.0, None,
                );
            }
            if matches!(fixture, "picture-room" | "texture-plaster-wall") {
                spawn_picture_room_staged_props(
                    commands, meshes, materials, textures, images, config,
                );
            }
        }
    }
}

fn style_color(rgb: [f32; 3]) -> Color {
    Color::srgb(rgb[0], rgb[1], rgb[2])
}

fn flat_preview_material(
    base_color: Color,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    label: &str,
    perceptual_roughness: f32,
    metallic: f32,
) -> StandardMaterial {
    let orm = textures.get_flat_orm(label, images, 1.0, perceptual_roughness, metallic);
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

fn floor_preview_material(
    _fixture: &str,
    config: &BuildingConfig,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
) -> StandardMaterial {
    floor_tile_material(
        style_color(config.visual_style.floor_color),
        textures,
        images,
        config.seed as u32,
    )
}

fn wood_preview_material(
    _fixture: &str,
    rgb: [f32; 3],
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    let mut material = wood_material(style_color(rgb), textures, images, seed);
    material.cull_mode = None;
    material
}

fn wall_preview_material(
    _fixture: &str,
    _rgb: [f32; 3],
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    seed: u32,
) -> StandardMaterial {
    let orm = textures.get_plaster_orm_now(seed, images);
    StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(textures.get_plaster_preview_albedo_now(seed, images)),
        normal_map_texture: Some(textures.get_plaster_normal_now(seed, images)),
        metallic_roughness_texture: Some(orm.clone()),
        occlusion_texture: Some(orm),
        perceptual_roughness: 1.0,
        ..default()
    }
}

fn wall_preview_mesh(_fixture: &str, seed: u32, mesh: &MeshData) -> MeshData {
    let mut mesh = game_core::plugins::building::mesh_util::subdivide_mesh_data(mesh, 0.5);
    let scale = 0.62;
    let mut new_colors = Vec::with_capacity(mesh.vertices.len());

    for i in 0..mesh.vertices.len() {
        let position = mesh.vertices[i];
        let normal = mesh.normals[i];

        mesh.uvs[i][0] *= scale;
        mesh.uvs[i][1] *= scale;

        let dirt = game_core::plugins::building::procedural_texture::global_dirt_color(
            seed, position, normal, 1.2,
        );
        let base = if mesh.colors.is_empty() {
            [1.0, 1.0, 1.0, 1.0]
        } else {
            mesh.colors[i]
        };
        new_colors.push([
            base[0] * dirt[0],
            base[1] * dirt[1],
            base[2] * dirt[2],
            base[3] * dirt[3],
        ]);
    }
    mesh.colors = new_colors;
    mesh
}

fn building_wall_bevel_mesh(config: &BuildingConfig) -> MeshData {
    let mut mesh = MeshData::default();
    let wall = config.tile_size;
    let bevel = 0.14;
    let top_y = config.foundation_height.max(0.0) + config.wall_height;
    let low_y = top_y - bevel * 0.45;

    let min_x = config.footprint.min.x + wall;
    let max_x = config.footprint.max.x - wall;
    let min_z = config.footprint.min.y + wall;
    let max_z = config.footprint.max.y - wall;

    append_sloped_quad(
        &mut mesh,
        [min_x, top_y, min_z],
        [max_x, top_y, min_z],
        [min_x, low_y, min_z + bevel],
        [max_x, low_y, min_z + bevel],
        [0.0, 0.42, 0.91],
    );
    append_sloped_quad(
        &mut mesh,
        [max_x, top_y, max_z],
        [min_x, top_y, max_z],
        [max_x, low_y, max_z - bevel],
        [min_x, low_y, max_z - bevel],
        [0.0, 0.42, -0.91],
    );
    append_sloped_quad(
        &mut mesh,
        [min_x, top_y, max_z],
        [min_x, top_y, min_z],
        [min_x + bevel, low_y, max_z],
        [min_x + bevel, low_y, min_z],
        [0.91, 0.42, 0.0],
    );
    append_sloped_quad(
        &mut mesh,
        [max_x, top_y, min_z],
        [max_x, top_y, max_z],
        [max_x - bevel, low_y, min_z],
        [max_x - bevel, low_y, max_z],
        [-0.91, 0.42, 0.0],
    );

    mesh
}

fn append_sloped_quad(
    mesh: &mut MeshData,
    top_a: [f32; 3],
    top_b: [f32; 3],
    low_a: [f32; 3],
    low_b: [f32; 3],
    normal: [f32; 3],
) {
    let base = mesh.vertices.len() as u32;
    mesh.vertices.extend([top_a, top_b, low_b, low_a]);
    mesh.normals.extend([normal; 4]);
    mesh.uvs
        .extend([[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
    mesh.indices
        .extend([base, base + 1, base + 2, base, base + 2, base + 3]);
}

fn spawn_furniture_contact_shadows(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    items: &[SceneObject],
) {
    let mut mesh = MeshData::default();
    for item in items {
        let half_w = item.width.max(0.35) * 0.20;
        let half_d = item.depth.max(0.35) * 0.20;
        append_floor_rect(
            &mut mesh,
            item.position.x - half_w,
            item.position.z - half_d,
            item.position.x + half_w,
            item.position.z + half_d,
            0.024,
        );
    }

    if mesh.is_empty() {
        return;
    }

    commands.spawn((
        Mesh3d(meshes.add(convert_mesh(&mesh))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(0.06, 0.05, 0.04, 0.07),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            cull_mode: None,
            ..default()
        })),
        Transform::default(),
        Name::new("Furniture Contact Shadows"),
    ));
}

fn spawn_picture_room_staged_props(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    config: &BuildingConfig,
) {
    let floor_y = config.foundation_height.max(0.0);
    let props = [
        (
            SceneObjectKind::Desk,
            Vec3::new(2.45, floor_y, 1.55),
            0.0,
            0.72,
            "Picture Room Bedside Table",
        ),
        (
            SceneObjectKind::Barrel,
            Vec3::new(6.55, floor_y, 2.55),
            std::f32::consts::FRAC_PI_2,
            1.18,
            "Picture Room Barrel",
        ),
        (
            SceneObjectKind::Crate,
            Vec3::new(6.12, floor_y, 2.38),
            0.24,
            0.82,
            "Picture Room Crate",
        ),
    ];
    let mut shadow_mesh = MeshData::default();

    for (item_type, position, rotation, scale, name) in props {
        let item = building_gen::furniture::single_item(item_type);
        let half_w = item.width.max(0.35) * scale * 0.22;
        let half_d = item.depth.max(0.35) * scale * 0.22;
        append_floor_rect(
            &mut shadow_mesh,
            position.x - half_w,
            position.z - half_d,
            position.x + half_w,
            position.z + half_d,
            floor_y + 0.027,
        );

        spawn_preview_scene_object(
            commands,
            meshes,
            materials,
            textures,
            images,
            &item,
            scale,
            Some((position, rotation, name)),
        );
    }

    if shadow_mesh.is_empty() {
        return;
    }

    commands.spawn((
        Mesh3d(meshes.add(convert_mesh(&shadow_mesh))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(0.06, 0.05, 0.04, 0.065),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            cull_mode: None,
            ..default()
        })),
        Transform::default(),
        Name::new("Picture Room Staged Prop Contact Shadows"),
    ));
}

#[allow(clippy::too_many_arguments)]
fn spawn_preview_scene_object(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    item: &SceneObject,
    scale: f32,
    override_transform: Option<(Vec3, f32, &str)>,
) {
    let (position, rotation, name) = override_transform.unwrap_or((
        Vec3::new(item.position.x, item.position.y, item.position.z),
        item.rotation,
        "",
    ));
    let transform = Transform::from_translation(position)
        .with_rotation(Quat::from_rotation_y(rotation))
        .with_scale(Vec3::splat(scale));

    if item.material_parts.is_empty() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&item.mesh))),
            MeshMaterial3d(materials.add(flat_preview_material(
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
            ))),
            transform,
            Name::new(if name.is_empty() {
                format!("{:?}", item.item_type)
            } else {
                name.to_string()
            }),
        ));
        return;
    }

    for (part_i, part) in item.material_parts.iter().enumerate() {
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&part.mesh))),
            MeshMaterial3d(materials.add(scene_part_material(
                part,
                textures,
                images,
                part_i as u32,
            ))),
            transform,
            Name::new(if name.is_empty() {
                format!("{:?} Part {}", item.item_type, part_i)
            } else {
                format!("{} Part {}", name, part_i)
            }),
        ));
    }
}

fn build_two_room_grid(config: &BuildingConfig) -> TileGrid {
    let w = config.tiles_x();
    let h = config.tiles_y();
    let origin = Vec2::new(config.footprint.min.x, config.footprint.min.y);
    let mut grid = TileGrid::new(w, h, config.tile_size, origin);
    let divider_y = h / 2;
    let door_x = w / 2;
    let wall = TileType::Wall(WallTile::exterior(WallShape::Straight(CardinalDir::Top)));

    for y in 0..h {
        for x in 0..w {
            if x == 0 || x == w - 1 || y == 0 || y == h - 1 || y == divider_y {
                grid.set(x, y, wall);
            } else {
                grid.set(x, y, TileType::Floor);
            }
        }
    }

    classify_wall_tiles(&mut grid);

    grid.set_wall_opening(
        door_x,
        divider_y,
        WallOpening::Door {
            render_panel: config.interior_door_render_panel,
        },
    );

    grid
}

fn building_contact_shadow_mesh(config: &BuildingConfig) -> MeshData {
    let mut mesh = MeshData::default();
    let inset = config.tile_size;
    let strip = 0.14;
    let y = 0.012;
    let min_x = config.footprint.min.x + inset;
    let max_x = config.footprint.max.x - inset;
    let min_z = config.footprint.min.y + inset;
    let max_z = config.footprint.max.y - inset;

    append_floor_rect(&mut mesh, min_x, min_z, max_x, min_z + strip, y);
    append_floor_rect(&mut mesh, min_x, max_z - strip, max_x, max_z, y);
    append_floor_rect(&mut mesh, min_x, min_z, min_x + strip, max_z, y);
    append_floor_rect(&mut mesh, max_x - strip, min_z, max_x, max_z, y);

    mesh
}

fn append_floor_rect(mesh: &mut MeshData, min_x: f32, min_z: f32, max_x: f32, max_z: f32, y: f32) {
    let base = mesh.vertices.len() as u32;
    mesh.vertices.extend([
        [min_x, y, min_z],
        [max_x, y, min_z],
        [max_x, y, max_z],
        [min_x, y, max_z],
    ]);
    mesh.normals.extend([[0.0, 1.0, 0.0]; 4]);
    mesh.uvs
        .extend([[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
    mesh.indices
        .extend([base, base + 1, base + 2, base, base + 2, base + 3]);
}

fn build_perimeter_opening_grid(config: &BuildingConfig, opening: WallOpening) -> TileGrid {
    let w = config.tiles_x();
    let h = config.tiles_y();
    let origin = Vec2::new(config.footprint.min.x, config.footprint.min.y);
    let mut grid = TileGrid::new(w, h, config.tile_size, origin);
    let wall = TileType::Wall(WallTile::exterior(WallShape::Straight(CardinalDir::Top)));

    for y in 0..h {
        for x in 0..w {
            if x == 0 || x == w - 1 || y == 0 || y == h - 1 {
                grid.set(x, y, wall);
            } else {
                grid.set(x, y, TileType::Floor);
            }
        }
    }

    classify_wall_tiles(&mut grid);

    let mid_x = w / 2;
    let mid_y = h / 2;
    grid.set_wall_opening(0, mid_y, opening);
    grid.set_wall_opening(w - 1, mid_y, opening);
    grid.set_wall_opening(mid_x, 0, opening);
    grid.set_wall_opening(mid_x, h - 1, opening);

    grid
}
