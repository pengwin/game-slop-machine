//! Parameter controls for concrete wall material generation.

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
    ConcreteWallDirtSettings, ConcreteWallGenerationRequest, ConcreteWallMaterialControls,
    ConcreteWallUvSettings, InspectorSceneState,
};

use super::super::{
    consts::PANEL_FONT_SIZE,
    control_panel::{control_rows, sync_schema_checkboxes, sync_schema_sliders},
    despawn_ui::despawn_ui,
};

#[derive(Component, Clone, Default)]
struct ConcreteWallControlsUi;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorSceneState::ConcreteWallMaterial),
        concrete_wall_controls_ui.spawn(),
    )
    .add_systems(
        Update,
        (
            sync_schema_sliders::<ConcreteWallMaterialControls>,
            sync_schema_sliders::<ConcreteWallDirtSettings>,
            sync_schema_sliders::<ConcreteWallUvSettings>,
            sync_schema_checkboxes::<ConcreteWallUvSettings>,
        )
            .run_if(in_state(InspectorSceneState::ConcreteWallMaterial)),
    )
    .add_systems(
        OnExit(InspectorSceneState::ConcreteWallMaterial),
        despawn_ui::<ConcreteWallControlsUi>,
    );
}

fn concrete_wall_controls_ui() -> impl SceneList {
    bsn_list![controls_panel()]
}

fn controls_panel() -> impl Scene {
    bsn! {
        (
            Name::new("Concrete Wall Controls UI")
            ConcreteWallControlsUi
            Node {
                position_type: PositionType::Absolute,
                bottom: px(12),
                left: px(12),
                min_width: px(330),
                max_height: px(980),
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
                (Text("Concrete Params") ThemedText),
                {control_rows::<ConcreteWallMaterialControls>(88.0)},
                {control_rows::<ConcreteWallDirtSettings>(88.0)},
                {control_rows::<ConcreteWallUvSettings>(88.0)},
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
    controls: Res<'_, ConcreteWallMaterialControls>,
) {
    commands.insert_resource(ConcreteWallGenerationRequest {
        params: controls.params.clone(),
    });
}

#[allow(clippy::needless_pass_by_value)]
fn handle_reset(
    _: On<'_, '_, Activate>,
    mut commands: Commands<'_, '_>,
    mut controls: ResMut<'_, ConcreteWallMaterialControls>,
    mut dirt_settings: ResMut<'_, ConcreteWallDirtSettings>,
    mut uv_settings: ResMut<'_, ConcreteWallUvSettings>,
) {
    *controls = ConcreteWallMaterialControls::default();
    *dirt_settings = ConcreteWallDirtSettings::default();
    *uv_settings = ConcreteWallUvSettings::default();
    commands.insert_resource(ConcreteWallGenerationRequest {
        params: controls.params.clone(),
    });
}
