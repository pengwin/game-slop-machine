use crate::mesh::colored_shapes::append_colored_box;
use crate::mesh::MeshData;

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
}

impl Default for ChairConfig {
    fn default() -> Self {
        Self {
            width: 0.38,
            height: 0.58,
            depth: 0.38,
            seat_height: 0.28,
            seat_thickness: 0.07,
            seat_color: [0.58, 0.42, 0.24, 1.0],
            frame_color: [0.40, 0.28, 0.16, 1.0],
            back_color: [0.50, 0.36, 0.20, 1.0],
            leg_thickness: 0.065,
            leg_inset: 0.045,
            back_thickness: 0.055,
            back_width_scale: 0.72,
            back_height: 0.28,
            rear_post_height: 0.58,
            support_rail_height: 0.08,
            support_rail_thickness: 0.045,
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
    let back_w = (actual_w * config.back_width_scale.clamp(0.2, 1.0)).max(leg_t);
    let back_center_y = (seat_h + seat_t + back_h / 2.0).min(actual_h - back_h / 2.0);

    append_colored_box(
        &mut mesh,
        [0.0, back_center_y, hd + back_t / 2.0],
        [back_w, back_h, back_t],
        config.back_color,
    );

    mesh
}
