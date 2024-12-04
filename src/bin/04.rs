use advent_of_code::util::{
    grid::{Grid, Inbound, Parse},
    position::Vec2,
    vecs::SortedVec,
};

advent_of_code::solution!(4);

const GOAL: &str = "XMAS";

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Grid<char> = Grid::parse(input);

    let mut res = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c != &'X' {
                continue;
            }

            res += find_all_xmas(&grid, i, j);
        }
    }
    Some(res)
}

fn find_all_xmas(input: &Grid<char>, i: usize, j: usize) -> u32 {
    let mut sum = 0;

    for x in -1..=1 {
        for y in -1..=1 {
            if x == 0 && y == 0 {
                continue;
            }
            sum += search_xmas(input, i as i64, j as i64, (x, y)) as u32;
        }
    }

    sum
}

fn search_xmas(grid: &Grid<char>, x: i64, y: i64, inc: (i64, i64)) -> bool {
    if !grid.inbound(&Vec2::new(x + inc.0 * 3, y + inc.1 * 3)) {
        return false;
    }
    let mut res = String::new();

    for i in 0..4 {
        res.push(grid[(x + inc.0 * i) as usize][(y + inc.1 * i) as usize]);
    }

    res == GOAL
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
