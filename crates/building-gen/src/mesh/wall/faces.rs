use super::WallMeshes;
use super::classify::{ExteriorFaceClass, WallCutout, WallFaceDir};
use crate::config::BuildingConfig;
use crate::mesh::MeshData;
use crate::mesh::math_util::{self, Quad};
use crate::tile::{TileGrid, TileType, WallAxis};

pub fn append_wall_box(
    meshes: &mut WallMeshes,
    bounds: ([f32; 3], [f32; 3]),
    axis: WallAxis,
    exterior_class: ExteriorFaceClass,
    exterior_faces: &[WallFaceDir],
    config: &BuildingConfig,
    cutout: Option<WallCutout>,
    grid: &TileGrid,
    grid_x: usize,
    grid_y: usize,
) {
    let cutout_dirs: &[WallFaceDir] = match axis {
        WallAxis::X => &[WallFaceDir::NegZ, WallFaceDir::PosZ],
        WallAxis::Z => &[WallFaceDir::NegX, WallFaceDir::PosX],
        WallAxis::Both => &[
            WallFaceDir::NegX,
            WallFaceDir::PosX,
            WallFaceDir::NegZ,
            WallFaceDir::PosZ,
        ],
    };

    for dir in [
        WallFaceDir::NegX,
        WallFaceDir::PosX,
        WallFaceDir::NegZ,
        WallFaceDir::PosZ,
    ] {
        let face_cutout = if cutout_dirs.contains(&dir) {
            cutout
        } else {
            None
        };
        let mesh = match exterior_class {
            ExteriorFaceClass::Corner if !exterior_faces.is_empty() => &mut meshes.exterior_corner,
            ExteriorFaceClass::TJunction if exterior_faces.contains(&dir) => {
                &mut meshes.exterior_t_junction
            }
            ExteriorFaceClass::Straight if exterior_faces.contains(&dir) => &mut meshes.exterior,
            _ => &mut meshes.wall,
        };
        append_wall_face(mesh, bounds, dir, config, face_cutout, grid, grid_x, grid_y);
    }
    append_wall_face(&mut meshes.top, bounds, WallFaceDir::PosY, config, None, grid, grid_x, grid_y);
}

struct FaceCorners {
    normal: [f32; 3],
    tl: [f32; 3],
    tr: [f32; 3],
    bl: [f32; 3],
    br: [f32; 3],
}

fn face_quad(dir: WallFaceDir, bounds: ([f32; 3], [f32; 3])) -> FaceCorners {
    let [min_x, min_y, min_z] = bounds.0;
    let [max_x, max_y, max_z] = bounds.1;
    match dir {
        WallFaceDir::PosX => FaceCorners {
            normal: [1.0, 0.0, 0.0],
            tl: [max_x, max_y, min_z],
            tr: [max_x, max_y, max_z],
            bl: [max_x, min_y, min_z],
            br: [max_x, min_y, max_z],
        },
        WallFaceDir::NegX => FaceCorners {
            normal: [-1.0, 0.0, 0.0],
            tl: [min_x, max_y, max_z],
            tr: [min_x, max_y, min_z],
            bl: [min_x, min_y, max_z],
            br: [min_x, min_y, min_z],
        },
        WallFaceDir::PosZ => FaceCorners {
            normal: [0.0, 0.0, 1.0],
            tl: [max_x, max_y, max_z],
            tr: [min_x, max_y, max_z],
            bl: [max_x, min_y, max_z],
            br: [min_x, min_y, max_z],
        },
        WallFaceDir::NegZ => FaceCorners {
            normal: [0.0, 0.0, -1.0],
            tl: [min_x, max_y, min_z],
            tr: [max_x, max_y, min_z],
            bl: [min_x, min_y, min_z],
            br: [max_x, min_y, min_z],
        },
        WallFaceDir::PosY => FaceCorners {
            normal: [0.0, 1.0, 0.0],
            tl: [min_x, max_y, max_z],
            tr: [max_x, max_y, max_z],
            bl: [min_x, max_y, min_z],
            br: [max_x, max_y, min_z],
        },
    }
}

fn compute_vertex_color(
    pos: [f32; 3],
    normal: [f32; 3],
    dir: WallFaceDir,
    grid: &TileGrid,
    gx: usize,
    gy: usize,
    config: &BuildingConfig,
) -> [f32; 4] {
    let mut tint = 1.0;

    // Height gradient (darker at bottom)
    let bottom_y = 0.0; // Assume wall base is 0.0 or close
    let height = (pos[1] - bottom_y).max(0.0);
    let bottom_dirt = (1.0 - height * 0.78).clamp(0.0, 1.0);
    tint -= bottom_dirt * 0.15;

    // Directional tint
    if normal[1] > 0.5 {
        // Top faces
        tint *= 1.15;
    } else if normal[1] < -0.5 {
        // Bottom faces
        tint *= 0.7;
    } else {
        // Side faces
        let side_tint = 1.0 - (normal[0].abs() * 0.04 + normal[2].abs() * 0.12);
        tint *= side_tint;
    }

    // Basic Ambient Occlusion based on grid
    if normal[1].abs() < 0.5 {
        let mut ao = 0.0;
        let check_x = match dir {
            WallFaceDir::PosX => (gx as isize) + 1,
            WallFaceDir::NegX => (gx as isize) - 1,
            _ => gx as isize,
        };
        let check_y = match dir {
            WallFaceDir::PosZ => (gy as isize) + 1,
            WallFaceDir::NegZ => (gy as isize) - 1,
            _ => gy as isize,
        };

        if check_x >= 0 && check_y >= 0 {
            if let TileType::Wall(_) = grid.get(check_x as usize, check_y as usize) {
                // If facing an adjacent wall, darken heavily (inner corner AO)
                ao += 0.2;
            }
        }
        
        // Darken corners of the tile itself
        let tile_center_x = (gx as f32) * config.tile_size;
        let tile_center_z = (gy as f32) * config.tile_size;
        let dx = (pos[0] - tile_center_x).abs();
        let dz = (pos[2] - tile_center_z).abs();
        if dx > config.tile_size * 0.4 && dz > config.tile_size * 0.4 {
            ao += 0.15; // Vertical edge AO
        }

        tint -= ao;
    }

    let c = tint.clamp(0.0, 1.0);
    [c, c, c, 1.0]
}

fn append_wall_face(
    mesh: &mut MeshData,
    bounds: ([f32; 3], [f32; 3]),
    dir: WallFaceDir,
    config: &BuildingConfig,
    cutout: Option<WallCutout>,
    grid: &TileGrid,
    grid_x: usize,
    grid_y: usize,
) {
    let [_, min_y, max_y] = [bounds.0[0], bounds.0[1], bounds.1[1]];
    let FaceCorners {
        normal,
        tl,
        tr,
        bl,
        br,
    } = face_quad(dir, bounds);

    let u_axis = math_util::sub3(tr, tl);
    let v_axis = math_util::sub3(bl, tl);
    let u_len = math_util::vec3_length(u_axis);
    let v_len = math_util::vec3_length(v_axis);

    let c_tl = compute_vertex_color(tl, normal, dir, grid, grid_x, grid_y, config);
    let c_tr = compute_vertex_color(tr, normal, dir, grid, grid_x, grid_y, config);
    let c_bl = compute_vertex_color(bl, normal, dir, grid, grid_x, grid_y, config);
    let c_br = compute_vertex_color(br, normal, dir, grid, grid_x, grid_y, config);

    match cutout {
        Some(WallCutout::Door) => {
            let door_width = config.door_width.min(u_len);
            let door_start = (u_len - door_width) / 2.0;
            let door_end = door_start + door_width;
            let door_h = config.door_height.min(max_y - min_y);
            append_wall_sub_faces(
                mesh, tl, tr, bl, br, c_tl, c_tr, c_bl, c_br, normal, u_len, v_len, door_start, door_end, 0.0, door_h,
            );
        }
        Some(WallCutout::Window) => {
            let window_width = config.window_width.min(u_len);
            let win_start = (u_len - window_width) / 2.0;
            let win_end = win_start + window_width;
            let sill = config.window_sill_height;
            let win_top = (sill + config.window_height).min(max_y - min_y);
            append_wall_sub_faces(
                mesh, tl, tr, bl, br, c_tl, c_tr, c_bl, c_br, normal, u_len, v_len, win_start, win_end, sill, win_top,
            );
        }
        None => {
            math_util::append_colored_quad_vertices(
                mesh,
                Quad {
                    tl,
                    tr,
                    bl,
                    br,
                    normal,
                    uv_min: [crate::mesh::math_util::get_uv(tl, normal)[0], crate::mesh::math_util::get_uv(bl, normal)[1]],
                    uv_max: [crate::mesh::math_util::get_uv(tr, normal)[0], crate::mesh::math_util::get_uv(tl, normal)[1]],
                },
                [c_tl, c_tr, c_bl, c_br],
                crate::mesh::SurfaceMaterial::Colored,
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn append_wall_sub_faces(
    mesh: &mut MeshData,
    tl: [f32; 3],
    tr: [f32; 3],
    bl: [f32; 3],
    br: [f32; 3],
    c_tl: [f32; 4],
    c_tr: [f32; 4],
    c_bl: [f32; 4],
    c_br: [f32; 4],
    normal: [f32; 3],
    u_len: f32,
    v_len: f32,
    cutout_start: f32,
    cutout_end: f32,
    cutout_bottom: f32,
    cutout_top: f32,
) {
    let cs = cutout_start.max(0.0).min(u_len);
    let ce = cutout_end.max(0.0).min(u_len);
    let cb = cutout_bottom.max(0.0).min(v_len);
    let ct = cutout_top.max(0.0).min(v_len);

    let lerp_face = |u: f32, v: f32| -> [f32; 3] {
        let ut = if u_len > 0.0 { u / u_len } else { 0.0 };
        let vt = if v_len > 0.0 { v / v_len } else { 0.0 };
        let top = math_util::lerp3(tl, tr, ut);
        let bot = math_util::lerp3(bl, br, ut);
        math_util::lerp3(bot, top, vt)
    };

    let lerp_color = |u: f32, v: f32| -> [f32; 4] {
        let ut = if u_len > 0.0 { u / u_len } else { 0.0 };
        let vt = if v_len > 0.0 { v / v_len } else { 0.0 };
        let c_top = [
            c_tl[0] + (c_tr[0] - c_tl[0]) * ut,
            c_tl[1] + (c_tr[1] - c_tl[1]) * ut,
            c_tl[2] + (c_tr[2] - c_tl[2]) * ut,
            1.0,
        ];
        let c_bot = [
            c_bl[0] + (c_br[0] - c_bl[0]) * ut,
            c_bl[1] + (c_br[1] - c_bl[1]) * ut,
            c_bl[2] + (c_br[2] - c_bl[2]) * ut,
            1.0,
        ];
        [
            c_bot[0] + (c_top[0] - c_bot[0]) * vt,
            c_bot[1] + (c_top[1] - c_bot[1]) * vt,
            c_bot[2] + (c_top[2] - c_bot[2]) * vt,
            1.0,
        ]
    };

    if cs > 0.0 {
        let a = lerp_face(0.0, 0.0);
        let b = lerp_face(cs, 0.0);
        let c = lerp_face(cs, v_len);
        let d = lerp_face(0.0, v_len);
        let ca = lerp_color(0.0, 0.0);
        let cb = lerp_color(cs, 0.0);
        let cc = lerp_color(cs, v_len);
        let cd = lerp_color(0.0, v_len);
        math_util::append_colored_quad_vertices(
            mesh,
            Quad {
                tl: d,
                tr: c,
                bl: a,
                br: b,
                normal,
                uv_min: [crate::mesh::math_util::get_uv(d, normal)[0], crate::mesh::math_util::get_uv(a, normal)[1]],
                uv_max: [crate::mesh::math_util::get_uv(c, normal)[0], crate::mesh::math_util::get_uv(d, normal)[1]],
            },
            [cd, cc, ca, cb],
            crate::mesh::SurfaceMaterial::Colored,
        );
    }

    if ce < u_len {
        let a = lerp_face(ce, 0.0);
        let b = lerp_face(u_len, 0.0);
        let c = lerp_face(u_len, v_len);
        let d = lerp_face(ce, v_len);
        let ca = lerp_color(ce, 0.0);
        let cb = lerp_color(u_len, 0.0);
        let cc = lerp_color(u_len, v_len);
        let cd = lerp_color(ce, v_len);
        math_util::append_colored_quad_vertices(
            mesh,
            Quad {
                tl: d,
                tr: c,
                bl: a,
                br: b,
                normal,
                uv_min: [crate::mesh::math_util::get_uv(d, normal)[0], crate::mesh::math_util::get_uv(a, normal)[1]],
                uv_max: [crate::mesh::math_util::get_uv(c, normal)[0], crate::mesh::math_util::get_uv(d, normal)[1]],
            },
            [cd, cc, ca, cb],
            crate::mesh::SurfaceMaterial::Colored,
        );
    }

    if cb > 0.0 {
        let a = lerp_face(cs, 0.0);
        let b = lerp_face(ce, 0.0);
        let c = lerp_face(ce, cb);
        let d = lerp_face(cs, cb);
        let ca = lerp_color(cs, 0.0);
        let cb_color = lerp_color(ce, 0.0);
        let cc = lerp_color(ce, cb);
        let cd = lerp_color(cs, cb);
        math_util::append_colored_quad_vertices(
            mesh,
            Quad {
                tl: d,
                tr: c,
                bl: a,
                br: b,
                normal,
                uv_min: [crate::mesh::math_util::get_uv(d, normal)[0], crate::mesh::math_util::get_uv(a, normal)[1]],
                uv_max: [crate::mesh::math_util::get_uv(c, normal)[0], crate::mesh::math_util::get_uv(d, normal)[1]],
            },
            [cd, cc, ca, cb_color],
            crate::mesh::SurfaceMaterial::Colored,
        );
    }

    if ct < v_len {
        let a = lerp_face(cs, ct);
        let b = lerp_face(ce, ct);
        let c = lerp_face(ce, v_len);
        let d = lerp_face(cs, v_len);
        let ca = lerp_color(cs, ct);
        let cb = lerp_color(ce, ct);
        let cc = lerp_color(ce, v_len);
        let cd = lerp_color(cs, v_len);
        math_util::append_colored_quad_vertices(
            mesh,
            Quad {
                tl: d,
                tr: c,
                bl: a,
                br: b,
                normal,
                uv_min: [crate::mesh::math_util::get_uv(d, normal)[0], crate::mesh::math_util::get_uv(a, normal)[1]],
                uv_max: [crate::mesh::math_util::get_uv(c, normal)[0], crate::mesh::math_util::get_uv(d, normal)[1]],
            },
            [cd, cc, ca, cb],
            crate::mesh::SurfaceMaterial::Colored,
        );
    }
}
