use piston_window::*;
use piston_window::Glyphs;
use piston_window::TextureSettings;
use rand::{rng, Rng};

static mut HIGHSCORE: i32 = 0;

#[derive(PartialEq)]
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

pub struct Game {
    snake: Snake,
    food: Vec<(i32,i32)>,
    width: i32,
    height: i32,
    score: i32,
    pub highscore: i32,
    game_over: bool
}

impl Game {
    fn new(width: i32, height: i32) -> Self {
        let rng = rng().random_range(0..width);
        Game {
            snake: Snake {
                body: vec![(width / 2, height / 2)],
                direction: Direction::Right,
        },
            food: vec![(rng, rng)],
            width,
            height,
            score: 0,
            highscore: 0,
            game_over: false
        }
    }
}

impl Game {
    fn handle_input(&mut self, key: Key) {
        match key {
            Key::W => if self.snake.direction != Direction::Down { self.snake.direction = Direction::Up },
            Key::D => if self.snake.direction != Direction::Left { self.snake.direction = Direction::Right },
            Key::S => if self.snake.direction != Direction::Up { self.snake.direction = Direction::Down },
            Key::A => if self.snake.direction != Direction::Right { self.snake.direction = Direction::Left },
            _ => {},
        }
    }
}

impl Game {
    fn check_collusion(&mut self) {
        let _game_over = self.game_over;
        let head = self.snake.body[0];

        if head.0 < 0 || head.1 > self.width - 1 || head.0 > self.height - 1 || head.1 < 0 {
            self.game_over = true;
        }
        if self.snake.body.iter().skip(1).any(|segment| segment == &head) {
            self.game_over = true;
        }
        // println!("Game over: {}, score: {}", game_over, self.score)
    }
}

impl Game {
    fn spawn_food(&mut self) {
        let &last_pos = self.snake.body.last().unwrap();
        let head = self.snake.body[0];
        let (x, y) = last_pos;

        if self.food[0] == head {
            loop { 
                let new_food = (rng().random_range(0..self.width), rng().random_range(0..self.height));
                if !self.snake.body.iter().any(|segment| segment == &new_food) {
                    self.food.pop();
                    self.food.push(new_food);

                    let len = self.snake.body.len();
                    if len >= 2 {
                        // get last and second to last segments
                        let tail = self.snake.body[len - 1];
                        let previous_seg = self.snake.body[len - 2];

                        // calculate direction from second to last segment to tail
                        let dx = tail.0 - previous_seg.0;
                        let dy = tail.1 - previous_seg.1;

                        // add new segment to opposite direction from second to last segment
                        let new_seg = (tail.0 + dx, tail.1 + dy);
                        self.snake.body.push(new_seg);
                    } else {
                        // if snake has only head use current direction
                        match self.snake.direction {
                            Direction::Up => self.snake.body.push((x, y + 1)),
                            Direction::Down => self.snake.body.push((x, y - 1)),
                            Direction::Right => self.snake.body.push((x - 1, y)),
                            Direction::Left => self.snake.body.push((x + 1, y)),
                        }
                    }
                    self.score += 1;
                    break;
                }
            }
        }
        println!("score: {}", self.score)
    }
}

impl Snake {
    fn move_forward(&mut self) {
        let mut head = self.body[0];
        
        println!("{:?}", head);
        match self.direction {
            Direction::Up => head.1 -= 1,
            Direction::Down => head.1 += 1, 
            Direction::Right => head.0 += 1,
            Direction::Left => head.0 -= 1,
        }
        self.body.insert(0, head);
        self.body.pop();
    }
}
fn main() {
    let (_window_width, window_height) = (512, 512);
    // grid size
    let (width, height) = (20, 20);

    // create game instance
    let mut _game = Game::new(width, height);

    // create window
    let mut _window: PistonWindow = WindowSettings::new("Snake Game", [width as f64 * 20.0, window_height as f64])
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
    let update_interval = std::time::Duration::from_millis(150);

    println!("{:?}", update_interval);

    while let Some(e) = _window.next() {

        if _game.game_over {
            println!("Moroojesta kuolit!");
            unsafe {
                if _game.score > HIGHSCORE {
                    HIGHSCORE = _game.score;
                }
            }
            _game = Game::new(width, height);
        }

        if let Some(Button::Keyboard(_key)) = e.press_args() {
            println!("moi");
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
        clear([0.2, 0.2, 0.2, 1.0], g);
        // Draw background rectangle for score
        rectangle(
            [0.1, 0.1, 0.1, 0.8],
            [10.0, 410.0, 200.0, 40.0],
            c.transform,
            g
        );

        // draw background for highscore
        rectangle(
            [0.1, 0.1, 0.1, 0.8],
            [10.0, 460.0, 200.0, 40.0],
            c.transform,
            g
        );
        
        // Draw score text
        let score_text = format!("Score: {}", _game.score);
        if let Err(e) = text::Text::new_color([1.0, 1.0, 1.0, 1.0], 18)
            .draw(&score_text, _glyphs, &c.draw_state, c.transform.trans(20.0, 440.0), g)
        {
            eprintln!("Failed to draw text: {:?}", e);
        }

        let score_text = format!("Highscore: {}", unsafe { HIGHSCORE });
        if let Err(e) = text::Text::new_color([1.0, 1.0, 1.0, 1.0], 18)
            .draw(&score_text, _glyphs, &c.draw_state, c.transform.trans(20.0, 490.0), g)
        {
            eprintln!("Failed to draw text: {:?}", e);
        }
        
        rectangle([0.3, 0.3, 0.3, 1.0],
            [1.0, 1.0 * 2.0, _game.height as f64 * 20.0, _game.height as f64 * 20.0],
            c.transform,
            g
        );
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
        _glyphs.factory.encoder.flush(device);
    });
}