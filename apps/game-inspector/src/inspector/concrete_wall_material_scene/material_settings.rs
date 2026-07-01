//! `StandardMaterial` controls for the concrete wall material preview.

use bevy::{
    feathers::{
        font_styles::InheritableFont,
        theme::{ThemeBackgroundColor, ThemedText},
        tokens,
    },
    input_focus::tab_navigation::TabGroup,
    prelude::*,
};
use game_core::plugins::inspector::{ConcreteWallMaterialSettings, InspectorSceneState};

use super::super::{
    consts::PANEL_FONT_SIZE,
    control_panel::{control_rows, sync_schema_checkboxes, sync_schema_sliders},
    despawn_ui::despawn_ui,
};

#[derive(Component, Clone, Default)]
struct ConcreteWallMaterialSettingsUi;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorSceneState::ConcreteWallMaterial),
        material_settings_ui.spawn(),
    )
    .add_systems(
        Update,
        (
            sync_schema_sliders::<ConcreteWallMaterialSettings>,
            sync_schema_checkboxes::<ConcreteWallMaterialSettings>,
        )
            .run_if(in_state(InspectorSceneState::ConcreteWallMaterial)),
    )
    .add_systems(
        OnExit(InspectorSceneState::ConcreteWallMaterial),
        despawn_ui::<ConcreteWallMaterialSettingsUi>,
    );
}

fn material_settings_ui() -> impl SceneList {
    bsn_list![settings_panel()]
}

fn settings_panel() -> impl Scene {
    bsn! {
        (
            Name::new("Concrete Wall Material Settings UI")
            ConcreteWallMaterialSettingsUi
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
                {control_rows::<ConcreteWallMaterialSettings>(88.0)},
            ]
        )
    }
}
