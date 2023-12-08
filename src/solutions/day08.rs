use num::integer::lcm;
use once_cell::sync::Lazy;
use regex::Regex;
use rustc_hash::FxHashMap as HashMap;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(d: char) -> Self {
        match d {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("invalid direction: {}", d),
        }
    }
}

static NODE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(\w{3}) = \((\w{3}), (\w{3})\)$").unwrap());

struct Input<'a> {
    directions: Vec<Direction>,
    graph: HashMap<&'a str, (&'a str, &'a str)>,
}

fn parse_input(input: &'_ str) -> Input<'_> {
    let mut lines = input.lines();

    let directions: Vec<_> = lines
        .next()
        .expect("directions missing")
        .chars()
        .map(Direction::from_char)
        .collect();
    lines.next();

    let mut graph = HashMap::default();

    for line in lines {
        let cap = NODE_RE.captures(line).expect("invalid format");

        let origin = cap.get(1).unwrap().as_str();
        let left = cap.get(2).unwrap().as_str();
        let right = cap.get(3).unwrap().as_str();

        graph.insert(origin, (left, right));
    }

    Input { directions, graph }
}

pub fn part_one(input: &str) -> u32 {
    let Input { directions, graph } = parse_input(input);
    let mut currect_node = "AAA";

    let mut steps = 0;

    for d in directions.iter().cycle() {
        if currect_node == "ZZZ" {
            break;
        }

        steps += 1;

        let (left, right) = graph[currect_node];
        match d {
            Direction::Left => {
                currect_node = left;
            }
            Direction::Right => {
                currect_node = right;
            }
        }
    }

    steps
}

fn solve_single_node(
    graph: &HashMap<&str, (&str, &str)>,
    directions: &[Direction],
    node: &str,
) -> usize {
    let mut current_node = node;

    let mut steps = 0;

    for d in directions.iter().cycle() {
        if current_node.ends_with('Z') {
            break;
        }

        steps += 1;

        let (left, right) = graph[current_node];

        match d {
            Direction::Left => {
                current_node = left;
            }
            Direction::Right => {
                current_node = right;
            }
        }
    }

    steps
}

pub fn part_two(input: &str) -> usize {
    let Input { directions, graph } = parse_input(input);

    graph
        .keys()
        .filter(|name| name.ends_with('A'))
        .map(|node| solve_single_node(&graph, &directions, node))
        .fold(1, lcm)
}

#[cfg(test)]
mod tests {
    use crate::{read_file, read_file_with_name};

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("examples", 8);
        assert_eq!(part_one(&input), 6);
    }

    #[test]
    fn test_part_two() {
        let input = read_file_with_name("examples", "08_2");
        assert_eq!(part_two(&input), 6);
    }
}
