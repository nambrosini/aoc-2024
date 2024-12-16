use std::collections::HashSet;

use aoc_util::grid::{Find, Grid, Inbound, Parse};
use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<i64> {
    let grid: Grid<char> = Grid::parse(input);
    let symbols: Vec<&char> = grid
        .iter()
        .flatten()
        .filter(|e| e != &&'.')
        .unique()
        .collect();

    let mut set = HashSet::new();
    for symbol in symbols {
        let positions = grid.find_all(symbol);
        let combi: Vec<_> = positions.iter().permutations(2).collect();
        for c in combi {
            let dist = c[0].distance_2d(c[1]);
            let antinode = *c[0] - dist;
            if grid.inbound(&antinode) {
                set.insert(antinode);
            }
        }
    }

    Some(set.len() as i64)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Grid<char> = Grid::parse(input);
    let mut new_grid: Grid<char> = grid.clone();
    let symbols: Vec<&char> = grid
        .iter()
        .flatten()
        .filter(|e| e != &&'.')
        .unique()
        .collect();

    let mut set = HashSet::new();
    for symbol in symbols {
        let positions = grid.find_all(symbol);
        let combi: Vec<_> = positions.iter().permutations(2).collect();
        for c in combi {
            let dist = c[0].distance_2d(c[1]);
            let mut current = *c[0] + dist;
            while grid.inbound(&current) {
                new_grid[current.x()][current.y()] = '#';
                set.insert(current);
                current += dist;
            }
        }
    }

    Some(set.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
