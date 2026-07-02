//! Parameter controls for plaster wall material generation.

use bevy::{
    feathers::{
        controls::FeathersButton,
        font_styles::InheritableFont,
        theme::{ThemeBackgroundColor, ThemedText},
        tokens,
    },
    input_focus::tab_navigation::TabGroup,
    prelude::*,
    ui_widgets::Activate,
};
use game_core::plugins::inspector::{
    InspectorSceneState, PlasterWallDirtSettings, PlasterWallEditableParams,
    PlasterWallGenerationRequest, PlasterWallUvSettings,
};

use super::super::{
    consts::PANEL_FONT_SIZE,
    control_panel::{control_rows, sync_schema_checkboxes, sync_schema_sliders},
    despawn_ui::despawn_ui,
};

#[derive(Component, Clone, Default)]
struct PlasterWallControlsUi;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorSceneState::PlasterWallMaterial),
        plaster_wall_controls_ui.spawn(),
    )
    .add_systems(
        Update,
        (
            sync_schema_sliders::<PlasterWallEditableParams>,
            sync_schema_sliders::<PlasterWallDirtSettings>,
            sync_schema_sliders::<PlasterWallUvSettings>,
            sync_schema_checkboxes::<PlasterWallUvSettings>,
        )
            .run_if(in_state(InspectorSceneState::PlasterWallMaterial)),
    )
    .add_systems(
        OnExit(InspectorSceneState::PlasterWallMaterial),
        despawn_ui::<PlasterWallControlsUi>,
    );
}

fn plaster_wall_controls_ui() -> impl SceneList {
    bsn_list![controls_panel()]
}

fn controls_panel() -> impl Scene {
    bsn! {
        (
            Name::new("Plaster Wall Controls UI")
            PlasterWallControlsUi
            Node {
                position_type: PositionType::Absolute,
                bottom: px(12),
                left: px(12),
                min_width: px(330),
                max_height: px(850),
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
                (Text("Plaster Params") ThemedText),
                {control_rows::<PlasterWallEditableParams>(88.0)},
                {control_rows::<PlasterWallDirtSettings>(88.0)},
                {control_rows::<PlasterWallUvSettings>(88.0)},
                command_buttons(),
            ]
        )
    }
}

fn command_buttons() -> impl Scene {
    bsn! {
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            column_gap: px(6),
            padding: UiRect::top(px(4)),
        }
        Children [
            (
                @FeathersButton
                Node {
                    flex_grow: 1.0,
                    justify_content: JustifyContent::Center,
                }
                on(handle_apply)
                Children [ (Text("Apply") ThemedText) ]
            ),
            (
                @FeathersButton
                Node {
                    flex_grow: 1.0,
                    justify_content: JustifyContent::Center,
                }
                on(handle_reset)
                Children [ (Text("Reset") ThemedText) ]
            )
        ]
    }
}

#[allow(clippy::needless_pass_by_value)]
fn handle_apply(
    _: On<'_, '_, Activate>,
    mut commands: Commands<'_, '_>,
    params: Res<'_, PlasterWallEditableParams>,
) {
    commands.insert_resource(PlasterWallGenerationRequest::new(params.value.clone()));
}

#[allow(clippy::needless_pass_by_value)]
fn handle_reset(
    _: On<'_, '_, Activate>,
    mut commands: Commands<'_, '_>,
    mut params: ResMut<'_, PlasterWallEditableParams>,
    mut dirt_settings: ResMut<'_, PlasterWallDirtSettings>,
    mut uv_settings: ResMut<'_, PlasterWallUvSettings>,
) {
    *params = PlasterWallEditableParams::new(
        game_core::plugins::inspector::plaster_wall_material_scene::default_plaster_params(),
    );
    *dirt_settings = PlasterWallDirtSettings::default();
    *uv_settings = PlasterWallUvSettings::default();
    commands.insert_resource(PlasterWallGenerationRequest::new(params.value.clone()));
}
