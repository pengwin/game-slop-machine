use crate::config::BuildingConfig;
use crate::geometry::{Axis, Rect, Vec3};
use crate::layout::RoofGeometry;

pub fn generate_roof(bounds: Rect, config: &BuildingConfig) -> RoofGeometry {
    let center = bounds.center();
    let width = bounds.width();
    let depth = bounds.height();

    let (ridge_axis, _ridge_length) = if width >= depth {
        (Axis::Horizontal, width)
    } else {
        (Axis::Vertical, depth)
    };

    let overhang = config.roof_overhang;

    match ridge_axis {
        Axis::Horizontal => {
            let ridge_y = center.y;
            RoofGeometry {
                ridge_start: Vec3::new(
                    bounds.min.x - overhang,
                    config.wall_height + config.roof_height,
                    ridge_y,
                ),
                ridge_end: Vec3::new(
                    bounds.max.x + overhang,
                    config.wall_height + config.roof_height,
                    ridge_y,
                ),
                slope_height: config.roof_height,
                overhang,
            }
        }
        Axis::Vertical => {
            let ridge_x = center.x;
            RoofGeometry {
                ridge_start: Vec3::new(
                    ridge_x,
                    config.wall_height + config.roof_height,
                    bounds.min.y - overhang,
                ),
                ridge_end: Vec3::new(
                    ridge_x,
                    config.wall_height + config.roof_height,
                    bounds.max.y + overhang,
                ),
                slope_height: config.roof_height,
                overhang,
            }
        }
    }
}

pub fn generate_roof_vertices(
    bounds: Rect,
    _roof: &RoofGeometry,
    config: &BuildingConfig,
) -> Vec<[f32; 3]> {
    let overhang = config.roof_overhang;
    let wall_h = config.wall_height;

    let min_x = bounds.min.x - overhang;
    let max_x = bounds.max.x + overhang;
    let min_z = bounds.min.y - overhang;
    let max_z = bounds.max.y + overhang;

    let center_y = (bounds.min.y + bounds.max.y) / 2.0;
    let ridge_y = wall_h + config.roof_height;

    vec![
        [min_x, wall_h, min_z],
        [max_x, wall_h, min_z],
        [max_x, ridge_y, center_y],
        [min_x, ridge_y, center_y],
        [min_x, wall_h, max_z],
        [max_x, wall_h, max_z],
    ]
}

pub fn generate_roof_indices() -> Vec<u32> {
    vec![0, 1, 2, 0, 2, 3, 1, 5, 4, 1, 4, 2, 4, 5, 2, 4, 2, 3]
}

pub fn generate_roof_normals() -> Vec<[f32; 3]> {
    vec![
        [0.0, 0.5, -0.866],
        [0.0, 0.5, -0.866],
        [0.0, 0.5, -0.866],
        [0.0, 0.5, 0.866],
        [0.0, 0.5, 0.866],
        [0.0, 0.5, 0.866],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> BuildingConfig {
        BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 10.0, 8.0),
            wall_height: 3.0,
            roof_height: 2.0,
            roof_overhang: 0.5,
            ..Default::default()
        }
    }

    #[test]
    fn test_roof_geometry() {
        let config = test_config();
        let roof = generate_roof(config.footprint, &config);

        assert!(roof.slope_height > 0.0);
        assert!(roof.overhang > 0.0);
    }

    #[test]
    fn test_roof_vertices_count() {
        let config = test_config();
        let roof = generate_roof(config.footprint, &config);
        let vertices = generate_roof_vertices(config.footprint, &roof, &config);

        assert_eq!(vertices.len(), 6);
    }

    #[test]
    fn test_roof_indices_count() {
        let indices = generate_roof_indices();
        assert_eq!(indices.len(), 18);
    }
}
