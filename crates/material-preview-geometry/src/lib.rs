//! Reusable Bevy preview meshes for material inspection.

use bevy::{
    asset::RenderAssetUsages, mesh::Indices, prelude::*,
    render::render_resource::PrimitiveTopology as RenderPrimitiveTopology,
};

const DEFAULT_WALL_HEIGHT: f32 = 2.4;
const DEFAULT_WALL_THICKNESS: f32 = 0.18;
const DEFAULT_WALL_HALF_EXTENT: f32 = 2.5;
const DEFAULT_TOP_OPENING_X: f32 = -0.7;
const DEFAULT_LEFT_WALL_END_Z: f32 = 1.25;
const DEFAULT_TILES_PER_METER: f32 = 0.32;
const DEFAULT_FACE_COLUMNS: u32 = 12;
const DEFAULT_FACE_ROWS: u32 = 8;

/// Static layout settings for the wall preview mesh.
#[derive(Clone, Debug, PartialEq)]
pub struct WallPreviewMeshSettings {
    /// Wall height in world units.
    pub height: f32,
    /// Wall thickness in world units.
    pub thickness: f32,
    /// Half extent of the square room footprint.
    pub half_extent: f32,
    /// X position where one top wall segment opens.
    pub top_opening_x: f32,
    /// Z position where the left wall segment ends.
    pub left_wall_end_z: f32,
}

impl Default for WallPreviewMeshSettings {
    fn default() -> Self {
        Self {
            height: DEFAULT_WALL_HEIGHT,
            thickness: DEFAULT_WALL_THICKNESS,
            half_extent: DEFAULT_WALL_HALF_EXTENT,
            top_opening_x: DEFAULT_TOP_OPENING_X,
            left_wall_end_z: DEFAULT_LEFT_WALL_END_Z,
        }
    }
}

/// Vertex-color dirt settings for the wall preview mesh.
#[derive(Clone, Debug, PartialEq)]
pub struct WallPreviewDirtSettings {
    /// Dirt amount that accumulates upward from the floor.
    pub floor_strength: f32,
    /// Dirt amount that accumulates in wall corner triangles.
    pub corner_strength: f32,
    /// Red multiplier for maximum dirt.
    pub color_r: f32,
    /// Green multiplier for maximum dirt.
    pub color_g: f32,
    /// Blue multiplier for maximum dirt.
    pub color_b: f32,
}

impl Default for WallPreviewDirtSettings {
    fn default() -> Self {
        Self {
            floor_strength: 0.62,
            corner_strength: 0.72,
            color_r: 0.66,
            color_g: 0.44,
            color_b: 0.26,
        }
    }
}

/// UV projection settings for the wall preview mesh.
#[derive(Clone, Debug, PartialEq)]
pub struct WallPreviewUvSettings {
    /// Uses per-face local UVs with deterministic offsets instead of world-space projection.
    pub per_face_offset: bool,
    /// Texture tiles per meter on the preview wall mesh.
    pub tiles_per_meter: f32,
    /// Horizontal subdivisions for each vertical wall face.
    pub face_columns: u32,
    /// Vertical subdivisions for each vertical wall face.
    pub face_rows: u32,
}

impl Default for WallPreviewUvSettings {
    fn default() -> Self {
        Self {
            per_face_offset: false,
            tiles_per_meter: DEFAULT_TILES_PER_METER,
            face_columns: DEFAULT_FACE_COLUMNS,
            face_rows: DEFAULT_FACE_ROWS,
        }
    }
}

/// Builds the material preview wall mesh.
#[must_use]
pub fn build_wall_preview_mesh(
    settings: &WallPreviewMeshSettings,
    dirt_settings: &WallPreviewDirtSettings,
    uv_settings: &WallPreviewUvSettings,
) -> Mesh {
    let mut builder = WallMeshBuilder::new(
        uv_mapping(uv_settings),
        face_columns(uv_settings),
        face_rows(uv_settings),
    );
    let height = settings.height.max(0.1);
    let thickness = settings.thickness.max(0.01);
    let half_thickness = thickness * 0.5;

    let min_x = -settings.half_extent;
    let max_x = settings.half_extent;
    let min_z = -settings.half_extent;
    let max_z = settings.half_extent;
    let top_opening_x = settings.top_opening_x;
    let left_wall_end_z = settings.left_wall_end_z;
    let outer_min_x = min_x - half_thickness;
    let outer_max_x = max_x + half_thickness;
    let outer_min_z = min_z - half_thickness;
    let outer_max_z = max_z + half_thickness;
    let inner_min_x = min_x + half_thickness;
    let inner_max_x = max_x - half_thickness;
    let inner_min_z = min_z + half_thickness;
    let inner_max_z = max_z - half_thickness;

    builder.push_boundary(
        &[
            Vec2::new(outer_min_x, outer_min_z),
            Vec2::new(outer_max_x, outer_min_z),
            Vec2::new(outer_max_x, outer_max_z),
            Vec2::new(top_opening_x, outer_max_z),
            Vec2::new(top_opening_x, inner_max_z),
            Vec2::new(inner_max_x, inner_max_z),
            Vec2::new(inner_max_x, inner_min_z),
            Vec2::new(inner_min_x, inner_min_z),
            Vec2::new(inner_min_x, left_wall_end_z),
            Vec2::new(outer_min_x, left_wall_end_z),
        ],
        height,
        dirt_settings,
    );

    builder.push_top_rect(outer_min_x, outer_max_x, outer_min_z, inner_min_z, height);
    builder.push_top_rect(inner_max_x, outer_max_x, inner_min_z, outer_max_z, height);
    builder.push_top_rect(top_opening_x, inner_max_x, inner_max_z, outer_max_z, height);
    builder.push_top_rect(
        outer_min_x,
        inner_min_x,
        inner_min_z,
        left_wall_end_z,
        height,
    );

    let mut mesh = Mesh::new(
        RenderPrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, builder.positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, builder.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, builder.uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, builder.colors);
    mesh.insert_indices(Indices::U32(builder.indices));
    if let Err(err) = mesh.generate_tangents() {
        warn!("Failed to generate tangents for material preview wall mesh: {err:?}");
    }
    mesh
}

struct WallMeshBuilder {
    uv_mapping: UvMapping,
    face_columns: usize,
    face_rows: usize,
    face_count: usize,
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    colors: Vec<[f32; 4]>,
    indices: Vec<u32>,
}

impl WallMeshBuilder {
    const fn new(uv_mapping: UvMapping, face_columns: usize, face_rows: usize) -> Self {
        Self {
            uv_mapping,
            face_columns,
            face_rows,
            face_count: 0,
            positions: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            colors: Vec::new(),
            indices: Vec::new(),
        }
    }

    fn push_boundary(
        &mut self,
        points: &[Vec2],
        height: f32,
        dirt_settings: &WallPreviewDirtSettings,
    ) {
        for i in 0..points.len() {
            let start = points[i];
            let end = points[(i + 1) % points.len()];
            let edge = end - start;
            let length = edge.length();
            if length <= f32::EPSILON {
                continue;
            }

            let outward = Vec3::new(edge.y, 0.0, -edge.x).normalize_or_zero();
            self.push_face(
                [
                    Vec3::new(start.x, 0.0, start.y),
                    Vec3::new(end.x, 0.0, end.y),
                    Vec3::new(end.x, height, end.y),
                    Vec3::new(start.x, height, start.y),
                ],
                outward,
                [length, height],
                true,
                dirt_settings,
            );
        }
    }

    fn push_top_rect(&mut self, min_x: f32, max_x: f32, min_z: f32, max_z: f32, height: f32) {
        let width = max_x - min_x;
        let depth = max_z - min_z;
        if width <= f32::EPSILON || depth <= f32::EPSILON {
            return;
        }

        self.push_face(
            [
                Vec3::new(min_x, height, min_z),
                Vec3::new(max_x, height, min_z),
                Vec3::new(max_x, height, max_z),
                Vec3::new(min_x, height, max_z),
            ],
            Vec3::Y,
            [width, depth],
            false,
            &WallPreviewDirtSettings::default(),
        );
    }

    fn push_face(
        &mut self,
        corners: [Vec3; 4],
        normal: Vec3,
        uv_size: [f32; 2],
        dirt: bool,
        dirt_settings: &WallPreviewDirtSettings,
    ) {
        let Ok(base) = u32::try_from(self.positions.len()) else {
            unreachable!("wall preview mesh vertex count fits in u32");
        };
        let face_index = self.face_count;
        self.face_count = self.face_count.saturating_add(1);
        let columns = if dirt { self.face_columns } else { 1 };
        let rows = if dirt { self.face_rows } else { 1 };
        let uv_projection = UvProjection::new(self.uv_mapping, normal, uv_size, face_index);
        let normal_array = normal.to_array();

        for row in 0..=rows {
            let local_v = ratio(row, rows);
            for column in 0..=columns {
                let local_u = ratio(column, columns);
                let bottom = corners[0].lerp(corners[1], local_u);
                let top = corners[3].lerp(corners[2], local_u);
                let position = bottom.lerp(top, local_v);

                self.positions.push(position.to_array());
                self.normals.push(normal_array);
                self.uvs.push(uv_projection.uv(position, local_u, local_v));
                self.colors.push(dirt_vertex_color(
                    local_u,
                    height_ratio(position, corners),
                    dirt,
                    dirt_settings,
                ));
            }
        }

        let winding_normal = (corners[1] - corners[0])
            .cross(corners[2] - corners[0])
            .normalize_or_zero();
        let same_winding = winding_normal.dot(Vec3::from_array(normal_array)) >= 0.0;
        let stride = columns + 1;

        for row in 0..rows {
            for column in 0..columns {
                let i0 = base + u32::try_from(row * stride + column).unwrap_or(0);
                let i1 = i0 + 1;
                let i3 = base + u32::try_from((row + 1) * stride + column).unwrap_or(0);
                let i2 = i3 + 1;

                if same_winding {
                    self.indices.extend([i0, i1, i2, i0, i2, i3]);
                } else {
                    self.indices.extend([i0, i2, i1, i0, i3, i2]);
                }
            }
        }
    }
}

const fn uv_mapping(settings: &WallPreviewUvSettings) -> UvMapping {
    let tiles_per_meter = settings.tiles_per_meter.clamp(0.01, 4.0);
    if settings.per_face_offset {
        UvMapping::PerFaceOffset { tiles_per_meter }
    } else {
        UvMapping::World { tiles_per_meter }
    }
}

fn face_columns(settings: &WallPreviewUvSettings) -> usize {
    usize::try_from(settings.face_columns.clamp(1, 96)).unwrap_or(1)
}

fn face_rows(settings: &WallPreviewUvSettings) -> usize {
    usize::try_from(settings.face_rows.clamp(1, 96)).unwrap_or(1)
}

#[derive(Clone, Copy)]
enum UvMapping {
    World { tiles_per_meter: f32 },
    PerFaceOffset { tiles_per_meter: f32 },
}

#[derive(Clone, Copy)]
enum UvProjection {
    WorldXy {
        tiles_per_meter: f32,
    },
    WorldZy {
        tiles_per_meter: f32,
    },
    WorldXz {
        tiles_per_meter: f32,
    },
    PerFace {
        width: f32,
        height: f32,
        offset: [f32; 2],
    },
}

impl UvProjection {
    fn new(uv_mapping: UvMapping, normal: Vec3, uv_size: [f32; 2], face_index: usize) -> Self {
        match uv_mapping {
            UvMapping::World { tiles_per_meter } => Self::world(normal, tiles_per_meter),
            UvMapping::PerFaceOffset { tiles_per_meter } => Self::PerFace {
                width: uv_size[0] * tiles_per_meter,
                height: uv_size[1] * tiles_per_meter,
                offset: face_uv_offset(face_index),
            },
        }
    }

    fn world(normal: Vec3, tiles_per_meter: f32) -> Self {
        if normal.y.abs() > normal.x.abs().max(normal.z.abs()) {
            Self::WorldXz { tiles_per_meter }
        } else if normal.x.abs() > normal.z.abs() {
            Self::WorldZy { tiles_per_meter }
        } else {
            Self::WorldXy { tiles_per_meter }
        }
    }

    fn uv(self, position: Vec3, local_u: f32, local_v: f32) -> [f32; 2] {
        match self {
            Self::WorldXy { tiles_per_meter } => {
                [position.x * tiles_per_meter, position.y * tiles_per_meter]
            }
            Self::WorldZy { tiles_per_meter } => {
                [position.z * tiles_per_meter, position.y * tiles_per_meter]
            }
            Self::WorldXz { tiles_per_meter } => {
                [position.x * tiles_per_meter, position.z * tiles_per_meter]
            }
            Self::PerFace {
                width,
                height,
                offset,
            } => [
                width.mul_add(local_u, offset[0]),
                height.mul_add(local_v, offset[1]),
            ],
        }
    }
}

fn ratio(value: usize, max: usize) -> f32 {
    let value = u16::try_from(value).unwrap_or(0);
    let max = u16::try_from(max.max(1)).unwrap_or(1);
    f32::from(value) / f32::from(max)
}

fn height_ratio(position: Vec3, corners: [Vec3; 4]) -> f32 {
    let min_y = corners
        .iter()
        .map(|corner| corner.y)
        .fold(f32::INFINITY, f32::min);
    let max_y = corners
        .iter()
        .map(|corner| corner.y)
        .fold(f32::NEG_INFINITY, f32::max);
    ((position.y - min_y) / (max_y - min_y).max(0.001)).clamp(0.0, 1.0)
}

fn dirt_vertex_color(
    local_u: f32,
    height_ratio: f32,
    enabled: bool,
    settings: &WallPreviewDirtSettings,
) -> [f32; 4] {
    if !enabled {
        return [1.0, 1.0, 1.0, 1.0];
    }

    let floor_dirt = (1.0 - height_ratio).powf(1.55) * settings.floor_strength;
    let left_corner = corner_triangle_dirt(local_u, height_ratio);
    let right_corner = corner_triangle_dirt(1.0 - local_u, height_ratio);
    let dirt = left_corner
        .max(right_corner)
        .mul_add(settings.corner_strength, floor_dirt)
        .clamp(0.0, 0.86);
    [
        lerp(1.0, settings.color_r, dirt),
        lerp(1.0, settings.color_g, dirt),
        lerp(1.0, settings.color_b, dirt),
        1.0,
    ]
}

fn corner_triangle_dirt(distance_from_edge: f32, height_ratio: f32) -> f32 {
    let horizontal = distance_from_edge / 0.38;
    let vertical = height_ratio / 0.72;
    (1.0 - horizontal - vertical).clamp(0.0, 1.0).powf(1.4)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (b - a).mul_add(t, a)
}

fn face_uv_offset(face_index: usize) -> [f32; 2] {
    let mut hash = u32::try_from(face_index).unwrap_or(0);
    hash ^= hash >> 16;
    hash = hash.wrapping_mul(0x7FEB_352D);
    hash ^= hash >> 15;
    hash = hash.wrapping_mul(0x846C_A68B);
    hash ^= hash >> 16;

    let u_bits = u16::try_from(hash & 0xFFFF).unwrap_or(0);
    let v_bits = u16::try_from(hash >> 16).unwrap_or(0);
    let u = (f32::from(u_bits) / f32::from(u16::MAX)) * 3.0;
    let v = (f32::from(v_bits) / f32::from(u16::MAX)) * 3.0;
    [u, v]
}
