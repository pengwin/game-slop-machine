//! Scene selector UI.

use bevy::{
    feathers::{
        controls::{FeathersMenu, FeathersMenuButton, FeathersMenuItem, FeathersMenuPopup},
        theme::{ThemeBackgroundColor, ThemedText},
        tokens,
    },
    input_focus::tab_navigation::TabGroup,
    prelude::*,
    ui_widgets::Activate,
};
use game_core::plugins::inspector_scene::InspectorScene;

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
                                        on(|_: On<'_, '_, Activate>, mut next_scene: ResMut<'_, NextState<InspectorScene>>| {
                                            next_scene.set(InspectorScene::None);
                                        })
                                    ),
                                    (
                                        @FeathersMenuItem {
                                            @caption: bsn! { Text("Simple scene") ThemedText }
                                        }
                                        on(|_: On<'_, '_, Activate>, mut next_scene: ResMut<'_, NextState<InspectorScene>>| {
                                            next_scene.set(InspectorScene::Simple);
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
    selected: Res<'_, State<InspectorScene>>,
    mut caption: Single<'_, '_, &mut Text, With<SceneSelectorCaption>>,
) {
    if selected.is_changed() {
        caption.0 = selected.get().label().to_string();
    }
}
