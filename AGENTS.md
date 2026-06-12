# Agent Rules for Game Slop Machine

## Project Structure
- Workspace: `Cargo.toml` (workspace root)
- Apps: `apps/` (runnable executables)
- Crates: `crates/` (libraries)
- Plugins: `crates/game-core/src/plugins/` (one directory per plugin)
- Each plugin has: `mod.rs`, plus meaningful feature-based files
- Keep components co-located with the systems that use them
- Split code into small, meaningful files by feature, not by type
- Main: `src/main.rs` (App setup only)

## Bevy 0.18.1 Conventions

### Entity Spawning
- Use component tuples or bundles for spawning
- Spawn: `commands.spawn((ComponentA, ComponentB, Transform::default()))`
- Required Components: `Camera3d` auto-inserts `Camera`, `Projection`

### State Management
- Use `DespawnOnExit(state)` for state-scoped cleanup
- Co-locate `OnEnter`/`OnExit` registration
- Bound Update systems with `run_if(in_state(...))`

### Components & Resources
- Derive `Component`, `Resource`
- Use `..default()` for initialization
- Name top-level entities with `Name::new("...")`
- Keep components close to the systems that use them (not in separate components.rs)

### Systems
- End with `_system` suffix
- `Startup` for one-time setup
- `Update` for per-frame logic
- `FixedUpdate` for physics/game logic
- Explicit ordering: `.before()` / `.after()` / `.chain()`

### Events
- Prefer events for cross-system communication
- Order readers after writers
- Use `run_if(on_event::<MyEvent>())`

### Cleanup Pattern
Use `CleanupOnExit` component with generic `cleanup_system::<T>`:
```rust
commands.spawn((Name::new("Enemy"), CleanupOnExit, ...));
app.add_systems(OnExit(GameState::InGame), cleanup_system::<CleanupOnExit>);
```

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
- Register plugins in `crates/game-core/src/plugins/mod.rs`
- Use plugin in apps via `game_core::plugins::GamePlugin`

### Code Style
- `use bevy::prelude::*;`
- `Commands` for spawn/despawn
- Query filters: `Changed<T>`, `Added<T>`, `Without<T>`

### Build Commands
- Dev: `cargo run -p game` (default profile)
- Release: `cargo build -p game --release`

## API Notes (0.18.1 Specific)
- `despawn()` is recursive by default (no `despawn_recursive()`)
- `ScalingMode` requires explicit import: `use bevy::camera::ScalingMode;`
- `GlobalAmbientLight` is the resource (not `AmbientLight` which is per-camera component)
- Function plugins must use snake_case: `pub fn game_plugin(app: &mut App)`
- Re-export with alias if needed: `pub use game::game_plugin as GamePlugin;`
