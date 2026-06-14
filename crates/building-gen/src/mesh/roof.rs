use super::math_util::{append_quad, cross3, normalize3, sub3};
use super::wall::building_top_y;
use super::MeshData;
use crate::config::BuildingConfig;
use crate::geometry::Rect;
use crate::layout::RoofGeometry;

pub fn generate_roof_mesh(bounds: Rect, roof: &RoofGeometry, config: &BuildingConfig) -> MeshData {
    let mut mesh = MeshData::default();
    let overhang = config.roof_overhang;
    let wall_h = building_top_y(config);

    let min_x = bounds.min.x - overhang;
    let max_x = bounds.max.x + overhang;
    let min_z = bounds.min.y - overhang;
    let max_z = bounds.max.y + overhang;
    let center_x = (bounds.min.x + bounds.max.x) / 2.0;
    let center_z = (bounds.min.y + bounds.max.y) / 2.0;
    let ridge_y = wall_h + config.roof_height;
    let ridge_runs_x = (roof.ridge_end.x - roof.ridge_start.x).abs()
        >= (roof.ridge_end.z - roof.ridge_start.z).abs();

    if ridge_runs_x {
        let ridge_min = [min_x, ridge_y, center_z];
        let ridge_max = [max_x, ridge_y, center_z];

        append_roof_quad(
            &mut mesh,
            ridge_min,
            ridge_max,
            [min_x, wall_h, min_z],
            [max_x, wall_h, min_z],
            [min_x, min_z],
            [max_x, center_z],
        );
        append_roof_quad(
            &mut mesh,
            ridge_max,
            ridge_min,
            [max_x, wall_h, max_z],
            [min_x, wall_h, max_z],
            [min_x, center_z],
            [max_x, max_z],
        );
    } else {
        let ridge_min = [center_x, ridge_y, min_z];
        let ridge_max = [center_x, ridge_y, max_z];

        append_roof_quad(
            &mut mesh,
            ridge_max,
            ridge_min,
            [min_x, wall_h, max_z],
            [min_x, wall_h, min_z],
            [min_z, center_x],
            [max_z, min_x],
        );
        append_roof_quad(
            &mut mesh,
            ridge_min,
            ridge_max,
            [max_x, wall_h, min_z],
            [max_x, wall_h, max_z],
            [min_z, center_x],
            [max_z, max_x],
        );
    }

    mesh
}

pub fn generate_gable_mesh(
    bounds: Rect,
    roof: &RoofGeometry,
    config: &BuildingConfig,
) -> MeshData {
    let mut mesh = MeshData::default();
    let overhang = config.roof_overhang;
    let wall_h = building_top_y(config);

    let min_x = bounds.min.x - overhang;
    let max_x = bounds.max.x + overhang;
    let min_z = bounds.min.y - overhang;
    let max_z = bounds.max.y + overhang;
    let center_x = (bounds.min.x + bounds.max.x) / 2.0;
    let center_z = (bounds.min.y + bounds.max.y) / 2.0;
    let ridge_y = wall_h + config.roof_height;
    let ridge_runs_x = (roof.ridge_end.x - roof.ridge_start.x).abs()
        >= (roof.ridge_end.z - roof.ridge_start.z).abs();

    if ridge_runs_x {
        let ridge_min = [min_x, ridge_y, center_z];
        let ridge_max = [max_x, ridge_y, center_z];

        append_roof_triangle(
            &mut mesh,
            [min_x, wall_h, min_z],
            [min_x, wall_h, max_z],
            ridge_min,
            [-1.0, 0.0, 0.0],
        );
        append_roof_triangle(
            &mut mesh,
            [max_x, wall_h, max_z],
            [max_x, wall_h, min_z],
            ridge_max,
            [1.0, 0.0, 0.0],
        );
    } else {
        let ridge_min = [center_x, ridge_y, min_z];
        let ridge_max = [center_x, ridge_y, max_z];

        append_roof_triangle(
            &mut mesh,
            [max_x, wall_h, min_z],
            [min_x, wall_h, min_z],
            ridge_min,
            [0.0, 0.0, -1.0],
        );
        append_roof_triangle(
            &mut mesh,
            [min_x, wall_h, max_z],
            [max_x, wall_h, max_z],
            ridge_max,
            [0.0, 0.0, 1.0],
        );
    }

    mesh
}

fn append_roof_quad(
    mesh: &mut MeshData,
    tl: [f32; 3],
    tr: [f32; 3],
    bl: [f32; 3],
    br: [f32; 3],
    uv_min: [f32; 2],
    uv_max: [f32; 2],
) {
    let normal = normalize3(cross3(sub3(tr, tl), sub3(bl, tl)));
    append_quad(mesh, tl, tr, bl, br, normal, uv_min, uv_max);
}

fn append_roof_triangle(
    mesh: &mut MeshData,
    a: [f32; 3],
    b: [f32; 3],
    c: [f32; 3],
    normal: [f32; 3],
) {
    let base = mesh.vertices.len() as u32;
    mesh.vertices.push(a);
    mesh.vertices.push(b);
    mesh.vertices.push(c);
    mesh.normals.push(normal);
    mesh.normals.push(normal);
    mesh.normals.push(normal);
    mesh.uvs.push([a[0], a[2]]);
    mesh.uvs.push([b[0], b[2]]);
    mesh.uvs.push([c[0], c[2]]);
    mesh.indices.extend_from_slice(&[base, base + 1, base + 2]);
}
