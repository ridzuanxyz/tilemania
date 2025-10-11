use bevy::prelude::*;

/// GameAssets resource tracks loading state of game assets
/// Currently a skeleton - will be populated with actual assets in Sprint 2-4
#[derive(Resource)]
pub struct GameAssets {
    pub loaded: bool,
    // Future fields:
    // pub fonts: HashMap<String, Handle<Font>>,
    // pub textures: HashMap<String, Handle<Image>>,
    // pub sounds: HashMap<String, Handle<AudioSource>>,
    // pub lexicon: Option<Handle<Lexicon>>,
}

impl Default for GameAssets {
    fn default() -> Self {
        Self { loaded: false }
    }
}

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut assets: ResMut<GameAssets>) {
    info!("📦 Initializing asset loading system");
    // Asset loading will be implemented in Sprint 2-4:
    // - Sprint 2: UI fonts and basic textures
    // - Sprint 3: Lexicon (CSW24.kwg)
    // - Sprint 4: Tile textures and board graphics
    // - Sprint 5: Sound effects and music
    assets.loaded = true;
    info!("✅ Asset system ready (placeholder mode)");
}
