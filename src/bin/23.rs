use std::{collections::{HashMap, HashSet}, hash::{Hash, Hasher}};

use itertools::Itertools;

advent_of_code::solution!(23);

type Map = HashMap<String, Vec<String>>;

fn parse(input: &str) -> HashMap<String, Vec<String>> {
    let mut map = Map::new();
    for l in input.lines() {
        let first = l[0..2].to_string();
        let second = l[3..].to_string();

        let entry = map.entry(first.clone()).or_default();
        entry.push(second.clone());
        let entry = map.entry(second).or_default();
        entry.push(first);
    };

    map
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse(input);
    let ts = map.keys().filter(|k| k.starts_with("t"));

    let mut res = vec![];
    for t in ts {
        res.append(&mut find_triplets(&map, t));
    }

    Some(res.iter().unique().count())
}

pub fn part_two(input: &str) -> Option<String> {
    let map = parse(input);

    let mut max = 0;
    let mut biggest_party = vec![];

    for k in map.keys() {
        for k2 in map.get(k).unwrap() {
            let party = find_party(&map, &[k.clone()], k2);
            if max < party.len() {
                max = party.len();
                biggest_party = party;
            }
        }
    }

    biggest_party.sort();

    Some(biggest_party.join(","))
}

fn find_party(map: &Map, party: &[String], pc: &String) -> Vec<String> {
    let conns: Vec<String> = map.get(pc).unwrap().clone();
    let mut vec = party.to_vec();

    for v in party {
        if !conns.contains(v) {
            return vec;
        }
    }

    vec.push(pc.clone());

    for v in conns {
        if !vec.contains(&v) {
            return find_party(map, &vec, &v);
        }
    }
    vec
}

fn find_triplets(map: &Map, key: &String) -> Vec<Triplet> {
    let mut res = Vec::new();
    let vs = map.get(key).unwrap();
    for second in vs {
        let v2 = map.get(second).unwrap();
        for third in v2 {
            if map.get(third).unwrap().contains(key) {
                let t = Triplet::new(key.clone(), second.clone(), third.clone());
                res.push(t);
            }
        }
    }

    res
}

struct Triplet {
    first: String,
    second: String,
    third: String
}

impl Triplet {
    fn new(first: String, second: String, third: String) -> Self {
        Triplet {
            first,
            second,
            third
        }
    }
}

impl PartialEq for Triplet {
    fn eq(&self, other: &Self) -> bool {
        let this = HashSet::from([&self.first, &self.second, &self.third]);
        let other = HashSet::from([&other.first, &other.second, &other.third]);
        // Convert both to HashSet for unordered comparison
        let this_set: HashSet<_> = this.iter().collect();
        let other_set: HashSet<_> = other.iter().collect();
        this_set == other_set
    }
}

impl Hash for Triplet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut fields = vec![&self.first, &self.second, &self.third];
        fields.sort(); // Sort to ensure order doesn't affect hash
        for field in fields {
            field.hash(state);
        }
    }
}

impl Eq for Triplet {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_triplets() {
        let input = "tb->bc\ncd->bc\ntb->cd";
        let res = part_one(input);

        assert_eq!(res, Some(1));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("co,de,ka,ta")));
    }
}
