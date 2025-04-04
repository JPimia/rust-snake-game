use piston_window::*;
use rand::{rng, Rng};

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
        println!("{:?}", key)
    }
}

impl Game {
    fn check_collusion(&mut self) {
        let game_over = self.game_over;
        let (x, y) = self.snake.body[0];
        let last_pos = self.snake.body[0];
        

        if (x, y) < (0, 0) || (x, y) > (19, 19) || (y, x) < (0, 0) || (y, x) > (19, 19){
            self.game_over = true;
        }
        if (x, y) == self.food[0] {
            self.food.pop();
            let rng = rng().random_range(0..19);
            self.food.insert(0, (rng, rng));
            self.snake.body.insert(0, last_pos);
            self.score += 1;
            println!("{:?}", self.food)
        }
        println!("Game over: {}, score: {}", game_over, self.score)
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
    // grid size
    let (width, height) = (20, 20);

    // create game instance
    let mut _game = Game::new(width, height);

    // create window
    let mut _window: PistonWindow = WindowSettings::new("Snake Game", [width as u32 * 20, height as u32 * 20])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut last_update = std::time::Instant::now();
    let update_interval = std::time::Duration::from_millis(150);

    println!("{:?}", update_interval);

    while let Some(e) = _window.next() {

        if _game.game_over {
            println!("Moroojesta kuolit!");
            break;
        }

        if let Some(Button::Keyboard(_key)) = e.press_args() {
            println!("moi");
            _game.handle_input(_key);
        }

        if last_update.elapsed() >= update_interval {
            println!("moi");
            _game.snake.move_forward();
            _game.check_collusion();
            last_update = std::time::Instant::now();
        }

        render(&mut _window, &e, &mut _game);
    }

}

fn render(_window: &mut PistonWindow, _event: &Event, _game: &mut Game) {
    _window.draw_2d(_event, |c, g, _| {
        clear([0.5, 0.5, 0.5, 1.0], g);
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
    });
    
}