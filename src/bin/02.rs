advent_of_code::solution!(2);

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.split(' ').map(|x| x.parse().unwrap()).collect())
        .collect()
}
pub fn part_one(input: &str) -> Option<i32> {
    let input = parse(input);
    let mut res = 0;
    for report in input {
        if is_safe(&report) {
            res += 1;
        }
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<i32> {
    let input = parse(input);
    let mut res = 0;
    for report in input {
        let mut safe = false;
        for i in 0..report.len() {
            let mut report = report.clone();
            report.remove(i);
            if is_safe(&report) {
                safe = true;
                break;
            }
        }
        if safe {
            res += 1;
        }
    }
    Some(res)
}

fn is_safe(report: &[i32]) -> bool {
    let x: Vec<_> = report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(x, y)| x - y)
        .collect();
    return x.iter().all(|x| (1..=3).contains(x)) || x.iter().all(|x| (-3..=-1).contains(x));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
