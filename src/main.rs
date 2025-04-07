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
        let _game_over = self.game_over;
        let head = self.snake.body[0];

        if head.0 < 0 || head.1 > 19 || head.0 > 19 || head.1 < 0 {
            println!("OKJSSADKSADOKASD");
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
                    println!("ASDASD");
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
            _game.snake.move_forward();
            _game.spawn_food();
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