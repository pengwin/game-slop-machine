//! Scene selector UI.

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
use game_core::plugins::inspector::InspectorSceneState;

use super::consts::PANEL_FONT_SIZE;

#[derive(Clone, Component, Default)]
struct SceneSelectorCaption;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, scene_selector_ui.spawn())
        .add_systems(Update, update_selector_caption);
}

fn scene_selector_ui() -> impl SceneList {
    bsn_list![selector_root()]
}

fn selector_root() -> impl Scene {
    bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            padding: px(12),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
        }
        TabGroup
        Pickable::IGNORE
        Children [
            (
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: px(8),
                    min_width: px(220),
                    padding: px(10),
                    border: px(1),
                    border_radius: px(6),
                }
                ThemeBackgroundColor(tokens::MENU_BG)
                InheritableFont {
                    font_size: PANEL_FONT_SIZE,
                }
                Children [
                    (Text("Scene") ThemedText),
                    (
                        @FeathersMenu
                        Children [
                            (
                                @FeathersMenuButton {
                                    @caption: bsn! {
                                        Text("None")
                                        ThemedText
                                        SceneSelectorCaption
                                    }
                                }
                                AccessibleLabel("Scene selector")
                                Node {
                                    width: percent(100),
                                }
                            ),
                            (
                                @FeathersMenuPopup
                                Children [
                                    (
                                        @FeathersMenuItem {
                                            @caption: bsn! { Text("None") ThemedText }
                                        }
                                        InheritableFont {
                                            font_size: PANEL_FONT_SIZE,
                                        }
                                        on(|_: On<'_, '_, Activate>, mut next_scene: ResMut<'_, NextState<InspectorSceneState>>| {
                                            next_scene.set(InspectorSceneState::None);
                                        })
                                    ),
                                    (
                                        @FeathersMenuItem {
                                            @caption: bsn! { Text("Simple scene") ThemedText }
                                        }
                                        InheritableFont {
                                            font_size: PANEL_FONT_SIZE,
                                        }
                                        on(|_: On<'_, '_, Activate>, mut next_scene: ResMut<'_, NextState<InspectorSceneState>>| {
                                            next_scene.set(InspectorSceneState::Simple);
                                        })
                                    ),
                                    (
                                        @FeathersMenuItem {
                                            @caption: bsn! { Text("Plaster wall material") ThemedText }
                                        }
                                        InheritableFont {
                                            font_size: PANEL_FONT_SIZE,
                                        }
                                        on(|_: On<'_, '_, Activate>, mut next_scene: ResMut<'_, NextState<InspectorSceneState>>| {
                                            next_scene.set(InspectorSceneState::PlasterWallMaterial);
                                        })
                                    )
                                ]
                            )
                        ]
                    )
                ]
            )
        ]
    }
}

#[allow(clippy::needless_pass_by_value)]
fn update_selector_caption(
    selected: Res<'_, State<InspectorSceneState>>,
    mut caption: Single<'_, '_, &mut Text, With<SceneSelectorCaption>>,
) {
    if selected.is_changed() {
        let new_label = selected.label().to_string();
        caption.0 = new_label;
    }
}
