# Bevy 0.15 Migration Status

## ✅ Completed Changes

### 1. Removed Incorrect Imports (22 files)
**Issue**: Attempted to import `bevy::text::TextStyle` and `bevy::ui::Style` which don't exist in Bevy 0.15.

**Solution**: Removed all incorrect import statements. The needed types (`Text`, `Text2d`, `TextFont`, `TextColor`, `Node`) are available in `bevy::prelude::*`.

**Files affected**: All stage files (1-5) in systems.rs, visuals.rs, ui.rs, pause.rs, powerups.rs, board.rs

### 2. Migrated Text2dBundle to Required Components (6 instances)
**Issue**: `Text2dBundle` is deprecated in Bevy 0.15. The old `TextStyle` struct for 2D text no longer exists.

**Old API**:
```rust
Text2dBundle {
    text: Text::from_section(
        "text",
        TextStyle {
            font: asset_server.load("font.ttf"),
            font_size: 48.0,
            color: Color::WHITE,
        },
    ),
    transform: Transform::from_xyz(x, y, z),
    ..default()
}
```

**New API** (Bevy 0.15):
```rust
(
    Text2d::new("text"),
    TextFont {
        font: asset_server.load("font.ttf"),
        font_size: 48.0,
        ..default()
    },
    TextColor(Color::WHITE),
    Transform::from_xyz(x, y, z),
)
```

**Files migrated**:
- src/stage1/systems.rs
- src/stage1/powerups.rs
- src/stage1/visuals.rs
- src/stage2/systems.rs
- src/stage2/visuals.rs
- src/stage3/board.rs

### 3. Migrated TextBundle to Required Components (1 complete file)
**Issue**: `TextBundle` is deprecated in Bevy 0.15.

**Old API**:
```rust
TextBundle::from_section(
    "text",
    TextStyle {
        font: asset_server.load("font.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    },
)
```

**New API** (Bevy 0.15):
```rust
(
    Text::new("text"),
    TextFont {
        font: asset_server.load("font.ttf"),
        font_size: 32.0,
        ..default()
    },
    TextColor(Color::WHITE),
    Node { ...optional styling... },
)
```

**Files fully migrated**:
- src/stage2/pause.rs (5 TextBundle instances converted)

## 🔄 Remaining Work

### 4. TextBundle Migration (52 remaining instances)
The following files still need TextBundle → (Text + TextFont + TextColor + Node) conversion:

**UI Files**:
- src/stage1/ui.rs
- src/stage1/pause.rs (remaining instances)
- src/stage2/ui.rs (20+ instances)
- src/stage3/ui.rs
- src/stage3/pause.rs
- src/stage4/ui.rs
- src/stage4/pause.rs
- src/stage5/ui.rs
- src/stage5/pause.rs

**Pattern to follow**: See src/stage2/pause.rs for the complete migration pattern.

### 5. Text Query Updates
Some systems query `&mut Text` and access `.sections[0]`. In Bevy 0.15:
- Text sections might be accessed differently
- Check if `text.0` or a different API is needed

**Files potentially affected**:
- src/stage1/visuals.rs (update_score_popups)
- src/stage2/visuals.rs (update_score_popups)
- Other files with text animation/update systems

### 6. API Method Changes
**delta_seconds() → delta_secs()**: ✅ Already fixed globally

**Other potential changes to check**:
- `AlignSelf::Global` removed → use different alignment
- Sprite color mutability changes
- Query filter changes

## 🚧 Build Blockers

### System Library Dependencies
The project cannot currently build due to missing system libraries:

```
Error: alsa.pc and libudev.pc not found
```

**Required packages** (Ubuntu/Debian):
```bash
sudo apt-get install libasound2-dev libudev-dev pkg-config
```

**Required packages** (Fedora/RHEL):
```bash
sudo dnf install alsa-lib-devel systemd-devel pkg-config
```

**Required packages** (Arch):
```bash
sudo pacman -S alsa-lib systemd pkg-config
```

Until these libraries are installed, the code cannot be compiled or tested.

## 📋 Migration Checklist

- [x] Remove incorrect bevy::text::TextStyle imports
- [x] Remove incorrect bevy::ui::Style imports
- [x] Convert all Text2dBundle → Text2d + TextFont + TextColor (6/6)
- [ ] Convert all TextBundle → Text + TextFont + TextColor + Node (5/57 completed)
- [ ] Update text query systems for new API
- [ ] Test compilation after system libraries installed
- [ ] Fix any remaining compilation errors
- [ ] Verify runtime behavior matches expected

## 🎯 Next Steps

1. **Install system libraries** on the build machine
2. **Complete TextBundle migration** following src/stage2/pause.rs pattern
3. **Run cargo build** and fix any remaining errors
4. **Test each stage** to ensure text rendering works correctly
5. **Check for performance issues** with the new required components pattern

## 📚 References

- [Bevy 0.15 Migration Guide](https://bevyengine.org/learn/migration-guides/0-14-to-0-15/)
- [Bevy 0.15 Required Components](https://bevyengine.org/news/bevy-0-15/#required-components)
- [Bevy 0.15 Text Rendering Changes](https://bevyengine.org/news/bevy-0-15/#text-rendering)
