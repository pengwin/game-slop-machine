use crate::mesh::MeshData;
use crate::mesh::math_util::{Quad, append_colored_quad, append_colored_triangle};

#[derive(Debug, Clone)]
pub struct BarrelConfig {
    pub max_radius_factor: f32,
    pub cap_radius_factor: f32,
    pub wood_color: [f32; 4],
    pub metal_color: [f32; 4],
    pub cap_color: [f32; 4],
}

impl Default for BarrelConfig {
    fn default() -> Self {
        Self {
            max_radius_factor: 1.25,
            cap_radius_factor: 1.0,
            wood_color: [0.4, 0.28, 0.15, 1.0],
            metal_color: [0.2, 0.2, 0.2, 1.0],
            cap_color: [0.25, 0.15, 0.05, 1.0],
        }
    }
}

pub fn generate_barrel_mesh(diameter: f32, h: f32, config: &BarrelConfig) -> MeshData {
    let mut mesh = MeshData::default();
    let r = diameter / 2.0;
    let sides = 16;
    let mid_r = r * config.max_radius_factor;
    let cap_r = r * config.cap_radius_factor;
    let rim_h = h * 0.08;

    let ring_h = 0.04;
    let ring_extrusion = 0.015;
    let recess_depth = 0.04;
    let recess_r = cap_r - 0.03;

    let wood_color = config.wood_color;
    let metal_color = config.metal_color;
    let cap_color = config.cap_color;

    let get_r = |y: f32| -> f32 {
        let half_body = (h / 2.0) - rim_h;
        let cy = y - h / 2.0;
        let t = cy / half_body;
        r + (mid_r - r) * (1.0 - t * t)
    };

    let mut profile = Vec::new();

    // Bottom recess and outer rim
    profile.push((recess_depth, recess_r, wood_color));
    profile.push((0.0, recess_r, wood_color));
    profile.push((0.0, cap_r, wood_color));
    profile.push((rim_h, r, wood_color));

    // Ring 1
    let y_r1 = h * 0.25;
    profile.push((y_r1 - ring_h / 2.0, get_r(y_r1 - ring_h / 2.0), wood_color));
    profile.push((
        y_r1 - ring_h / 2.0,
        get_r(y_r1) + ring_extrusion,
        metal_color,
    ));
    profile.push((
        y_r1 + ring_h / 2.0,
        get_r(y_r1) + ring_extrusion,
        metal_color,
    ));
    profile.push((y_r1 + ring_h / 2.0, get_r(y_r1 + ring_h / 2.0), wood_color));

    // Ring 2
    let y_r2 = h * 0.5;
    profile.push((y_r2 - ring_h / 2.0, get_r(y_r2 - ring_h / 2.0), wood_color));
    profile.push((
        y_r2 - ring_h / 2.0,
        get_r(y_r2) + ring_extrusion,
        metal_color,
    ));
    profile.push((
        y_r2 + ring_h / 2.0,
        get_r(y_r2) + ring_extrusion,
        metal_color,
    ));
    profile.push((y_r2 + ring_h / 2.0, get_r(y_r2 + ring_h / 2.0), wood_color));

    // Ring 3
    let y_r3 = h * 0.75;
    profile.push((y_r3 - ring_h / 2.0, get_r(y_r3 - ring_h / 2.0), wood_color));
    profile.push((
        y_r3 - ring_h / 2.0,
        get_r(y_r3) + ring_extrusion,
        metal_color,
    ));
    profile.push((
        y_r3 + ring_h / 2.0,
        get_r(y_r3) + ring_extrusion,
        metal_color,
    ));
    profile.push((y_r3 + ring_h / 2.0, get_r(y_r3 + ring_h / 2.0), wood_color));

    // Top rim and recess
    profile.push((h - rim_h, r, wood_color));
    profile.push((h, cap_r, wood_color));
    profile.push((h, recess_r, wood_color));
    profile.push((h - recess_depth, recess_r, wood_color));

    for i in 0..sides {
        let angle0 = std::f32::consts::TAU * i as f32 / sides as f32;
        let angle1 = std::f32::consts::TAU * (i + 1) as f32 / sides as f32;

        let nx = ((angle0 + angle1) / 2.0).cos();
        let nz = ((angle0 + angle1) / 2.0).sin();

        for p in profile.windows(2) {
            let (y0, r0, _) = p[0];
            let (y1, r1, color1) = p[1];

            if (y1 - y0).abs() < 1e-5 && (r1 - r0).abs() < 1e-5 {
                continue;
            }

            let dy = y1 - y0;
            let dr = r1 - r0;

            let mut n_r = dy;
            let mut n_y = -dr;
            let len = (n_r * n_r + n_y * n_y).sqrt();
            if len > 0.0 {
                n_r /= len;
                n_y /= len;
            } else {
                n_r = 1.0;
                n_y = 0.0;
            }

            let norm = [nx * n_r, n_y, nz * n_r];

            let x0_bottom = angle0.cos() * r0;
            let z0_bottom = angle0.sin() * r0;
            let x1_bottom = angle1.cos() * r0;
            let z1_bottom = angle1.sin() * r0;

            let x0_top = angle0.cos() * r1;
            let z0_top = angle0.sin() * r1;
            let x1_top = angle1.cos() * r1;
            let z1_top = angle1.sin() * r1;

            append_colored_quad(
                &mut mesh,
                Quad {
                    tl: [x0_top, y1, z0_top],
                    tr: [x1_top, y1, z1_top],
                    bl: [x0_bottom, y0, z0_bottom],
                    br: [x1_bottom, y0, z1_bottom],
                    normal: norm,
                    uv_min: [0.0, 0.0],
                    uv_max: [1.0, 1.0],
                },
                color1,
            );
        }

        let tx0 = angle0.cos() * recess_r;
        let tz0 = angle0.sin() * recess_r;
        let tx1 = angle1.cos() * recess_r;
        let tz1 = angle1.sin() * recess_r;

        append_colored_triangle(
            &mut mesh,
            [0.0, h - recess_depth, 0.0],
            [tx1, h - recess_depth, tz1],
            [tx0, h - recess_depth, tz0],
            [0.0, 1.0, 0.0],
            cap_color,
        );
        append_colored_triangle(
            &mut mesh,
            [0.0, recess_depth, 0.0],
            [tx0, recess_depth, tz0],
            [tx1, recess_depth, tz1],
            [0.0, -1.0, 0.0],
            cap_color,
        );
    }

    mesh
}
