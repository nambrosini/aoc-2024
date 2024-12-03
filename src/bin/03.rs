use regex::Regex;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mults: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();

    Some(
        mults
            .iter()
            .map(|m| {
                m.strip_prefix("mul(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .split(',')
                    .map(|x| x.parse::<u32>().unwrap())
                    .product::<u32>()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don't\(\))").unwrap();

    let mults: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();

    let mut res = 0;
    let mut doo = true;

    for statement in mults {
        if statement.starts_with("don't") {
            doo = false;
            continue;
        }

        if statement.starts_with("do") {
            doo = true;
            continue;
        }

        if doo {
            res += statement
                .strip_prefix("mul(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .product::<u32>();
        }
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
