use crate::mesh::MeshData;
use crate::mesh::SurfaceMaterial;
use crate::mesh::colored_shapes::{append_material_box, append_material_cylinder};

#[derive(Debug, Clone, PartialEq)]
pub enum ChairBackStyle {
    None,
    Solid,
    Spikes(u32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChairSeatShape {
    Rectangle,
    Round,
}

#[derive(Debug, Clone)]
pub struct ChairConfig {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub seat_height: f32,
    pub seat_thickness: f32,
    pub seat_shape: ChairSeatShape,
    pub leg_count: u32,
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
            seat_shape: ChairSeatShape::Rectangle,
            leg_count: 4,
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

    // Seat
    let seat_y = seat_h + seat_t / 2.0;
    match config.seat_shape {
        ChairSeatShape::Rectangle => {
            append_material_box(
                &mut mesh,
                [0.0, seat_y, 0.0],
                [actual_w, seat_t, actual_d],
                config.seat_color,
                SurfaceMaterial::Wood,
            );
        }
        ChairSeatShape::Round => {
            append_material_cylinder(
                &mut mesh,
                [0.0, seat_y, 0.0],
                actual_w / 2.0,
                actual_d / 2.0,
                seat_t,
                16,
                config.seat_color,
                SurfaceMaterial::Wood,
            );
        }
    }

    let mut leg_positions = Vec::new();
    let leg_h = seat_h; // legs go up to the bottom of the seat
    let leg_x = (hw - leg_inset - leg_t / 2.0).max(0.0);
    let leg_z_rear = hd - leg_inset - leg_t / 2.0;
    let leg_x_amp = leg_x;
    let leg_z_amp = (hd - leg_inset - leg_t / 2.0).max(0.0);

    match config.leg_count {
        1 => {
            // One thick center leg
            append_material_cylinder(
                &mut mesh,
                [0.0, leg_h / 2.0, 0.0],
                leg_t * 0.8,
                leg_t * 0.8,
                leg_h,
                8,
                config.frame_color,
                SurfaceMaterial::Wood,
            );
        }
        2 => {
            // Two legs horizontally aligned, far out
            leg_positions.push((-leg_x_amp, 0.0));
            leg_positions.push((leg_x_amp, 0.0));
        }
        3 => {
            // Triangle: 1 front, 2 rear (120 degree spread)
            // Front: angle = 270 deg
            leg_positions.push((0.0, -leg_z_amp));
            // Rear Right: angle = 30 deg (sin = 0.5, cos = 0.866)
            leg_positions.push((leg_x_amp * 0.866025, leg_z_amp * 0.5));
            // Rear Left: angle = 150 deg
            leg_positions.push((-leg_x_amp * 0.866025, leg_z_amp * 0.5));
        }
        _ => {
            // Standard 4 corners
            leg_positions.push((-leg_x_amp, -leg_z_amp));
            leg_positions.push((leg_x_amp, -leg_z_amp));
            leg_positions.push((-leg_x_amp, leg_z_amp));
            leg_positions.push((leg_x_amp, leg_z_amp));
        }
    }

    // Draw the non-center legs
    for (lx, lz) in leg_positions {
        match config.seat_shape {
            ChairSeatShape::Rectangle => {
                append_material_box(
                    &mut mesh,
                    [lx, leg_h / 2.0, lz],
                    [leg_t, leg_h, leg_t],
                    config.frame_color,
                    SurfaceMaterial::Wood,
                );
            }
            ChairSeatShape::Round => {
                append_material_cylinder(
                    &mut mesh,
                    [lx, leg_h / 2.0, lz],
                    leg_t / 2.0,
                    leg_t / 2.0,
                    leg_h,
                    8,
                    config.frame_color,
                    SurfaceMaterial::Wood,
                );
            }
        }
    }

    // Backrest
    if config.back_style != ChairBackStyle::None {
        let rear_post_actual_h = config.rear_post_height.max(seat_h + seat_t).min(actual_h);
        // Start from center of seat so there is no gap
        let post_bottom = seat_h + seat_t / 2.0;
        let post_h = rear_post_actual_h - post_bottom;
        let post_y = post_bottom + post_h / 2.0;

        let post_z = leg_z_rear;

        for lx in [-leg_x, leg_x] {
            append_material_box(
                &mut mesh,
                [lx, post_y, post_z],
                [leg_t, post_h, leg_t],
                config.frame_color,
                SurfaceMaterial::Wood,
            );
        }

        let back_h = config.back_height.clamp(0.01, actual_h);
        let back_t = config.back_thickness.clamp(0.01, actual_d * 0.35);
        let back_w = (leg_x * 2.0 + leg_t).max(leg_t);
        let back_center_y = rear_post_actual_h - back_h / 2.0;

        match config.back_style {
            ChairBackStyle::Solid => {
                let solid_h = post_h;
                let solid_center_y = post_y;
                append_material_box(
                    &mut mesh,
                    [0.0, solid_center_y, post_z],
                    [back_w, solid_h, back_t],
                    config.back_color,
                    SurfaceMaterial::Wood,
                );
            }
            ChairBackStyle::Spikes(count) => {
                // Top rail
                append_material_box(
                    &mut mesh,
                    [0.0, back_center_y, post_z],
                    [back_w, back_h, back_t],
                    config.back_color,
                    SurfaceMaterial::Wood,
                );

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
                            append_material_box(
                                &mut mesh,
                                [x, slat_center_y, post_z],
                                [slat_w, slat_h, back_t],
                                config.back_color,
                                SurfaceMaterial::Wood,
                            );
                        }
                    }
                }
            }
            ChairBackStyle::None => {}
        }
    }

    mesh
}
