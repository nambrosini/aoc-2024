use aoc_util::{
    algs::astar::astar,
    grid::{Find, Grid, Inbound, Parse, Read},
    position::Vec2,
};

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::parse(input);

    let path = astar(&grid, grid.find(&'S'), grid.find(&'E'), Some('#')).unwrap();

    let mut save = vec![];
    for (i, p) in path.iter().enumerate() {
        let len_to_now = i + 2;
        for d in Vec2::DIRECTIONS {
            let pd = *p + d;
            if !grid.inbound(&pd) || grid.get(&pd) != '#' {
                continue;
            }
            let new_pos = *p + d * 2;
            if !grid.inbound(&new_pos) || grid.get(&new_pos) == '#' {
                continue;
            }
            let cheat_start_i = path.iter().position(|x| x == &new_pos).unwrap();
            let cheat_path = &path[cheat_start_i..];

            if cheat_path.len() + len_to_now > path.len() {
                continue;
            }
            save.push(path.len() - cheat_path.len() - len_to_now);
        }
    }

    Some(save.iter().filter(|x| **x >= 100).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = Grid::parse(input);

    let path = astar(&grid, grid.find(&'S'), grid.find(&'E'), Some('#')).unwrap();

    let mut save: Vec<usize> = Vec::new();
    for (i, p) in path.iter().enumerate() {
        for (j, p2) in path.iter().enumerate().skip(i) {
            if p.manhattan(p2) > 20 {
                continue;
            }
            let pat = i + (path.len() - j);
            if path.len() - pat - p.manhattan(p2) as usize + 1 >= 100 {
                save.push(path.len() - pat - p.manhattan(p2) as usize + 1);
            }
        }
    }

    Some(save.iter().filter(|&&x| x >= 100).count())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_part_one() {
//         let result = part_one(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, Some(5));
//     }
//
//     #[test]
//     fn test_part_two() {
//         let result = part_two(&advent_of_code::template::read_file("examples", DAY));
//         assert_eq!(result, Some(285));
//     }
// }
