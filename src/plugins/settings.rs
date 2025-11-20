use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Global game settings resource
#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct GameSettings {
    pub audio: AudioSettings,
    pub gameplay: GameplaySettings,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioSettings {
    pub music_enabled: bool,
    pub sfx_enabled: bool,
    pub music_volume: f32,  // 0.0 - 1.0
    pub sfx_volume: f32,    // 0.0 - 1.0
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameplaySettings {
    pub dictionary: String,
    pub default_time_limit: u32,  // seconds
    pub default_difficulty: u8,   // 1-5
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            audio: AudioSettings {
                music_enabled: true,
                sfx_enabled: true,
                music_volume: 0.5,
                sfx_volume: 0.7,
            },
            gameplay: GameplaySettings {
                dictionary: "TML".to_string(),
                default_time_limit: 25 * 60, // 25 minutes in seconds
                default_difficulty: 3, // Medium
            },
        }
    }
}

impl GameSettings {
    /// Get the settings file path
    fn config_path() -> PathBuf {
        #[cfg(target_os = "linux")]
        {
            // Use XDG Base Directory specification on Linux
            if let Ok(config_dir) = std::env::var("XDG_CONFIG_HOME") {
                PathBuf::from(config_dir).join("tilemania").join("settings.toml")
            } else if let Ok(home) = std::env::var("HOME") {
                PathBuf::from(home).join(".config").join("tilemania").join("settings.toml")
            } else {
                PathBuf::from("settings.toml")
            }
        }

        #[cfg(target_os = "windows")]
        {
            // Use AppData on Windows
            if let Ok(appdata) = std::env::var("APPDATA") {
                PathBuf::from(appdata).join("TileMania").join("settings.toml")
            } else {
                PathBuf::from("settings.toml")
            }
        }

        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
        {
            PathBuf::from("settings.toml")
        }
    }

    /// Load settings from file, or use defaults if file doesn't exist
    pub fn load() -> Self {
        let path = Self::config_path();

        if let Ok(contents) = fs::read_to_string(&path) {
            match toml::from_str(&contents) {
                Ok(settings) => {
                    info!("✅ Loaded settings from {:?}", path);
                    return settings;
                }
                Err(e) => {
                    warn!("⚠️ Failed to parse settings file: {}. Using defaults.", e);
                }
            }
        }

        info!("ℹ️ No settings file found. Using defaults.");
        Self::default()
    }

    /// Save settings to file
    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path();

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let contents = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize settings: {}", e))?;

        fs::write(&path, contents)
            .map_err(|e| format!("Failed to write settings file: {}", e))?;

        info!("💾 Saved settings to {:?}", path);
        Ok(())
    }
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        let settings = GameSettings::load();
        app.insert_resource(settings);
    }
}
