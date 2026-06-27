# Agent Rules for Game Slop Machine

This project uses Bevy 0.19, so the rules here are specific to that version.

## Context

You can use sources in `references/` dir to search for implementations, DO NOT reference it directly from code as creates.
Useful references:
- [Bevy 0.19 Examples](references/bevy-release-0.19.0/examples) preferable source for implementation details
- [Old Legacy source base of this project](references/game-slop-machine-main) source for procedural generation and headless solution


## Bevy 0.19 Conventions

### Build Commands
- `just build` - builds the project in release mode
- `just lint` - runs clippy with all warnings as errors
- `just fmt` - formats the codebase


## ECS / Bevy / BSN Architecture Best Practices

* Keep procedural generation outside ECS when possible.
* Generators should produce pure data: configs, layouts, mesh data, spawn descriptions.
* Use ECS as the runtime representation of the game world, not as the main place for generation algorithms.
* Use BSN / `Scene` trait for declarative composition of reusable entity trees.
* Use BSN for prefab-like objects: UI, props, NPC roots, camera rigs, debug widgets, building root scenes.
* Do not use BSN as the main procedural generation engine.
* Do not describe large generated meshes, tile grids, walls, floors, or procedural geometry directly in BSN.
* Prefer this flow: pure generation → semantic world data → BSN/root scene composition → ECS spawn → visual representation.
* Prefer feature-based plugins over global `components/`, `systems/`, `resources/` folders.
* Each plugin should own its components, resources, messages, systems, scenes, and spawn logic.
* Keep components small. A component should describe one fact or capability.
* Avoid large “god components” that contain layout, mesh data, runtime state, rendering data, and debug info together.
* Use resources only for true global state: config, seed, registries, selected entity, shared caches.
* Do not store the whole game world inside one resource.
* Use `SystemSet`s to define clear execution order: input, commands, AI, simulation, generation, spawning, presentation, UI.
* Do not rely on system insertion order. Explicitly use `.before()`, `.after()`, `.chain()`, or ordered system sets.
* Use `States` for major game modes: loading, menu, in-game, paused.
* Use `OnEnter` for setup and `OnExit` for cleanup.
* Use messages/events for communication between systems instead of direct coupling.
* Input systems should create requests; generation and spawn systems should process those requests.
* Prefer root entities for complex objects. Example: `BuildingRoot` with child entities for floor mesh, wall mesh, roof mesh, props, and debug visualization.
* Batch visual-only geometry into meshes. Do not create entities for every tile, wall segment, board, or decorative detail unless it is interactive.
* Interactive objects should be entities. Static visual detail should usually be mesh data, vertex colors, materials, decals, or instanced rendering.
* Keep rendering and presentation separate from simulation and generation.
* Save procedural recipes and important changes, not the entire ECS world.
* Prefer deterministic generation from seed + config.
* Keep systems small, focused, and boring.
* Avoid hidden side effects and unclear ownership between plugins.
* BSN scenes should describe reusable structure; systems should perform runtime logic; generators should create procedural content.

## Feature-first Module Structure

Prefer organizing code by gameplay feature, not by ECS type.

### DO: group components, systems, resources, messages by concrete feature

Good:

```text
plugins/
  combat/
    mod.rs

    bullets.rs
    weapons.rs
    damage.rs
    health.rs
    hit_detection.rs

  building/
    mod.rs

    root.rs
    walls.rs
    floors.rs
    roofs.rs
    doors.rs
    windows.rs
    debug.rs

  player/
    mod.rs

    movement.rs
    input.rs
    interaction.rs
    selection.rs
```

Each module should contain the components, resources, messages, systems, helper functions, and plugin registration related to that specific feature.

Example:

```rust
// plugins/combat/bullets.rs

#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
    pub damage: f32,
}

#[derive(Message)]
pub struct SpawnBulletRequest {
    pub origin: Vec3,
    pub direction: Vec3,
    pub damage: f32,
}

pub fn spawn_bullets(...) {
    // spawn bullet entities
}

pub fn move_bullets(...) {
    // update bullet movement
}

pub fn despawn_expired_bullets(...) {
    // cleanup
}

pub fn plugin(app: &mut App) {
    app.add_message::<SpawnBulletRequest>()
        .add_systems(
            Update,
            (
                spawn_bullets.in_set(GameSet::Spawning),
                move_bullets.in_set(GameSet::Simulation),
                despawn_expired_bullets.in_set(GameSet::Despawn),
            ),
        );
}
```

Then the parent plugin composes subfeatures:

```rust
// plugins/combat/mod.rs

mod bullets;
mod weapons;
mod damage;
mod health;
mod hit_detection;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        bullets::plugin(app);
        weapons::plugin(app);
        damage::plugin(app);
        health::plugin(app);
        hit_detection::plugin(app);
    }
}
```

---

### DON'T: create large generic ECS-type files inside every plugin

Avoid this:

```text
plugins/
  combat/
    mod.rs
    components.rs
    systems.rs
    resources.rs
    messages.rs
```

This looks clean at first, but eventually `components.rs` and `systems.rs` become unrelated dumps.

Bad:

```rust
// plugins/combat/components.rs

pub struct Bullet;
pub struct Weapon;
pub struct Health;
pub struct DamagePopup;
pub struct Hitbox;
pub struct Hurtbox;
pub struct ProjectileLifetime;
pub struct Explosion;
```

Better:

```text
combat/
  bullets.rs      // Bullet, ProjectileLifetime, SpawnBulletRequest, bullet systems
  weapons.rs      // Weapon, fire/reload systems
  health.rs       // Health, death systems
  damage.rs       // Damage, DamageEvent/Message, damage application
  hit_detection.rs
```

---

### Rule of thumb

If a file contains many unrelated components or systems, split it by feature.

Prefer:

```text
feature module = data + behavior + registration for one concrete gameplay concept
```

Avoid:

```text
components.rs = all data
systems.rs = all behavior
resources.rs = all globals
messages.rs = all requests
```

The goal is that an agent can open `bullets.rs` or `doors.rs` and see the full local implementation of that feature without jumping across five generic files.



