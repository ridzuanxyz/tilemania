use bevy::prelude::*;

/// Tracks which stage just finished to show correct results screen
#[derive(Resource, Default, Debug, Clone, Copy, PartialEq)]
pub enum LastStageCompleted {
    #[default]
    None,
    Stage1,
    Stage2,
    Stage3,
    Stage4,
    Stage5,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    MainMenu,
    StageSelect,
    GameBoard,
    Stage1Playing,
    Stage1Paused,
    Stage2Start,
    Stage2Playing,
    Stage2Paused,
    Stage3Playing,
    Stage3Paused,
    Stage4Playing,
    Stage4Paused,
    Stage5Playing,
    Stage5Paused,
    Results,
    Settings,
    DebugMenu,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .init_resource::<LastStageCompleted>()
            .add_systems(OnEnter(GameState::Splash), enter_splash)
            .add_systems(OnEnter(GameState::MainMenu), enter_main_menu)
            .add_systems(OnEnter(GameState::StageSelect), enter_stage_select)
            .add_systems(OnEnter(GameState::GameBoard), enter_game_board)
            .add_systems(OnEnter(GameState::Stage2Start), enter_stage2_start)
            .add_systems(OnEnter(GameState::Results), enter_results)
            .add_systems(OnEnter(GameState::Settings), enter_settings)
            .add_systems(OnEnter(GameState::DebugMenu), enter_debug_menu);
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

fn enter_stage_select() {
    info!("🎯 Entering Stage Select");
}

fn enter_game_board() {
    info!("🎮 Entering Game Board");
}

fn enter_results() {
    info!("🏆 Entering Results");
}

fn enter_stage2_start() {
    info!("🎮 Entering Stage 2 Start");
}

fn enter_settings() {
    info!("⚙️  Entering Settings");
}

fn enter_debug_menu() {
    info!("🐛 Entering Debug Menu");
}
