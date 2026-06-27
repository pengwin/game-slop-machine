use bevy::prelude::*;
use building_gen::config::{BuildingConfig, BuildingVisualStyle, RoomSpec};
use building_gen::geometry::{Rect, Vec2};
use building_gen::tile::{CardinalDir, TileGrid, TileType, WallShape, WallTile};
use building_gen::tile_converter::classify_wall_tiles;

use super::procedural_texture::ProceduralTextures;
use super::render::spawn_building_layout;

#[derive(Resource)]
pub struct CurrentBuilding {
    pub entities: Vec<Entity>,
}

fn despawn_current_building(commands: &mut Commands, existing: Option<ResMut<CurrentBuilding>>) {
    if let Some(mut existing) = existing {
        for entity in existing.entities.drain(..) {
            commands.entity(entity).despawn();
        }
    }
}

/// Spawns a building when B is pressed.
pub fn spawn_building_on_command(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut textures: ResMut<ProceduralTextures>,
    mut images: ResMut<Assets<Image>>,
    input: Res<ButtonInput<KeyCode>>,
    existing: Option<ResMut<CurrentBuilding>>,
) {
    if !input.just_pressed(KeyCode::KeyB) {
        return;
    }

    despawn_current_building(&mut commands, existing);

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
        &mut textures,
        &mut images,
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

/// Spawns the same picture-room building used by the texture-plaster-wall fixture when L is pressed.
pub fn spawn_texture_plaster_wall_on_command(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut textures: ResMut<ProceduralTextures>,
    mut images: ResMut<Assets<Image>>,
    input: Res<ButtonInput<KeyCode>>,
    existing: Option<ResMut<CurrentBuilding>>,
) {
    if !input.just_pressed(KeyCode::KeyL) {
        return;
    }

    despawn_current_building(&mut commands, existing);

    let config = texture_plaster_wall_config();
    let layout = building_gen::generate_layout(&config);
    let entities = spawn_building_layout(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut textures,
        &mut images,
        &config,
        &layout,
        Transform::default(),
        "Texture Plaster Wall",
    );

    commands.insert_resource(CurrentBuilding { entities });
}

fn texture_plaster_wall_config() -> BuildingConfig {
    BuildingConfig {
        seed: 17,
        has_stove: false,
        footprint: Rect::new(0.0, 0.0, 8.0, 6.0),
        entrance: Vec2::new(4.0, 0.0),
        room_specs: vec![RoomSpec::new("bedroom", 4)],
        render_roof: false,
        furniture: true,
        wall_height: 2.0,
        foundation_width: 0.22,
        foundation_height: 0.10,
        window_width: 0.75,
        window_height: 1.0,
        window_sill_height: 0.65,
        visual_style: BuildingVisualStyle {
            wall_color: [0.84, 0.75, 0.57],
            wall_top_color: [0.91, 0.81, 0.62],
            exterior_wall_color: [0.86, 0.76, 0.57],
            corner_color: [0.91, 0.81, 0.61],
            t_junction_color: [0.86, 0.76, 0.57],
            floor_color: [0.63, 0.60, 0.51],
            door_color: [0.34, 0.21, 0.13],
            trim_color: [0.48, 0.32, 0.19],
            foundation_color: [0.58, 0.58, 0.56],
            ..Default::default()
        },
        ..Default::default()
    }
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
