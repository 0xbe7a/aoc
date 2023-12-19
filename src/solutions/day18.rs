use std::collections::HashSet;

use once_cell::sync::Lazy;
use regex::Regex;

static INPUT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"([UDLR]) (\d+) \(#([0-9a-f]+)\)").unwrap());

#[derive(Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn from_char(s: &str) -> Self {
        match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("invalid direction"),
        }
    }

    fn from_idx(s: &str) -> Self {
        match s {
            "0" => Self::Right,
            "1" => Self::Down,
            "2" => Self::Left,
            "3" => Self::Up,
            _ => panic!("invalid direction"),
        }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = (Direction, u16, &str)> {
    INPUT_REGEX.captures_iter(input).map(|m| {
        let direction = Direction::from_char(m.get(1).unwrap().as_str());
        let distance = m.get(2).unwrap().as_str().parse::<u16>().unwrap();
        let rgb_code = m.get(3).unwrap().as_str();

        (direction, distance, rgb_code)
    })
}

pub fn part_one(input: &str) -> u32 {
    let mut position = (0, 0);
    let mut cells = HashSet::new();

    cells.insert(position);

    let (mut min_x, mut max_x) = (0, 0);
    let (mut min_y, mut max_y) = (0, 0);

    for (direction, distance, _) in parse_input(input) {
        let (dy, dx) = match direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };

        for _ in 0..distance {
            position.0 += dy;
            position.1 += dx;

            min_y = min_y.min(position.0);
            max_y = max_y.max(position.0);
            min_x = min_x.min(position.1);
            max_x = max_x.max(position.1);

            cells.insert(position);
        }
    }

    let mut area = 0;

    for y in min_y..=max_y {
        let mut outside = true;
        for x in min_x..=max_x {
            if cells.contains(&(y, x)) {
                area += 1;

                if cells.contains(&(y - 1, x)) {
                    outside = !outside;
                }
            } else if !outside {
                area += 1;
            }
        }
    }

    area
}

pub fn part_two(input: &str) -> i64 {
    let mut position = (0, 0);
    let mut edge_distance: i64 = 0;
    let mut area = 0;

    for (_, _, hex) in parse_input(input) {
        let distance = u32::from_str_radix(&hex[..5], 16).unwrap();
        let direction = Direction::from_idx(&hex[5..]);

        let (dy, dx) = match direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };

        let next_position = (
            position.0 + dy * distance as i64,
            position.1 + dx * distance as i64
        );

        area += (position.1 * next_position.0) - (next_position.1 * position.0);

        edge_distance += distance as i64;
        position = next_position;
    }

    let inner = (area / 2) - (edge_distance / 2) + 1;
    inner + edge_distance
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("examples", 18);
        assert_eq!(part_one(&input), 62);
    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 18);
        assert_eq!(part_two(&input), 952408144115);
    }
}
