use bevy::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_core)
            .add_systems(Update, core_systems);
    }
}

fn setup_core(mut commands: Commands) {
    // Spawn camera (Bevy 0.15+ style)
    commands.spawn(Camera2d);

    info!("🎮 TileMania Core initialized");
    info!("📚 Lexicon: CSW24 (280,886 words)");
    info!("🎯 Sprint 1, Week 2: Core Architecture");
}

fn core_systems() {
    // Core update systems will be added as needed
    // Currently intentionally empty - placeholder for future systems
}
