use bevy::asset::RenderAssetUsages;
use bevy::mesh::Indices;
use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use building_gen::config::BuildingConfig;
use building_gen::geometry::{Rect, Vec2};
use building_gen::mesh::{generate_building_mesh, MeshData};
use building_gen::tile::{CardinalDir, TileGrid, TileType, WallShape, WallTile};
use building_gen::tile_converter::classify_wall_tiles;

#[derive(Resource)]
pub struct CurrentBuilding {
    pub entities: Vec<Entity>,
}

/// Spawns a building when B is pressed.
pub fn spawn_building_on_command(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    input: Res<ButtonInput<KeyCode>>,
    existing: Option<ResMut<CurrentBuilding>>,
) {
    if !input.just_pressed(KeyCode::KeyB) {
        return;
    }

    if let Some(mut existing) = existing {
        for entity in existing.entities.drain(..) {
            commands.entity(entity).despawn();
        }
    }

    let config = BuildingConfig {
        footprint: Rect::new(0.0, 0.0, 5.0, 5.0),
        tile_size: 0.5,
        wall_thickness: 0.5,
        wall_height: 3.0,
        ..Default::default()
    };
    let grid = build_corner_grid(&config);

    let roof = building_gen::roof::generate_roof(config.footprint, &config);
    let bmesh = generate_building_mesh(&grid, &config, &roof);

    let mut entities = Vec::new();

    if !bmesh.foundation_mesh.is_empty() {
        entities.push(
            commands
                .spawn((
                    Mesh3d(meshes.add(convert_mesh(&bmesh.foundation_mesh))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::srgb(0.42, 0.42, 0.38),
                        perceptual_roughness: 0.95,
                        ..default()
                    })),
                    Transform::default(),
                    Name::new("Foundation"),
                ))
                .id(),
        );
    }

    if !bmesh.wall_mesh.is_empty() {
        entities.push(
            commands
                .spawn((
                    Mesh3d(meshes.add(convert_mesh(&bmesh.wall_mesh))),
                    MeshMaterial3d(materials.add(Color::srgb(0.8, 0.8, 0.8))),
                    Transform::default(),
                    Name::new("Walls"),
                ))
                .id(),
        );
    }

    if !bmesh.wall_top_mesh.is_empty() {
        entities.push(
            commands
                .spawn((
                    Mesh3d(meshes.add(convert_mesh(&bmesh.wall_top_mesh))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::srgb(0.18, 0.18, 0.18),
                        cull_mode: None,
                        ..default()
                    })),
                    Transform::default(),
                    Name::new("Wall Top Faces"),
                ))
                .id(),
        );
    }

    if !bmesh.exterior_wall_mesh.is_empty() {
        entities.push(
            commands
                .spawn((
                    Mesh3d(meshes.add(convert_mesh(&bmesh.exterior_wall_mesh))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::srgb(0.92, 0.88, 0.68),
                        cull_mode: None,
                        ..default()
                    })),
                    Transform::default(),
                    Name::new("Exterior Wall Faces"),
                ))
                .id(),
        );
    }

    if !bmesh.exterior_corner_mesh.is_empty() {
        entities.push(
            commands
                .spawn((
                    Mesh3d(meshes.add(convert_mesh(&bmesh.exterior_corner_mesh))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::srgb(0.96, 0.9, 0.62),
                        cull_mode: None,
                        ..default()
                    })),
                    Transform::default(),
                    Name::new("Exterior Corner Faces"),
                ))
                .id(),
        );
    }

    if !bmesh.exterior_t_junction_mesh.is_empty() {
        entities.push(
            commands
                .spawn((
                    Mesh3d(meshes.add(convert_mesh(&bmesh.exterior_t_junction_mesh))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::srgb(0.86, 0.78, 0.48),
                        cull_mode: None,
                        ..default()
                    })),
                    Transform::default(),
                    Name::new("Exterior T-Junction Faces"),
                ))
                .id(),
        );
    }

    if !bmesh.floor_mesh.is_empty() {
        entities.push(
            commands
                .spawn((
                    Mesh3d(meshes.add(convert_mesh(&bmesh.floor_mesh))),
                    MeshMaterial3d(materials.add(Color::srgb(0.6, 0.6, 0.6))),
                    Transform::default(),
                    Name::new("Floor"),
                ))
                .id(),
        );
    }

    if !bmesh.roof_mesh.is_empty() {
        entities.push(
            commands
                .spawn((
                    Mesh3d(meshes.add(convert_mesh(&bmesh.roof_mesh))),
                    MeshMaterial3d(materials.add(Color::srgb(0.55, 0.35, 0.2))),
                    Transform::default(),
                    Name::new("Roof"),
                ))
                .id(),
        );
    }

    if !bmesh.door_mesh.is_empty() {
        entities.push(
            commands
                .spawn((
                    Mesh3d(meshes.add(convert_mesh(&bmesh.door_mesh))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::srgb(0.4, 0.2, 0.0),
                        cull_mode: None,
                        ..default()
                    })),
                    Transform::default(),
                    Name::new("Doors"),
                ))
                .id(),
        );
    }

    if !bmesh.window_mesh.is_empty() {
        entities.push(
            commands
                .spawn((
                    Mesh3d(meshes.add(convert_mesh(&bmesh.window_mesh))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::srgba(0.45, 0.7, 1.0, 0.45),
                        alpha_mode: AlphaMode::Blend,
                        cull_mode: None,
                        ..default()
                    })),
                    Transform::default(),
                    Name::new("Windows"),
                ))
                .id(),
        );
    }

    commands.insert_resource(CurrentBuilding { entities });
}

/// Builds a simple L-shaped corner: two perpendicular exterior walls with floor inside.
///
/// Layout (10x10 grid, y=0 at bottom):
/// ```text
///   9  W . . . . . . . . .
///   8  W . . . . . . . . .
///   7  W . . . . . . . . .
///   6  W . . . . . . . . .
///   5  W F F F F F F F F F
///   4  W F F F F F F F F F
///   3  W F F F F F F F F F
///   2  W F F F F F F F F F
///   1  W F F F F F F F F F
///   0  W F F F F F F F F F
///      0 1 2 3 4 5 6 7 8 9
/// ```
fn build_corner_grid(config: &BuildingConfig) -> TileGrid {
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

    grid
}

/// Converts a `MeshData` into a Bevy `Mesh`.
fn convert_mesh(data: &MeshData) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.vertices.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, data.uvs.clone());
    mesh.insert_indices(Indices::U32(data.indices.clone()));

    mesh
}
