use crate::mesh::MeshData;
use crate::mesh::colored_shapes::{append_colored_beveled_box, append_colored_box};

#[derive(Debug, Clone)]
pub struct BedConfig {
    pub num_pillows: u32,
    pub pillow_size: [f32; 3], // width, height, depth
    pub headboard_height: f32, // relative to h
    pub footboard_height: f32,
    pub frame_height: f32, // height of the side rails
    pub wood_color: [f32; 4],
    pub sheet_color: [f32; 4],
    pub blanket_color: [f32; 4],
}

impl Default for BedConfig {
    fn default() -> Self {
        Self {
            num_pillows: 1,
            pillow_size: [0.4, 0.08, 0.25],
            headboard_height: 1.0,
            footboard_height: 0.7,
            frame_height: 0.15,
            wood_color: [0.5, 0.3, 0.15, 1.0],
            sheet_color: [0.95, 0.95, 0.95, 1.0],
            blanket_color: [0.65, 0.35, 0.25, 1.0],
        }
    }
}

pub fn generate_bed_mesh(w: f32, h: f32, d: f32, config: &BedConfig) -> MeshData {
    let mut mesh = MeshData::default();

    let frame_color = config.wood_color;
    let sheet_color = config.sheet_color;
    let blanket_color = config.blanket_color;

    let pt = 0.08;
    let front_h = h * config.footboard_height;
    let back_h = h * config.headboard_height;

    let px = w / 2.0 - pt / 2.0;
    let pz = d / 2.0 - pt / 2.0;

    append_colored_box(
        &mut mesh,
        [-px, front_h / 2.0, -pz],
        [pt, front_h, pt],
        frame_color,
    );
    append_colored_box(
        &mut mesh,
        [px, front_h / 2.0, -pz],
        [pt, front_h, pt],
        frame_color,
    );
    append_colored_box(
        &mut mesh,
        [-px, back_h / 2.0, pz],
        [pt, back_h, pt],
        frame_color,
    );
    append_colored_box(
        &mut mesh,
        [px, back_h / 2.0, pz],
        [pt, back_h, pt],
        frame_color,
    );

    let hb_h = back_h - config.frame_height;
    append_colored_box(
        &mut mesh,
        [0.0, config.frame_height + hb_h / 2.0, pz],
        [w - pt * 2.0, hb_h, pt / 2.0],
        frame_color,
    );

    let fb_h = front_h - config.frame_height;
    append_colored_box(
        &mut mesh,
        [0.0, config.frame_height + fb_h / 2.0, -pz],
        [w - pt * 2.0, fb_h, pt / 2.0],
        frame_color,
    );

    let rail_h = config.frame_height;
    let rail_y = 0.12 + rail_h / 2.0;
    let rail_len = d - pt * 2.0;
    append_colored_box(
        &mut mesh,
        [-px, rail_y, 0.0],
        [pt / 2.0, rail_h, rail_len],
        frame_color,
    );
    append_colored_box(
        &mut mesh,
        [px, rail_y, 0.0],
        [pt / 2.0, rail_h, rail_len],
        frame_color,
    );

    let mattress_w = w - pt * 1.5;
    let mattress_d = d - pt * 1.5;
    let mattress_y = rail_y;
    let mattress_h = rail_h + 0.02;
    append_colored_box(
        &mut mesh,
        [0.0, mattress_y, 0.0],
        [mattress_w, mattress_h, mattress_d],
        sheet_color,
    );

    let blanket_z_min = -pz + pt / 2.0;
    let blanket_z_max = pz - pt / 2.0 - 0.3;
    if blanket_z_max > blanket_z_min {
        let blanket_len = blanket_z_max - blanket_z_min;
        let blanket_z = blanket_z_min + blanket_len / 2.0;
        let blanket_w = mattress_w + 0.02;
        let blanket_h = mattress_h + 0.02;
        append_colored_box(
            &mut mesh,
            [0.0, mattress_y + 0.01, blanket_z],
            [blanket_w, blanket_h, blanket_len],
            blanket_color,
        );
    }

    if config.num_pillows > 0 {
        let pillow_y = mattress_y + mattress_h / 2.0 + config.pillow_size[1] / 2.0;
        let pillow_z = pz - pt / 2.0 - 0.15;

        let total_pillow_w = config.pillow_size[0] * config.num_pillows as f32;
        let spacing = if config.num_pillows > 1 {
            (mattress_w - 0.1 - total_pillow_w) / (config.num_pillows as f32 - 1.0).max(1.0)
        } else {
            0.0
        };

        let start_x = if config.num_pillows == 1 {
            0.0
        } else {
            -(total_pillow_w + spacing * (config.num_pillows as f32 - 1.0)) / 2.0
                + config.pillow_size[0] / 2.0
        };

        for i in 0..config.num_pillows {
            let px = start_x + (config.pillow_size[0] + spacing) * i as f32;
            let bevel_amount = config.pillow_size[1] * 0.4;
            append_colored_beveled_box(
                &mut mesh,
                [px, pillow_y, pillow_z],
                config.pillow_size,
                bevel_amount,
                sheet_color,
            );
        }
    }

    mesh
}
