use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    MainMenu,
    GameBoard,
    Stage1Playing,
    Stage1Paused,
    Stage2Playing,
    Stage2Paused,
    Stage3Playing,
    Stage3Paused,
    Results,
    Settings,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .add_systems(OnEnter(GameState::Splash), enter_splash)
            .add_systems(OnEnter(GameState::MainMenu), enter_main_menu)
            .add_systems(OnEnter(GameState::GameBoard), enter_game_board)
            .add_systems(OnEnter(GameState::Results), enter_results)
            .add_systems(OnEnter(GameState::Settings), enter_settings);
    }
}

fn enter_splash() {
    info!("📺 Entering Splash screen");
    // Asset loading will trigger transition to MainMenu
    // (handled in splash.rs update_splash function)
}

fn enter_main_menu() {
    info!("📋 Entering Main Menu");
}

fn enter_game_board() {
    info!("🎮 Entering Game Board");
}

fn enter_results() {
    info!("🏆 Entering Results");
}

fn enter_settings() {
    info!("⚙️  Entering Settings");
}
