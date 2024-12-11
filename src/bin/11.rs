use std::collections::HashMap;

advent_of_code::solution!(11);

type Stones = HashMap<i64, i64>;

fn parse(input: &str) -> Stones {
    let v: Vec<i64> = input
        .strip_suffix("\n")
        .unwrap()
        .split(' ')
        .map(|c| c.parse::<i64>().unwrap())
        .collect();
    let mut map = HashMap::new();
    for x in v {
        let entry = map.entry(x).or_insert(0);
        *entry += 1;
    }
    map
}

pub fn part_one(input: &str) -> Option<i64> {
    let input = parse(input);
    Some(calc(input, 25))
}

pub fn part_two(input: &str) -> Option<i64> {
    let input = parse(input);
    Some(calc(input, 75))
}

fn calc(input: Stones, cycles: usize) -> i64 {
    let mut input = input.clone();
    for _ in 0..cycles {
        let mut new: Stones = Stones::new();
        for (stone, n) in input {
            let digits = (stone as f64).log10() as u32 + 1;
            if stone == 0 {
                let entry = new.entry(1).or_insert(0);
                *entry += n;
            } else if digits % 2 == 0 {
                let entry = new.entry(stone / 10i64.pow(digits / 2)).or_insert(0);
                *entry += n;
                let entry = new.entry(stone % 10i64.pow(digits / 2)).or_insert(0);
                *entry += n;
            } else {
                let entry = new.entry(stone * 2024).or_insert(0);
                *entry += n;
            }
        }
        input = new;
    }

    input.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
