use bevy::prelude::*;
use building_gen::config::BuildingConfig;
use building_gen::geometry::{Rect, Vec2};
use building_gen::tile::{CardinalDir, TileGrid, TileType, WallShape, WallTile};
use building_gen::tile_converter::classify_wall_tiles;

use super::procedural_texture::ProceduralTextures;
use super::render::spawn_building_layout;

#[derive(Resource)]
pub struct CurrentBuilding {
    pub entities: Vec<Entity>,
}

/// Spawns a building when B is pressed.
pub fn spawn_building_on_command(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    textures: Res<ProceduralTextures>,
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

    let layout = building_gen::generate_layout(&config);
    let entities = spawn_building_layout(
        &mut commands,
        &mut meshes,
        &mut materials,
        &textures,
        &config,
        &building_gen::layout::BuildingLayout {
            tile_grid: grid,
            ..layout
        },
        Transform::default(),
        "Building",
    );

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
