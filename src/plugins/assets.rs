use bevy::prelude::*;

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
    // Future fields (Sprint 2-4):
    // pub fonts: HashMap<String, Handle<Font>>,
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

fn start_loading_assets(mut assets: ResMut<GameAssets>) {
    info!("📦 Initializing asset loading system");

    // Simulate asset count for demonstration
    // In Sprint 2-4, this will be actual asset counts
    assets.total_assets = 10;  // Mock: fonts, textures, sounds, etc.
    assets.loaded_assets = 0;
    assets.state = AssetLoadingState::Loading;
    assets.progress = 0.0;

    info!("📊 Total assets to load: {}", assets.total_assets);
}

/// Simulates progressive asset loading
/// In Sprint 2-4, this will be replaced with actual asset loading logic
fn simulate_asset_loading(
    time: Res<Time>,
    mut assets: ResMut<GameAssets>,
) {
    if assets.state == AssetLoadingState::Loading {
        // Simulate loading progress over 2 seconds
        let load_speed = 5.0; // assets per second
        let delta = time.delta_secs() * load_speed;

        assets.loaded_assets = (assets.loaded_assets as f32 + delta)
            .min(assets.total_assets as f32) as usize;

        assets.update_progress();

        // Check if all assets loaded
        if assets.loaded_assets >= assets.total_assets {
            assets.state = AssetLoadingState::Loaded;
            info!("✅ Asset loading complete! Progress: {:.0}%", assets.progress * 100.0);
        }
    }
}
