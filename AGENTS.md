# Agent Rules for Game Slop Machine

This project uses Bevy 0.19, so the rules here are specific to that version.

## Bevy 0.19 Conventions

### Plugin Rules
- Each plugin is a directory under `crates/game-core/src/plugins/`
- Plugin structure by feature, not by type:
  ```
  crates/game-core/src/plugins/my_plugin/
    mod.rs         # Plugin struct/fn, re-exports
    feature_a.rs   # Feature with its own components, systems, resources
    feature_b.rs   # Another feature
    config.rs      # Plugin configuration resource
  ```
- Do NOT use: components.rs, resources.rs, systems.rs
- DO use: camera.rs, light.rs, player.rs, enemies.rs, etc.

### Code Style
- `use bevy::prelude::*;`
- `Commands` for spawn/despawn
- Query filters: `Changed<T>`, `Added<T>`, `Without<T>`

### Build Commands
- `just build` - builds the project in release mode
- `just lint` - runs clippy with all warnings as errors
- `just fmt` - formats the codebase

## API Notes (0.19 Specific)
- `despawn()` is recursive by default (no `despawn_recursive()`)
- `ScalingMode` requires explicit import: `use bevy::camera::ScalingMode;`
- `GlobalAmbientLight` is the resource (not `AmbientLight` which is per-camera component)
- Function plugins must use snake_case: `pub fn game_plugin(app: &mut App)`
- Re-export with alias if needed: `pub use game::game_plugin as GamePlugin;`
- Mesh creation: `use bevy::asset::RenderAssetUsages;` and `use bevy::mesh::Indices;`
