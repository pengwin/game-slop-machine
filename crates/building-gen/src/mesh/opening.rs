use super::math_util::append_quad;
use super::wall::{building_base_y, wall_bounds_for_tile};
use super::MeshData;
use crate::config::BuildingConfig;
use crate::tile::{TileGrid, TileType, WallAxis, WallOpening};

pub fn generate_opening_trim_mesh(grid: &TileGrid, config: &BuildingConfig) -> MeshData {
    let mut mesh = MeshData::default();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let TileType::Wall(wall) = grid.get(x, y) else {
                continue;
            };
            let Some(opening) = wall.opening else {
                continue;
            };

            let bounds = wall_bounds_for_tile(grid, x, y, wall, config);
            let [min_x, min_y, min_z] = bounds.0;
            let [max_x, _max_y, max_z] = bounds.1;

            let (opening_width, opening_bottom, opening_height) = match opening {
                WallOpening::Door { .. } | WallOpening::Doorway => (
                    config.door_width,
                    min_y,
                    config.door_height.min(config.wall_height),
                ),
                WallOpening::Window { .. } => (
                    config.window_width,
                    building_base_y(config) + config.window_sill_height,
                    config.window_height,
                ),
            };

            match wall.main_axis() {
                WallAxis::Z => {
                    let width = max_z - min_z;
                    let trim_width = opening_width.min(width);
                    let start = min_z + (width - trim_width) / 2.0;
                    let end = start + trim_width;
                    append_opening_trim_on_x_face(
                        &mut mesh,
                        max_x,
                        start,
                        end,
                        opening_bottom,
                        opening_bottom + opening_height,
                        1.0,
                        config,
                    );
                    append_opening_trim_on_x_face(
                        &mut mesh,
                        min_x,
                        start,
                        end,
                        opening_bottom,
                        opening_bottom + opening_height,
                        -1.0,
                        config,
                    );
                }
                WallAxis::X | WallAxis::Both => {
                    let width = max_x - min_x;
                    let trim_width = opening_width.min(width);
                    let start = min_x + (width - trim_width) / 2.0;
                    let end = start + trim_width;
                    append_opening_trim_on_z_face(
                        &mut mesh,
                        max_z,
                        start,
                        end,
                        opening_bottom,
                        opening_bottom + opening_height,
                        1.0,
                        config,
                    );
                    append_opening_trim_on_z_face(
                        &mut mesh,
                        min_z,
                        start,
                        end,
                        opening_bottom,
                        opening_bottom + opening_height,
                        -1.0,
                        config,
                    );
                }
            }
        }
    }

    mesh
}

fn append_opening_trim_on_x_face(
    mesh: &mut MeshData,
    x: f32,
    start_z: f32,
    end_z: f32,
    bottom_y: f32,
    top_y: f32,
    side: f32,
    config: &BuildingConfig,
) {
    let inset = config.opening_trim_thickness.max(0.0);
    let depth = config.opening_trim_depth.max(0.0) * side.signum();
    let x = x + depth;
    let normal = [side.signum(), 0.0, 0.0];

    append_trim_rects(start_z, end_z, bottom_y, top_y, inset, |a, b, c, d| {
        if side > 0.0 {
            append_quad(
                mesh,
                [x, d, a],
                [x, d, b],
                [x, c, a],
                [x, c, b],
                normal,
                [0.0, 0.0],
                [1.0, 1.0],
            );
        } else {
            append_quad(
                mesh,
                [x, d, b],
                [x, d, a],
                [x, c, b],
                [x, c, a],
                normal,
                [0.0, 0.0],
                [1.0, 1.0],
            );
        }
    });
}

fn append_opening_trim_on_z_face(
    mesh: &mut MeshData,
    z: f32,
    start_x: f32,
    end_x: f32,
    bottom_y: f32,
    top_y: f32,
    side: f32,
    config: &BuildingConfig,
) {
    let inset = config.opening_trim_thickness.max(0.0);
    let depth = config.opening_trim_depth.max(0.0) * side.signum();
    let z = z + depth;
    let normal = [0.0, 0.0, side.signum()];

    append_trim_rects(start_x, end_x, bottom_y, top_y, inset, |a, b, c, d| {
        if side > 0.0 {
            append_quad(
                mesh,
                [a, d, z],
                [b, d, z],
                [a, c, z],
                [b, c, z],
                normal,
                [0.0, 0.0],
                [1.0, 1.0],
            );
        } else {
            append_quad(
                mesh,
                [b, d, z],
                [a, d, z],
                [b, c, z],
                [a, c, z],
                normal,
                [0.0, 0.0],
                [1.0, 1.0],
            );
        }
    });
}

fn append_trim_rects<F>(start: f32, end: f32, bottom: f32, top: f32, trim: f32, mut append_rect: F)
where
    F: FnMut(f32, f32, f32, f32),
{
    let trim = trim.min((end - start) / 3.0).min((top - bottom) / 3.0);
    if trim <= f32::EPSILON {
        return;
    }

    append_rect(start - trim, start, bottom, top);
    append_rect(end, end + trim, bottom, top);
    append_rect(start - trim, end + trim, top, top + trim);
    append_rect(start - trim, end + trim, bottom - trim, bottom);
}

pub fn generate_door_mesh(grid: &TileGrid, config: &BuildingConfig) -> MeshData {
    let mut mesh = MeshData::default();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let TileType::Wall(wall) = grid.get(x, y) else {
                continue;
            };
            if !matches!(wall.opening, Some(WallOpening::Door { render_panel: true })) {
                continue;
            }

            let bounds = wall_bounds_for_tile(grid, x, y, wall, config);
            let [min_x, min_y, min_z] = bounds.0;
            let [max_x, _max_y, max_z] = bounds.1;
            let h = min_y + config.door_height;

            match wall.main_axis() {
                WallAxis::Z => {
                    let width = max_z - min_z;
                    let door_width = config.door_width.min(width);
                    let door_start = (width - door_width) / 2.0;
                    let ds = min_z + door_start;
                    let de = ds + door_width;
                    let cx = (min_x + max_x) / 2.0;
                    let t = max_x - min_x;

                    append_quad(
                        &mut mesh,
                        [cx + t / 2.0, h, ds],
                        [cx + t / 2.0, h, de],
                        [cx + t / 2.0, min_y, ds],
                        [cx + t / 2.0, min_y, de],
                        [1.0, 0.0, 0.0],
                        [0.0, 0.0],
                        [door_width, h],
                    );
                    append_quad(
                        &mut mesh,
                        [cx - t / 2.0, h, de],
                        [cx - t / 2.0, h, ds],
                        [cx - t / 2.0, min_y, de],
                        [cx - t / 2.0, min_y, ds],
                        [-1.0, 0.0, 0.0],
                        [0.0, 0.0],
                        [door_width, h],
                    );
                }
                WallAxis::X | WallAxis::Both => {
                    let width = max_x - min_x;
                    let door_width = config.door_width.min(width);
                    let door_start = (width - door_width) / 2.0;
                    let ds = min_x + door_start;
                    let de = ds + door_width;
                    let cz = (min_z + max_z) / 2.0;
                    let t = max_z - min_z;

                    append_quad(
                        &mut mesh,
                        [ds, h, cz + t / 2.0],
                        [de, h, cz + t / 2.0],
                        [ds, min_y, cz + t / 2.0],
                        [de, min_y, cz + t / 2.0],
                        [0.0, 0.0, 1.0],
                        [0.0, 0.0],
                        [door_width, h],
                    );
                    append_quad(
                        &mut mesh,
                        [de, h, cz - t / 2.0],
                        [ds, h, cz - t / 2.0],
                        [de, min_y, cz - t / 2.0],
                        [ds, min_y, cz - t / 2.0],
                        [0.0, 0.0, -1.0],
                        [0.0, 0.0],
                        [door_width, h],
                    );
                }
            }
        }
    }

    mesh
}

pub fn generate_window_mesh(grid: &TileGrid, config: &BuildingConfig) -> MeshData {
    let mut mesh = MeshData::default();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let TileType::Wall(wall) = grid.get(x, y) else {
                continue;
            };
            if !matches!(
                wall.opening,
                Some(WallOpening::Window { render_glass: true })
            ) {
                continue;
            }

            let bounds = wall_bounds_for_tile(grid, x, y, wall, config);
            let [min_x, _min_y, min_z] = bounds.0;
            let [max_x, _max_y, max_z] = bounds.1;
            let sill = building_base_y(config) + config.window_sill_height;
            let wh = config.window_height;

            match wall.main_axis() {
                WallAxis::Z => {
                    let width = max_z - min_z;
                    let window_width = config.window_width.min(width);
                    let win_start = (width - window_width) / 2.0;
                    let ws = min_z + win_start;
                    let we = ws + window_width;
                    let offset = 0.02;

                    append_quad(
                        &mut mesh,
                        [max_x - offset, sill + wh, ws],
                        [max_x - offset, sill + wh, we],
                        [max_x - offset, sill, ws],
                        [max_x - offset, sill, we],
                        [1.0, 0.0, 0.0],
                        [0.0, 0.0],
                        [window_width, wh],
                    );
                    append_quad(
                        &mut mesh,
                        [min_x + offset, sill + wh, we],
                        [min_x + offset, sill + wh, ws],
                        [min_x + offset, sill, we],
                        [min_x + offset, sill, ws],
                        [-1.0, 0.0, 0.0],
                        [0.0, 0.0],
                        [window_width, wh],
                    );
                }
                WallAxis::X | WallAxis::Both => {
                    let width = max_x - min_x;
                    let window_width = config.window_width.min(width);
                    let win_start = (width - window_width) / 2.0;
                    let ws = min_x + win_start;
                    let we = ws + window_width;
                    let offset = 0.02;

                    append_quad(
                        &mut mesh,
                        [ws, sill + wh, max_z - offset],
                        [we, sill + wh, max_z - offset],
                        [ws, sill, max_z - offset],
                        [we, sill, max_z - offset],
                        [0.0, 0.0, 1.0],
                        [0.0, 0.0],
                        [window_width, wh],
                    );
                    append_quad(
                        &mut mesh,
                        [we, sill + wh, min_z + offset],
                        [ws, sill + wh, min_z + offset],
                        [we, sill, min_z + offset],
                        [ws, sill, min_z + offset],
                        [0.0, 0.0, -1.0],
                        [0.0, 0.0],
                        [window_width, wh],
                    );
                }
            }
        }
    }

    mesh
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Rect, Vec2};
    use crate::tile::{CardinalDir, WallShape, WallTile};

    fn exterior_wall(shape: WallShape) -> TileType {
        TileType::Wall(WallTile::exterior(shape))
    }

    #[test]
    fn test_opening_trim_generated_for_window() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 1.0, 1.0),
            tile_size: 1.0,
            wall_thickness: 0.25,
            window_width: 0.5,
            ..Default::default()
        };
        let mut grid = TileGrid::new(1, 2, config.tile_size, Vec2::ZERO);
        grid.set(
            0,
            0,
            TileType::Wall(
                WallTile::exterior(WallShape::Straight(CardinalDir::Top))
                    .with_opening(WallOpening::Window { render_glass: true }),
            ),
        );

        let mesh = generate_opening_trim_mesh(&grid, &config);

        assert!(!mesh.is_empty());

        let disabled = BuildingConfig {
            opening_trim_thickness: 0.0,
            ..config
        };
        assert!(generate_opening_trim_mesh(&grid, &disabled).is_empty());
    }

    #[test]
    fn test_door_mesh_width_is_clamped_to_wall_segment() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 1.0, 3.0),
            tile_size: 0.5,
            door_width: 0.9,
            ..Default::default()
        };
        let mut grid = TileGrid::new(1, 3, config.tile_size, Vec2::ZERO);
        grid.set(
            0,
            1,
            exterior_wall(WallShape::Straight(CardinalDir::Bottom))
                .wall()
                .map_or(TileType::Empty, |wall| {
                    TileType::Wall(wall.with_opening(WallOpening::Door { render_panel: true }))
                }),
        );
        grid.set(0, 2, TileType::Floor);

        let mesh = generate_door_mesh(&grid, &config);
        let min_x = mesh
            .vertices
            .iter()
            .map(|v| v[0])
            .fold(f32::INFINITY, f32::min);
        let max_x = mesh
            .vertices
            .iter()
            .map(|v| v[0])
            .fold(f32::NEG_INFINITY, f32::max);

        assert!(min_x >= 0.0);
        assert!(max_x <= config.tile_size);
    }
}
