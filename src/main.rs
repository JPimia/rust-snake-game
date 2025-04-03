use std::{clone, f32::consts::E, thread::sleep, time::Duration};

use piston_window::*;
use rand::{rng, Rng};

enum Direction {
    Up,
    Down,
    Right,
    Left,
}    

struct Snake {
    body: Vec<(i32, i32)>,
    direction: Direction,
}

struct Game {
    snake: Snake,
    food: Vec<(i32,i32)>,
    width: i32,
    height: i32,
    score: i32,
    game_over: bool
}

impl Game {
    fn new(width: i32, height: i32) -> Self {
        let mut rng = rng();
        Game {
            snake: Snake {
                body: vec![(width / 2, height / 2)],
                direction: Direction::Right,
        },
            food: vec![(rng.random_range(0..width),rng.random_range(0..height))],
            width,
            height,
            score: 0,
            game_over: false
        }
    }
}

impl Game {
    fn handle_input(&mut self, key: Event) {
        if let Some(Button::Keyboard(Key::W)) = key.press_args() {
            self.snake.direction = Direction::Right;
        }
        println!("{:?}", key)
    }
}

impl Game {
    fn move_forward(&mut self) {
        let mut head = self.snake.body[0];

        match self.snake.direction {
            Direction::Down => head.1 += 1,
            Direction::Left => head.0 -= 1,
            Direction::Right => head.0 += 1,
            Direction::Up => head.1 -= 1
        }
        println!("{:?}", head)
    }
}
fn main() {
    // grid size
    let (width, height) = (20, 20);

    // create game instance
    let mut _game = Game::new(width, height);

    // create window
    let mut _window: PistonWindow = WindowSettings::new("Snake Game", [width as u32 * 20, height as u32 * 20])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(e) = _window.next() {

        if let Some(Button::Keyboard(_key)) = e.press_args() {
            _game.handle_input(e);
        }
        _game.move_forward();
    }

    render(&mut _window, &mut _game);
}

fn render(_window: &mut PistonWindow, _game: &mut Game) {
    while let Some(e) = &_window.next() {
        _window.draw_2d(e, |c, g, _| {

            // draw snake
            for (x, y) in &_game.snake.body {
                // println!("MATO");
                rectangle([1.0, 0.0, 0.0, 1.0], // red
                    [*x as f64 * 20.0, *y as f64 * 20.0, 20.0, 20.0], // rectangle
                    c.transform,
                    g,
                );
            }

            // draw food
            for (x, y) in &_game.food {
                // println!("FOOD");
                rectangle([0.0, 1.0, 0.0, 1.0], // red
                    [*x as f64 * 20.0, *y as f64 * 20.0, 20.0, 20.0], // rectangle
                    c.transform,
                    g,
                );
            }
            clear([0.5, 0.5, 0.5, 1.0], g);
        });
    }
}