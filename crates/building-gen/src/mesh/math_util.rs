use super::{MeshData, SurfaceMaterial};

/// Four corners of a quad with normal and UV coordinates.
pub struct Quad {
    pub tl: [f32; 3],
    pub tr: [f32; 3],
    pub bl: [f32; 3],
    pub br: [f32; 3],
    pub normal: [f32; 3],
    pub uv_min: [f32; 2],
    pub uv_max: [f32; 2],
}

/// Calculates continuous world-space UV coordinates based on normal direction
/// preserving winding to ensure correct tangent generation.
pub fn get_uv(pos: [f32; 3], normal: [f32; 3]) -> [f32; 2] {
    let [x, y, z] = pos;
    if normal[0].abs() > 0.5 {
        if normal[0] > 0.0 {
            [z, y] // PosX
        } else {
            [-z, y] // NegX
        }
    } else if normal[2].abs() > 0.5 {
        if normal[2] > 0.0 {
            [-x, y] // PosZ
        } else {
            [x, y] // NegZ
        }
    } else {
        [x, z] // PosY or NegY
    }
}

/// Appends a quad (two triangles) defined by four corners.
///
/// Vertices are expected in winding order such that the cross product of
/// (tr - tl) x (bl - tl) points in the same direction as `normal`.
pub fn append_quad(mesh: &mut MeshData, quad: Quad) {
    append_quad_with_material(mesh, quad, SurfaceMaterial::Colored);
}

pub fn append_quad_with_material(mesh: &mut MeshData, quad: Quad, material: SurfaceMaterial) {
    let base = mesh.vertices.len() as u32;

    mesh.vertices.push(quad.tl);
    mesh.vertices.push(quad.tr);
    mesh.vertices.push(quad.bl);
    mesh.vertices.push(quad.br);

    mesh.normals.push(quad.normal);
    mesh.normals.push(quad.normal);
    mesh.normals.push(quad.normal);
    mesh.normals.push(quad.normal);

    mesh.uvs.push([quad.uv_min[0], quad.uv_max[1]]);
    mesh.uvs.push([quad.uv_max[0], quad.uv_max[1]]);
    mesh.uvs.push([quad.uv_min[0], quad.uv_min[1]]);
    mesh.uvs.push([quad.uv_max[0], quad.uv_min[1]]);

    // Triangle 1: tl, tr, br  |  Triangle 2: tl, br, bl
    mesh.indices.push(base);
    mesh.indices.push(base + 1);
    mesh.indices.push(base + 3);
    mesh.indices.push(base);
    mesh.indices.push(base + 3);
    mesh.indices.push(base + 2);
    mesh.surface_materials.extend([material; 2]);
}

pub fn append_colored_quad(mesh: &mut MeshData, quad: Quad, color: [f32; 4]) {
    append_colored_quad_with_material(mesh, quad, color, SurfaceMaterial::Colored);
}

pub fn append_colored_quad_with_material(
    mesh: &mut MeshData,
    quad: Quad,
    color: [f32; 4],
    material: SurfaceMaterial,
) {
    append_colored_quad_vertices(mesh, quad, [color; 4], material);
}

pub fn append_colored_quad_vertices(
    mesh: &mut MeshData,
    quad: Quad,
    colors: [[f32; 4]; 4],
    material: SurfaceMaterial,
) {
    append_quad_with_material(mesh, quad, material);
    mesh.colors.extend(colors);
}

pub fn append_colored_triangle(
    mesh: &mut MeshData,
    a: [f32; 3],
    b: [f32; 3],
    c: [f32; 3],
    normal: [f32; 3],
    color: [f32; 4],
) {
    append_colored_triangle_with_material(mesh, a, b, c, normal, color, SurfaceMaterial::Colored);
}

pub fn append_colored_triangle_with_material(
    mesh: &mut MeshData,
    a: [f32; 3],
    b: [f32; 3],
    c: [f32; 3],
    normal: [f32; 3],
    color: [f32; 4],
    material: SurfaceMaterial,
) {
    let base = mesh.vertices.len() as u32;
    mesh.vertices.extend([a, b, c]);
    mesh.normals.extend([normal; 3]);
    mesh.uvs.extend([[0.5, 0.5], [0.0, 1.0], [1.0, 1.0]]);
    mesh.colors.extend([color; 3]);
    mesh.indices.extend([base, base + 1, base + 2]);
    mesh.surface_materials.push(material);
}

pub fn sub3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub fn lerp3(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

pub fn cross3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

pub fn vec3_length(v: [f32; 3]) -> f32 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

pub fn normalize3(v: [f32; 3]) -> [f32; 3] {
    let len = vec3_length(v);
    if len < f32::EPSILON {
        return [0.0, 1.0, 0.0];
    }
    [v[0] / len, v[1] / len, v[2] / len]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_quad_winding() {
        let mut mesh = MeshData::default();
        append_quad(
            &mut mesh,
            Quad {
                tl: [0.0, 0.0, 1.0],
                tr: [1.0, 0.0, 1.0],
                bl: [0.0, 0.0, 0.0],
                br: [1.0, 0.0, 0.0],
                normal: [0.0, 1.0, 0.0],
                uv_min: [0.0, 0.0],
                uv_max: [1.0, 1.0],
            },
        );
        assert_eq!(mesh.vertices.len(), 4);
        assert_eq!(mesh.indices.len(), 6);
        assert_eq!(mesh.indices, vec![0, 1, 3, 0, 3, 2]);
    }
}
