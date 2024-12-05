use std::cmp::Ordering;

advent_of_code::solution!(5);

fn parse(input: &str) -> (Vec<(i64, i64)>, Vec<Vec<i64>>) {
    let split: Vec<&str> = input.split("\n\n").collect();
    let map = split[0]
        .lines()
        .map(|l| {
            let s: Vec<i64> = l.split('|').map(|x| x.parse().unwrap()).collect();
            (s[0], s[1])
        })
        .collect();

    let orders = split[1]
        .lines()
        .map(|l| l.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();
    (map, orders)
}

pub fn part_one(input: &str) -> Option<i64> {
    let (rules, orders) = parse(input);
    is_valid(&rules, &orders[0]);

    let res = orders
        .iter()
        .filter(|x| is_valid(&rules, x))
        .map(|x| x[x.len() / 2])
        .sum();

    Some(res)
}

fn is_valid(rules: &[(i64, i64)], order: &[i64]) -> bool {
    order[..order.len() - 1]
        .iter()
        .zip(order.iter().skip(1))
        .all(|(&a, &b)| rules.contains(&(a, b)))
}

pub fn part_two(input: &str) -> Option<i64> {
    let (rules, orders) = parse(input);

    let res = orders
        .iter()
        .filter(|x| !is_valid(&rules, x))
        .map(|x| {
            let mut x: Vec<i64> = x.to_vec();
            x.sort_by(|a, b| {
                if rules.contains(&(*a, *b)) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            x.iter().rev().copied().collect::<Vec<i64>>()
        })
        .map(|x| x[x.len() / 2])
        .sum();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
