use piston_window::*;

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}    

pub struct Snake {
    pub body: Vec<(i32, i32)>,
    pub direction: Direction,
}

impl Snake {
    pub fn move_forward(&mut self) {
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