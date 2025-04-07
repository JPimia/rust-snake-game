# Rust Snake Game

A classic Snake game implemented in Rust using the Piston game engine.

![Snake Game Screenshot](screenshot.png)

## Features

- Classic snake gameplay with WASD controls
- Score tracking and highscore persistence
- Clean, modular code structure
- Smooth rendering with Piston

## Requirements

- Rust (latest stable version)
- Cargo (comes with Rust)

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/rust-snake-game.git
   cd rust-snake-game
   ```

2. Build and run the game:
   ```
   cargo run
   ```

## Controls

- **W**: Move snake up
- **A**: Move snake left
- **S**: Move snake down
- **D**: Move snake right
- **ESC**: Exit game


## Development

The game is structured in a modular way:

- `constants.rs`: Contains all game constants, colors, and settings
- `snake.rs`: Implements the snake movement and behavior
- `game.rs`: Manages game state, collision detection, and food spawning
- `main.rs`: Handles the game loop, rendering, and user input

## Customization

You can easily customize the game by modifying the constants in `src/constants.rs`:

- Change colors
- Adjust game speed
- Modify grid size
- Update window dimensions

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Piston](https://github.com/PistonDevelopers/piston) - The game engine used
- [Fira Sans](https://github.com/mozilla/Fira) - The font used for text rendering
