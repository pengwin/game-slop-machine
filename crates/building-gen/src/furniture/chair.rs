use crate::mesh::colored_shapes::append_colored_box;
use crate::mesh::MeshData;

#[derive(Debug, Clone, PartialEq)]
pub enum ChairBackStyle {
    Solid,
    Spikes(u32),
}

#[derive(Debug, Clone)]
pub struct ChairConfig {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub seat_height: f32,
    pub seat_thickness: f32,
    pub seat_color: [f32; 4],
    pub frame_color: [f32; 4],
    pub back_color: [f32; 4],
    pub leg_thickness: f32,
    pub leg_inset: f32,
    pub back_thickness: f32,
    pub back_width_scale: f32,
    pub back_height: f32,
    pub rear_post_height: f32,
    pub support_rail_height: f32,
    pub support_rail_thickness: f32,
    pub middle_slat_width: f32,
    pub back_style: ChairBackStyle,
}

impl Default for ChairConfig {
    fn default() -> Self {
        Self {
            width: 0.38,
            height: 0.70,
            depth: 0.38,
            seat_height: 0.26,
            seat_thickness: 0.06,
            seat_color: [0.55, 0.40, 0.22, 1.0],
            frame_color: [0.45, 0.32, 0.18, 1.0],
            back_color: [0.55, 0.40, 0.22, 1.0],
            leg_thickness: 0.06,
            leg_inset: 0.0,
            back_thickness: 0.06,
            back_width_scale: 1.0,
            back_height: 0.10,
            rear_post_height: 0.70,
            support_rail_height: 0.0,
            support_rail_thickness: 0.0,
            middle_slat_width: 0.06,
            back_style: ChairBackStyle::Spikes(1),
        }
    }
}

pub fn generate_chair_mesh(w: f32, h: f32, d: f32, config: &ChairConfig) -> MeshData {
    let mut mesh = MeshData::default();

    let actual_w = if w > 0.0 { w } else { config.width };
    let actual_h = if h > 0.0 { h } else { config.height };
    let actual_d = if d > 0.0 { d } else { config.depth };
    let hw = actual_w / 2.0;
    let hd = actual_d / 2.0;

    let seat_h = config.seat_height.clamp(0.05, actual_h * 0.8);
    let seat_t = config.seat_thickness.clamp(0.01, actual_h * 0.25);
    let leg_t = config
        .leg_thickness
        .clamp(0.01, actual_w.min(actual_d) * 0.35);
    let leg_inset = config.leg_inset.max(0.0);
    let leg_x = (hw - leg_inset - leg_t / 2.0).max(0.0);
    let front_z = -hd + leg_inset + leg_t / 2.0;
    let rear_z = hd - leg_inset - leg_t / 2.0;
    let leg_h = seat_h.max(0.01);

    append_colored_box(
        &mut mesh,
        [0.0, seat_h + seat_t / 2.0, 0.0],
        [actual_w, seat_t, actual_d],
        config.seat_color,
    );

    for (lx, lz, post_h) in [
        (-leg_x, front_z, leg_h),
        (leg_x, front_z, leg_h),
        (
            -leg_x,
            rear_z,
            config.rear_post_height.max(leg_h).min(actual_h),
        ),
        (
            leg_x,
            rear_z,
            config.rear_post_height.max(leg_h).min(actual_h),
        ),
    ] {
        append_colored_box(
            &mut mesh,
            [lx, post_h / 2.0, lz],
            [leg_t, post_h, leg_t],
            config.frame_color,
        );
    }

    let rail_h = config.support_rail_height.clamp(0.0, seat_h);
    let rail_t = config
        .support_rail_thickness
        .clamp(0.0, actual_w.min(actual_d) * 0.25);
    if rail_h > 0.0 && rail_t > 0.0 {
        append_colored_box(
            &mut mesh,
            [0.0, rail_h, front_z],
            [(leg_x * 2.0 + leg_t).max(leg_t), rail_t, rail_t],
            config.frame_color,
        );
        append_colored_box(
            &mut mesh,
            [0.0, rail_h, rear_z],
            [(leg_x * 2.0 + leg_t).max(leg_t), rail_t, rail_t],
            config.frame_color,
        );
    }

    let back_h = config.back_height.clamp(0.01, actual_h);
    let back_t = config.back_thickness.clamp(0.01, actual_d * 0.35);
    let back_w = (leg_x * 2.0 + leg_t).max(leg_t);
    let rear_post_actual_h = config.rear_post_height.max(leg_h).min(actual_h);
    let back_center_y = rear_post_actual_h - back_h / 2.0;

    if config.back_style != ChairBackStyle::Solid {
        append_colored_box(
            &mut mesh,
            [0.0, back_center_y, rear_z],
            [back_w, back_h, back_t],
            config.back_color,
        );
    }

    match config.back_style {
        ChairBackStyle::Solid => {
            let solid_h = rear_post_actual_h - (seat_h + seat_t / 2.0);
            if solid_h > 0.0 {
                let solid_center_y = (seat_h + seat_t / 2.0) + solid_h / 2.0;
                append_colored_box(
                    &mut mesh,
                    [0.0, solid_center_y, rear_z],
                    [back_w, solid_h, back_t],
                    config.back_color,
                );
            }
        }
        ChairBackStyle::Spikes(count) => {
            if count > 0 {
                let slat_w = config.middle_slat_width.clamp(0.0, back_w);
                let slat_bottom_y = seat_h + seat_t / 2.0;
                let slat_top_y = rear_post_actual_h - back_h;
                if slat_w > 0.0 && slat_top_y > slat_bottom_y {
                    let slat_h = slat_top_y - slat_bottom_y;
                    let slat_center_y = slat_bottom_y + slat_h / 2.0;
                    
                    let inner_w = back_w - 2.0 * leg_t;
                    let spacing = inner_w / (count as f32 + 1.0);
                    let start_x = -(inner_w / 2.0) + spacing;
                    
                    for i in 0..count {
                        let x = start_x + spacing * (i as f32);
                        append_colored_box(
                            &mut mesh,
                            [x, slat_center_y, rear_z],
                            [slat_w, slat_h, back_t],
                            config.back_color,
                        );
                    }
                }
            }
        }
    }

    mesh
}
