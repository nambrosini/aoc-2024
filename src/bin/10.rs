use std::collections::HashSet;

use aoc_util::{
    grid::{Find, Grid, Inbound, Parse},
    position::Vec2,
};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<i64> {
    let grid = Grid::parse(input);
    let starts = grid.find_all(&0);
    let mut sum = 0;
    for start in starts {
        let mut set = HashSet::new();
        trailheads(&grid, start, &mut set);
        sum += set.len();
    }
    Some(sum as i64)
}

fn trailheads(grid: &Grid<i64>, start: Vec2, visited: &mut HashSet<Vec2>) {
    let current_value = grid[start.x()][start.y()];
    if grid[start.x()][start.y()] == 9 {
        visited.insert(start);
        return;
    }

    for dir in Vec2::DIRECTIONS {
        let new_pos = start + dir;
        if !grid.inbound(&new_pos) {
            continue;
        }
        let new_value = grid[new_pos.x()][new_pos.y()];
        if new_value == current_value + 1 {
            trailheads(grid, new_pos, visited);
        }
    }
}

pub fn part_two(input: &str) -> Option<i64> {
    let grid = Grid::parse(input);
    let starts = grid.find_all(&0);
    let mut sum = 0;
    for start in starts {
        let v = distinct_trailheads(&grid, start);
        sum += v;
    }
    Some(sum)
}

fn distinct_trailheads(grid: &Grid<i64>, start: Vec2) -> i64 {
    let current_value = grid[start.x()][start.y()];
    if grid[start.x()][start.y()] == 9 {
        return 1;
    }

    let mut sum = 0;
    for dir in Vec2::DIRECTIONS {
        let new_pos = start + dir;
        if !grid.inbound(&new_pos) {
            continue;
        }
        let new_value = grid[new_pos.x()][new_pos.y()];
        if new_value == current_value + 1 {
            sum += distinct_trailheads(grid, new_pos);
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
