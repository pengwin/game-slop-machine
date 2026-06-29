//! Progress UI for plaster wall material generation.

use bevy::{
    feathers::{
        font_styles::InheritableFont,
        theme::{ThemeBackgroundColor, ThemedText},
        tokens,
    },
    input_focus::tab_navigation::TabGroup,
    prelude::*,
};
use game_core::plugins::inspector::{InspectorSceneState, PlasterWallGenerationProgress};

use super::super::{consts::PANEL_FONT_SIZE, despawn_ui::despawn_ui};

#[derive(Component, Clone, Default)]
struct PlasterWallProgressUi;

#[derive(Component, Clone, Default)]
struct PlasterWallProgressStatusText;

#[derive(Component, Clone, Default)]
struct PlasterWallProgressFill;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InspectorSceneState::PlasterWallMaterial),
        plaster_wall_progress_ui.spawn(),
    )
    .add_systems(
        Update,
        sync_plaster_wall_progress.run_if(in_state(InspectorSceneState::PlasterWallMaterial)),
    )
    .add_systems(
        OnExit(InspectorSceneState::PlasterWallMaterial),
        despawn_ui::<PlasterWallProgressUi>,
    );
}

fn plaster_wall_progress_ui() -> impl SceneList {
    bsn_list![progress_panel()]
}

fn progress_panel() -> impl Scene {
    bsn! {
        (
            Name::new("Plaster Wall Progress UI")
            PlasterWallProgressUi
            Node {
                position_type: PositionType::Absolute,
                bottom: px(330),
                right: px(12),
                min_width: px(300),
                padding: px(8),
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
                (Text("Plaster Material") ThemedText),
                (
                    Text("Queued")
                    ThemedText
                    PlasterWallProgressStatusText
                ),
                (
                    Node {
                        width: percent(100),
                        height: px(12),
                        border: px(1),
                    }
                    BackgroundColor(Color::srgba(0.08, 0.09, 0.10, 0.88))
                    Children [
                        (
                            Node {
                                width: percent(0),
                                height: percent(100),
                            }
                            BackgroundColor(Color::srgb(0.78, 0.66, 0.42))
                            PlasterWallProgressFill
                        )
                    ]
                )
            ]
        )
    }
}

#[allow(clippy::needless_pass_by_value)]
fn sync_plaster_wall_progress(
    progress: Option<Res<'_, PlasterWallGenerationProgress>>,
    mut status: Single<'_, '_, &mut Text, With<PlasterWallProgressStatusText>>,
    mut fill: Single<'_, '_, &mut Node, With<PlasterWallProgressFill>>,
) {
    let Some(progress) = progress else {
        status.0 = "Queued".to_string();
        fill.width = percent(0);
        return;
    };

    let percent_value = (progress.fraction * 100.0).round();
    status.0 = format!("{} {:.0}%", progress.status.label(), percent_value);
    fill.width = percent(percent_value);
}
