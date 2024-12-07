advent_of_code::solution!(7);

use std::collections::HashSet;

use itertools::{repeat_n, Itertools};

fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|x| {
            let mut split = x.split(": ");
            let res = split.next().unwrap().parse().unwrap();
            let vals = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            (res, vals)
        })
        .collect()
}
pub fn part_one(input: &str) -> Option<i64> {
    let input = parse(input);
    let operators = ['+', '*'];
    let mut sum = 0;

    'outer: for (target, nums) in input {
        let n = nums.len();
        let mut combi: HashSet<Vec<&char>> = repeat_n(operators.iter(), n - 1)
            .multi_cartesian_product()
            .collect();
        for c in operators.iter().combinations(n - 1) {
            combi.insert(c);
        }
        for ops in combi {
            let mut res = nums[0];
            for i in 1..n {
                match ops[i - 1] {
                    '*' => res *= nums[i],
                    '+' => res += nums[i],
                    _ => unreachable!(),
                }
            }
            if res == target {
                sum += res;
                continue 'outer;
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let input = parse(input);
    let operators = ["+", "*", "||"];
    let mut sum = 0;

    'outer: for (target, nums) in input {
        let n = nums.len();
        let mut combi: HashSet<Vec<&&str>> = repeat_n(operators.iter(), n - 1)
            .multi_cartesian_product()
            .collect();
        for c in operators.iter().combinations(n - 1) {
            combi.insert(c);
        }
        // let combi = operators.iter().permutations(n - 1);
        for ops in combi {
            let mut res = nums[0];
            for i in 1..n {
                match *ops[i - 1] {
                    "*" => res *= nums[i],
                    "+" => res += nums[i],
                    "||" => res = res * 10i64.pow((nums[i] as f64).log10() as u32 + 1) + nums[i],
                    _ => unreachable!(),
                }
            }
            if res == target {
                sum += res;
                continue 'outer;
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
