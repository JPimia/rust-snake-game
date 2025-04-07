use piston_window::types::Color;

// Colors
pub const BACKGROUND_COLOR: Color = [0.2, 0.2, 0.2, 1.0];
pub const SNAKE_COLOR: Color = [1.0, 0.0, 0.0, 1.0];  // Red
pub const FOOD_COLOR: Color = [0.0, 1.0, 0.0, 1.0];  // Green
pub const GRID_COLOR: Color = [0.3, 0.3, 0.3, 1.0];
pub const TEXT_COLOR: Color = [1.0, 1.0, 1.0, 1.0];  // White
pub const SCORE_BG_COLOR: Color = [0.1, 0.1, 0.1, 0.8];

// Game settings
pub const GRID_SIZE: f64 = 20.0;
pub const TEXT_SIZE: u32 = 18;
pub const UPDATE_INTERVAL_MS: u64 = 150;

// Window settings
pub const WINDOW_WIDTH: f64 = 512.0;
pub const WINDOW_HEIGHT: f64 = 512.0;
pub const GRID_WIDTH: i32 = 20;
pub const GRID_HEIGHT: i32 = 20;

// UI positions
pub const SCORE_POS_X: f64 = 20.0;
pub const SCORE_POS_Y: f64 = 440.0;
pub const HIGHSCORE_POS_Y: f64 = 480.0;
pub const SCORE_BG_WIDTH: f64 = 200.0;
pub const SCORE_BG_HEIGHT: f64 = 40.0; 