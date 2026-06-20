use bevy::prelude::*;

/// Current seed for procedural generation. Press R to cycle.
#[derive(Resource)]
pub struct GenerationSeed(pub u64);

impl Default for GenerationSeed {
    fn default() -> Self {
        Self(42)
    }
}

/// Increments the generation seed when R is pressed.
pub fn cycle_seed_on_command(input: Res<ButtonInput<KeyCode>>, mut seed: ResMut<GenerationSeed>) {
    if input.just_pressed(KeyCode::KeyR) {
        seed.0 = seed.0.wrapping_add(1);
        println!("Seed: {}", seed.0);
    }
}
