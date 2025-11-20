# ⚙️ Settings System Documentation

**Feature**: Interactive Settings Management
**Status**: ✅ Complete (Sprint 1, Week 3)
**Version**: 1.0.0

---

## 📋 Overview

The Settings System provides a comprehensive, user-friendly interface for managing game preferences with persistent storage. All settings are automatically saved to disk and loaded on game startup.

### Key Features
- 🔊 **Audio Controls**: Toggle and adjust music/SFX volume
- 🎮 **Gameplay Preferences**: Dictionary, timer, and difficulty defaults
- 💾 **Persistence**: TOML-based storage with platform-specific paths
- 🎨 **Interactive UI**: Real-time updates with visual feedback
- ⚡ **Auto-load**: Settings automatically restored on game launch

---

## 🎯 User Guide

### Accessing Settings

1. **From Main Menu**: Click the "Settings" button
2. **Keyboard Shortcut**: Press `ESC` to return to main menu from anywhere

### Available Settings

#### Audio Settings

| Setting | Controls | Range | Default |
|---------|----------|-------|---------|
| **Music Toggle** | ON/OFF button | - | ON |
| **Music Volume** | +/- buttons | 0-100% | 50% |
| **SFX Toggle** | ON/OFF button | - | ON |
| **SFX Volume** | +/- buttons | 0-100% | 70% |

- Volume adjusts in **10% increments**
- Changes apply instantly (when audio system is integrated)
- Toggle buttons immediately enable/disable audio

#### Gameplay Settings

| Setting | Options | Default |
|---------|---------|---------|
| **Dictionary** | CSW24 → TWL06 → SOWPODS | CSW24 |
| **Timer** | 10:00 → 15:00 → 25:00 → 30:00 → Unlimited | 25:00 |
| **Difficulty** | Very Easy → Easy → Medium → Hard → Very Hard | Medium |

- **Dictionary**: Selects which word list to use for validation
- **Timer**: Default time limit for timed game modes
- **Difficulty**: Default AI difficulty and game challenge level

### Saving Settings

1. **Manual Save**: Click the "💾 Save" button
   - Displays confirmation message in logs
   - Persists all changes to disk

2. **Auto-Load**: Settings automatically load on next game launch
   - No action required
   - Falls back to defaults if file doesn't exist

### Keyboard Controls

- **ESC**: Return to Main Menu (also closes settings)
- **Click**: Interact with all buttons

---

## 🔧 Technical Details

### Architecture

**Components**:
- `GameSettings` - Main resource containing all settings
- `SettingsPlugin` - Manages initialization and persistence
- `SettingButton` - Button component types for interactions
- `SettingLabel` - Text labels that update dynamically

**Systems**:
- `update_settings()` - Spawns/despawns settings UI
- `handle_setting_buttons()` - Processes button interactions
- `update_labels()` - Updates text displays when values change

### File Structure

```
src/
├── plugins/
│   └── settings.rs         # GameSettings resource + persistence
└── ui/
    └── settings.rs         # Settings UI + interaction systems
```

### Data Structure

```rust
#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct GameSettings {
    pub audio: AudioSettings,
    pub gameplay: GameplaySettings,
}

pub struct AudioSettings {
    pub music_enabled: bool,
    pub sfx_enabled: bool,
    pub music_volume: f32,  // 0.0 - 1.0
    pub sfx_volume: f32,    // 0.0 - 1.0
}

pub struct GameplaySettings {
    pub dictionary: String,
    pub default_time_limit: u32,  // seconds (0 = unlimited)
    pub default_difficulty: u8,   // 1-5
}
```

### Storage Location

Settings are stored in **TOML format** at platform-specific locations:

**Linux**:
```
~/.config/tilemania/settings.toml
```

**Windows**:
```
%APPDATA%\TileMania\settings.toml
```

**Example settings.toml**:
```toml
[audio]
music_enabled = true
sfx_enabled = true
music_volume = 0.5
sfx_volume = 0.7

[gameplay]
dictionary = "CSW24"
default_time_limit = 1500
default_difficulty = 3
```

---

## 👨‍💻 Developer Guide

### Accessing Settings in Game Systems

Settings are available as a Bevy resource in any system:

```rust
use crate::plugins::settings::GameSettings;

fn my_audio_system(
    settings: Res<GameSettings>,
    // ... other parameters
) {
    // Access audio settings
    if settings.audio.music_enabled {
        let volume = settings.audio.music_volume;
        // Play music at specified volume
    }

    if settings.audio.sfx_enabled {
        let sfx_volume = settings.audio.sfx_volume;
        // Play sound effects at specified volume
    }
}

fn my_gameplay_system(
    settings: Res<GameSettings>,
    // ... other parameters
) {
    // Access gameplay settings
    let dictionary = &settings.gameplay.dictionary;
    let time_limit = settings.gameplay.default_time_limit;
    let difficulty = settings.gameplay.default_difficulty;

    // Use settings to configure game
}
```

### Modifying Settings Programmatically

```rust
fn modify_settings_system(
    mut settings: ResMut<GameSettings>,
) {
    // Modify settings
    settings.audio.music_volume = 0.8;
    settings.gameplay.default_difficulty = 4;

    // Save to disk
    if let Err(e) = settings.save() {
        error!("Failed to save settings: {}", e);
    }
}
```

### Adding New Settings

1. **Update Data Structure** (src/plugins/settings.rs):
```rust
pub struct GameplaySettings {
    pub dictionary: String,
    pub default_time_limit: u32,
    pub default_difficulty: u8,
    pub new_setting: bool,  // Add your new field
}
```

2. **Update Default Implementation**:
```rust
impl Default for GameSettings {
    fn default() -> Self {
        Self {
            // ...
            gameplay: GameplaySettings {
                // ...
                new_setting: true,  // Add default value
            },
        }
    }
}
```

3. **Add UI Component** (src/ui/settings.rs):
```rust
#[derive(Clone)]
pub enum SettingType {
    // ...
    NewSetting,  // Add enum variant
}

// Add spawning function in spawn_settings_ui()
spawn_toggle_row(
    container,
    &font_medium,
    SettingType::NewSetting,
    SettingButton::NewSettingToggle,
    settings,
);
```

4. **Add Button Handler**:
```rust
pub enum SettingButton {
    // ...
    NewSettingToggle,
}

// Add case in handle_setting_buttons()
SettingButton::NewSettingToggle => {
    settings.gameplay.new_setting = !settings.gameplay.new_setting;
    update_labels(&settings, &mut label_query);
}
```

5. **Add Label Update**:
```rust
// Add case in update_labels()
SettingType::NewSetting => {
    format!("New Setting: {}", if settings.gameplay.new_setting { "ON" } else { "OFF" })
}
```

---

## 🧪 Testing

### Manual Testing Checklist

- [ ] **Music Toggle**: Click toggles between ON/OFF, label updates
- [ ] **Music Volume**: +/- buttons change percentage (0-100%)
- [ ] **SFX Toggle**: Click toggles between ON/OFF, label updates
- [ ] **SFX Volume**: +/- buttons change percentage (0-100%)
- [ ] **Dictionary Cycle**: Rotates through CSW24 → TWL06 → SOWPODS → CSW24
- [ ] **Timer Cycle**: Rotates through 10:00 → 15:00 → 25:00 → 30:00 → Unlimited → 10:00
- [ ] **Difficulty Cycle**: Rotates through Very Easy → Easy → Medium → Hard → Very Hard → Very Easy
- [ ] **Save Button**: Click displays "✅ Settings saved successfully" in logs
- [ ] **Back Button**: Returns to Main Menu
- [ ] **ESC Key**: Returns to Main Menu
- [ ] **Persistence**: Close game, reopen, settings are restored
- [ ] **File Creation**: Check `~/.config/tilemania/settings.toml` exists after save

### Automated Testing

```bash
# Run unit tests
cargo test settings

# Run integration tests
cargo test --test integration_settings

# Check for warnings
cargo clippy -- -W clippy::all
```

---

## 🎨 UI Design

### Color Scheme

| Element | Color | Purpose |
|---------|-------|---------|
| Background | `rgb(0.12, 0.12, 0.18)` | Dark, non-distracting |
| Title | `rgb(0.9, 0.9, 1.0)` | High contrast |
| Labels | `rgb(1.0, 1.0, 1.0)` | White for readability |
| Buttons | `rgb(0.25, 0.25, 0.35)` | Subtle gray |
| Volume Down | `rgb(0.4, 0.2, 0.2)` | Red hint |
| Volume Up | `rgb(0.2, 0.4, 0.2)` | Green hint |
| Save Button | `rgb(0.2, 0.6, 0.3)` | Green for confirm |
| Back Button | `rgb(0.3, 0.3, 0.4)` | Neutral gray |

### Layout

```
┌─────────────────────────────────────┐
│        ⚙ Settings                   │
│                                     │
│  🎵 Music: ON          [Toggle]    │
│  Music Volume: 50%     [-] [+]     │
│  🔊 Sound Effects: ON  [Toggle]    │
│  SFX Volume: 70%       [-] [+]     │
│  📚 Dictionary: CSW24  [Change]    │
│  ⏱ Timer: 25:00       [Change]    │
│  🎮 Difficulty: Medium [Change]    │
│                                     │
│      [💾 Save]    [← Back]          │
│                                     │
│  Click buttons • Press ESC to exit │
└─────────────────────────────────────┘
```

---

## 📚 Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `serde` | 1.0 | Serialization/deserialization |
| `toml` | 0.8 | TOML parsing and writing |
| `bevy` | 0.15 | Game engine and UI |

---

## 🔮 Future Enhancements

### Planned Features
- [ ] **Keyboard Navigation**: Arrow keys to navigate settings
- [ ] **Tooltips**: Hover descriptions for each setting
- [ ] **Sound Preview**: Play sample when adjusting volume
- [ ] **Reset to Defaults**: Button to restore factory settings
- [ ] **Profiles**: Multiple setting profiles for different players
- [ ] **Cloud Sync**: Optional cloud backup of settings
- [ ] **Accessibility**: Screen reader support, high contrast mode

### Integration Points
- **Audio System**: Connect volume controls to actual playback
- **Game Modes**: Use default difficulty/timer when starting games
- **Word Validation**: Switch dictionaries based on setting
- **Analytics**: Track most common settings choices

---

## 🐛 Troubleshooting

### Settings Not Saving

**Symptoms**: Changes don't persist after restarting game

**Solutions**:
1. Check file permissions on config directory
2. Verify disk space available
3. Check logs for "Failed to save settings" errors
4. Manually verify file exists at expected path:
   ```bash
   # Linux
   cat ~/.config/tilemania/settings.toml

   # Windows
   type %APPDATA%\TileMania\settings.toml
   ```

### Settings File Corrupted

**Symptoms**: Game fails to load, crashes on startup

**Solutions**:
1. Delete settings file (will regenerate with defaults):
   ```bash
   # Linux
   rm ~/.config/tilemania/settings.toml

   # Windows
   del %APPDATA%\TileMania\settings.toml
   ```
2. Manually edit file to fix syntax errors
3. Check logs for TOML parsing errors

### UI Not Responding

**Symptoms**: Clicking buttons does nothing

**Solutions**:
1. Verify `handle_setting_buttons` system is registered
2. Check button interaction system is running
3. Verify no other UI is blocking input
4. Check logs for system errors

---

## 📝 Changelog

### Version 1.0.0 (2025-11-20)
- ✅ Initial release
- ✅ Audio settings (music/SFX toggle + volume)
- ✅ Gameplay settings (dictionary, timer, difficulty)
- ✅ TOML persistence with platform-specific paths
- ✅ Interactive UI with real-time updates
- ✅ Save/Back buttons
- ✅ ESC key support

---

## 📄 License

Part of TileMania project. See main LICENSE file for details.

---

## 🤝 Contributing

To contribute improvements to the settings system:

1. Read [CONTRIBUTING.md](CONTRIBUTING.md)
2. Create feature branch: `git checkout -b feature/settings-enhancement`
3. Make changes and test thoroughly
4. Submit PR with description and screenshots
5. Ensure all tests pass

---

## 📞 Support

- **Issues**: https://github.com/ridzuanxyz/tilemania/issues
- **Documentation**: https://github.com/ridzuanxyz/tilemania/tree/main/docs
- **Contact**: See README.md for contact information
