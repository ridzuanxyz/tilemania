use bevy::prelude::*;

mod plugins;
use plugins::{CorePlugin, StatePlugin, AssetPlugin, InputPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "TileMania - Scrabble Learning Game".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            CorePlugin,
            StatePlugin,
            AssetPlugin,
            InputPlugin,
        ))
        .run();
}
