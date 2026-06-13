use bevy::prelude::*;
use bevy::asset::RenderAssetUsages;
use bevy::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
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

    let mut entities = Vec::new();

    // Spawn tiles
    for y in 0..layout.tile_grid.height {
        for x in 0..layout.tile_grid.width {
            let tile = layout.tile_grid.get(x, y);
            if tile == TileType::Empty {
                continue;
            }

            let world_pos = layout.tile_grid.world_pos(x, y);
            let (scale_x, scale_y, scale_z) = building_gen::tile_scale(tile, &config.0);
            let (r, g, b) = building_gen::tile_color(tile);

            // Choose mesh based on tile type
            let mesh_handle = match tile {
                TileType::Floor => floor_mesh.clone(),
                _ => wall_mesh.clone(),
            };

            let material = materials.add(Color::srgb(r, g, b));

            let entity = commands
                .spawn((
                    Mesh3d(mesh_handle),
                    MeshMaterial3d(material),
                    Transform {
                        translation: Vec3::new(world_pos.x, 0.0, world_pos.y),
                        scale: Vec3::new(scale_x, scale_y, scale_z),
                        ..default()
                    },
                ))
                .id();

            entities.push(entity);
        }
    }

    commands.insert_resource(CurrentBuilding { layout, entities });
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
