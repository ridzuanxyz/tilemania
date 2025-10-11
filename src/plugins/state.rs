use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    MainMenu,
    GameBoard,
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

fn enter_splash(mut next_state: ResMut<NextState<GameState>>) {
    info!("📺 Entering Splash screen");
    // Auto-transition to main menu
    // In future sprints, will add 2-second timer with logo animation
    next_state.set(GameState::MainMenu);
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
