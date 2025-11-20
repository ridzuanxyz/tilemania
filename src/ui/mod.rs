pub mod components;
pub mod splash;
pub mod main_menu;
pub mod stage_select;
pub mod game_board;
pub mod results;
pub mod settings;

use bevy::prelude::*;
use components::button;

/// UI Plugin manages all user interface components
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                button::update_button_interaction,
                splash::update_splash,
                main_menu::update_main_menu,
                stage_select::update_stage_select,
                stage_select::handle_stage_buttons,
                // game_board::update_game_board, // Disabled - Stage2Plugin handles GameBoard state
                results::update_results,
                settings::update_settings,
                settings::handle_setting_buttons,
            ));
    }
}
