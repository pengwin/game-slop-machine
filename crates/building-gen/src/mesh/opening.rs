use super::MeshData;
use super::colored_shapes::append_material_box;
use super::math_util::{Quad, append_colored_quad, append_quad};
use super::wall::{building_base_y, wall_bounds_for_tile};
use crate::config::BuildingConfig;
use crate::mesh::SurfaceMaterial;
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

            let is_window = matches!(opening, WallOpening::Window { .. });
            let trim_depth = match opening {
                WallOpening::Door {
                    render_panel: false,
                }
                | WallOpening::Doorway => 0.0,
                WallOpening::Door { render_panel: true } | WallOpening::Window { .. } => {
                    config.opening_trim_depth
                }
            };
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
                        is_window,
                        trim_depth,
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
                        is_window,
                        trim_depth,
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
                        is_window,
                        trim_depth,
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
                        is_window,
                        trim_depth,
                        config,
                    );
                }
            }
        }
    }

    mesh
}

#[allow(clippy::too_many_arguments)]
fn append_opening_trim_on_x_face(
    mesh: &mut MeshData,
    x: f32,
    start_z: f32,
    end_z: f32,
    bottom_y: f32,
    top_y: f32,
    side: f32,
    is_window: bool,
    trim_depth: f32,
    config: &BuildingConfig,
) {
    let inset = config.opening_trim_thickness.max(0.0);
    if inset <= f32::EPSILON {
        return;
    }
    let wall_x = x;
    let front_x = wall_x + trim_depth.max(0.0) * side.signum();

    append_trim_rects(start_z, end_z, bottom_y, top_y, inset, |a, b, c, d| {
        append_trim_box_x(mesh, wall_x, front_x, a, b, c, d, side);
    });
    if is_window {
        append_window_muntins_on_x_face(
            mesh, wall_x, front_x, start_z, end_z, bottom_y, top_y, side, inset,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn append_opening_trim_on_z_face(
    mesh: &mut MeshData,
    z: f32,
    start_x: f32,
    end_x: f32,
    bottom_y: f32,
    top_y: f32,
    side: f32,
    is_window: bool,
    trim_depth: f32,
    config: &BuildingConfig,
) {
    let inset = config.opening_trim_thickness.max(0.0);
    if inset <= f32::EPSILON {
        return;
    }
    let wall_z = z;
    let front_z = wall_z + trim_depth.max(0.0) * side.signum();

    append_trim_rects(start_x, end_x, bottom_y, top_y, inset, |a, b, c, d| {
        append_trim_box_z(mesh, wall_z, front_z, a, b, c, d, side);
    });
    if is_window {
        append_window_muntins_on_z_face(
            mesh, wall_z, front_z, start_x, end_x, bottom_y, top_y, side, inset,
        );
    }
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

#[allow(clippy::too_many_arguments)]
fn append_window_muntins_on_x_face(
    mesh: &mut MeshData,
    wall_x: f32,
    front_x: f32,
    start_z: f32,
    end_z: f32,
    bottom_y: f32,
    top_y: f32,
    side: f32,
    trim: f32,
) {
    let bar = (trim * 0.55).max(0.025);
    let mid_z = (start_z + end_z) * 0.5;
    let mid_y = (bottom_y + top_y) * 0.5;
    append_trim_box_x(
        mesh,
        wall_x,
        front_x,
        mid_z - bar * 0.5,
        mid_z + bar * 0.5,
        bottom_y,
        top_y,
        side,
    );
    append_trim_box_x(
        mesh,
        wall_x,
        front_x,
        start_z,
        end_z,
        mid_y - bar * 0.5,
        mid_y + bar * 0.5,
        side,
    );
}

#[allow(clippy::too_many_arguments)]
fn append_window_muntins_on_z_face(
    mesh: &mut MeshData,
    wall_z: f32,
    front_z: f32,
    start_x: f32,
    end_x: f32,
    bottom_y: f32,
    top_y: f32,
    side: f32,
    trim: f32,
) {
    let bar = (trim * 0.55).max(0.025);
    let mid_x = (start_x + end_x) * 0.5;
    let mid_y = (bottom_y + top_y) * 0.5;
    append_trim_box_z(
        mesh,
        wall_z,
        front_z,
        mid_x - bar * 0.5,
        mid_x + bar * 0.5,
        bottom_y,
        top_y,
        side,
    );
    append_trim_box_z(
        mesh,
        wall_z,
        front_z,
        start_x,
        end_x,
        mid_y - bar * 0.5,
        mid_y + bar * 0.5,
        side,
    );
}

#[allow(clippy::too_many_arguments)]
fn append_trim_box_x(
    mesh: &mut MeshData,
    wall_x: f32,
    front_x: f32,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    side: f32,
) {
    let depth = (front_x - wall_x).abs();
    if depth <= f32::EPSILON {
        append_rect_x(mesh, wall_x, a, b, c, d, side, [side.signum(), 0.0, 0.0]);
        mesh.colors.extend([[1.0, 1.0, 1.0, 1.0]; 4]);
        return;
    }

    append_material_box(
        mesh,
        [(wall_x + front_x) * 0.5, (c + d) * 0.5, (a + b) * 0.5],
        [depth, (d - c).abs(), (b - a).abs()],
        [1.0, 1.0, 1.0, 1.0],
        SurfaceMaterial::Wood,
    );
}

#[allow(clippy::too_many_arguments)]
fn append_trim_box_z(
    mesh: &mut MeshData,
    wall_z: f32,
    front_z: f32,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    side: f32,
) {
    let depth = (front_z - wall_z).abs();
    if depth <= f32::EPSILON {
        append_rect_z(mesh, wall_z, a, b, c, d, side, [0.0, 0.0, side.signum()]);
        mesh.colors.extend([[1.0, 1.0, 1.0, 1.0]; 4]);
        return;
    }

    append_material_box(
        mesh,
        [(a + b) * 0.5, (c + d) * 0.5, (wall_z + front_z) * 0.5],
        [(b - a).abs(), (d - c).abs(), depth],
        [1.0, 1.0, 1.0, 1.0],
        SurfaceMaterial::Wood,
    );
}

#[allow(clippy::too_many_arguments)]
fn append_rect_x(
    mesh: &mut MeshData,
    x: f32,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    side: f32,
    normal: [f32; 3],
) {
    if side > 0.0 {
        append_quad(
            mesh,
            Quad {
                tl: [x, d, a],
                tr: [x, d, b],
                bl: [x, c, a],
                br: [x, c, b],
                normal,
                uv_min: [0.0, 0.0],
                uv_max: [1.0, 1.0],
            },
        );
    } else {
        append_quad(
            mesh,
            Quad {
                tl: [x, d, b],
                tr: [x, d, a],
                bl: [x, c, b],
                br: [x, c, a],
                normal,
                uv_min: [0.0, 0.0],
                uv_max: [1.0, 1.0],
            },
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn append_rect_z(
    mesh: &mut MeshData,
    z: f32,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    side: f32,
    normal: [f32; 3],
) {
    if side > 0.0 {
        append_quad(
            mesh,
            Quad {
                tl: [a, d, z],
                tr: [b, d, z],
                bl: [a, c, z],
                br: [b, c, z],
                normal,
                uv_min: [0.0, 0.0],
                uv_max: [1.0, 1.0],
            },
        );
    } else {
        append_quad(
            mesh,
            Quad {
                tl: [b, d, z],
                tr: [a, d, z],
                bl: [b, c, z],
                br: [a, c, z],
                normal,
                uv_min: [0.0, 0.0],
                uv_max: [1.0, 1.0],
            },
        );
    }
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

                    append_door_panel_x(&mut mesh, cx + t / 2.0, ds, de, min_y, h, 1.0);
                    append_door_panel_x(&mut mesh, cx - t / 2.0, ds, de, min_y, h, -1.0);
                }
                WallAxis::X | WallAxis::Both => {
                    let width = max_x - min_x;
                    let door_width = config.door_width.min(width);
                    let door_start = (width - door_width) / 2.0;
                    let ds = min_x + door_start;
                    let de = ds + door_width;
                    let cz = (min_z + max_z) / 2.0;
                    let t = max_z - min_z;

                    append_door_panel_z(&mut mesh, cz + t / 2.0, ds, de, min_y, h, 1.0);
                    append_door_panel_z(&mut mesh, cz - t / 2.0, ds, de, min_y, h, -1.0);
                }
            }
        }
    }

    mesh
}

pub fn generate_door_hardware_mesh(grid: &TileGrid, config: &BuildingConfig) -> MeshData {
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
            let handle_y = min_y + (config.door_height * 0.48).min(config.wall_height * 0.65);
            let handle_color = [0.72, 0.56, 0.28, 1.0];

            match wall.main_axis() {
                WallAxis::Z => {
                    let width = max_z - min_z;
                    let door_width = config.door_width.min(width);
                    let ds = min_z + (width - door_width) * 0.5;
                    let de = ds + door_width;
                    append_handle_x(
                        &mut mesh,
                        max_x + 0.018,
                        de - door_width * 0.18,
                        handle_y,
                        1.0,
                        handle_color,
                    );
                    append_handle_x(
                        &mut mesh,
                        min_x - 0.018,
                        ds + door_width * 0.18,
                        handle_y,
                        -1.0,
                        handle_color,
                    );
                }
                WallAxis::X | WallAxis::Both => {
                    let width = max_x - min_x;
                    let door_width = config.door_width.min(width);
                    let ds = min_x + (width - door_width) * 0.5;
                    let de = ds + door_width;
                    append_handle_z(
                        &mut mesh,
                        max_z + 0.018,
                        de - door_width * 0.18,
                        handle_y,
                        1.0,
                        handle_color,
                    );
                    append_handle_z(
                        &mut mesh,
                        min_z - 0.018,
                        ds + door_width * 0.18,
                        handle_y,
                        -1.0,
                        handle_color,
                    );
                }
            }
        }
    }

    mesh
}

#[allow(clippy::too_many_arguments)]
fn append_door_panel_x(
    mesh: &mut MeshData,
    x: f32,
    ds: f32,
    de: f32,
    bottom: f32,
    top: f32,
    side: f32,
) {
    let normal = [side.signum(), 0.0, 0.0];
    append_rect_x(mesh, x, ds, de, bottom, top, side, normal);

    let offset = 0.012 * side.signum();
    let width = de - ds;
    let panel = (width * 0.045).max(0.025);
    let rail = (top - bottom) * 0.055;
    for z in [ds + width * 0.34, ds + width * 0.66] {
        append_rect_x(
            mesh,
            x + offset,
            z - panel * 0.5,
            z + panel * 0.5,
            bottom + rail,
            top - rail,
            side,
            normal,
        );
    }
    for y in [
        bottom + (top - bottom) * 0.30,
        bottom + (top - bottom) * 0.70,
    ] {
        append_rect_x(
            mesh,
            x + offset,
            ds + panel,
            de - panel,
            y - rail * 0.5,
            y + rail * 0.5,
            side,
            normal,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn append_door_panel_z(
    mesh: &mut MeshData,
    z: f32,
    ds: f32,
    de: f32,
    bottom: f32,
    top: f32,
    side: f32,
) {
    let normal = [0.0, 0.0, side.signum()];
    append_rect_z(mesh, z, ds, de, bottom, top, side, normal);

    let offset = 0.012 * side.signum();
    let width = de - ds;
    let panel = (width * 0.045).max(0.025);
    let rail = (top - bottom) * 0.055;
    for x in [ds + width * 0.34, ds + width * 0.66] {
        append_rect_z(
            mesh,
            z + offset,
            x - panel * 0.5,
            x + panel * 0.5,
            bottom + rail,
            top - rail,
            side,
            normal,
        );
    }
    for y in [
        bottom + (top - bottom) * 0.30,
        bottom + (top - bottom) * 0.70,
    ] {
        append_rect_z(
            mesh,
            z + offset,
            ds + panel,
            de - panel,
            y - rail * 0.5,
            y + rail * 0.5,
            side,
            normal,
        );
    }
}

fn append_handle_x(mesh: &mut MeshData, x: f32, z: f32, y: f32, side: f32, color: [f32; 4]) {
    let w = 0.10;
    let h = 0.055;
    let normal = [side.signum(), 0.0, 0.0];
    if side > 0.0 {
        append_colored_quad(
            mesh,
            Quad {
                tl: [x, y + h, z - w * 0.5],
                tr: [x, y + h, z + w * 0.5],
                bl: [x, y - h, z - w * 0.5],
                br: [x, y - h, z + w * 0.5],
                normal,
                uv_min: [0.0, 0.0],
                uv_max: [1.0, 1.0],
            },
            color,
        );
    } else {
        append_colored_quad(
            mesh,
            Quad {
                tl: [x, y + h, z + w * 0.5],
                tr: [x, y + h, z - w * 0.5],
                bl: [x, y - h, z + w * 0.5],
                br: [x, y - h, z - w * 0.5],
                normal,
                uv_min: [0.0, 0.0],
                uv_max: [1.0, 1.0],
            },
            color,
        );
    }
}

fn append_handle_z(mesh: &mut MeshData, z: f32, x: f32, y: f32, side: f32, color: [f32; 4]) {
    let w = 0.10;
    let h = 0.055;
    let normal = [0.0, 0.0, side.signum()];
    if side > 0.0 {
        append_colored_quad(
            mesh,
            Quad {
                tl: [x - w * 0.5, y + h, z],
                tr: [x + w * 0.5, y + h, z],
                bl: [x - w * 0.5, y - h, z],
                br: [x + w * 0.5, y - h, z],
                normal,
                uv_min: [0.0, 0.0],
                uv_max: [1.0, 1.0],
            },
            color,
        );
    } else {
        append_colored_quad(
            mesh,
            Quad {
                tl: [x + w * 0.5, y + h, z],
                tr: [x - w * 0.5, y + h, z],
                bl: [x + w * 0.5, y - h, z],
                br: [x - w * 0.5, y - h, z],
                normal,
                uv_min: [0.0, 0.0],
                uv_max: [1.0, 1.0],
            },
            color,
        );
    }
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
                        Quad {
                            tl: [max_x - offset, sill + wh, ws],
                            tr: [max_x - offset, sill + wh, we],
                            bl: [max_x - offset, sill, ws],
                            br: [max_x - offset, sill, we],
                            normal: [1.0, 0.0, 0.0],
                            uv_min: [0.0, 0.0],
                            uv_max: [window_width, wh],
                        },
                    );
                    append_quad(
                        &mut mesh,
                        Quad {
                            tl: [min_x + offset, sill + wh, we],
                            tr: [min_x + offset, sill + wh, ws],
                            bl: [min_x + offset, sill, we],
                            br: [min_x + offset, sill, ws],
                            normal: [-1.0, 0.0, 0.0],
                            uv_min: [0.0, 0.0],
                            uv_max: [window_width, wh],
                        },
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
                        Quad {
                            tl: [ws, sill + wh, max_z - offset],
                            tr: [we, sill + wh, max_z - offset],
                            bl: [ws, sill, max_z - offset],
                            br: [we, sill, max_z - offset],
                            normal: [0.0, 0.0, 1.0],
                            uv_min: [0.0, 0.0],
                            uv_max: [window_width, wh],
                        },
                    );
                    append_quad(
                        &mut mesh,
                        Quad {
                            tl: [we, sill + wh, min_z + offset],
                            tr: [ws, sill + wh, min_z + offset],
                            bl: [we, sill, min_z + offset],
                            br: [ws, sill, min_z + offset],
                            normal: [0.0, 0.0, -1.0],
                            uv_min: [0.0, 0.0],
                            uv_max: [window_width, wh],
                        },
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
        assert!(
            mesh.indices.len() / 6 >= 12,
            "window trim should include frame plus muntin crossbars"
        );

        let disabled = BuildingConfig {
            opening_trim_thickness: 0.0,
            ..config
        };
        assert!(generate_opening_trim_mesh(&grid, &disabled).is_empty());
    }

    #[test]
    fn test_opening_trim_extends_back_to_wall_face() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 1.0, 1.0),
            tile_size: 1.0,
            wall_thickness: 0.25,
            opening_trim_depth: 0.08,
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

        let TileType::Wall(wall) = grid.get(0, 0) else {
            panic!("test grid should contain an opening wall");
        };
        let bounds = wall_bounds_for_tile(&grid, 0, 0, wall, &config);
        let mesh = generate_opening_trim_mesh(&grid, &config);

        match wall.main_axis() {
            WallAxis::Z => {
                let min_x = bounds.0[0];
                let max_x = bounds.1[0];
                assert!(mesh.vertices.iter().any(|v| approx(v[0], min_x)));
                assert!(
                    mesh.vertices
                        .iter()
                        .any(|v| approx(v[0], min_x - config.opening_trim_depth))
                );
                assert!(mesh.vertices.iter().any(|v| approx(v[0], max_x)));
                assert!(
                    mesh.vertices
                        .iter()
                        .any(|v| approx(v[0], max_x + config.opening_trim_depth))
                );
            }
            WallAxis::X | WallAxis::Both => {
                let min_z = bounds.0[2];
                let max_z = bounds.1[2];
                assert!(mesh.vertices.iter().any(|v| approx(v[2], min_z)));
                assert!(
                    mesh.vertices
                        .iter()
                        .any(|v| approx(v[2], min_z - config.opening_trim_depth))
                );
                assert!(mesh.vertices.iter().any(|v| approx(v[2], max_z)));
                assert!(
                    mesh.vertices
                        .iter()
                        .any(|v| approx(v[2], max_z + config.opening_trim_depth))
                );
            }
        }
    }

    #[test]
    fn test_doorway_without_panel_generates_trim() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 1.0, 1.0),
            tile_size: 1.0,
            wall_thickness: 0.25,
            ..Default::default()
        };
        let mut grid = TileGrid::new(1, 1, config.tile_size, Vec2::ZERO);
        grid.set(
            0,
            0,
            TileType::Wall(
                WallTile::interior(WallShape::Straight(CardinalDir::Top))
                    .with_opening(WallOpening::Doorway),
            ),
        );

        let TileType::Wall(wall) = grid.get(0, 0) else {
            panic!("test grid should contain an opening wall");
        };
        let bounds = wall_bounds_for_tile(&grid, 0, 0, wall, &config);
        let mesh = generate_opening_trim_mesh(&grid, &config);
        assert!(!mesh.is_empty());
        assert!(
            mesh.vertices
                .iter()
                .all(|v| approx(v[2], bounds.0[2]) || approx(v[2], bounds.1[2]))
        );

        grid.set(
            0,
            0,
            TileType::Wall(
                WallTile::interior(WallShape::Straight(CardinalDir::Top)).with_opening(
                    WallOpening::Door {
                        render_panel: false,
                    },
                ),
            ),
        );

        let TileType::Wall(wall) = grid.get(0, 0) else {
            panic!("test grid should contain an opening wall");
        };
        let bounds = wall_bounds_for_tile(&grid, 0, 0, wall, &config);
        let mesh = generate_opening_trim_mesh(&grid, &config);
        assert!(!mesh.is_empty());
        assert!(
            mesh.vertices
                .iter()
                .all(|v| approx(v[2], bounds.0[2]) || approx(v[2], bounds.1[2]))
        );
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
        assert!(
            mesh.indices.len() / 6 > 2,
            "door mesh should include raised panel/plank detail"
        );
        assert!(!generate_door_hardware_mesh(&grid, &config).is_empty());
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

    fn approx(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.0001
    }
}
