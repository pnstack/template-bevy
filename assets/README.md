# Assets Directory

This directory contains all game assets organized by type:

## Structure

```
assets/
├── textures/     # Sprite sheets, images, UI elements
├── audio/        # Sound effects and music
│   ├── music/    # Background music tracks
│   └── sfx/      # Sound effects
└── fonts/        # Custom fonts for UI
```

## Adding Assets

1. **Textures**: Place PNG, JPEG, or other image files in `textures/`
2. **Audio**: Place OGG, WAV, or MP3 files in `audio/`
3. **Fonts**: Place TTF or OTF files in `fonts/`

## Loading Assets in Bevy

```rust
use bevy::prelude::*;

fn load_assets(asset_server: Res<AssetServer>) {
    // Load a texture
    let texture: Handle<Image> = asset_server.load("textures/player.png");
    
    // Load audio
    let music: Handle<AudioSource> = asset_server.load("audio/music/background.ogg");
    
    // Load a font
    let font: Handle<Font> = asset_server.load("fonts/game_font.ttf");
}
```
