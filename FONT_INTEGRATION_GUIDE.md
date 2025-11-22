# Font Integration Guide

## Overview

This document explains how custom fonts, including emoji support, have been integrated into TileMania.

## Available Fonts

The game now supports four font types:

1. **Regular** (FiraSans-Regular.ttf) - Default font for body text
2. **Medium** (FiraSans-Medium.ttf) - Medium weight for emphasis
3. **Bold** (FiraSans-Bold.ttf) - Bold weight for headings
4. **Emoji** (NotoColorEmoji-Regular.ttf) - Color emoji support

## Font Loading System

Fonts are automatically loaded at startup through the `AssetPlugin` in `src/plugins/assets.rs`:

```rust
// Fonts are loaded and stored in GameAssets resource
pub struct GameAssets {
    pub fonts: HashMap<String, Handle<Font>>,
    // ... other fields
}
```

The font loading happens in `start_loading_assets()` and fonts become available once `GameAssets::is_loaded()` returns true.

## Using Fonts in UI Components

### Basic Text (Default Font)

```rust
use crate::ui::components::{TextComponent, TextStyle, TextColorVariant};

// Uses default Regular font
TextComponent::spawn(
    &mut commands,
    "Hello World",
    TextStyle::Body,
    TextColorVariant::Primary,
);
```

### Text with Custom Font

```rust
use crate::ui::components::{TextComponent, TextStyle, TextColorVariant, FontType};
use crate::plugins::assets::GameAssets;

fn my_ui_system(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    // Using Bold font
    TextComponent::spawn_with_font(
        &mut commands,
        "Important Message!",
        TextStyle::Heading,
        TextColorVariant::Primary,
        FontType::Bold,
        Some(&game_assets),
    );
}
```

### Text with Emoji

```rust
use crate::ui::components::{TextComponent, TextStyle, TextColorVariant, FontType};
use crate::plugins::assets::GameAssets;

fn emoji_ui_system(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    // Display emoji text
    TextComponent::spawn_with_font(
        &mut commands,
        "Great job! 🎉 You earned a bonus! ⭐",
        TextStyle::Body,
        TextColorVariant::Success,
        FontType::Emoji,
        Some(&game_assets),
    );
}
```

### Text with Node Styling and Custom Font

```rust
use bevy::prelude::*;
use crate::ui::components::{TextComponent, TextStyle, TextColorVariant, FontType};
use crate::plugins::assets::GameAssets;

fn styled_text_system(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    let node = Node {
        margin: UiRect::all(Val::Px(10.0)),
        padding: UiRect::all(Val::Px(5.0)),
        ..default()
    };

    TextComponent::spawn_with_node_and_font(
        &mut commands,
        "Styled Text 🌟",
        TextStyle::Subheading,
        TextColorVariant::Accent,
        node,
        FontType::Emoji,
        Some(&game_assets),
    );
}
```

## Font Types

The `FontType` enum provides type-safe font selection:

```rust
pub enum FontType {
    Regular,  // FiraSans-Regular
    Medium,   // FiraSans-Medium
    Bold,     // FiraSans-Bold
    Emoji,    // NotoColorEmoji-Regular
}
```

## Best Practices

### 1. Font Selection

- Use **Regular** for body text and descriptions
- Use **Medium** for subtle emphasis or subheadings
- Use **Bold** for headings and important labels
- Use **Emoji** when displaying emoji characters or kid-friendly icons

### 2. Emoji Support

When using emojis in your text:
- Set the `font_type` to `FontType::Emoji`
- Pass `Some(&game_assets)` to ensure the font is loaded
- Check that `game_assets.is_loaded()` is true before creating emoji text

### 3. Performance

- Fonts are loaded once at startup and reused
- Font handles are stored in `GameAssets` resource
- No need to reload fonts for each text component

## Example: Mixed Font UI

```rust
fn create_score_display(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    // Wait for assets to load
    if !game_assets.is_loaded() {
        return;
    }

    // Title in Bold
    TextComponent::spawn_with_font(
        &mut commands,
        "SCORE",
        TextStyle::Title,
        TextColorVariant::Primary,
        FontType::Bold,
        Some(&game_assets),
    );

    // Score value in Medium
    TextComponent::spawn_with_font(
        &mut commands,
        "1,250",
        TextStyle::Heading,
        TextColorVariant::Accent,
        FontType::Medium,
        Some(&game_assets),
    );

    // Bonus indicator with emoji
    TextComponent::spawn_with_font(
        &mut commands,
        "BONUS! 🎯",
        TextStyle::Body,
        TextColorVariant::Success,
        FontType::Emoji,
        Some(&game_assets),
    );
}
```

## Integration Checklist

- [x] NotoColorEmoji-Regular.ttf added to `assets/fonts/`
- [x] Font loading system updated in `src/plugins/assets.rs`
- [x] `FontType` enum added to `src/ui/components/text.rs`
- [x] Font support methods added to `TextComponent`
- [x] Public exports updated in `src/ui/components/mod.rs`
- [ ] Update UI screens to use custom fonts where appropriate
- [ ] Add emoji support to reward/feedback messages
- [ ] Test emoji rendering on different platforms

## Troubleshooting

### Fonts not loading?

Check that `game_assets.state == AssetLoadingState::Loaded` before using fonts.

### Emoji not displaying correctly?

Ensure you're using `FontType::Emoji` and passing `Some(&game_assets)` to the spawn methods.

### Font file not found?

Verify the font files are in `assets/fonts/` and the paths in `start_loading_assets()` are correct.

## Future Enhancements

- Font fallback chains (primary font with emoji fallback)
- Dynamic font loading for additional languages
- Font size presets for different screen sizes
- RTL (right-to-left) text support for internationalization
