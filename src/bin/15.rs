use std::collections::VecDeque;

use advent_of_code::util::{
    direction::Direction,
    grid::{Find, Grid, Parse, Read, Write},
};

advent_of_code::solution!(15);

type Movements = Vec<Direction>;

const EMPTY: char = '.';
const WALL: char = '#';
const BOX: char = 'O';
const ROBOT: char = '@';

// Part 2
const OPEN_BOX: char = '[';
const CLOSE_BOX: char = ']';

fn parse(input: &str) -> (Grid<char>, Movements) {
    let mut lines = input.split("\n\n");
    let grid = Grid::parse(lines.next().unwrap());
    let movements = lines
        .next()
        .unwrap()
        .trim()
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => unreachable!(),
        })
        .collect();
    (grid, movements)
}

pub fn part_one(input: &str) -> Option<i64> {
    let (mut grid, movements) = parse(input);

    for m in movements {
        move_robot(&mut grid, m);
    }

    Some(grid.find_all(&BOX).iter().map(|b| b.x * 100 + b.y).sum())
}

fn move_robot(grid: &mut Grid<char>, dir: Direction) {
    let robot = grid.find(&ROBOT);
    let new_pos = robot + dir;
    let next = grid.get(&new_pos);
    match next {
        EMPTY => {
            grid.set(&robot, EMPTY);
            grid.set(&new_pos, ROBOT);
        }
        WALL => (),
        BOX => {
            let mut current = new_pos;
            while grid.get(&current) == BOX {
                current += dir;
            }

            if grid.get(&current) == WALL {
                return;
            }
            grid.set(&robot, EMPTY);

            if grid.get(&current) == EMPTY {
                while current != new_pos {
                    current -= dir;
                    grid.set(&(current + dir), BOX);
                }
            }
            grid[new_pos.x()][new_pos.y()] = ROBOT;
        }
        _ => (),
    }
}

#[allow(dead_code)]
fn print(grid: &Grid<char>) {
    for row in grid.iter() {
        for e in row.iter() {
            print!("{}", e);
        }
        println!();
    }
}

pub fn part_two(input: &str) -> Option<i64> {
    let (old_grid, movements) = parse(input);

    let mut grid = Grid::new();
    for r in old_grid {
        let mut row = vec![];
        for e in r {
            match e {
                EMPTY => row.extend_from_slice(&[EMPTY, EMPTY]),
                WALL => row.extend_from_slice(&[WALL, WALL]),
                BOX => row.extend_from_slice(&['[', ']']),
                ROBOT => row.extend_from_slice(&[ROBOT, '.']),
                _ => unreachable!(),
            }
        }
        grid.push(row);
    }

    for m in movements {
        move_robot_2(&mut grid, m);
    }

    Some(
        grid.find_all(&OPEN_BOX)
            .iter()
            .map(|b| b.x * 100 + b.y)
            .sum(),
    )
}

fn move_robot_2(grid: &mut Grid<char>, dir: Direction) {
    let robot = grid.find(&ROBOT);
    let new_pos = robot + dir;
    let next = grid.get(&new_pos);
    match next {
        EMPTY => {
            grid.set(&robot, EMPTY);
            grid.set(&new_pos, ROBOT);
        }
        WALL => (),
        OPEN_BOX | CLOSE_BOX => match dir {
            Direction::Left | Direction::Right => {
                let mut current = new_pos;
                while grid.get(&current) == OPEN_BOX || grid.get(&current) == CLOSE_BOX {
                    current += dir * 2;
                }

                if grid.get(&current) == WALL {
                    return;
                }
                grid.set(&robot, EMPTY);

                if grid.get(&current) == EMPTY {
                    while current != new_pos {
                        if dir == Direction::Left {
                            grid.set(&current, OPEN_BOX);
                            grid.set(&(current - dir), CLOSE_BOX);
                        } else {
                            grid.set(&current, CLOSE_BOX);
                            grid.set(&(current - dir), OPEN_BOX);
                        }
                        current -= dir * 2;
                    }
                }
                grid[new_pos.x()][new_pos.y()] = ROBOT;
            }
            Direction::Up | Direction::Down => {
                let mut boxes = VecDeque::new();
                let mut queue = VecDeque::new();
                queue.push_back((new_pos, grid.get(&new_pos)));
                if grid.get(&new_pos) == OPEN_BOX {
                    queue.push_back((new_pos + Direction::Right, CLOSE_BOX));
                } else {
                    queue.push_back((new_pos + Direction::Left, OPEN_BOX));
                }
                while let Some((pos, b)) = queue.pop_front() {
                    if b == WALL {
                        return;
                    }
                    if b == EMPTY || boxes.contains(&(pos, b)) {
                        continue;
                    }
                    boxes.push_back((pos, b));
                    queue.push_back((pos + dir, grid.get(&(pos + dir))));

                    let right = (pos + Direction::Right, grid.get(&(pos + Direction::Right)));
                    let left = (pos + Direction::Left, grid.get(&(pos + Direction::Left)));
                    if b == OPEN_BOX && !queue.contains(&right) {
                        queue.push_back(right);
                    }
                    if b == CLOSE_BOX && !queue.contains(&left) {
                        queue.push_back(left);
                    }
                }

                for (pos, _) in boxes.iter() {
                    grid.set(pos, EMPTY);
                }
                for (pos, b) in boxes {
                    grid.set(&(pos + dir), b);
                }

                grid.set(&robot, EMPTY);
                grid.set(&(robot + dir), ROBOT);
            }
        },
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
