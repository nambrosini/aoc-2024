use crate::util::position::Vec2;
use std::fmt::Display;

pub type Grid<T> = Vec<Vec<T>>;

pub trait Print {
    fn print(&self);
    fn string(&self) -> String;
}

pub trait Find<T> {
    fn find(&self, value: T) -> Vec2;
    fn find_all(&self, value: T) -> Vec<Vec2>;
}

pub trait Inbound<T> {
    fn inbound(&self, point: T) -> bool;
}

pub trait Parse {
    fn parse(input: &str) -> Self;
}

impl<T> Find<T> for Grid<T>
where
    T: Eq,
{
    fn find(&self, value: T) -> Vec2 {
        for (i, row) in self.iter().enumerate() {
            if let Some(j) = row.iter().position(|e| e == &value) {
                return Vec2::from_usize(i, j);
            }
        }

        unreachable!()
    }

    fn find_all(&self, value: T) -> Vec<Vec2> {
        let mut res = Vec::new();
        for (i, row) in self.iter().enumerate() {
            for (j, e) in row.iter().enumerate() {
                if e == &value {
                    res.push(Vec2::from_usize(i, j));
                }
            }
        }
        res
    }
}

impl<T> Print for Grid<T>
where
    T: Display,
{
    fn print(&self) {
        for row in self {
            for e in row {
                print!("{}", e);
            }
            println!()
        }
        println!()
    }

    fn string(&self) -> String {
        let mut res = String::new();
        for row in self {
            for e in row {
                res.push_str(&e.to_string());
            }
            res.push('\n');
        }
        res[..res.len() - 1].to_string()
    }
}

impl<T> Inbound<&Vec2> for Grid<T> {
    fn inbound(&self, pos: &Vec2) -> bool {
        pos.x >= 0 && pos.x < self.len() as i64 && pos.y >= 0 && pos.y < self[0].len() as i64
    }
}

impl<T: From<char>> Parse for Grid<T> {
    fn parse(input: &str) -> Self {
        input
            .lines()
            .map(|c| c.chars().map(|c| c.into()).collect())
            .collect()
    }
}
