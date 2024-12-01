advent_of_code::solution!(1);

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for l in input.lines() {
        let mut split = l.split("   ");
        let l = split.next().unwrap();
        let r = split.next().unwrap();

        left.push(l.parse().unwrap());
        right.push(r.parse().unwrap());
    }

    (left, right)
}

pub fn part_one(input: &str) -> Option<i32> {
    let (mut left, mut right) = parse(input);
    left.sort_unstable();
    right.sort_unstable();

    let mut sum = 0;

    for (x, y) in right.iter().zip(left.iter()) {
        sum += (x - y).abs();
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse(input);

    let mut sum = 0;

    for l in left {
        sum += l * right.iter().filter(|&&x| x == l).count() as i32;
    }

    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
