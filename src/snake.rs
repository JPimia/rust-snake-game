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
        let new_head = self.body.first()
            .map(|&(x, y)| match self.direction {
                Direction::Up => (x, y - 1),
                Direction::Down => (x, y + 1),
                Direction::Right => (x + 1, y),
                Direction::Left => (x - 1, y),
            }).unwrap();
        
        self.body.insert(0, new_head);
        self.body.pop();
    }
}