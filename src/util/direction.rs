use std::ops::Mul;

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
    None,
    Opposite,
}

impl From<Vec2> for Direction {
    fn from(value: Vec2) -> Self {
        match value {
            Vec2 { x: -1, y: 0 } => Direction::Up,
            Vec2 { x: 1, y: 0 } => Direction::Down,
            Vec2 { x: 0, y: -1 } => Direction::Left,
            Vec2 { x: 0, y: 1 } => Direction::Right,
            _ => unreachable!(),
        }
    }
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
            Rotation::None => *self,
            Rotation::Opposite => match self {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
            },
        }
    }
}

impl Mul<i64> for Direction {
    type Output = Vec2;

    fn mul(self, rhs: i64) -> Self::Output {
        let v: Vec2 = self.into();
        v * rhs
    }
}
