use crate::fixtures;
use bevy::prelude::*;
use building_gen::mesh::MeshData;
use building_gen::mesh::colored_shapes::append_colored_box;
use building_gen::mesh::math_util::{Quad, append_quad};
use game_core::plugins::building::mesh_util::convert_mesh;
use game_core::plugins::building::procedural_texture::ProceduralTextures;
use game_core::plugins::building::render::{
    brick_material, concrete_material, floor_tile_material, plaster_material, road_material,
    roof_tile_material, spawn_building_layout, stone_material, wood_material,
};

pub fn spawn_texture_preview(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    fixture: &str,
) {
    println!("Spawning texture preview fixture: {}", fixture);
    if fixture != "texture-plaster-wall" {
        commands.spawn((
            Name::new("Texture Fixture Side Light"),
            PointLight {
                color: Color::srgb(1.0, 0.92, 0.78),
                intensity: 950.0,
                range: 6.0,
                shadow_maps_enabled: true,
                ..default()
            },
            Transform::from_xyz(-1.8, 2.2, 2.0),
        ));
    }

    match fixture {
        "texture-plaster-wall" => spawn_plaster_wall(commands, meshes, materials, textures, images),
        "texture-wood-table" => spawn_wood_table(commands, meshes, materials, textures, images),
        "texture-material-board" => {
            spawn_material_board(commands, meshes, materials, textures, images)
        }
        _ => spawn_material_board(commands, meshes, materials, textures, images),
    }
}

fn spawn_plaster_wall(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
) {
    let config = fixtures::config_for_fixture("picture-room");
    let layout = building_gen::generate_layout(&config);
    spawn_building_layout(
        commands,
        meshes,
        materials,
        textures,
        images,
        &config,
        &layout,
        Transform::default(),
        "Texture Picture Room Corner",
    );
}

fn spawn_wood_table(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
) {
    let table = textured_table_mesh();
    commands.spawn((
        Mesh3d(meshes.add(convert_mesh(&table))),
        MeshMaterial3d(materials.add(wood_material(
            Color::srgb(0.78, 0.56, 0.34),
            textures,
            images,
            0,
        ))),
        Transform::default(),
        Name::new("Texture Wood Table"),
    ));

    let detail = tabletop_plank_detail_mesh(1.25, 0.72, 0.78);
    commands.spawn((
        Mesh3d(meshes.add(convert_mesh(&detail))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::default(),
        Name::new("Texture Wood Table Plank Detail"),
    ));
}

fn spawn_material_board(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
) {
    let swatches = [
        (
            "Plaster",
            plaster_material(Color::srgb(0.86, 0.82, 0.72), textures, images, 0),
        ),
        (
            "Wood",
            wood_material(Color::srgb(0.78, 0.56, 0.34), textures, images, 0),
        ),
        (
            "Floor",
            floor_tile_material(Color::srgb(0.62, 0.57, 0.47), textures, images, 0),
        ),
        (
            "Brick",
            brick_material(Color::srgb(0.92, 0.72, 0.58), textures, images, 0),
        ),
        (
            "Roof",
            roof_tile_material(Color::srgb(0.78, 0.36, 0.24), textures, images, 0),
        ),
        (
            "Stone",
            stone_material(Color::srgb(0.68, 0.66, 0.58), textures, images, 0),
        ),
        (
            "Road",
            road_material(Color::srgb(0.62, 0.52, 0.40), textures, images, 0),
        ),
        (
            "Concrete",
            concrete_material(Color::srgb(0.68, 0.66, 0.62), textures, images, 0),
        ),
    ];

    let width = 0.82;
    let height = 0.82;
    let gap = 0.16;
    let total = swatches.len() as f32 * width + (swatches.len() as f32 - 1.0) * gap;
    let start = -total * 0.5 + width * 0.5;

    for (i, (name, mut material)) in swatches.into_iter().enumerate() {
        let mesh = vertical_quad_mesh(width, height, 0.45, 0.0);
        material.cull_mode = None;
        material.unlit = true;
        commands.spawn((
            Mesh3d(meshes.add(convert_mesh(&mesh))),
            MeshMaterial3d(materials.add(material)),
            Transform::from_xyz(start + i as f32 * (width + gap), 0.0, 0.0),
            Name::new(format!("Texture {} Swatch", name)),
        ));
    }
}

fn vertical_quad_mesh(width: f32, height: f32, repeat_meters: f32, z: f32) -> MeshData {
    let mut mesh = MeshData::default();
    let hw = width * 0.5;
    append_quad(
        &mut mesh,
        Quad {
            tl: [-hw, height, z],
            tr: [hw, height, z],
            bl: [-hw, 0.0, z],
            br: [hw, 0.0, z],
            normal: [0.0, 0.0, 1.0],
            uv_min: [0.0, 0.0],
            uv_max: [width / repeat_meters, height / repeat_meters],
        },
    );
    debug_assert_eq!(mesh.vertices.len(), mesh.uvs.len());
    mesh
}

fn textured_table_mesh() -> MeshData {
    let mut mesh = MeshData::default();
    append_textured_box(
        &mut mesh,
        [0.0, 0.66, 0.0],
        [1.25, 0.12, 0.78],
        [0.36, 0.24, 0.36],
    );
    append_textured_box(
        &mut mesh,
        [0.0, 0.57, 0.0],
        [0.98, 0.07, 0.52],
        [0.30, 0.24, 0.30],
    );

    let leg_x = 0.47;
    let leg_z = 0.26;
    for (x, z) in [
        (-leg_x, -leg_z),
        (leg_x, -leg_z),
        (-leg_x, leg_z),
        (leg_x, leg_z),
    ] {
        append_textured_box(
            &mut mesh,
            [x, 0.27, z],
            [0.12, 0.54, 0.12],
            [0.18, 0.28, 0.18],
        );
    }

    debug_assert_eq!(mesh.vertices.len(), mesh.uvs.len());
    mesh
}

fn tabletop_plank_detail_mesh(width: f32, top_y: f32, depth: f32) -> MeshData {
    let mut mesh = MeshData::default();
    let groove_color = [0.12, 0.07, 0.035, 1.0];
    let groove_w = (width * 0.014).clamp(0.006, 0.014);
    let groove_h = 0.008;
    let y = top_y + groove_h * 0.5 + 0.004;

    for x in [-width * 0.25, 0.0, width * 0.25] {
        append_colored_box(
            &mut mesh,
            [x, y, 0.0],
            [groove_w, groove_h, depth * 0.90],
            groove_color,
        );
    }

    mesh
}

fn append_textured_box(mesh: &mut MeshData, center: [f32; 3], size: [f32; 3], repeat: [f32; 3]) {
    let [cx, cy, cz] = center;
    let [sx, sy, sz] = size;
    let hw = sx * 0.5;
    let hh = sy * 0.5;
    let hd = sz * 0.5;
    let rx = repeat[0].max(0.001);
    let ry = repeat[1].max(0.001);
    let rz = repeat[2].max(0.001);

    append_quad(
        mesh,
        Quad {
            tl: [cx - hw, cy + hh, cz + hd],
            tr: [cx + hw, cy + hh, cz + hd],
            bl: [cx - hw, cy + hh, cz - hd],
            br: [cx + hw, cy + hh, cz - hd],
            normal: [0.0, 1.0, 0.0],
            uv_min: [0.0, 0.0],
            uv_max: [sx / rx, sz / rz],
        },
    );
    append_quad(
        mesh,
        Quad {
            tl: [cx - hw, cy - hh, cz - hd],
            tr: [cx + hw, cy - hh, cz - hd],
            bl: [cx - hw, cy - hh, cz + hd],
            br: [cx + hw, cy - hh, cz + hd],
            normal: [0.0, -1.0, 0.0],
            uv_min: [0.0, 0.0],
            uv_max: [sx / rx, sz / rz],
        },
    );
    append_quad(
        mesh,
        Quad {
            tl: [cx - hw, cy + hh, cz - hd],
            tr: [cx + hw, cy + hh, cz - hd],
            bl: [cx - hw, cy - hh, cz - hd],
            br: [cx + hw, cy - hh, cz - hd],
            normal: [0.0, 0.0, -1.0],
            uv_min: [0.0, 0.0],
            uv_max: [sx / rx, sy / ry],
        },
    );
    append_quad(
        mesh,
        Quad {
            tl: [cx + hw, cy + hh, cz + hd],
            tr: [cx - hw, cy + hh, cz + hd],
            bl: [cx + hw, cy - hh, cz + hd],
            br: [cx - hw, cy - hh, cz + hd],
            normal: [0.0, 0.0, 1.0],
            uv_min: [0.0, 0.0],
            uv_max: [sx / rx, sy / ry],
        },
    );
    append_quad(
        mesh,
        Quad {
            tl: [cx - hw, cy + hh, cz + hd],
            tr: [cx - hw, cy + hh, cz - hd],
            bl: [cx - hw, cy - hh, cz + hd],
            br: [cx - hw, cy - hh, cz - hd],
            normal: [-1.0, 0.0, 0.0],
            uv_min: [0.0, 0.0],
            uv_max: [sz / rz, sy / ry],
        },
    );
    append_quad(
        mesh,
        Quad {
            tl: [cx + hw, cy + hh, cz - hd],
            tr: [cx + hw, cy + hh, cz + hd],
            bl: [cx + hw, cy - hh, cz - hd],
            br: [cx + hw, cy - hh, cz + hd],
            normal: [1.0, 0.0, 0.0],
            uv_min: [0.0, 0.0],
            uv_max: [sz / rz, sy / ry],
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn textured_table_uvs_are_box_projected() {
        let mesh = textured_table_mesh();
        assert_eq!(mesh.vertices.len(), mesh.uvs.len());
        assert!(mesh.uvs.iter().any(|uv| uv[0] > 1.0 || uv[1] > 1.0));
    }

    #[test]
    fn plaster_wall_uses_meter_scaled_uvs() {
        let mesh = vertical_quad_mesh(2.4, 1.7, 1.5, 0.0);
        assert_eq!(mesh.vertices.len(), mesh.uvs.len());
        assert!(mesh.uvs.iter().any(|uv| (uv[0] - 1.6).abs() < 0.001));
    }
}
