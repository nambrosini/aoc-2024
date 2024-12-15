use std::collections::HashSet;

use advent_of_code::util::{
    direction::{Direction, Rotation},
    grid::{Find, Grid, Inbound, Parse},
    position::Vec2,
};

advent_of_code::solution!(6);

const START: char = '^';

pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::parse(input);
    Some(find_visited(&grid).len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = Grid::parse(input);
    let start = grid.find(&START);
    let visited: HashSet<Vec2> = find_visited(&grid)
        .iter()
        .filter(|x| x != &&start)
        .cloned()
        .collect();
    Some(visited.iter().filter(|x| is_loop(&grid, &start, x)).count())
}

fn is_loop(grid: &Grid<char>, start: &Vec2, block: &Vec2) -> bool {
    let mut current = *start;
    let mut dir = Direction::Up;
    let mut visited = HashSet::new();

    loop {
        visited.insert((current, dir));
        let next = current + dir;
        if !grid.inbound(&next) {
            break;
        }
        if visited.contains(&(next, dir)) {
            return true;
        }
        if grid[next.x()][next.y()] == '#' || &next == block {
            dir = dir.rotate(Rotation::Clock);
            continue;
        }
        current = next;
    }
    false
}

fn find_visited(grid: &Grid<char>) -> HashSet<Vec2> {
    let mut current = grid.find(&START);
    let mut dir = Direction::Up;
    let mut visited = HashSet::new();

    loop {
        visited.insert(current);
        let next = current + dir;
        if !grid.inbound(&next) {
            break;
        }
        if grid[next.x()][next.y()] == '#' {
            dir = dir.rotate(Rotation::Clock);
            continue;
        }
        current = next;
    }

    visited
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
