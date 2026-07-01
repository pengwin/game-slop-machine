//! Global light controls for inspector scenes.

use bevy::{
    feathers::{
        controls::{
            FeathersCheckbox, FeathersMenu, FeathersMenuButton, FeathersMenuItem,
            FeathersMenuPopup, FeathersSlider,
        },
        font_styles::InheritableFont,
        theme::{ThemeBackgroundColor, ThemedText},
        tokens,
    },
    input_focus::tab_navigation::TabGroup,
    prelude::*,
    ui::Checked,
    ui_widgets::{
        Activate, SliderPrecision, SliderStep, SliderValue, ValueChange, checkbox_self_update,
        slider_self_update,
    },
};
use game_core::plugins::{
    global_lighting::{GlobalLightControls, GlobalLightControlsSlider},
    inspector::InspectorSceneState,
};

use super::super::{consts::PANEL_FONT_SIZE, despawn_ui::despawn_ui};

#[derive(Component, Clone, Default)]
struct GlobalLightUi;

#[derive(Component, Clone, Default)]
struct ShadowMapSizeCaption;

#[derive(Component, Clone, Default)]
struct GlobalLightSlider {
    setting: GlobalLightControlsSlider,
}

#[derive(Component, Clone, Default)]
struct ShadowsEnabledCheckbox;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorSceneState::Simple),
        global_light_ui.spawn(),
    )
    .add_systems(
        Update,
        (
            sync_global_light_sliders,
            sync_shadows_enabled_checkbox,
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
                light_slider(GlobalLightControlsSlider::AmbientBrightness),
                light_slider(GlobalLightControlsSlider::SunIlluminance),
                light_slider(GlobalLightControlsSlider::SunElevationDegrees),
                light_slider(GlobalLightControlsSlider::SunAzimuthDegrees),
                shadows_checkbox(),
                light_slider(GlobalLightControlsSlider::ShadowDepthBias),
                light_slider(GlobalLightControlsSlider::ShadowNormalBias),
                light_slider(GlobalLightControlsSlider::CascadeMinimumDistance),
                light_slider(GlobalLightControlsSlider::CascadeFirstFarBound),
                light_slider(GlobalLightControlsSlider::CascadeMaximumDistance),
                light_slider(GlobalLightControlsSlider::CascadeOverlapProportion),
                shadow_map_size_menu(),
            ]
        )
    }
}

fn light_slider(setting: GlobalLightControlsSlider) -> impl Scene {
    let handler_setting = setting.clone();
    let label = setting.label();
    let min = setting.min();
    let max = setting.max();
    let step = setting.step();
    let precision = setting.precision();

    bsn! {
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: px(4),
        }
        InheritableFont {
            font_size: PANEL_FONT_SIZE,
        }
        Children [
            (
                Text(label)
                ThemedText
                Node {
                    width: px(80),
                }
            ),
            (
                @FeathersSlider {
                    @min: min,
                    @max: max,
                    @value: min,
                }
                template_value(GlobalLightSlider { setting })
                SliderStep(step)
                SliderPrecision(precision)
                on(slider_self_update)
                on(
                    move |
                        change: On<'_, '_, ValueChange<f32>>,
                        mut controls: ResMut<'_, GlobalLightControls>,
                    | {
                        handler_setting.set(&mut controls, change.value);
                    }
                )
            )
        ]
    }
}

fn shadows_checkbox() -> impl Scene {
    bsn! {
        (
            @FeathersCheckbox {
                @caption: bsn! { Text("Shadows") ThemedText }
            }
            InheritableFont {
                font_size: PANEL_FONT_SIZE,
            }
            ShadowsEnabledCheckbox
            Checked
            on(checkbox_self_update)
            on(
                |
                    change: On<'_, '_, ValueChange<bool>>,
                    mut controls: ResMut<'_, GlobalLightControls>,
                | {
                    controls.shadows_enabled = change.value;
                }
            )
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
fn sync_global_light_sliders(
    mut commands: Commands<'_, '_>,
    controls: Res<'_, GlobalLightControls>,
    sliders: Query<'_, '_, (Entity, &GlobalLightSlider, &SliderValue)>,
) {
    for (entity, slider, value) in &sliders {
        let expected = slider.setting.value(&controls);
        if (value.0 - expected).abs() > 0.001 {
            commands.entity(entity).insert(SliderValue(expected));
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn sync_shadows_enabled_checkbox(
    mut commands: Commands<'_, '_>,
    controls: Res<'_, GlobalLightControls>,
    checkboxes: Query<'_, '_, (Entity, Has<Checked>), With<ShadowsEnabledCheckbox>>,
) {
    for (entity, checked) in &checkboxes {
        if controls.shadows_enabled && !checked {
            commands.entity(entity).insert(Checked);
        } else if !controls.shadows_enabled && checked {
            commands.entity(entity).remove::<Checked>();
        }
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
