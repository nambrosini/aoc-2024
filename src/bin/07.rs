advent_of_code::solution!(7);

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

    let mut sum = 0;
    for (target, nums) in input {
        if recurse_part1(nums[0], 1, &nums, '+', target)
            || recurse_part1(nums[0], 1, &nums, '*', target)
        {
            sum += target;
        }
    }

    Some(sum)
}

fn recurse_part1(start: i64, index: usize, nums: &[i64], op: char, target: i64) -> bool {
    let res = match op {
        '+' => start + nums[index],
        '*' => start * nums[index],
        _ => unreachable!(),
    };

    if index == nums.len() - 1 {
        return res == target;
    }

    recurse_part1(res, index + 1, nums, '+', target)
        || recurse_part1(res, index + 1, nums, '*', target)
}

pub fn part_two(input: &str) -> Option<i64> {
    let input = parse(input);
    let mut sum = 0;
    for (target, nums) in input {
        if recurse_part2(nums[0], 1, &nums, '+', target)
            || recurse_part2(nums[0], 1, &nums, '*', target)
            || recurse_part2(nums[0], 1, &nums, '|', target)
        {
            sum += target;
        }
    }

    Some(sum)
}

fn recurse_part2(start: i64, index: usize, nums: &[i64], op: char, target: i64) -> bool {
    let res = match op {
        '+' => start + nums[index],
        '*' => start * nums[index],
        '|' => start * 10i64.pow((nums[index] as f64).log10() as u32 + 1) + nums[index],
        _ => unreachable!(),
    };

    if index == nums.len() - 1 {
        return res == target;
    }

    recurse_part2(res, index + 1, nums, '+', target)
        || recurse_part2(res, index + 1, nums, '*', target)
        || recurse_part2(res, index + 1, nums, '|', target)
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
