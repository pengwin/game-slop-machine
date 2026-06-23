use bevy::prelude::*;
use building_gen::furniture::{self, FurnitureType};
use game_core::plugins::building::mesh_util::convert_mesh;
use game_core::plugins::building::procedural_texture::ProceduralTextures;
use game_core::plugins::building::render::scene_part_material;

pub fn spawn_furniture_preview(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    textures: &mut ProceduralTextures,
    images: &mut Assets<Image>,
    fixture: &str,
) {
    let items: Vec<furniture::FurnitureItem> = match fixture {
        "table" => vec![furniture::single_item(FurnitureType::Table)],
        "chair" => vec![furniture::single_item(FurnitureType::Chair)],
        "bed" => vec![furniture::single_item(FurnitureType::Bed)],
        "stove" => vec![furniture::single_item(FurnitureType::Stove)],
        "counter" => vec![furniture::single_item(FurnitureType::Counter)],
        "desk" => vec![furniture::single_item(FurnitureType::Desk)],
        "barrel" => vec![furniture::single_item(FurnitureType::Barrel)],
        "crate" => vec![furniture::single_item(FurnitureType::Crate)],
        "bench" => vec![furniture::single_item(FurnitureType::Bench)],
        "shelf" => vec![furniture::single_item(FurnitureType::Shelf)],
        "stool" => vec![furniture::single_item(FurnitureType::Stool)],
        "all-furniture" => vec![
            furniture::single_item(FurnitureType::Table),
            furniture::single_item(FurnitureType::Chair),
            furniture::single_item(FurnitureType::Bed),
            furniture::single_item(FurnitureType::Stove),
            furniture::single_item(FurnitureType::Counter),
            furniture::single_item(FurnitureType::Desk),
            furniture::single_item(FurnitureType::Barrel),
            furniture::single_item(FurnitureType::Crate),
            furniture::single_item(FurnitureType::Bench),
            furniture::single_item(FurnitureType::Shelf),
            furniture::single_item(FurnitureType::Stool),
        ],
        _ => vec![furniture::single_item(FurnitureType::Table)],
    };

    // Spawn items in a row
    let spacing = 1.2;
    let total_width = (items.len() as f32 - 1.0) * spacing;
    let start_x = -total_width / 2.0;

    for (i, item) in items.iter().enumerate() {
        let x = start_x + i as f32 * spacing;

        if !item.material_parts.is_empty() {
            for (part_i, part) in item.material_parts.iter().enumerate() {
                commands.spawn((
                    Mesh3d(meshes.add(convert_mesh(&part.mesh))),
                    MeshMaterial3d(materials.add(scene_part_material(
                        part,
                        textures,
                        images,
                        (i + part_i) as u32,
                    ))),
                    Transform {
                        translation: Vec3::new(x, 0.0, 0.0),
                        rotation: Quat::from_rotation_y(item.rotation),
                        ..default()
                    },
                    Name::new(format!("{:?} Part {}", item.item_type, part_i)),
                ));
            }
        } else if !item.mesh.is_empty() {
            commands.spawn((
                Mesh3d(meshes.add(convert_mesh(&item.mesh))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: if item.mesh.colors.is_empty() {
                        Color::srgb(item.color[0], item.color[1], item.color[2])
                    } else {
                        Color::WHITE
                    },
                    perceptual_roughness: 0.85,
                    ..default()
                })),
                Transform {
                    translation: Vec3::new(x, 0.0, 0.0),
                    rotation: Quat::from_rotation_y(item.rotation),
                    ..default()
                },
                Name::new(format!("{:?}", item.item_type)),
            ));
        }

        // Label
        println!(
            "  [{:?}] {:.2} x {:.2} x {:.2}",
            item.item_type, item.width, item.height, item.depth
        );
    }

    println!("Spawned {} furniture items", items.len());
}
