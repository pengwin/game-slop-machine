# Agent Rules for Game Slop Machine

## Project Structure
- Plugins: `src/plugins/` (one directory per plugin)
- Each plugin has: `mod.rs`, `components.rs`, `resources.rs`, `systems.rs`
- Keep components and resources co-located with their plugins
- Split code into small, meaningful files
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
- Each plugin is a directory under `src/plugins/`
- Plugin structure:
  ```
  src/plugins/my_plugin/
    mod.rs        # Plugin struct/fn, re-exports
    components.rs # Plugin-specific components
    resources.rs  # Plugin-specific resources
    systems.rs    # Plugin systems
  ```
- Register plugins in `src/plugins/mod.rs` and `src/main.rs`

### Code Style
- `use bevy::prelude::*;`
- `Commands` for spawn/despawn
- Query filters: `Changed<T>`, `Added<T>`, `Without<T>`

### Build Commands
- Dev: `cargo run --features dev` (dynamic linking for faster compile)
- Release: `cargo build --release`

## API Notes (0.18.1 Specific)
- `despawn()` is recursive by default (no `despawn_recursive()`)
- `ScalingMode` requires explicit import: `use bevy::camera::ScalingMode;`
- `GlobalAmbientLight` is the resource (not `AmbientLight` which is per-camera component)
- Function plugins must use snake_case: `pub fn game_plugin(app: &mut App)`
- Re-export with alias if needed: `pub use game::game_plugin as GamePlugin;`
