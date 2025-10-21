use std::collections::HashMap;

advent_of_code::solution!(22);

type Map = HashMap<Vec<i64>, i64>;

fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut input = parse(input);

    for _ in 0..2000 {
        input.iter_mut().for_each(|n| *n = evolve(*n));
    }

    Some(input.iter().sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    solve_two(input, 2000)
}

fn solve_two(input: &str, amount: usize) -> Option<i64> {
    let input = parse(input);
    let mut map = HashMap::<Vec<i64>, i64>::new();

    for secret in input {
        let m = get_bananas_seq(secret, amount);
        m.iter().for_each(|(k, v)| {
            let entry = map.entry(k.to_vec()).or_insert(0);
            *entry += v;
        });
    }

    map.values().max().copied()
}

fn get_bananas_seq(secret: i64, amount: usize) -> Map {
    let mut map: Map = HashMap::new();
    let seq = get_sequence(secret, amount);
    let differences = get_diffs(&seq);
    for (i, w) in differences.windows(4).enumerate() {
        if !map.contains_key(w) {
            map.insert(w.to_vec(), seq[i+4]);
        }
    }
    map
}

fn get_sequence(secret: i64, amount: usize) -> Vec<i64> {
    let mut secret = secret;
    let mut v = Vec::with_capacity(2001);

    for _ in 0..=amount {
        v.push(secret % 10);
        secret = evolve(secret);
    }

    v
}

fn get_diffs(seq: &[i64]) -> Vec<i64> {
    seq.windows(2).map(|w| w[1] - w[0]).collect()
}

fn evolve(secret: i64) -> i64 {
    let sec = prune(mix(secret * 64, secret));
    let sec = prune(mix(sec / 32, sec));
    prune(mix(sec * 2048, sec))
}

fn mix(given: i64, secret: i64) -> i64 {
    given ^ secret
}

fn prune(secret: i64) -> i64 {
    secret % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        let secret = 42;
        let given = 15;

        assert_eq!(mix(given, secret), 37);
    }

    #[test]
    fn test_prune() {
        let secret = 100000000;

        assert_eq!(prune(secret), 16113920);
    }

    #[test]
    fn test_next_secrets() {
        let mut sec = 123;
        let next_ten: [i64; 10] = [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];

        next_ten.iter().for_each(|n| {
            sec = evolve(sec);
            assert_eq!(&sec, n);
        });
    }

    #[test]
    fn test_sequence() {
        let secret = 123;
        let sequence = get_sequence(secret, 10);
        let expected = [3, 0, 6, 5, 4, 4, 6, 4, 4, 2];

        assert_eq!(sequence, expected);
    }

    #[test]
    fn test_diffs() {
        let secret = 123;
        let sequence = get_sequence(secret, 10);
        let diffs = get_diffs(&sequence);
        let expected = [-3, 6, -1, -1, 0, 2, -2, 0, -2];

        assert_eq!(diffs, expected);
    }

    #[test]
    fn banana_seq_123() {
        let secret = 123;
        let map: Map = [
            (Vec::<i64>::from([-3, 6, -1, -1]), 3i64),
            (Vec::<i64>::from([6, -1, -1, 0]), 0i64),
            (Vec::<i64>::from([-1, -1, 0, 2]), 6i64),
            (Vec::<i64>::from([-1, 0, 2, -2]), 5i64),
            (Vec::<i64>::from([0, 2, -2, 0]), 4i64),
            (Vec::<i64>::from([2, -2, 0, -2]), 4i64),
        ].iter().cloned().collect();

        let mut result = HashMap::new();
        get_bananas_seq(secret, 10);

        assert_eq!(result, map);
    }

    #[test]
    fn test_part_two_123() {
        let secret = "123";
        let result = solve_two(secret, 10);

        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
