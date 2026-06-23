use crate::mesh::MeshData;
use crate::mesh::SurfaceMaterial;
use crate::mesh::colored_shapes::append_material_box;

#[derive(Debug, Clone)]
pub struct TableConfig {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub top_color: [f32; 4],
    pub leg_color: [f32; 4],
    pub apron_color: [f32; 4],
    pub top_thickness: f32,
    pub support_plate_height: f32,
    pub support_plate_inset: f32,
    pub leg_thickness: f32,
    pub leg_inset: f32,
    pub apron_height: f32,
    pub apron_thickness: f32,
}

impl Default for TableConfig {
    fn default() -> Self {
        Self {
            width: 0.8,
            height: 0.55,
            depth: 0.5,
            top_color: [0.6, 0.45, 0.25, 1.0],
            leg_color: [0.44, 0.32, 0.18, 1.0],
            apron_color: [0.48, 0.36, 0.2, 1.0],
            top_thickness: 0.08,
            support_plate_height: 0.05,
            support_plate_inset: 0.08,
            leg_thickness: 0.09,
            leg_inset: 0.09,
            apron_height: 0.06,
            apron_thickness: 0.04,
        }
    }
}

pub fn generate_table_mesh(w: f32, h: f32, d: f32, config: &TableConfig) -> MeshData {
    let mut mesh = MeshData::default();

    let actual_w = if w > 0.0 { w } else { config.width };
    let actual_h = if h > 0.0 { h } else { config.height };
    let actual_d = if d > 0.0 { d } else { config.depth };
    let hw = actual_w / 2.0;
    let hd = actual_d / 2.0;

    let top_thickness = config.top_thickness.clamp(0.01, actual_h * 0.35);
    let support_h = config.support_plate_height.clamp(0.0, actual_h * 0.25);
    let leg_h = (actual_h - top_thickness + support_h * 0.5).max(0.01);
    let leg_t = config
        .leg_thickness
        .clamp(0.01, actual_w.min(actual_d) * 0.35);
    let leg_inset = config.leg_inset.max(0.0);
    let leg_x = (hw - leg_inset - leg_t / 2.0).max(0.0);
    let leg_z = (hd - leg_inset - leg_t / 2.0).max(0.0);

    append_material_box(
        &mut mesh,
        [0.0, actual_h - top_thickness / 2.0, 0.0],
        [actual_w, top_thickness, actual_d],
        config.top_color,
        SurfaceMaterial::Wood,
    );
    append_tabletop_plank_detail(&mut mesh, actual_w, actual_h, actual_d);

    if support_h > 0.0 {
        let support_inset = config.support_plate_inset.max(0.0);
        append_material_box(
            &mut mesh,
            [0.0, actual_h - top_thickness - support_h / 2.0, 0.0],
            [
                (actual_w - support_inset * 2.0).max(leg_t),
                support_h,
                (actual_d - support_inset * 2.0).max(leg_t),
            ],
            config.apron_color,
            SurfaceMaterial::Wood,
        );
    }

    for (lx, lz) in [
        (-leg_x, -leg_z),
        (leg_x, -leg_z),
        (-leg_x, leg_z),
        (leg_x, leg_z),
    ] {
        append_material_box(
            &mut mesh,
            [lx, leg_h / 2.0, lz],
            [leg_t, leg_h, leg_t],
            config.leg_color,
            SurfaceMaterial::Wood,
        );
    }

    let apron_h = config.apron_height.clamp(0.0, leg_h * 0.5);
    let apron_t = config
        .apron_thickness
        .clamp(0.0, actual_w.min(actual_d) * 0.2);
    if apron_h > 0.0 && apron_t > 0.0 {
        let apron_y = leg_h - apron_h / 2.0;
        let apron_w = (actual_w - leg_inset * 2.0).max(leg_t);
        let apron_d = (actual_d - leg_inset * 2.0).max(leg_t);

        append_material_box(
            &mut mesh,
            [0.0, apron_y, -hd + leg_inset + apron_t / 2.0],
            [apron_w, apron_h, apron_t],
            config.apron_color,
            SurfaceMaterial::Wood,
        );
        append_material_box(
            &mut mesh,
            [0.0, apron_y, hd - leg_inset - apron_t / 2.0],
            [apron_w, apron_h, apron_t],
            config.apron_color,
            SurfaceMaterial::Wood,
        );
        append_material_box(
            &mut mesh,
            [-hw + leg_inset + apron_t / 2.0, apron_y, 0.0],
            [apron_t, apron_h, apron_d],
            config.apron_color,
            SurfaceMaterial::Wood,
        );
        append_material_box(
            &mut mesh,
            [hw - leg_inset - apron_t / 2.0, apron_y, 0.0],
            [apron_t, apron_h, apron_d],
            config.apron_color,
            SurfaceMaterial::Wood,
        );
    }

    mesh
}

fn append_tabletop_plank_detail(mesh: &mut MeshData, width: f32, top_y: f32, depth: f32) {
    let groove_color = [0.13, 0.075, 0.038, 1.0];
    let groove_w = (width * 0.014).clamp(0.006, 0.012);
    let groove_h = 0.006;
    let groove_d = depth * 0.92;
    let y = top_y + groove_h * 0.5 + 0.003;

    for x in [-width * 0.25, 0.0, width * 0.25] {
        append_material_box(
            mesh,
            [x, y, 0.0],
            [groove_w, groove_h, groove_d],
            groove_color,
            SurfaceMaterial::Wood,
        );
    }
}
