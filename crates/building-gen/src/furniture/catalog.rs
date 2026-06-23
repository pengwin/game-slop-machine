use super::barrel::BarrelConfig;
use super::bed::BedConfig;
use super::chair::ChairConfig;
use super::counter::CounterConfig;
use super::shelf::ShelfConfig;
use super::table::TableConfig;
use super::{FurnitureItem, FurnitureType};
use crate::mesh::MeshData;
use crate::mesh::SurfaceMaterial;
use crate::scene::SceneMeshPart;

pub fn single_item(item_type: FurnitureType) -> FurnitureItem {
    use crate::geometry::Vec3;

    let (w, h, d, color, mesh) = match item_type {
        FurnitureType::Table => {
            let table_config = TableConfig::default();
            let (w, h, d) = (table_config.width, table_config.height, table_config.depth);
            (
                w,
                h,
                d,
                [0.6, 0.45, 0.25],
                super::table::generate_table_mesh(w, h, d, &table_config),
            )
        }
        FurnitureType::Chair => {
            let chair_config = ChairConfig::default();
            let (w, h, d) = (chair_config.width, chair_config.height, chair_config.depth);
            (
                w,
                h,
                d,
                [0.5, 0.35, 0.2],
                super::chair::generate_chair_mesh(w, h, d, &chair_config),
            )
        }
        FurnitureType::Bed => {
            let (w, h, d) = (1.0, 0.45, 0.9);
            (
                w,
                h,
                d,
                [0.9, 0.9, 0.85],
                super::bed::generate_bed_mesh(w, h, d, &BedConfig::default()),
            )
        }
        FurnitureType::Stove => {
            let (w, h, d) = (1.4, 2.5, 0.8);
            (
                w,
                h,
                d,
                [0.25, 0.25, 0.25],
                super::stove::generate_stove_mesh(w, h, d, &super::stove::StoveConfig::default()),
            )
        }
        FurnitureType::Counter => {
            let (w, h, d) = (0.9, 0.9, 0.5);
            (
                w,
                h,
                d,
                [0.55, 0.4, 0.25],
                super::counter::generate_counter_mesh(w, h, d, &CounterConfig::default()),
            )
        }
        FurnitureType::Desk => {
            let (w, h, d) = (0.7, 0.75, 0.45);
            (
                w,
                h,
                d,
                [0.5, 0.35, 0.2],
                super::desk::generate_desk_mesh(w, h, d, [0.5, 0.35, 0.2]),
            )
        }
        FurnitureType::Barrel => {
            let (d, h) = (0.4, 0.6);
            (
                d,
                h,
                d,
                [0.4, 0.28, 0.15],
                super::barrel::generate_barrel_mesh(d, h, &BarrelConfig::default()),
            )
        }
        FurnitureType::Crate => {
            let (w, h, d) = (0.5, 0.5, 0.5);
            (
                w,
                h,
                d,
                [0.65, 0.55, 0.35],
                super::crate_mesh::generate_crate_mesh(w, h, d, [0.65, 0.55, 0.35]),
            )
        }
        FurnitureType::Bench => {
            let (w, h, d) = (0.8, 0.45, 0.35);
            (
                w,
                h,
                d,
                [0.45, 0.32, 0.18],
                super::bench::generate_bench_mesh(w, h, d, [0.45, 0.32, 0.18]),
            )
        }
        FurnitureType::Shelf => {
            let (w, h, d) = (0.6, 1.2, 0.3);
            (
                w,
                h,
                d,
                [0.5, 0.35, 0.2],
                super::shelf::generate_shelf_mesh(w, h, d, &ShelfConfig::default()),
            )
        }
        FurnitureType::Stool => {
            let stool_config = super::chair::ChairConfig {
                seat_shape: super::chair::ChairSeatShape::Round,
                leg_count: 3,
                back_style: super::chair::ChairBackStyle::None,
                width: 0.35,
                depth: 0.35,
                height: 0.45,
                seat_height: 0.40,
                ..super::chair::ChairConfig::default()
            };
            let (w, h, d) = (stool_config.width, stool_config.height, stool_config.depth);
            (
                w,
                h,
                d,
                [0.5, 0.35, 0.2],
                super::chair::generate_chair_mesh(w, h, d, &stool_config),
            )
        }
    };

    let rotation = if matches!(item_type, FurnitureType::Stove) {
        std::f32::consts::PI
    } else {
        0.0
    };

    let material_parts = material_parts_for_item(item_type, &mesh, color);

    FurnitureItem {
        position: Vec3::ZERO,
        rotation,
        item_type,
        width: w,
        height: h,
        depth: d,
        color,
        mesh,
        material_parts,
    }
}

fn material_parts_for_item(
    _item_type: FurnitureType,
    mesh: &MeshData,
    fallback_color: [f32; 3],
) -> Vec<SceneMeshPart> {
    if mesh.is_empty() {
        return Vec::new();
    }

    let has_colors = mesh.colors.len() == mesh.vertices.len();
    let mut parts: Vec<SceneMeshPart> = Vec::new();
    let mut part_vertex_maps: Vec<Vec<(usize, u32)>> = Vec::new();

    for (tri_i, tri) in mesh.indices.chunks_exact(3).enumerate() {
        let color = if has_colors {
            mesh.colors[tri[0] as usize]
        } else {
            [fallback_color[0], fallback_color[1], fallback_color[2], 1.0]
        };
        let material = mesh
            .surface_materials
            .get(tri_i)
            .copied()
            .unwrap_or(SurfaceMaterial::Colored);
        let rgb = [color[0], color[1], color[2]];
        let part_index = parts
            .iter()
            .position(|part| part.material == material && close_rgb(part.color, rgb))
            .unwrap_or_else(|| {
                parts.push(SceneMeshPart {
                    material,
                    color: rgb,
                    mesh: MeshData::default(),
                });
                part_vertex_maps.push(Vec::new());
                parts.len() - 1
            });
        push_triangle(
            &mut parts[part_index].mesh,
            &mut part_vertex_maps[part_index],
            mesh,
            tri,
            material,
        );
    }

    parts
}

fn push_triangle(
    out: &mut MeshData,
    vertex_map: &mut Vec<(usize, u32)>,
    source: &MeshData,
    tri: &[u32],
    material: SurfaceMaterial,
) {
    let mut out_indices = [0; 3];
    for (out_slot, idx) in out_indices.iter_mut().zip(tri) {
        let source_index = *idx as usize;
        if let Some((_, mapped_index)) = vertex_map
            .iter()
            .find(|(mapped_source, _)| *mapped_source == source_index)
        {
            *out_slot = *mapped_index;
            continue;
        }

        let new_index = out.vertices.len() as u32;
        vertex_map.push((source_index, new_index));
        out.vertices.push(source.vertices[source_index]);
        out.normals.push(source.normals[source_index]);
        out.uvs.push(source.uvs[source_index]);
        *out_slot = new_index;
    }
    out.indices.extend(out_indices);
    out.surface_materials.push(material);
}

fn close_rgb(a: [f32; 3], b: [f32; 3]) -> bool {
    (a[0] - b[0]).abs() < 0.002 && (a[1] - b[1]).abs() < 0.002 && (a[2] - b[2]).abs() < 0.002
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn material_parts_merge_back_to_fallback_mesh_shape() {
        for item_type in [
            FurnitureType::Table,
            FurnitureType::Chair,
            FurnitureType::Bed,
            FurnitureType::Shelf,
            FurnitureType::Counter,
            FurnitureType::Desk,
            FurnitureType::Stove,
            FurnitureType::Barrel,
            FurnitureType::Crate,
            FurnitureType::Bench,
            FurnitureType::Stool,
        ] {
            let item = single_item(item_type);
            assert!(
                !item.material_parts.is_empty(),
                "{item_type:?} should expose material-tagged parts"
            );
            assert!(
                item.fallback_parts_match_mesh(),
                "{item_type:?} material parts should preserve fallback mesh shape"
            );
        }
    }

    #[test]
    fn bed_blanket_is_fabric_not_wood() {
        let item = single_item(FurnitureType::Bed);
        let blanket = item
            .material_parts
            .iter()
            .find(|part| close_rgb(part.color, [0.65, 0.35, 0.25]))
            .expect("bed should expose blanket color as a material part");

        assert_eq!(blanket.material, SurfaceMaterial::Fabric);
    }

    #[test]
    fn material_parts_use_surface_tags_not_color_guessing() {
        let bed = single_item(FurnitureType::Bed);
        assert!(
            bed.material_parts
                .iter()
                .any(|part| part.material == SurfaceMaterial::Wood)
        );
        assert!(
            bed.material_parts
                .iter()
                .any(|part| part.material == SurfaceMaterial::Fabric)
        );
        assert!(
            bed.material_parts
                .iter()
                .filter(|part| part.material == SurfaceMaterial::Fabric)
                .all(|part| part.material != SurfaceMaterial::Wood)
        );

        let table = single_item(FurnitureType::Table);
        assert!(
            table
                .material_parts
                .iter()
                .all(|part| part.material == SurfaceMaterial::Wood)
        );

        let counter = single_item(FurnitureType::Counter);
        for material in [
            SurfaceMaterial::Wood,
            SurfaceMaterial::Stone,
            SurfaceMaterial::Metal,
        ] {
            assert!(
                counter
                    .material_parts
                    .iter()
                    .any(|part| part.material == material),
                "counter should expose {material:?} material parts"
            );
        }
    }
}
