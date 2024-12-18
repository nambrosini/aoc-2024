use std::collections::{BinaryHeap, HashMap};

use aoc_util::{
    grid::{Grid, Inbound, Read, Write},
    position::Vec2,
};

advent_of_code::solution!(18);

fn parse(input: &str) -> Vec<Vec2> {
    input
        .trim()
        .lines()
        .map(|l| {
            let s: Vec<i64> = l.split(',').map(|x| x.parse().unwrap()).collect();
            Vec2::new(s[1], s[0])
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let bytes = parse(input);
    let mut grid = Grid::create_empty(71, 71, '.');

    (0..1024).for_each(|i| {
        grid.set(&bytes[i], '#');
    });

    Some(
        astar(&grid, Vec2::new(0, 0), Vec2::new(70, 70))
            .unwrap()
            .len(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let bytes = parse(input);
    let mut grid = Grid::create_empty(71, 71, '.');

    (0..1024).for_each(|i| {
        grid.set(&bytes[i], '#');
    });

    let mut last_path: Vec<Vec2> = astar(&grid, Vec2::new(0, 0), Vec2::new(70, 70)).unwrap();
    for b in bytes.iter().skip(1024) {
        grid.set(b, '#');
        if !last_path.contains(b) {
            continue;
        }
        if let Some(path) = astar(&grid, Vec2::new(0, 0), Vec2::new(70, 70)) {
            last_path = path;
        } else {
            return Some(format!("{},{}", b.y, b.x));
        }
    }

    None
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    pos: Vec2,
    f: i64,
}

impl Node {
    fn new(pos: Vec2, f: i64) -> Self {
        Self { pos, f }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f.cmp(&self.f)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn reconstruct_path(came_from: &HashMap<Vec2, Vec2>, start: Vec2, goal: Vec2) -> Vec<Vec2> {
    let mut path = Vec::new();
    let mut current = goal;
    while current != start {
        path.push(current);
        current = came_from[&current];
    }
    path.push(start);
    path.reverse();
    path
}

fn astar(grid: &Grid<char>, start: Vec2, goal: Vec2) -> Option<Vec<Vec2>> {
    let mut open_set = BinaryHeap::new();
    open_set.push(Node::new(start, start.manhattan(&goal)));

    let mut came_from = HashMap::new();

    let mut g_score = HashMap::new();
    g_score.insert(start, 0i64);

    while let Some(current) = open_set.pop() {
        if current.pos == goal {
            return Some(reconstruct_path(&came_from, start, goal));
        }

        let current_g = g_score[&current.pos];
        for dir in Vec2::DIRECTIONS {
            let new_pos = current.pos + dir;
            if !grid.inbound(&new_pos) || grid.get(&new_pos) == '#' {
                continue;
            }
            let new_g = current_g + 1;

            let neighbor_g = g_score
                .get(&(current.pos + dir))
                .copied()
                .unwrap_or(i64::MAX);
            if new_g < neighbor_g {
                came_from.insert(new_pos, current.pos);
                g_score.insert(new_pos, new_g);
                let f = new_g + new_pos.manhattan(&goal);
                open_set.push(Node::new(new_pos, f));
            }
        }
    }
    None
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_part_one() {
//         let result = part_one(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, Some(22));
//     }
//
//     #[test]
//     fn test_part_two() {
//         let result = part_two(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, Some("6,1".to_string()));
//     }
// }
