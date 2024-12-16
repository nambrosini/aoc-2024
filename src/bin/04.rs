use aoc_util::{
    grid::{Grid, Inbound, Parse},
    position::Vec2,
    vecs::SortedVec,
};

advent_of_code::solution!(4);

const GOAL: &[char; 4] = &['X', 'M', 'A', 'S'];

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Grid<char> = Grid::parse(input);

    Some(
        grid.iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|&(_, &c)| c == 'X')
                    .map(|(j, _)| find_all_xmas(&grid, i, j))
                    .sum::<usize>()
            })
            .sum(),
    )
}

fn find_all_xmas(grid: &Grid<char>, i: usize, j: usize) -> usize {
    Vec2::DIR_WITH_DIAGONALS
        .iter()
        .filter(|dir| search_xmas(grid, i as i64, j as i64, dir))
        .count()
}

fn search_xmas(grid: &Grid<char>, x: i64, y: i64, inc: &Vec2) -> bool {
    if !grid.inbound(&Vec2::new(x + inc.x * 3, y + inc.y * 3)) {
        return false;
    }
    GOAL.iter().enumerate().all(|(i, &goal)| {
        grid[(x + inc.x * i as i64) as usize][(y + inc.y * i as i64) as usize] == goal
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);

    let mut res = 0;
    for (i, row) in grid.iter().enumerate().skip(1) {
        if i == grid.len() - 1 {
            continue;
        }
        for (j, c) in row.iter().enumerate().skip(1) {
            if j == grid.len() - 1 || c != &'A' {
                continue;
            }
            res += find_all_x_mas(&grid, i, j);
        }
    }
    Some(res)
}

fn find_all_x_mas(grid: &Grid<char>, i: usize, j: usize) -> u32 {
    let mut res = SortedVec::new();
    let mut res1 = SortedVec::new();
    for k in -1..=1 {
        res.push(grid[(i as i64 + k) as usize][(j as i64 + k) as usize]);
        res1.push(grid[(i as i64 + k) as usize][(j as i64 - k) as usize]);
    }

    if res == res1 {
        return 1;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
