use crate::mesh::MeshData;
use crate::mesh::colored_shapes::append_colored_box;

#[derive(Debug, Clone)]
pub struct CounterConfig {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub cabinet_color: [f32; 4],
    pub panel_color: [f32; 4],
    pub countertop_color: [f32; 4],
    pub trim_color: [f32; 4],
    pub handle_color: [f32; 4],
    pub countertop_height: f32,
    pub countertop_overhang: f32,
    pub toe_kick_height: f32,
    pub toe_kick_width_scale: f32,
    pub drawer_height_ratio: f32,
    pub panel_gap: f32,
    pub panel_columns: u32,
    pub front_detail_depth: f32,
    pub handle_depth: f32,
}

impl Default for CounterConfig {
    fn default() -> Self {
        Self {
            width: 0.9,
            height: 0.9,
            depth: 0.5,
            cabinet_color: [0.55, 0.4, 0.25, 1.0],
            panel_color: [0.4125, 0.3, 0.1875, 1.0],
            countertop_color: [0.82, 0.80, 0.72, 1.0],
            trim_color: [0.18, 0.14, 0.1, 1.0],
            handle_color: [0.08, 0.07, 0.06, 1.0],
            countertop_height: 0.08,
            countertop_overhang: 0.04,
            toe_kick_height: 0.08,
            toe_kick_width_scale: 0.78,
            drawer_height_ratio: 0.28,
            panel_gap: 0.018,
            panel_columns: 2,
            front_detail_depth: 0.022,
            handle_depth: 0.018,
        }
    }
}

pub fn generate_counter_mesh(w: f32, h: f32, d: f32, config: &CounterConfig) -> MeshData {
    let mut mesh = MeshData::default();

    let actual_w = if w > 0.0 { w } else { config.width };
    let actual_h = if h > 0.0 { h } else { config.height };
    let actual_d = if d > 0.0 { d } else { config.depth };

    let top_h = config.countertop_height.clamp(0.01, actual_h * 0.3);
    let kick_h = config.toe_kick_height.clamp(0.0, actual_h * 0.25);
    let panel_gap = config.panel_gap.max(0.0);
    let panel_columns = config.panel_columns.max(1);
    let front_detail_depth = config.front_detail_depth.max(0.001);
    let handle_depth = config.handle_depth.max(0.001);
    let front_z = actual_d / 2.0 + front_detail_depth / 2.0;

    append_colored_box(
        &mut mesh,
        [0.0, (actual_h - top_h) / 2.0, 0.0],
        [actual_w, actual_h - top_h, actual_d],
        config.cabinet_color,
    );
    append_colored_box(
        &mut mesh,
        [0.0, actual_h - top_h / 2.0, 0.0],
        [
            actual_w + config.countertop_overhang * 2.0,
            top_h,
            actual_d + config.countertop_overhang * 2.0,
        ],
        config.countertop_color,
    );
    append_colored_box(
        &mut mesh,
        [0.0, kick_h / 2.0, actual_d / 2.0 + front_detail_depth / 2.0],
        [
            actual_w * config.toe_kick_width_scale.clamp(0.0, 1.0),
            kick_h,
            front_detail_depth,
        ],
        config.trim_color,
    );

    let storage_h = (actual_h - top_h - kick_h).max(0.01);
    let drawer_h = storage_h * config.drawer_height_ratio.clamp(0.0, 0.8);
    let door_h = (storage_h - drawer_h - panel_gap * 3.0).max(0.0);
    let drawer_y = kick_h + door_h + panel_gap * 2.0 + drawer_h / 2.0;
    let door_y = kick_h + panel_gap + door_h / 2.0;
    let panel_w =
        ((actual_w - panel_gap * (panel_columns as f32 + 1.0)) / panel_columns as f32).max(0.001);

    for column in 0..panel_columns {
        let x = -actual_w / 2.0 + panel_gap + panel_w / 2.0 + column as f32 * (panel_w + panel_gap);
        let handle_side = if column < panel_columns / 2 {
            -1.0
        } else {
            1.0
        };
        append_colored_box(
            &mut mesh,
            [x, drawer_y, front_z],
            [panel_w, drawer_h, front_detail_depth],
            config.panel_color,
        );
        if door_h > 0.01 {
            append_colored_box(
                &mut mesh,
                [x, door_y, front_z],
                [panel_w, door_h, front_detail_depth],
                config.panel_color,
            );
            append_colored_box(
                &mut mesh,
                [
                    x - handle_side * panel_w * 0.18,
                    door_y + door_h * 0.12,
                    front_z + front_detail_depth / 2.0 + handle_depth / 2.0,
                ],
                [handle_depth, door_h * 0.28, handle_depth],
                config.handle_color,
            );
        }
        if drawer_h > 0.01 {
            append_colored_box(
                &mut mesh,
                [
                    x,
                    drawer_y,
                    front_z + front_detail_depth / 2.0 + handle_depth / 2.0,
                ],
                [panel_w * 0.46, handle_depth, handle_depth],
                config.handle_color,
            );
        }
    }

    mesh
}
