use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(19);

fn parse(input: &str) -> (HashSet<String>, Vec<String>) {
    let mut split = input.split("\n\n");
    let first = split
        .next()
        .unwrap()
        .split(", ")
        .map(|x| x.to_string())
        .collect();
    let second = split
        .next()
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect();

    (first, second)
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    let counts = counts(&input.1, &input.0).filter(|x| *x > 0).collect_vec();
    Some(counts.len())
}
pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);
    let counts = counts(&input.1, &input.0).filter(|x| *x > 0).collect_vec();
    Some(counts.iter().sum::<usize>())
}

fn counts<'a>(
    targets: &'a [String],
    patterns: &'a HashSet<String>,
) -> impl Iterator<Item = usize> + use<'a> {
    let mut known = HashMap::from([("", 1)]);

    targets
        .iter()
        .map(move |target| count(target.as_str(), patterns, &mut known))
}
fn count<'a>(t: &'a str, a: &'a HashSet<String>, known: &mut HashMap<&'a str, usize>) -> usize {
    if let Some(r) = known.get(t) {
        return *r;
    }
    let r = a
        .iter()
        .filter_map(|p| t.strip_prefix(p))
        .map(|t| count(t, a, known))
        .sum();
    known.entry(t).and_modify(|e| *e = r).or_insert(r);
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
