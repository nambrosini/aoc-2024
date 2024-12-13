use std::ops::{Add, Mul, Sub};
use std::str::FromStr;

use super::direction::Direction;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

pub const fn v(x: i64, y: i64) -> Vec2 {
    Vec2::new(x, y)
}

impl Vec2 {
    pub const NORTH: Self = v(0, -1);
    pub const EAST: Self = v(1, 0);
    pub const SOUTH: Self = v(0, 1);
    pub const WEST: Self = v(-1, 0);

    pub const DIRECTIONS: [Self; 4] = [Self::NORTH, Self::EAST, Self::SOUTH, Self::WEST];
    pub const DIR_WITH_DIAGONALS: [Self; 8] = [
        Self::NORTH,
        v(-1, 1),
        Self::EAST,
        v(1, 1),
        Self::SOUTH,
        v(1, -1),
        Self::WEST,
        v(-1, -1),
    ];

    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub const fn abs(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    pub const fn from_usize(x: usize, y: usize) -> Self {
        Vec2::new(x as i64, y as i64)
    }

    pub fn x(&self) -> usize {
        self.x as usize
    }

    pub fn y(&self) -> usize {
        self.y as usize
    }

    pub fn distance(&self, other: &Self) -> u64 {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }

    pub fn distance_2d(&self, other: &Self) -> Self {
        Self {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }
}

impl FromStr for Vec2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Vec2::NORTH),
            "R" => Ok(Vec2::EAST),
            "D" => Ok(Vec2::SOUTH),
            "L" => Ok(Vec2::WEST),
            _ => Err(()),
        }
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        v(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<i64> for Vec2 {
    type Output = Self;

    fn add(self, rhs: i64) -> Self::Output {
        v(self.x + rhs, self.y + rhs)
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Vec2) -> Self::Output {
        v(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<i64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i64) -> Self::Output {
        v(self.x * rhs, self.y * rhs)
    }
}

impl Add<Direction> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Direction) -> Self::Output {
        let dir: Vec2 = rhs.into();
        self + dir
    }
}
