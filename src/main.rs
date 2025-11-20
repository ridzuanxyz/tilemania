use bevy::prelude::*;

mod plugins;
mod ui;
mod lexicon;
mod scoring;
mod stage1;
mod stage2;
mod stage3;
mod stage4;
mod stage5;

use plugins::{CorePlugin, StatePlugin, AssetPlugin, InputPlugin, SettingsPlugin};
use ui::UiPlugin;
use stage1::Stage1Plugin;
use stage2::Stage2Plugin;
use stage3::Stage3Plugin;
use stage4::Stage4Plugin;
use stage5::Stage5Plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "TileMania - Word Tile Strategy Game".to_string(),
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
            SettingsPlugin,
            UiPlugin,
            Stage1Plugin,
            Stage2Plugin,
            Stage3Plugin,
            Stage4Plugin,
            Stage5Plugin,
        ))
        .run();
}
