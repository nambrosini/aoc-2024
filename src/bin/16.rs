use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use aoc_util::{
    direction::{Direction, Rotation},
    grid::{Find, Grid, Parse, Read},
    position::Vec2,
};

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<i64> {
    let grid = Grid::parse(input);
    let start = grid.find(&'S');
    let end = grid.find(&'E');

    Some(bfs(&grid, start, end).0)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = Grid::parse(input);
    let start = grid.find(&'S');
    let end = grid.find(&'E');

    let (_, points) = bfs(&grid, start, end);

    Some(points.len() + 1)
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Node {
    pos: Vec2,
    dir: Direction,
    cost: i64,
    path: Vec<Vec2>,
}

impl Node {
    fn new(pos: Vec2, dir: Direction, cost: i64, path: Vec<Vec2>) -> Self {
        Self {
            pos,
            dir,
            cost,
            path,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.cost.cmp(&other.cost) {
            std::cmp::Ordering::Less => Ordering::Greater,
            std::cmp::Ordering::Equal => Ordering::Equal,
            std::cmp::Ordering::Greater => Ordering::Less,
        }
    }
}

fn bfs(grid: &Grid<char>, start: Vec2, end: Vec2) -> (i64, HashSet<Vec2>) {
    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    let mut visited: HashMap<Vec2, i64> = HashMap::new();
    queue.push(Node::new(start, Direction::Right, 0, vec![]));
    visited.insert(start, 0);
    let rotations = [Rotation::Clock, Rotation::None, Rotation::Counter];

    let mut min_cost = i64::MAX;
    let mut all_points = HashSet::new();

    while let Some(node) = queue.pop() {
        if node.pos == end {
            if node.cost <= min_cost {
                min_cost = node.cost;
                for p in node.path {
                    all_points.insert(p);
                }
            }
            continue;
        }

        for rotation in &rotations {
            let new_dir = node.dir.rotate(rotation);
            let new_pos = node.pos + new_dir;
            if grid.get(&new_pos) == '#' {
                continue;
            }
            let new_cost = node.cost + if node.dir == new_dir { 1 } else { 1001 };
            let vis = visited.get(&new_pos).unwrap_or(&i64::MAX);

            if new_cost - 1000 > *vis {
                continue;
            } else if &new_cost < vis {
                visited.insert(new_pos, new_cost);
            }
            let mut path = node.path.clone();
            path.push(node.pos);
            queue.push(Node::new(new_pos, new_dir, new_cost, path));
        }
    }

    (min_cost, all_points)
}

#[allow(dead_code)]
fn print(grid: &Grid<char>, points: &HashSet<Vec2>) {
    for (i, row) in grid.iter().enumerate() {
        for (j, e) in row.iter().enumerate() {
            if points.contains(&Vec2::from_usize(i, j)) {
                print!("O");
            } else {
                print!("{}", e);
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
