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
fn main() {
    let (width, height) = (20, 20);
    let mut _game = Game::new(width, height);
    let mut _window: PistonWindow = WindowSettings::new("Snake Game", [width as u32 * 20, height as u32 * 20])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut _snake: Snake = Snake {body: vec![(16,32)], direction: Direction::Right};
    render(&mut _window, &mut _game);
}

fn render(_window: &mut PistonWindow, _game: &mut Game) {
    while let Some(e) = &_window.next() {
        _window.draw_2d(e, |c, g, _| {
            for (x, y) in &_game.snake.body {
                println!("MATO");
                rectangle([1.0, 0.0, 0.0, 1.0], // red
                    [*x as f64 * 20.0, *y as f64 * 20.0, 20.0, 20.0], // rectangle
                    c.transform,
                    g,
                );
            }
            for (x, y) in &_game.food {
                println!("FOOD");
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