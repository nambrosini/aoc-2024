use aoc_util::{
    grid::{Grid, Inbound, Parse, ToMap},
    position::Vec2,
};

advent_of_code::solution!(12);

fn parse(input: &str) -> Vec<Vec<Vec2>> {
    let grid = Grid::parse(input);
    let map = grid.to_map();
    let mut gardens: Vec<Vec<Vec2>> = vec![];
    let mut visited: Vec<Vec2> = vec![];

    for (pos, e) in map.iter() {
        let mut group = vec![];
        if visited.contains(pos) {
            continue;
        }
        recurse(&grid, *pos, e, &mut group);
        gardens.push(group.clone());
        visited.append(&mut group);
    }
    gardens
}

pub fn part_one(input: &str) -> Option<usize> {
    let gardens = parse(input);
    let mut sum = 0;

    for garden in gardens {
        let mut perimeter = 0;
        for plot in garden.iter() {
            let mut adjacent = 0;
            for dir in Vec2::DIRECTIONS {
                let new = *plot + dir;
                if garden.contains(&new) {
                    adjacent += 1;
                }
            }
            perimeter += 4 - adjacent;
        }
        sum += garden.len() * perimeter;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let gardens = parse(input);
    let mut sum = 0;

    for garden in gardens {
        let corners = count_corners(&garden);
        sum += garden.len() * corners;
    }

    Some(sum)
}

fn count_corners(garden: &[Vec2]) -> usize {
    let mut sum = 0;

    for plot in garden {
        sum += (0..4)
            .map(|d| (Vec2::DIRECTIONS[d], Vec2::DIRECTIONS[(d + 1) % 4]))
            .map(|(d1, d2)| {
                (
                    garden.contains(plot),
                    garden.contains(&(*plot + d1)),
                    garden.contains(&(*plot + d2)),
                    garden.contains(&(*plot + d1 + d2)),
                )
            })
            .filter(|(plant, left, right, mid)| {
                (left != plant && right != plant)
                    || (left == plant && right == plant && mid != plant)
            })
            .count();
    }

    sum
}

fn recurse(grid: &Grid<char>, start: Vec2, symbol: &char, visited: &mut Vec<Vec2>) {
    if &grid[start.x()][start.y()] == symbol {
        visited.push(start);
    } else {
        return;
    }

    for dir in Vec2::DIRECTIONS {
        let new = start + dir;
        if !grid.inbound(&new) || visited.contains(&new) {
            continue;
        }
        recurse(grid, new, symbol, visited);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
