use super::MeshData;

/// Appends a quad (two triangles) defined by four corners.
///
/// Vertices are expected in winding order such that the cross product of
/// (tr - tl) x (bl - tl) points in the same direction as `normal`.
pub fn append_quad(
    mesh: &mut MeshData,
    tl: [f32; 3],
    tr: [f32; 3],
    bl: [f32; 3],
    br: [f32; 3],
    normal: [f32; 3],
    uv_min: [f32; 2],
    uv_max: [f32; 2],
) {
    let base = mesh.vertices.len() as u32;

    mesh.vertices.push(tl);
    mesh.vertices.push(tr);
    mesh.vertices.push(bl);
    mesh.vertices.push(br);

    mesh.normals.push(normal);
    mesh.normals.push(normal);
    mesh.normals.push(normal);
    mesh.normals.push(normal);

    mesh.uvs.push([uv_min[0], uv_max[1]]);
    mesh.uvs.push([uv_max[0], uv_max[1]]);
    mesh.uvs.push([uv_min[0], uv_min[1]]);
    mesh.uvs.push([uv_max[0], uv_min[1]]);

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
        // Quad in XZ plane at y=0, normal pointing up.
        append_quad(
            &mut mesh,
            [0.0, 0.0, 1.0], // tl
            [1.0, 0.0, 1.0], // tr
            [0.0, 0.0, 0.0], // bl
            [1.0, 0.0, 0.0], // br
            [0.0, 1.0, 0.0],
            [0.0, 0.0],
            [1.0, 1.0],
        );
        assert_eq!(mesh.vertices.len(), 4);
        assert_eq!(mesh.indices.len(), 6);
        // Indices should be: 0,1,3, 0,3,2
        assert_eq!(mesh.indices, vec![0, 1, 3, 0, 3, 2]);
    }
}
