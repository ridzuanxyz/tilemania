use bevy::prelude::*;
use std::collections::HashMap;

/// Asset loading state tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(dead_code)]  // Failed state reserved for future error handling
pub enum AssetLoadingState {
    #[default]
    NotStarted,
    Loading,
    Loaded,
    Failed,
}

/// GameAssets resource tracks loading state of game assets
#[derive(Resource)]
pub struct GameAssets {
    pub state: AssetLoadingState,
    pub progress: f32,  // 0.0 to 1.0
    pub total_assets: usize,
    pub loaded_assets: usize,
    // Font assets
    pub fonts: HashMap<String, Handle<Font>>,
    // Future fields (Sprint 2-4):
    // pub textures: HashMap<String, Handle<Image>>,
    // pub sounds: HashMap<String, Handle<AudioSource>>,
    // pub lexicon: Option<Handle<Lexicon>>,
}

impl Default for GameAssets {
    fn default() -> Self {
        Self {
            state: AssetLoadingState::NotStarted,
            progress: 0.0,
            total_assets: 0,
            loaded_assets: 0,
            fonts: HashMap::new(),
        }
    }
}

impl GameAssets {
    pub fn is_loaded(&self) -> bool {
        self.state == AssetLoadingState::Loaded
    }

    #[allow(dead_code)]  // Reserved for future use
    pub fn is_loading(&self) -> bool {
        self.state == AssetLoadingState::Loading
    }

    pub fn update_progress(&mut self) {
        if self.total_assets > 0 {
            self.progress = self.loaded_assets as f32 / self.total_assets as f32;
        }
    }
}

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameAssets>()
            .add_systems(Startup, start_loading_assets)
            .add_systems(Update, simulate_asset_loading);
    }
}

fn start_loading_assets(
    mut assets: ResMut<GameAssets>,
    asset_server: Res<AssetServer>,
) {
    info!("📦 Initializing asset loading system");

    // Load fonts
    let font_paths = vec![
        ("regular", "fonts/FiraSans-Regular.ttf"),
        ("medium", "fonts/FiraSans-Medium.ttf"),
        ("bold", "fonts/FiraSans-Bold.ttf"),
        ("emoji", "fonts/NotoColorEmoji-Regular.ttf"),
    ];

    for (name, path) in font_paths {
        let handle: Handle<Font> = asset_server.load(path);
        assets.fonts.insert(name.to_string(), handle);
        info!("📝 Loading font: {} from {}", name, path);
    }

    assets.total_assets = assets.fonts.len();
    assets.loaded_assets = 0;
    assets.state = AssetLoadingState::Loading;
    assets.progress = 0.0;

    info!("📊 Total assets to load: {}", assets.total_assets);
}

/// Checks asset loading progress
fn simulate_asset_loading(
    mut assets: ResMut<GameAssets>,
    asset_server: Res<AssetServer>,
) {
    if assets.state == AssetLoadingState::Loading {
        // Check how many fonts are loaded
        let mut loaded_count = 0;
        for handle in assets.fonts.values() {
            if asset_server.is_loaded_with_dependencies(handle.id()) {
                loaded_count += 1;
            }
        }

        assets.loaded_assets = loaded_count;
        assets.update_progress();

        // Check if all assets loaded
        if assets.loaded_assets >= assets.total_assets {
            assets.state = AssetLoadingState::Loaded;
            info!("✅ Asset loading complete! All {} fonts loaded", assets.total_assets);
        }
    }
}
