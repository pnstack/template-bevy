# Template Bevy

A Bevy game engine template designed for indie game developers. This template provides a solid foundation with organized modules for components, systems, resources, states, and plugins.

## Features

- 🎮 Bevy game engine with ECS architecture
- 📁 Organized folder structure for game development
- 🧩 Modular plugin system
- 🎯 Game state management (Loading, Menu, Playing, Paused, GameOver)
- 🏃 Movement and physics components
- 🤖 Auto-movement system for obstacles and NPCs
- 💥 AABB collision detection system
- 🎯 Dynamic obstacle spawning with randomization
- 📷 Smooth camera follow system
- 🎨 User interface with score display and health bar
- ❤️ Health and combat systems
- 🎵 Audio and settings resources
- 🧪 Comprehensive test suite
- 🚀 CI/CD with GitHub Actions
- 📦 Cross-platform builds
- 🐳 Docker support
- ❄️ Nix flakes for reproducible environments

## Installation

### From Source

```bash
git clone https://github.com/pnstack/template-bevy.git
cd template-bevy
cargo build --release
```

### Quick Development Build

For faster compile times during development, use the `dev` feature:

```bash
cargo run --features dev
```

### From Releases

Download the latest binary from the [Releases](https://github.com/pnstack/template-bevy/releases) page.

## Usage

### Running the Game

```bash
# Development build (faster compilation)
cargo run --features dev

# Release build (optimized)
cargo run --release
```

### Controls

- **A/D** or **Arrow Left/Right** - Move player horizontally
- **Spacebar** - Jump (only when on ground)
- **ESC** - Quit game

## Project Structure

```
template-bevy/
├── .github/workflows/    # CI/CD workflows
├── src/
│   ├── components/       # ECS components (Player, Health, Speed, etc.)
│   ├── systems/          # ECS systems (movement, setup, etc.)
│   ├── resources/        # Global resources (Score, Settings, Timer)
│   ├── states/           # Game states (Loading, Menu, Playing, etc.)
│   ├── plugins/          # Custom Bevy plugins
│   ├── game/             # Core game logic and constants
│   ├── lib.rs            # Library root
│   └── main.rs           # Application entry point
├── assets/
│   ├── textures/         # Sprites and images
│   ├── audio/            # Music and sound effects
│   └── fonts/            # Custom fonts
├── tests/                # Integration tests
├── examples/             # Usage examples
└── docs/                 # Documentation
```

## Architecture

### Components

Components are data containers attached to entities:

```rust
use template_bevy::components::{Player, Health, Speed};

// Spawn a player entity
commands.spawn((
    Player,
    Health::new(100.0),
    Speed(200.0),
    Transform::default(),
));
```

### Systems

Systems process entities with specific components:

```rust
fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Speed, &mut Transform), With<Player>>,
) {
    // Movement logic here
}
```

### Resources

Resources are global state:

```rust
use template_bevy::resources::{Score, GameSettings};

fn update_score(mut score: ResMut<Score>) {
    score.add(100);
}
```

### States

States control game flow:

```rust
use template_bevy::states::GameState;

app.add_systems(Update, gameplay_system.run_if(in_state(GameState::Playing)));
```

## Core Systems

The template includes several ready-to-use systems:

### Auto-Movement

Entities with the `AutoMove` component will automatically move in the specified direction:

```rust
use template_bevy::components::{AutoMove, Obstacle};

// Create an obstacle that moves left
commands.spawn((
    Obstacle,
    AutoMove::left(150.0), // Speed of 150 pixels/second
    // ... other components
));
```

### Obstacle Spawning

The `spawn_obstacles` system automatically spawns obstacles at regular intervals with random properties (position, size, speed).

### Camera Follow

The camera automatically follows the player with smooth interpolation:

```rust
use template_bevy::components::{CameraFollow, MainCamera};

// Camera will smoothly follow the target
commands.spawn((
    MainCamera,
    CameraFollow::new(player_entity)
        .with_offset(Vec3::new(0.0, 50.0, 0.0))
        .with_smoothing(0.05),
    Camera2dBundle::default(),
));
```

### Collision Detection

AABB collision detection between player and obstacles:

- Platform collisions for landing and ground detection
- Obstacle collisions that apply damage to the player

### User Interface

Built-in UI components for displaying game information:

- **Score Display**: Shows current score in the top-left corner
- **Health Bar**: Visual health indicator with color changes based on health level

## Development

### Prerequisites

- Rust 1.70 or later
- For Linux: `libasound2-dev`, `libudev-dev` (audio and input support)

```bash
# Ubuntu/Debian
sudo apt-get install libasound2-dev libudev-dev

# Fedora
sudo dnf install alsa-lib-devel systemd-devel
```

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Development build with dynamic linking (faster iteration)
cargo build --features dev
```

### Running Tests

```bash
cargo test
```

### Running Clippy (Linter)

```bash
cargo clippy -- -D warnings
```

### Formatting Code

```bash
cargo fmt
```

## Adding Game Content

### Adding a New Component

1. Create the component in `src/components/mod.rs`:

```rust
#[derive(Component, Debug)]
pub struct Enemy {
    pub damage: f32,
}
```

2. Export it in the module.

### Adding a New System

1. Create a new file in `src/systems/`:

```rust
pub fn enemy_ai(query: Query<&Transform, With<Enemy>>) {
    // AI logic
}
```

2. Export it in `src/systems/mod.rs`
3. Register it in `src/plugins/mod.rs`

### Adding a New State

1. Add the state variant in `src/states/mod.rs`
2. Add state-specific systems in your plugin

## Performance Tips

1. Use `--features dev` during development for faster compile times
2. Use `--release` for testing actual game performance
3. The template includes optimized dependency builds by default

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
