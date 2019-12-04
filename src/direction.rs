
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    pub fn oposite(&self) -> Direction {
        match self {
            Direction::UP => Direction::DOWN,
            Direction::RIGHT => Direction::LEFT,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
        }
    }
}