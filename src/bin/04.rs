advent_of_code::solution!(4);

const GOAL: &str = "XMAS";
const GOAL_2: &str = "MAS";

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}
pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);

    let mut res = 0;
    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c != &'X' {
                continue;
            }

            res += find_all_xmas(&input, i, j);
        }
    }
    Some(res)
}

fn find_all_xmas(input: &[Vec<char>], i: usize, j: usize) -> u32 {
    let mut sum = 0;

    for x in -1..=1 {
        for y in -1..=1 {
            if x == 0 && y == 0 {
                continue;
            }
            sum += search_xmas(input, i as i32, j as i32, (x, y)) as u32;
        }
    }

    sum
}

fn search_xmas(input: &[Vec<char>], x: i32, y: i32, inc: (i32, i32)) -> bool {
    let mut res = String::new();

    for i in 0..4 {
        if let Some(row) = input.get((x + inc.0 * i) as usize) {
            if let Some(c) = row.get((y + inc.1 * i) as usize) {
                res.push(*c);
            }
        }
    }

    res == GOAL
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);

    let mut res = 0;
    for (i, row) in input.iter().enumerate() {
        if i == 0 {
            continue;
        }
        for (j, c) in row.iter().enumerate() {
            if j == 0 {
                continue;
            }
            if c != &'A' {
                continue;
            }
            res += find_all_x_mas(&input, i, j);
        }
    }
    Some(res)
}

fn find_all_x_mas(input: &[Vec<char>], i: usize, j: usize) -> u32 {
    println!("{}, {}", i, j);
    let mut res = String::new();
    let mut res1 = String::new();
    for k in -1..=1 {
        if let Some(c) = get(input, (i as i32 + k) as usize, (j as i32 + k) as usize) {
            res.push(c);
        } else {
            return 0;
        }
        if let Some(c) = get(input, (i as i32 + k) as usize, (j as i32 - k) as usize) {
            res1.push(c);
        } else {
            return 0;
        }
    }

    println!("\t{}", res);
    println!("\t{}", res1);

    if (res == GOAL_2 || res.chars().rev().collect::<String>() == GOAL_2)
        && (res1 == GOAL_2 || res1.chars().rev().collect::<String>() == GOAL_2)
    {
        return 1;
    }

    0
}

fn get(input: &[Vec<char>], i: usize, j: usize) -> Option<char> {
    if let Some(row) = input.get(i) {
        return row.get(j).copied();
    }
    None
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
