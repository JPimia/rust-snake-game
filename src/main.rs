use piston_window::*;
use piston_window::Glyphs;
use piston_window::TextureSettings;
use snake_game::game::Game;
use snake_game::game::HIGHSCORE;
use snake_game::constants::*;

fn main() {
    let (width, height) = (GRID_WIDTH, GRID_HEIGHT);

    let mut _game = Game::new(width, height);

    let mut _window: PistonWindow = WindowSettings::new("Snake Game", [width as f64 * GRID_SIZE, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut _glyphs = Glyphs::new(
        "assets/FiraSans-Regular.ttf",
        _window.create_texture_context(),
        TextureSettings::new(),
    ).unwrap_or_else(|e| {
        println!("Error: {}", e);
        panic!("Failed to create glyphs");
    });

    let mut last_update = std::time::Instant::now();
    let update_interval = std::time::Duration::from_millis(UPDATE_INTERVAL_MS);

    while let Some(e) = _window.next() {
        if _game.game_over {
            unsafe {
                if _game.score > HIGHSCORE {
                    HIGHSCORE = _game.score;
                }
            }
            _game = Game::new(width, height);
        }

        if let Some(Button::Keyboard(_key)) = e.press_args() {
            _game.handle_input(_key);
        }

        if last_update.elapsed() >= update_interval {
            _game.snake.move_forward();
            _game.spawn_food();
            _game.check_collusion();
            last_update = std::time::Instant::now();
        }
        render(&mut _window, &e, &mut _game, &mut _glyphs);
    }
}

fn render(_window: &mut PistonWindow, _event: &Event, _game: &mut Game, _glyphs: &mut Glyphs) {
    _window.draw_2d(_event, |c, g, device| {
        clear(BACKGROUND_COLOR, g);
        
        // Score background rectangle
        rectangle(
            SCORE_BG_COLOR,
            [10.0, SCORE_POS_Y - 30.0, SCORE_BG_WIDTH, SCORE_BG_HEIGHT],
            c.transform,
            g
        );

        // Highscore background rectangle
        rectangle(
            SCORE_BG_COLOR,
            [10.0, HIGHSCORE_POS_Y - 30.0, SCORE_BG_WIDTH, SCORE_BG_HEIGHT],
            c.transform,
            g
        );
        
        // Draw score
        let score_text = format!("Score: {}", _game.score);
        if let Err(e) = text::Text::new_color(TEXT_COLOR, TEXT_SIZE)
            .draw(&score_text, _glyphs, &c.draw_state, c.transform.trans(SCORE_POS_X, SCORE_POS_Y), g)
        {
            eprintln!("Failed to draw text: {:?}", e);
        }

        // Draw highscore
        let score_text = format!("Highscore: {}", unsafe { HIGHSCORE });
        if let Err(e) = text::Text::new_color(TEXT_COLOR, TEXT_SIZE)
            .draw(&score_text, _glyphs, &c.draw_state, c.transform.trans(SCORE_POS_X, HIGHSCORE_POS_Y), g)
        {
            eprintln!("Failed to draw text: {:?}", e);
        }
        
        // Draw play area
        rectangle(GRID_COLOR,
            [1.0, 1.0 * 2.0, _game.height as f64 * GRID_SIZE, _game.height as f64 * GRID_SIZE],
            c.transform,
            g
        );
        
        // Draw snake
        for (x, y) in &_game.snake.body {
            rectangle(SNAKE_COLOR,
                [*x as f64 * GRID_SIZE, *y as f64 * GRID_SIZE, GRID_SIZE, GRID_SIZE],
                c.transform,
                g,
            );
        }

        // Draw food
        for (x, y) in &_game.food {
            rectangle(FOOD_COLOR,
                [*x as f64 * GRID_SIZE, *y as f64 * GRID_SIZE, GRID_SIZE, GRID_SIZE],
                c.transform,
                g,
            );
        }
        _glyphs.factory.encoder.flush(device);
    });
}