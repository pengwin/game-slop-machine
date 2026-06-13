use bevy::asset::RenderAssetUsages;
use bevy::mesh::Indices;
use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use building_gen::config::BuildingConfig;
use building_gen::tile::TileGrid;
use building_gen::tile::TileType;

use super::config::BuildingGenConfig;

#[derive(Resource)]
pub struct CurrentBuilding {
    pub layout: building_gen::layout::BuildingLayout,
    pub entities: Vec<Entity>,
}

/// Spawns a building when B is pressed.
///
/// Uses the simplified mesh approach:
/// - One unit cube mesh for all wall-like tiles (scaled per tile)
/// - One floor quad mesh for floor tiles (scaled per tile)
/// - Colors are applied via materials
pub fn spawn_building_on_command(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<BuildingGenConfig>,
    input: Res<ButtonInput<KeyCode>>,
    existing: Option<ResMut<CurrentBuilding>>,
) {
    if !input.just_pressed(KeyCode::KeyB) {
        return;
    }

    // Remove existing building if any
    if let Some(mut existing) = existing {
        for entity in existing.entities.drain(..) {
            commands.entity(entity).despawn();
        }
    }

    let seed = 42;
    let layout = building_gen::generate_layout(&config.0, seed);

    // Create shared meshes (one for walls, one for floors)
    let wall_mesh = meshes.add(create_unit_cube_mesh());
    let floor_mesh = meshes.add(create_floor_quad_mesh());
    let wall_material = materials.add(Color::srgb(0.8, 0.8, 0.8));
    let floor_material = materials.add(Color::srgb(0.6, 0.6, 0.6));
    let door_material = materials.add(Color::srgb(0.4, 0.2, 0.0));
    let window_material = materials.add(Color::srgb(0.5, 0.7, 1.0));

    let mut entities = Vec::new();

    // Spawn tiles
    for y in 0..layout.tile_grid.height {
        for x in 0..layout.tile_grid.width {
            let tile = layout.tile_grid.get(x, y);
            if tile == TileType::Empty || tile == TileType::Doorway {
                continue;
            }

            let world_pos = layout.tile_grid.world_pos(x, y);
            spawn_tile_geometry(
                &mut commands,
                &mut entities,
                TileRenderAssets {
                    wall_mesh: wall_mesh.clone(),
                    floor_mesh: floor_mesh.clone(),
                    wall_material: wall_material.clone(),
                    floor_material: floor_material.clone(),
                    door_material: door_material.clone(),
                    window_material: window_material.clone(),
                },
                tile,
                &layout.tile_grid,
                x,
                y,
                Vec3::new(world_pos.x, 0.0, world_pos.y),
                &config.0,
            );
        }
    }

    commands.insert_resource(CurrentBuilding { layout, entities });
}

struct TileRenderAssets {
    wall_mesh: Handle<Mesh>,
    floor_mesh: Handle<Mesh>,
    wall_material: Handle<StandardMaterial>,
    floor_material: Handle<StandardMaterial>,
    door_material: Handle<StandardMaterial>,
    window_material: Handle<StandardMaterial>,
}

fn spawn_tile_geometry(
    commands: &mut Commands,
    entities: &mut Vec<Entity>,
    assets: TileRenderAssets,
    tile: TileType,
    grid: &TileGrid,
    x: usize,
    y: usize,
    position: Vec3,
    config: &BuildingConfig,
) {
    match tile {
        TileType::Floor => {
            let (sx, sy, sz) = building_gen::tile_scale(tile, config);
            spawn_cube(
                commands,
                entities,
                assets.floor_mesh,
                assets.floor_material,
                position,
                Vec3::new(sx, sy, sz),
            );
        }
        TileType::Window => {
            let (sx, _, sz) = oriented_tile_scale(TileType::Wall, grid, x, y, config);
            let sill = config.window_sill_height;
            let top_y = sill + config.window_height;
            let top_h = (config.wall_height - top_y).max(0.0);
            let glass = glass_scale(Vec3::new(sx, config.window_height, sz), config);

            if sill > 0.0 {
                spawn_cube(
                    commands,
                    entities,
                    assets.wall_mesh.clone(),
                    assets.wall_material.clone(),
                    position,
                    Vec3::new(sx, sill, sz),
                );
            }
            spawn_cube(
                commands,
                entities,
                assets.wall_mesh.clone(),
                assets.window_material,
                position + Vec3::Y * sill,
                glass,
            );
            if top_h > 0.0 {
                spawn_cube(
                    commands,
                    entities,
                    assets.wall_mesh,
                    assets.wall_material,
                    position + Vec3::Y * top_y,
                    Vec3::new(sx, top_h, sz),
                );
            }
        }
        TileType::Door => {
            let (sx, _, sz) = oriented_tile_scale(TileType::Wall, grid, x, y, config);
            let lintel_h = (config.wall_height - config.door_height).max(0.0);
            let door = door_scale(Vec3::new(sx, config.door_height, sz), config);

            spawn_cube(
                commands,
                entities,
                assets.wall_mesh.clone(),
                assets.door_material,
                position,
                door,
            );
            if lintel_h > 0.0 {
                spawn_cube(
                    commands,
                    entities,
                    assets.wall_mesh,
                    assets.wall_material,
                    position + Vec3::Y * config.door_height,
                    Vec3::new(sx, lintel_h, sz),
                );
            }
        }
        TileType::Wall | TileType::WallCorner => {
            let (sx, sy, sz) = oriented_tile_scale(tile, grid, x, y, config);
            spawn_cube(
                commands,
                entities,
                assets.wall_mesh,
                assets.wall_material,
                position,
                Vec3::new(sx, sy, sz),
            );
        }
        TileType::Empty | TileType::Doorway => {}
    }
}

fn spawn_cube(
    commands: &mut Commands,
    entities: &mut Vec<Entity>,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    translation: Vec3,
    scale: Vec3,
) {
    let entity = commands
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform {
                translation,
                scale,
                ..default()
            },
        ))
        .id();

    entities.push(entity);
}

fn glass_scale(scale: Vec3, config: &BuildingConfig) -> Vec3 {
    thin_opening_scale(scale, config.wall_thickness * 0.35)
}

fn door_scale(scale: Vec3, config: &BuildingConfig) -> Vec3 {
    thin_opening_scale(scale, config.wall_thickness * 0.65)
}

fn thin_opening_scale(mut scale: Vec3, thickness: f32) -> Vec3 {
    if scale.x < scale.z {
        scale.x = thickness;
    } else {
        scale.z = thickness;
    }

    scale
}

fn oriented_tile_scale(
    tile: TileType,
    grid: &TileGrid,
    x: usize,
    y: usize,
    config: &BuildingConfig,
) -> (f32, f32, f32) {
    let (sx, sy, sz) = building_gen::tile_scale(tile, config);

    if !matches!(
        tile,
        TileType::Wall | TileType::WallCorner | TileType::Door | TileType::Window
    ) {
        return (sx, sy, sz);
    }

    if wall_runs_along_z(grid, x, y) {
        (config.wall_thickness, sy, config.tile_size)
    } else {
        (sx, sy, sz)
    }
}

fn wall_runs_along_z(grid: &TileGrid, x: usize, y: usize) -> bool {
    let left = x > 0 && is_room_side(grid.get(x - 1, y));
    let right = x + 1 < grid.width && is_room_side(grid.get(x + 1, y));
    let down = y > 0 && is_room_side(grid.get(x, y - 1));
    let up = y + 1 < grid.height && is_room_side(grid.get(x, y + 1));

    (left || right) && !(down || up)
}

fn is_room_side(tile: TileType) -> bool {
    matches!(
        tile,
        TileType::Floor | TileType::Doorway | TileType::Door | TileType::Window
    )
}

/// Creates a unit cube mesh (1x1x1, origin at bottom center).
fn create_unit_cube_mesh() -> Mesh {
    let vertices = building_gen::unit_cube_vertices();
    let normals = building_gen::unit_cube_normals();
    let uvs = building_gen::unit_cube_uvs();
    let indices = building_gen::unit_cube_indices();

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}

/// Creates a floor quad mesh (1x1 at y=0).
fn create_floor_quad_mesh() -> Mesh {
    let vertices = building_gen::floor_quad_vertices();
    let normals = building_gen::floor_quad_normals();
    let uvs = building_gen::floor_quad_uvs();
    let indices = building_gen::floor_quad_indices();

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}
