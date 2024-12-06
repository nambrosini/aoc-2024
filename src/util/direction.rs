use super::position::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum Rotation {
    Clock,
    Counter,
}

impl From<Direction> for Vec2 {
    fn from(dir: Direction) -> Vec2 {
        match dir {
            Direction::Up => Vec2::new(-1, 0),
            Direction::Down => Vec2::new(1, 0),
            Direction::Left => Vec2::new(0, -1),
            Direction::Right => Vec2::new(0, 1),
        }
    }
}

impl Direction {
    pub fn rotate(&self, sense: Rotation) -> Self {
        match sense {
            Rotation::Clock => match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            },
            Rotation::Counter => match self {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
        }
    }
}
