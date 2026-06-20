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
}
