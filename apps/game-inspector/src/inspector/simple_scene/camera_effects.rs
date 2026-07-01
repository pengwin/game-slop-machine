//! Camera effects controls for inspector scenes.

use bevy::{
    feathers::{
        font_styles::InheritableFont,
        theme::{ThemeBackgroundColor, ThemedText},
        tokens,
    },
    input_focus::tab_navigation::TabGroup,
    prelude::*,
};
use game_core::plugins::{global_camera::CameraEffects, inspector::InspectorSceneState};

use super::super::{
    consts::PANEL_FONT_SIZE,
    control_panel::{control_rows, sync_schema_checkboxes},
    despawn_ui::despawn_ui,
};

#[derive(Component, Clone, Default)]
struct CameraEffectsUi;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorSceneState::Simple),
        camera_effects_ui.spawn(),
    )
    .add_systems(Update, sync_schema_checkboxes::<CameraEffects>)
    .add_systems(
        OnExit(InspectorSceneState::Simple),
        despawn_ui::<CameraEffectsUi>,
    );
}

fn camera_effects_ui() -> impl SceneList {
    bsn_list![camera_effects_panel()]
}

fn camera_effects_panel() -> impl Scene {
    bsn! {
        (
            Name::new("Camera Effects UI")
            CameraEffectsUi
            Node {
                position_type: PositionType::Absolute,
                top: px(112),
                left: px(12),
                min_width: px(330),
                padding: px(8),
                border: px(1),
                border_radius: px(6),
                flex_direction: FlexDirection::Column,
                row_gap: px(4),
            }
            TabGroup
            Pickable::IGNORE
            ThemeBackgroundColor(tokens::MENU_BG)
            InheritableFont {
                font_size: PANEL_FONT_SIZE,
            }
            Children [
                (Text("Camera Effects") ThemedText),
                {control_rows::<CameraEffects>(88.0)},
            ]
        )
    }
}
