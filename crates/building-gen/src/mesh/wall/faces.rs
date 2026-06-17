use super::classify::{ExteriorFaceClass, WallCutout, WallFaceDir};
use super::WallMeshes;
use crate::config::BuildingConfig;
use crate::mesh::MeshData;
use crate::mesh::math_util;
use crate::tile::WallAxis;

pub fn append_wall_box(
    meshes: &mut WallMeshes,
    bounds: ([f32; 3], [f32; 3]),
    axis: WallAxis,
    exterior_class: ExteriorFaceClass,
    exterior_faces: &[WallFaceDir],
    config: &BuildingConfig,
    cutout: Option<WallCutout>,
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
            ExteriorFaceClass::Straight if exterior_faces.contains(&dir) => {
                &mut meshes.exterior
            }
            _ => &mut meshes.wall,
        };
        append_wall_face(mesh, bounds, dir, config, face_cutout);
    }
    append_wall_face(&mut meshes.top, bounds, WallFaceDir::PosY, config, None);
}

fn append_wall_face(
    mesh: &mut MeshData,
    bounds: ([f32; 3], [f32; 3]),
    dir: WallFaceDir,
    config: &BuildingConfig,
    cutout: Option<WallCutout>,
) {
    let [min_x, min_y, min_z] = bounds.0;
    let [max_x, max_y, max_z] = bounds.1;

    let (normal, tl, tr, bl, br) = match dir {
        WallFaceDir::PosX => {
            let n = [1.0, 0.0, 0.0];
            (
                n,
                [max_x, max_y, min_z],
                [max_x, max_y, max_z],
                [max_x, min_y, min_z],
                [max_x, min_y, max_z],
            )
        }
        WallFaceDir::NegX => {
            let n = [-1.0, 0.0, 0.0];
            (
                n,
                [min_x, max_y, max_z],
                [min_x, max_y, min_z],
                [min_x, min_y, max_z],
                [min_x, min_y, min_z],
            )
        }
        WallFaceDir::PosZ => {
            let n = [0.0, 0.0, 1.0];
            (
                n,
                [max_x, max_y, max_z],
                [min_x, max_y, max_z],
                [max_x, min_y, max_z],
                [min_x, min_y, max_z],
            )
        }
        WallFaceDir::NegZ => {
            let n = [0.0, 0.0, -1.0];
            (
                n,
                [min_x, max_y, min_z],
                [max_x, max_y, min_z],
                [min_x, min_y, min_z],
                [max_x, min_y, min_z],
            )
        }
        WallFaceDir::PosY => {
            let n = [0.0, 1.0, 0.0];
            (
                n,
                [min_x, max_y, max_z],
                [max_x, max_y, max_z],
                [min_x, max_y, min_z],
                [max_x, max_y, min_z],
            )
        }
    };

    let u_axis = math_util::sub3(tr, tl);
    let v_axis = math_util::sub3(bl, tl);

    let u_len = math_util::vec3_length(u_axis);
    let v_len = math_util::vec3_length(v_axis);

    match cutout {
        Some(WallCutout::Door) => {
            let door_width = config.door_width.min(u_len);
            let door_start = (u_len - door_width) / 2.0;
            let door_end = door_start + door_width;
            let door_h = config.door_height.min(max_y - min_y);
            append_wall_sub_faces(
                mesh, tl, tr, bl, br, normal, u_len, v_len, door_start, door_end, 0.0, door_h,
            );
        }
        Some(WallCutout::Window) => {
            let window_width = config.window_width.min(u_len);
            let win_start = (u_len - window_width) / 2.0;
            let win_end = win_start + window_width;
            let sill = config.window_sill_height;
            let win_top = (sill + config.window_height).min(max_y - min_y);
            append_wall_sub_faces(
                mesh, tl, tr, bl, br, normal, u_len, v_len, win_start, win_end, sill, win_top,
            );
        }
        None => {
            math_util::append_quad(mesh, tl, tr, bl, br, normal, [0.0, 0.0], [u_len, v_len]);
        }
    }
}

fn append_wall_sub_faces(
    mesh: &mut MeshData,
    tl: [f32; 3],
    tr: [f32; 3],
    bl: [f32; 3],
    br: [f32; 3],
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

    if cs > 0.0 {
        let a = lerp_face(0.0, 0.0);
        let b = lerp_face(cs, 0.0);
        let c = lerp_face(cs, v_len);
        let d = lerp_face(0.0, v_len);
        math_util::append_quad(mesh, d, c, a, b, normal, [0.0, 0.0], [cs, v_len]);
    }

    if ce < u_len {
        let a = lerp_face(ce, 0.0);
        let b = lerp_face(u_len, 0.0);
        let c = lerp_face(u_len, v_len);
        let d = lerp_face(ce, v_len);
        math_util::append_quad(mesh, d, c, a, b, normal, [ce, 0.0], [u_len, v_len]);
    }

    if cb > 0.0 {
        let a = lerp_face(cs, 0.0);
        let b = lerp_face(ce, 0.0);
        let c = lerp_face(ce, cb);
        let d = lerp_face(cs, cb);
        math_util::append_quad(mesh, d, c, a, b, normal, [cs, 0.0], [ce, cb]);
    }

    if ct < v_len {
        let a = lerp_face(cs, ct);
        let b = lerp_face(ce, ct);
        let c = lerp_face(ce, v_len);
        let d = lerp_face(cs, v_len);
        math_util::append_quad(mesh, d, c, a, b, normal, [cs, ct], [ce, v_len]);
    }
}
