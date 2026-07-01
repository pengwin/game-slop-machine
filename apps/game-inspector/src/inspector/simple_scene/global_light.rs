//! Global light controls for inspector scenes.

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
use game_core::plugins::{global_lighting::GlobalLightControls, inspector::InspectorSceneState};

use super::super::{
    consts::PANEL_FONT_SIZE,
    control_panel::{control_rows, sync_schema_checkboxes, sync_schema_sliders},
    despawn_ui::despawn_ui,
};

#[derive(Component, Clone, Default)]
struct GlobalLightUi;

#[derive(Component, Clone, Default)]
struct ShadowMapSizeCaption;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorSceneState::Simple),
        global_light_ui.spawn(),
    )
    .add_systems(
        Update,
        (
            sync_schema_sliders::<GlobalLightControls>,
            sync_schema_checkboxes::<GlobalLightControls>,
            sync_shadow_map_size_caption,
        ),
    )
    .add_systems(
        OnExit(InspectorSceneState::Simple),
        despawn_ui::<GlobalLightUi>,
    );
}

fn global_light_ui() -> impl SceneList {
    bsn_list![global_light_panel()]
}

fn global_light_panel() -> impl Scene {
    bsn! {
        (
            Name::new("Global Light UI")
            GlobalLightUi
            Node {
                position_type: PositionType::Absolute,
                bottom: px(12),
                left: px(12),
                min_width: px(330),
                max_height: px(600),
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
                (Text("Global Light") ThemedText),
                {control_rows::<GlobalLightControls>(80.0)},
                shadow_map_size_menu(),
            ]
        )
    }
}

fn shadow_map_size_menu() -> impl Scene {
    bsn! {
        (
            @FeathersMenu
            Children [
                (
                    @FeathersMenuButton {
                        @caption: bsn! {
                            Text("2048")
                            ThemedText
                            ShadowMapSizeCaption
                        }
                    }
                    AccessibleLabel("Shadow map size selector")
                    Node {
                        width: percent(100),
                    }
                ),
                (
                    @FeathersMenuPopup
                    Children [
                        shadow_map_size_item(1024),
                        shadow_map_size_item(2048),
                        shadow_map_size_item(4096),
                        shadow_map_size_item(8192),
                    ]
                )
            ]
        )
    }
}

fn shadow_map_size_item(size: usize) -> impl Scene {
    let label = match size {
        1024 => "Shadow map 1024",
        2048 => "Shadow map 2048",
        4096 => "Shadow map 4096",
        8192 => "Shadow map 8192",
        _ => "Shadow map",
    };

    bsn! {
        (
            @FeathersMenuItem {
                @caption: bsn! { Text(label) ThemedText }
            }
            InheritableFont {
                font_size: PANEL_FONT_SIZE,
            }
            on(move |_: On<'_, '_, Activate>, mut controls: ResMut<'_, GlobalLightControls>| {
                controls.shadow_map_size = size;
            })
        )
    }
}

#[allow(clippy::needless_pass_by_value)]
fn sync_shadow_map_size_caption(
    controls: Res<'_, GlobalLightControls>,
    mut caption: Single<'_, '_, &mut Text, With<ShadowMapSizeCaption>>,
) {
    if controls.is_changed() {
        caption.0 = controls.shadow_map_size.to_string();
    }
}
