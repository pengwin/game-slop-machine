//! A small preview scene for validating rendering and inspector scene selection.

mod camera;
mod geometry;
mod lighting;
mod plugin;
mod root;
mod scene_sets;
mod state;

pub use plugin::SimpleScenePlugin;
pub use root::SimpleSceneRoot;
pub use state::InspectorScene;
