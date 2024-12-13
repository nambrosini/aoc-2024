use advent_of_code::util::position::{v, Vec2};
use regex::Regex;

advent_of_code::solution!(13);

struct Machine {
    a: Vec2,
    b: Vec2,
    prize: Vec2,
}

fn parse(input: &str) -> Vec<Machine> {
    let re_button = Regex::new(r"Button \w: \w\+(?P<x>\d+), \w\+(?P<y>\d+)").unwrap();
    let re_prize = Regex::new(r"Prize: X=(?<x>\d+), Y=(?<y>\d+)").unwrap();
    input
        .split("\n\n")
        .map(|m| {
            let mut iter = m.lines();

            let caps = re_button.captures(iter.next().unwrap()).unwrap();
            let button_a = v(caps["x"].parse().unwrap(), caps["y"].parse().unwrap());

            let caps = re_button.captures(iter.next().unwrap()).unwrap();
            let button_b = v(caps["x"].parse().unwrap(), caps["y"].parse().unwrap());

            let caps = re_prize.captures(iter.next().unwrap()).unwrap();
            let prize = v(caps["x"].parse().unwrap(), caps["y"].parse().unwrap());

            Machine {
                a: button_a,
                b: button_b,
                prize,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<i64> {
    let machines = parse(input);

    let mut sum = 0;
    for machine in machines {
        let b = (machine.a.x * machine.prize.y - machine.a.y * machine.prize.x)
            / (machine.a.x * machine.b.y - machine.a.y * machine.b.x);
        let a = (machine.prize.x - machine.b.x * b) / machine.a.x;

        if machine.a.x * a + machine.b.x * b == machine.prize.x
            && machine.a.y * a + machine.b.y * b == machine.prize.y
        {
            sum += a * 3 + b;
        }
    }

    Some(sum)
}

const PRIZE_INCREASE: i64 = 10000000000000;

pub fn part_two(input: &str) -> Option<i64> {
    let machines = parse(input);

    let machines: Vec<Machine> = machines
        .iter()
        .map(|machine| Machine {
            a: machine.a,
            b: machine.b,
            prize: machine.prize + PRIZE_INCREASE,
        })
        .collect();

    let mut sum = 0;
    for machine in machines {
        let b = (machine.a.x * machine.prize.y - machine.a.y * machine.prize.x)
            / (machine.a.x * machine.b.y - machine.a.y * machine.b.x);
        let a = (machine.prize.x - machine.b.x * b) / machine.a.x;

        if machine.a.x * a + machine.b.x * b == machine.prize.x
            && machine.a.y * a + machine.b.y * b == machine.prize.y
        {
            sum += a * 3 + b;
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
        assert_eq!(result, Some(480));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
