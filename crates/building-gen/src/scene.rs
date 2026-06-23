use crate::geometry::Vec3;
use crate::mesh::MeshData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SceneObjectKind {
    Table,
    Chair,
    Bed,
    Shelf,
    Counter,
    Desk,
    Stove,
    Barrel,
    Crate,
    Bench,
    Stool,
}

pub use crate::mesh::SurfaceMaterial as SceneMaterialKind;

#[derive(Debug, Clone)]
pub struct SceneMeshPart {
    pub material: SceneMaterialKind,
    pub color: [f32; 3],
    pub mesh: MeshData,
}

#[derive(Debug, Clone)]
pub struct SceneObject {
    pub position: Vec3,
    pub rotation: f32,
    pub item_type: SceneObjectKind,
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub color: [f32; 3],
    pub mesh: MeshData,
    pub material_parts: Vec<SceneMeshPart>,
}

impl SceneObject {
    pub fn fallback_parts_match_mesh(&self) -> bool {
        let mut merged = MeshData::default();
        for part in &self.material_parts {
            merged.merge_from(&part.mesh);
        }
        merged.vertices.len() == self.mesh.vertices.len()
            && merged.indices.len() == self.mesh.indices.len()
            && merged.normals.len() == self.mesh.normals.len()
            && merged.uvs.len() == self.mesh.uvs.len()
            && merged.surface_materials.len() == self.mesh.surface_materials.len()
    }
}
