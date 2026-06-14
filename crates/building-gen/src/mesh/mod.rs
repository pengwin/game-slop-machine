//! 3D mesh generation for building tiles.
//!
//! Uses simple box primitives scaled to the appropriate dimensions.
//! This is much simpler than generating custom meshes with cutouts.
//!
//! Each tile type maps to:
//! - A base mesh (unit cube or floor quad)
//! - A scale transform (width, height, depth)
//! - A color/material hint

mod foundation;
mod floor;
mod math_util;
mod opening;
mod primitives;
mod roof;
mod wall;

use crate::config::BuildingConfig;
use crate::layout::RoofGeometry;
use crate::tile::TileGrid;

pub use primitives::{floor_quad, floor_quad_indices, floor_quad_normals, floor_quad_uvs};
pub use primitives::{unit_cube, unit_cube_indices, unit_cube_normals, unit_cube_uvs};
pub use primitives::{tile_color, tile_scale};

/// Raw mesh data (vertices, normals, UVs, indices).
#[derive(Debug, Clone, Default)]
pub struct MeshData {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

impl MeshData {
    pub fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }
}

/// Complete building mesh split by material.
#[derive(Debug, Clone, Default)]
pub struct BuildingMesh {
    pub foundation_mesh: MeshData,
    pub wall_mesh: MeshData,
    pub wall_top_mesh: MeshData,
    pub exterior_wall_mesh: MeshData,
    pub exterior_corner_mesh: MeshData,
    pub exterior_t_junction_mesh: MeshData,
    pub floor_mesh: MeshData,
    pub roof_mesh: MeshData,
    pub gable_mesh: MeshData,
    pub opening_trim_mesh: MeshData,
    pub door_mesh: MeshData,
    pub window_mesh: MeshData,
}

/// Generates a complete merged building mesh from the tile grid.
///
/// Instead of spawning one entity per tile, this produces a single `MeshData`
/// per material category. Exposed faces between wall and empty space are emitted,
/// internal faces between adjacent walls are skipped.
pub fn generate_building_mesh(
    grid: &TileGrid,
    config: &BuildingConfig,
    roof: &RoofGeometry,
) -> BuildingMesh {
    let wall_meshes = wall::generate_wall_meshes(grid, config);
    BuildingMesh {
        foundation_mesh: foundation::generate_foundation_mesh(config),
        wall_mesh: wall_meshes.wall,
        wall_top_mesh: wall_meshes.top,
        exterior_wall_mesh: wall_meshes.exterior,
        exterior_corner_mesh: wall_meshes.exterior_corner,
        exterior_t_junction_mesh: wall_meshes.exterior_t_junction,
        floor_mesh: floor::generate_floor_mesh(grid, config),
        roof_mesh: roof::generate_roof_mesh(config.footprint, roof, config),
        gable_mesh: roof::generate_gable_mesh(config.footprint, roof, config),
        opening_trim_mesh: opening::generate_opening_trim_mesh(grid, config),
        door_mesh: opening::generate_door_mesh(grid, config),
        window_mesh: opening::generate_window_mesh(grid, config),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::BuildingConfig;
    use crate::geometry::Rect;
    use crate::layout::RoofGeometry;

    #[test]
    fn test_generate_building_mesh_not_empty() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 10.0, 8.0),
            ..Default::default()
        };
        let layout = crate::generate_layout(&config, 42);
        let roof = RoofGeometry {
            ridge_start: crate::geometry::Vec3::new(0.0, 5.0, 4.0),
            ridge_end: crate::geometry::Vec3::new(10.0, 5.0, 4.0),
            slope_height: 2.0,
            overhang: 0.5,
        };
        let bmesh = generate_building_mesh(&layout.tile_grid, &config, &roof);

        assert!(!bmesh.wall_mesh.is_empty(), "wall mesh should not be empty");
        assert!(
            !bmesh.foundation_mesh.is_empty(),
            "foundation mesh should not be empty"
        );
        assert!(
            !bmesh.wall_top_mesh.is_empty(),
            "wall top mesh should not be empty"
        );
        assert!(
            !bmesh.floor_mesh.is_empty(),
            "floor mesh should not be empty"
        );
        assert!(!bmesh.roof_mesh.is_empty(), "roof mesh should not be empty");
        assert!(!bmesh.gable_mesh.is_empty(), "gable mesh should not be empty");
    }

    #[test]
    fn test_generate_building_mesh_index_integrity() {
        let config = BuildingConfig {
            footprint: Rect::new(0.0, 0.0, 10.0, 8.0),
            ..Default::default()
        };
        let layout = crate::generate_layout(&config, 42);
        let roof = RoofGeometry {
            ridge_start: crate::geometry::Vec3::new(0.0, 5.0, 4.0),
            ridge_end: crate::geometry::Vec3::new(10.0, 5.0, 4.0),
            slope_height: 2.0,
            overhang: 0.5,
        };
        let bmesh = generate_building_mesh(&layout.tile_grid, &config, &roof);

        for (name, data) in [
            ("foundation", &bmesh.foundation_mesh),
            ("wall", &bmesh.wall_mesh),
            ("wall_top", &bmesh.wall_top_mesh),
            ("exterior_wall", &bmesh.exterior_wall_mesh),
            ("exterior_corner", &bmesh.exterior_corner_mesh),
            ("exterior_t_junction", &bmesh.exterior_t_junction_mesh),
            ("floor", &bmesh.floor_mesh),
            ("roof", &bmesh.roof_mesh),
            ("gable", &bmesh.gable_mesh),
            ("opening_trim", &bmesh.opening_trim_mesh),
            ("door", &bmesh.door_mesh),
            ("window", &bmesh.window_mesh),
        ] {
            if data.is_empty() {
                continue;
            }

            let max_idx = data.indices.iter().copied().max().unwrap_or(0) as usize;
            assert!(
                max_idx < data.vertices.len(),
                "{name}: index {max_idx} out of bounds ({} verts)",
                data.vertices.len()
            );
            assert_eq!(
                data.vertices.len(),
                data.normals.len(),
                "{name}: vertex/normal count mismatch"
            );
            assert_eq!(
                data.vertices.len(),
                data.uvs.len(),
                "{name}: vertex/uv count mismatch"
            );
        }
    }
}
