use crate::mesh::MeshData;
use crate::mesh::SurfaceMaterial;
use crate::mesh::colored_shapes::append_material_box;

#[derive(Debug, Clone, Copy)]
pub struct StoveConfig {
    pub has_wood: bool,
    pub chimney_wall_thickness: f32,
    pub chimney_height: f32,
    pub base_color: [f32; 4],
    pub wood_color: [f32; 4],
    pub fire_color: [f32; 4],
    pub mantel_color: [f32; 4],
}

impl Default for StoveConfig {
    fn default() -> Self {
        Self {
            has_wood: true,
            chimney_wall_thickness: 0.10,
            chimney_height: 1.3, // Typical height to reach a 2.5m ceiling from a 1.2m base
            base_color: [0.5, 0.5, 0.5, 1.0],
            wood_color: [0.45, 0.3, 0.15, 1.0],
            fire_color: [0.9, 0.4, 0.1, 1.0],
            mantel_color: [0.4, 0.28, 0.18, 1.0],
        }
    }
}

pub fn generate_stove_mesh(w: f32, _h: f32, d: f32, config: &StoveConfig) -> MeshData {
    let mut mesh = MeshData::default();

    let base_color = config.base_color;
    let wood_color = config.wood_color;
    let mantel_color = config.mantel_color;
    let fire_color = config.fire_color;

    // Dimensions
    let base_h = 0.15;
    let main_h = 0.9;
    let mantel_h = 0.15;
    let chimney_h = config.chimney_height;

    let back_z = d / 2.0;

    // 1. Base (hearth)
    append_material_box(
        &mut mesh,
        [0.0, base_h / 2.0, 0.0],
        [w, base_h, d],
        base_color,
        SurfaceMaterial::Stone,
    );

    // 2. Main Body (walls forming the fireplace opening)
    let wall_t = 0.2; // Thickness of the side and back walls
    let main_w = w * 0.9;
    let main_d = d * 0.85;
    let main_y = base_h + main_h / 2.0;
    let main_z = back_z - main_d / 2.0;

    // Left wall
    append_material_box(
        &mut mesh,
        [-main_w / 2.0 + wall_t / 2.0, main_y, main_z],
        [wall_t, main_h, main_d],
        base_color,
        SurfaceMaterial::Stone,
    );
    // Right wall
    append_material_box(
        &mut mesh,
        [main_w / 2.0 - wall_t / 2.0, main_y, main_z],
        [wall_t, main_h, main_d],
        base_color,
        SurfaceMaterial::Stone,
    );
    // Back wall
    append_material_box(
        &mut mesh,
        [0.0, main_y, back_z - wall_t / 2.0],
        [main_w - 2.0 * wall_t, main_h, wall_t],
        base_color,
        SurfaceMaterial::Stone,
    );

    // 3. Fire and Wood inside
    if config.has_wood {
        let wood_y = base_h + 0.05;
        let wood_w = 0.1;
        let wood_len = 0.45;

        // Log 1 left
        append_material_box(
            &mut mesh,
            [-0.15, wood_y, main_z],
            [wood_w, wood_w, wood_len],
            wood_color,
            SurfaceMaterial::Wood,
        );
        // Log 2 right
        append_material_box(
            &mut mesh,
            [0.15, wood_y, main_z],
            [wood_w, wood_w, wood_len],
            wood_color,
            SurfaceMaterial::Wood,
        );
        // Log 3 across
        append_material_box(
            &mut mesh,
            [0.0, wood_y + wood_w, main_z],
            [wood_len, wood_w, wood_w],
            wood_color,
            SurfaceMaterial::Wood,
        );
        // Fire block
        append_material_box(
            &mut mesh,
            [0.0, wood_y + wood_w / 2.0, main_z - 0.05],
            [0.25, 0.2, 0.25],
            fire_color,
            SurfaceMaterial::Colored,
        );
    }

    // 4. Mantelpiece
    let mantel_d = main_d + 0.1;
    let mantel_z = back_z - mantel_d / 2.0;
    let mantel_y = base_h + main_h + mantel_h / 2.0;
    append_material_box(
        &mut mesh,
        [0.0, mantel_y, mantel_z],
        [main_w + 0.15, mantel_h, mantel_d],
        mantel_color,
        SurfaceMaterial::Wood,
    );

    // 5. Chimney (stepped, tapering to the ceiling)
    if chimney_h > 0.0 {
        let steps = 4;
        let step_h = chimney_h / steps as f32;
        for i in 0..steps {
            let t = i as f32 / steps as f32;
            let cur_w = main_w * (0.85 - t * 0.4);
            let cur_d = main_d * (0.85 - t * 0.4);
            let cur_y = base_h + main_h + mantel_h + step_h * (i as f32) + step_h / 2.0;
            let cur_z = back_z - cur_d / 2.0;

            if i == steps - 1 {
                let wall_th = config.chimney_wall_thickness;
                // Left
                append_material_box(
                    &mut mesh,
                    [-cur_w / 2.0 + wall_th / 2.0, cur_y, cur_z],
                    [wall_th, step_h, cur_d],
                    base_color,
                    SurfaceMaterial::Stone,
                );
                // Right
                append_material_box(
                    &mut mesh,
                    [cur_w / 2.0 - wall_th / 2.0, cur_y, cur_z],
                    [wall_th, step_h, cur_d],
                    base_color,
                    SurfaceMaterial::Stone,
                );
                // Back
                append_material_box(
                    &mut mesh,
                    [0.0, cur_y, cur_z + cur_d / 2.0 - wall_th / 2.0],
                    [cur_w - 2.0 * wall_th, step_h, wall_th],
                    base_color,
                    SurfaceMaterial::Stone,
                );
                // Front
                append_material_box(
                    &mut mesh,
                    [0.0, cur_y, cur_z - cur_d / 2.0 + wall_th / 2.0],
                    [cur_w - 2.0 * wall_th, step_h, wall_th],
                    base_color,
                    SurfaceMaterial::Stone,
                );

                // Dark hole interior
                append_material_box(
                    &mut mesh,
                    [0.0, cur_y + 0.01, cur_z], // slightly raised so it's clearly above the solid block below
                    [cur_w - 2.0 * wall_th, step_h, cur_d - 2.0 * wall_th],
                    [0.15, 0.15, 0.15, 1.0], // Dark charcoal color
                    SurfaceMaterial::Stone,
                );
            } else {
                append_material_box(
                    &mut mesh,
                    [0.0, cur_y, cur_z],
                    [cur_w, step_h, cur_d],
                    base_color,
                    SurfaceMaterial::Stone,
                );
            }
        }
    }

    mesh
}
