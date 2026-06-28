//! Camera preset controls for inspector scenes.

use bevy::{
    feathers::{
        controls::{FeathersMenu, FeathersMenuButton, FeathersMenuItem, FeathersMenuPopup},
        font_styles::InheritableFont,
        theme::{ThemeBackgroundColor, ThemedText},
        tokens,
    },
    input_focus::tab_navigation::TabGroup,
    prelude::*,
    ui_widgets::Activate,
};
use game_core::plugins::{global_camera::CameraPreset, inspector::InspectorSceneState};

use super::{consts::PANEL_FONT_SIZE, despawn_ui::despawn_ui};

#[derive(Component, Clone, Default)]
struct CameraPresetsUi;

#[derive(Clone, Component, Default)]
struct CameraPresetCaption;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorSceneState::Simple),
        camera_presets_ui.spawn(),
    )
    .add_systems(Update, update_camera_preset_caption)
    .add_systems(
        OnExit(InspectorSceneState::Simple),
        despawn_ui::<CameraPresetsUi>,
    );
}

fn camera_presets_ui() -> impl SceneList {
    bsn_list![camera_presets_panel()]
}

fn camera_presets_panel() -> impl Scene {
    bsn! {
        (
            Name::new("Camera Presets UI")
            CameraPresetsUi
            Node {
                position_type: PositionType::Absolute,
                top: px(12),
                left: px(244),
                min_width: px(330),
                padding: px(10),
                border: px(1),
                border_radius: px(6),
                flex_direction: FlexDirection::Column,
                row_gap: px(8),
            }
            TabGroup
            Pickable::IGNORE
            ThemeBackgroundColor(tokens::MENU_BG)
            InheritableFont {
                font_size: PANEL_FONT_SIZE,
            }
            Children [
                (Text("Camera Preset") ThemedText),
                (
                    @FeathersMenu
                    Children [
                        (
                            @FeathersMenuButton {
                                @caption: bsn! {
                                    Text("Default game orthographic")
                                    ThemedText
                                    CameraPresetCaption
                                }
                            }
                            AccessibleLabel("Camera preset selector")
                            Node {
                                width: percent(100),
                            }
                        ),
                        (
                            @FeathersMenuPopup
                            Children [
                                camera_preset_item(CameraPreset::DefaultGame),
                                camera_preset_item(CameraPreset::DefaultGamePerspective),
                                camera_preset_item(CameraPreset::DefaultGameIsometricPerspective),
                            ]
                        )
                    ]
                )
            ]
        )
    }
}

fn camera_preset_item(preset: CameraPreset) -> impl Scene {
    let label = preset.label();

    bsn! {
        (
            @FeathersMenuItem {
                @caption: bsn! { Text(label) ThemedText }
            }
            InheritableFont {
                font_size: PANEL_FONT_SIZE,
            }
            on(move |_: On<'_, '_, Activate>, mut selected: ResMut<'_, CameraPreset>| {
                *selected = preset;
            })
        )
    }
}

#[allow(clippy::needless_pass_by_value)]
fn update_camera_preset_caption(
    selected: Res<'_, CameraPreset>,
    mut caption: Single<'_, '_, &mut Text, With<CameraPresetCaption>>,
) {
    if selected.is_changed() {
        caption.0 = selected.label().to_string();
    }
}
