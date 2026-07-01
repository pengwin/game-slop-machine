//! `StandardMaterial` controls for the plaster wall material preview.

use bevy::{
    feathers::{
        font_styles::InheritableFont,
        theme::{ThemeBackgroundColor, ThemedText},
        tokens,
    },
    input_focus::tab_navigation::TabGroup,
    prelude::*,
};
use game_core::plugins::inspector::{InspectorSceneState, PlasterWallMaterialSettings};

use super::super::{
    consts::PANEL_FONT_SIZE,
    control_panel::{control_rows, sync_schema_checkboxes, sync_schema_sliders},
    despawn_ui::despawn_ui,
};

#[derive(Component, Clone, Default)]
struct PlasterWallMaterialSettingsUi;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorSceneState::PlasterWallMaterial),
        material_settings_ui.spawn(),
    )
    .add_systems(
        Update,
        (
            sync_schema_sliders::<PlasterWallMaterialSettings>,
            sync_schema_checkboxes::<PlasterWallMaterialSettings>,
        )
            .run_if(in_state(InspectorSceneState::PlasterWallMaterial)),
    )
    .add_systems(
        OnExit(InspectorSceneState::PlasterWallMaterial),
        despawn_ui::<PlasterWallMaterialSettingsUi>,
    );
}

fn material_settings_ui() -> impl SceneList {
    bsn_list![settings_panel()]
}

fn settings_panel() -> impl Scene {
    bsn! {
        (
            Name::new("Plaster Wall Material Settings UI")
            PlasterWallMaterialSettingsUi
            Node {
                position_type: PositionType::Absolute,
                bottom: px(12),
                right: px(12),
                min_width: px(330),
                max_height: px(300),
                padding: px(8),
                border: px(1),
                border_radius: px(6),
                flex_direction: FlexDirection::Column,
                row_gap: px(4),
                overflow: Overflow::scroll_y(),
            }
            TabGroup
            Pickable::IGNORE
            ThemeBackgroundColor(tokens::MENU_BG)
            InheritableFont {
                font_size: PANEL_FONT_SIZE,
            }
            Children [
                (Text("Material Settings") ThemedText),
                {control_rows::<PlasterWallMaterialSettings>(88.0)},
            ]
        )
    }
}
