use std::collections::{HashMap, VecDeque};

use aoc_util::{
    algs::astar::{astar, astar_all},
    direction::Direction,
    grid::{Find, Grid, Parse},
    position::Vec2,
};
use itertools::Itertools;

advent_of_code::solution!(21);

const KEYPAD: &str = "789\n456\n123\nx0A";
const DIRECTIONAL: &str = "x^A\n<v>";

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let codes = parse(input);

    let mut sum: usize = 0;

    for code in codes {
        let path = get_shortest_path(code);
        println!("{code}: {}", path);
        sum += path.len() * code[..code.len() - 1].parse::<usize>().unwrap();
    }

    Some(sum as u32)
}
fn get_shortest_path(code: &str) -> String {
    let keypad: Grid<char> = Grid::parse(KEYPAD);
    let directional: Grid<char> = Grid::parse(DIRECTIONAL);

    let path = path_to_code(&keypad, code);
    let path = path_to_code(&directional, &path);
    path_to_code(&directional, &path)
}

// fn get_shortest_path_len(code: &str) -> usize {
//     let keypad: Grid<char> = Grid::parse(KEYPAD);
//     let directional: Grid<char> = Grid::parse(DIRECTIONAL);
//
//     let path = find_paths(&keypad, vec![code.to_string()]);
//     let path = find_paths(&directional, path);
//     find_paths(&directional, path)
//         .iter()
//         .map(|x| x.len())
//         .min()
//         .unwrap()
// }

pub fn find_paths(grid: &Grid<char>, codes: Vec<String>) -> Vec<String> {
    let start = grid.find(&'A');
    let mut paths: Vec<String> = Vec::new();
    let mut queue: VecDeque<Path> = codes.iter().map(|c| Path::new("", c, start)).collect();

    while let Some(path) = queue.pop_front() {
        let goal = grid.find(&path.code.chars().next().unwrap());
        let p = astar_all(grid, path.pos, goal, Some('x'));
        println!("{:?}", p);
        for new_path in &p {
            let current_position = new_path.last().unwrap();
            let mut new_p = String::new();
            for i in 0..new_path.len() - 1 {
                let dir: Direction = (new_path[i + 1] - new_path[i]).into();
                new_p.push_str(&dir.to_string());
            }
            new_p.push('A');
            if path.code.len() == 1 {
                paths.push(new_p);
            } else {
                queue.push_back(Path::new(
                    &format!("{}{}", path.path, new_p),
                    &path.code[1..],
                    *current_position,
                ));
            }
        }
    }

    paths
}

struct Path {
    path: String,
    code: String,
    pos: Vec2,
}

impl Path {
    fn new(path: &str, code: &str, pos: Vec2) -> Self {
        Self {
            path: path.to_string(),
            code: code.to_string(),
            pos,
        }
    }
}

pub fn path_to_code(grid: &Grid<char>, code: &str) -> String {
    let mut start = grid.find(&'A');

    let mut path: String = String::new();
    for c in code.chars() {
        let goal = grid.find(&c);
        let p = astar(grid, start, goal, Some('x')).expect("Path not found");
        start = *p.iter().last().unwrap();
        let mut current_p = String::new();
        for i in 0..p.len() - 1 {
            let dir: Direction = (p[i + 1] - p[i]).into();
            current_p.push_str(&dir.to_string());
        }
        current_p = optimize(&current_p);
        path.push_str(&current_p);
        path.push('A');
    }

    path
}

fn optimize(path: &str) -> String {
    let mut map = HashMap::new();
    for c in path.chars() {
        let entry = map.entry(c).or_insert(0);
        *entry += 1;
    }

    if let Some(up) = map.get(&'^') {
        if let Some(down) = map.get(&'v') {
            if up - down > 0 {
                map.insert('^', up - down);
                map.remove(&'v');
            } else if up - down == 0 {
                map.remove(&'^');
                map.remove(&'v');
            } else {
                map.insert('v', down - up);
                map.remove(&'v');
            }
        }
    }

    if let Some(left) = map.get(&'<') {
        if let Some(right) = map.get(&'>') {
            if left - right > 0 {
                map.insert('<', left - right);
                map.remove(&'>');
            } else if left - right == 0 {
                map.remove(&'<');
                map.remove(&'>');
            } else {
                map.insert('>', right - left);
                map.remove(&'>');
            }
        }
    }

    let best = ['<', 'v', '^', '>'].iter().rev();
    let mut res = String::new();
    for b in best {
        if let Some(count) = map.get(b) {
            for _ in 0..*count {
                res.push(*b);
            }
        }
    }

    res
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_029A() {
        let keypad: Grid<char> = Grid::parse(KEYPAD);
        let directional: Grid<char> = Grid::parse(DIRECTIONAL);
        // Keypad
        // let actual = find_paths(&keypad, vec!["029A".to_string()]);
        // let expected = String::from("<A^A>^^AvvvA");
        // println!("Actual: {:?}\nExpected: {}", actual, expected);
        // assert!(actual.contains(&expected));
        let actual = path_to_code(&keypad, "029A");
        let expected = "<A^A>^^AvvvA";
        assert_eq!(actual, expected);

        // First directional
        let actual = path_to_code(&directional, "<A^A>^^AvvvA");
        let expected = "v<<A>>^A<A>AvA<^AA>A<vAAA>^A";
        assert_eq!(actual, expected);

        // Second directional
        let actual = path_to_code(&directional, &actual);
        let expected =
            String::from("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A");
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_179A() {
        let expected = "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A";
        let actual = get_shortest_path("179A");

        assert_eq!(expected.len(), actual.len());
    }
    //
    // #[test]
    // fn test_full_path() {
    //     let expected = "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A";
    //     let actual = get_shortest_path_len("029A");
    //
    //     assert_eq!(expected.len(), actual);
    // }
    //
    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(126384));
    // }
    //
    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
