use super::MeshData;

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

/// Appends a quad (two triangles) defined by four corners.
///
/// Vertices are expected in winding order such that the cross product of
/// (tr - tl) x (bl - tl) points in the same direction as `normal`.
pub fn append_quad(mesh: &mut MeshData, quad: Quad) {
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
