use aoc_util::{
    algs::astar::astar,
    grid::{Grid, Write},
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
        astar(&grid, Vec2::new(0, 0), Vec2::new(70, 70), Some('#'))
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

    let mut last_path: Vec<Vec2> =
        astar(&grid, Vec2::new(0, 0), Vec2::new(70, 70), Some('#')).unwrap();
    for b in bytes.iter().skip(1024) {
        grid.set(b, '#');
        if !last_path.contains(b) {
            continue;
        }
        if let Some(path) = astar(&grid, Vec2::new(0, 0), Vec2::new(70, 70), Some('#')) {
            last_path = path;
        } else {
            return Some(format!("{},{}", b.y, b.x));
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
