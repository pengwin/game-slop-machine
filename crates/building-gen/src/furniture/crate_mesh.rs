use crate::mesh::MeshData;
use crate::mesh::SurfaceMaterial;
use crate::mesh::colored_shapes::append_material_box;

pub fn generate_crate_mesh(w: f32, h: f32, d: f32, color: [f32; 3]) -> MeshData {
    let mut mesh = MeshData::default();
    let wood_color = [color[0], color[1], color[2], 1.0];
    let metal_color = [0.2, 0.2, 0.2, 1.0];

    append_material_box(
        &mut mesh,
        [0.0, h / 2.0, 0.0],
        [w, h, d],
        wood_color,
        SurfaceMaterial::Wood,
    );
    // Simple metal straps
    let t = 0.02;
    append_material_box(
        &mut mesh,
        [0.0, h / 2.0, d / 2.0],
        [w, t, t],
        metal_color,
        SurfaceMaterial::Metal,
    );
    append_material_box(
        &mut mesh,
        [0.0, h / 2.0, -d / 2.0],
        [w, t, t],
        metal_color,
        SurfaceMaterial::Metal,
    );
    mesh
}
