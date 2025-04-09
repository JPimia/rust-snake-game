use piston_window::*;
use rand::{rng, Rng};
use crate::snake::{Snake, Direction};

pub static mut HIGHSCORE: i32 = 0;

pub struct Game {
    pub snake: Snake,
    pub food: Vec<(i32,i32)>,
    pub width: i32,
    pub height: i32,
    pub score: i32,
    pub game_over: bool
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
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

    pub fn handle_input(&mut self, key: Key) {
        match key {
            Key::W => if self.snake.direction != Direction::Down { self.snake.direction = Direction::Up },
            Key::D => if self.snake.direction != Direction::Left { self.snake.direction = Direction::Right },
            Key::S => if self.snake.direction != Direction::Up { self.snake.direction = Direction::Down },
            Key::A => if self.snake.direction != Direction::Right { self.snake.direction = Direction::Left },
            _ => {},
        }
    }

    pub fn check_collision(&mut self) {
        let head = self.snake.body[0];
        let is_out_of_bounds = !(0..self.width).contains(&head.0) ||
                                !(0..self.height).contains(&head.1); 

        let self_collided = self.snake.body
            .iter()
            .skip(1)
            .any(|seg| seg == &head);

        self.game_over = is_out_of_bounds || self_collided;
    }

    pub fn spawn_food(&mut self) {
        let &last_pos = self.snake.body.last().unwrap();
        let head = self.snake.body[0];
        let (x, y) = last_pos;

        if self.food[0] == head {
            
            let new_food = std::iter::repeat_with(|| {
                (rng().random_range(0..self.width), rng().random_range(0..self.height))
            })
            .find(|pos| !self.snake.body.contains(pos))
            .unwrap();

            self.food.pop();
            self.food.push(new_food);

            if !self.snake.body.iter().any(|segment| segment == &new_food) {
                
                let len = self.snake.body.len();
                if len >= 2 {
                    // get the last segment and the second to last segment
                    let tail = self.snake.body[len - 1];
                    let previous_seg = self.snake.body[len - 2];
                    // calculate the difference between the last segment and the second to last segment
                    let dx = tail.0 - previous_seg.0;
                    let dy = tail.1 - previous_seg.1;
                    // add the difference to the last segment to get the new segment
                    let new_seg = (tail.0 + dx, tail.1 + dy);
                    // push the new segment to the snake's body
                    self.snake.body.push(new_seg);
                } else {
                    match self.snake.direction {
                        Direction::Up => self.snake.body.push((x, y + 1)),
                        Direction::Down => self.snake.body.push((x, y - 1)),
                        Direction::Right => self.snake.body.push((x - 1, y)),
                        Direction::Left => self.snake.body.push((x + 1, y)),
                    }
                }
                self.score += 1;
            }
            
        }
    }
} 