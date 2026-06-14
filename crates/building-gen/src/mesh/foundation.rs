use super::math_util::append_quad;
use super::MeshData;
use crate::config::BuildingConfig;

pub fn generate_foundation_mesh(config: &BuildingConfig) -> MeshData {
    let mut mesh = MeshData::default();
    let width = config.foundation_width.max(0.0);
    if width <= f32::EPSILON {
        return mesh;
    }

    let offset = config.foundation_wall_offset.max(0.0);
    let top_y = config.foundation_height.max(0.0);
    let bottom_y = 0.0;
    let inner_min_x = config.footprint.min.x - offset;
    let inner_max_x = config.footprint.max.x + offset;
    let inner_min_z = config.footprint.min.y - offset;
    let inner_max_z = config.footprint.max.y + offset;
    let outer_min_x = inner_min_x - width;
    let outer_max_x = inner_max_x + width;
    let outer_min_z = inner_min_z - width;
    let outer_max_z = inner_max_z + width;

    append_foundation_quad(
        &mut mesh,
        outer_min_x,
        outer_max_x,
        outer_min_z,
        inner_min_z,
        top_y,
    );
    append_foundation_quad(
        &mut mesh,
        outer_min_x,
        outer_max_x,
        inner_max_z,
        outer_max_z,
        top_y,
    );
    append_foundation_quad(
        &mut mesh,
        outer_min_x,
        inner_min_x,
        inner_min_z,
        inner_max_z,
        top_y,
    );
    append_foundation_quad(
        &mut mesh,
        inner_max_x,
        outer_max_x,
        inner_min_z,
        inner_max_z,
        top_y,
    );
    append_foundation_sides(
        &mut mesh,
        outer_min_x,
        outer_max_x,
        outer_min_z,
        outer_max_z,
        bottom_y,
        top_y,
    );

    mesh
}

fn append_foundation_quad(
    mesh: &mut MeshData,
    min_x: f32,
    max_x: f32,
    min_z: f32,
    max_z: f32,
    y: f32,
) {
    if max_x <= min_x || max_z <= min_z {
        return;
    }

    append_quad(
        mesh,
        [min_x, y, max_z],
        [max_x, y, max_z],
        [min_x, y, min_z],
        [max_x, y, min_z],
        [0.0, 1.0, 0.0],
        [min_x, min_z],
        [max_x, max_z],
    );
}

fn append_foundation_sides(
    mesh: &mut MeshData,
    min_x: f32,
    max_x: f32,
    min_z: f32,
    max_z: f32,
    bottom_y: f32,
    top_y: f32,
) {
    append_quad(
        mesh,
        [min_x, top_y, min_z],
        [max_x, top_y, min_z],
        [min_x, bottom_y, min_z],
        [max_x, bottom_y, min_z],
        [0.0, 0.0, -1.0],
        [min_x, bottom_y],
        [max_x, top_y],
    );
    append_quad(
        mesh,
        [max_x, top_y, max_z],
        [min_x, top_y, max_z],
        [max_x, bottom_y, max_z],
        [min_x, bottom_y, max_z],
        [0.0, 0.0, 1.0],
        [min_x, bottom_y],
        [max_x, top_y],
    );
    append_quad(
        mesh,
        [min_x, top_y, max_z],
        [min_x, top_y, min_z],
        [min_x, bottom_y, max_z],
        [min_x, bottom_y, min_z],
        [-1.0, 0.0, 0.0],
        [min_z, bottom_y],
        [max_z, top_y],
    );
    append_quad(
        mesh,
        [max_x, top_y, min_z],
        [max_x, top_y, max_z],
        [max_x, bottom_y, min_z],
        [max_x, bottom_y, max_z],
        [1.0, 0.0, 0.0],
        [min_z, bottom_y],
        [max_z, top_y],
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foundation_width_controls_mesh() {
        let config = BuildingConfig {
            foundation_width: 0.75,
            foundation_wall_offset: 0.25,
            ..Default::default()
        };
        let mesh = generate_foundation_mesh(&config);

        assert_eq!(mesh.vertices.len(), 32);
        assert_eq!(mesh.indices.len(), 48);

        let disabled = BuildingConfig {
            foundation_width: 0.0,
            ..config
        };
        assert!(generate_foundation_mesh(&disabled).is_empty());
    }
}
