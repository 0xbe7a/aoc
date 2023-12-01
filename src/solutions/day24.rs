
use std::{
    cmp::max,
    collections::{BinaryHeap},
};

use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

type Coords = (i32, i32);

struct Input {
    blizzards: Vec<(Coords, BlizzardDirection)>,
    open: HashSet<Coords>,
    max_x: usize,
    max_y: usize,
}

#[derive(Clone, Debug, Copy)]
enum BlizzardDirection {
    North,
    South,
    West,
    East,
}

fn parse_input(input: &str) -> Input {
    let mut blizzards = Vec::new();
    let mut open = HashSet::default();

    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            let coords = (x as i32, y as i32);
            (max_x, max_y) = (max(max_x, x), max(max_y, y));
            match c {
                '#' => {}
                '.' => {
                    open.insert(coords);
                }
                '>' => {
                    blizzards.push((coords, BlizzardDirection::East));
                    open.insert(coords);
                }
                'v' => {
                    blizzards.push((coords, BlizzardDirection::South));
                    open.insert(coords);
                }
                '^' => {
                    blizzards.push((coords, BlizzardDirection::North));
                    open.insert(coords);
                }
                '<' => {
                    blizzards.push((coords, BlizzardDirection::West));
                    open.insert(coords);
                }
                _ => panic!("Unknown char"),
            };
        }
    }

    Input {
        blizzards,
        open,
        max_x,
        max_y,
    }
}

#[derive(Clone, Debug)]
struct SearchPoint {
    coords: Coords,
    time: usize,
    heuristic: usize,
}

impl SearchPoint {
    fn estimate_cost(&self) -> usize {
        self.time + self.heuristic
    }
}

impl Ord for SearchPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.estimate_cost().cmp(&self.estimate_cost())
    }
}

impl PartialOrd for SearchPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.estimate_cost().partial_cmp(&self.estimate_cost())
    }
}

impl Eq for SearchPoint {}

impl PartialEq for SearchPoint {
    fn eq(&self, other: &Self) -> bool {
        self.estimate_cost() == other.estimate_cost()
    }
}

fn get_blizzard_tiles(
    blizzards: &[(Coords, BlizzardDirection)],
    time: usize,
    max_x: usize,
    max_y: usize,
) -> HashSet<Coords> {
    let mut blizzard_map = HashSet::default();

    for (coords, dir) in blizzards {
        let p = match dir {
            BlizzardDirection::North => (
                coords.0,
                (coords.1 - 1 + -(time as i32)).rem_euclid(max_y as i32 - 1) + 1,
            ),
            BlizzardDirection::South => (
                coords.0,
                (coords.1 - 1 + time as i32).rem_euclid(max_y as i32 - 1) + 1,
            ),
            BlizzardDirection::West => (
                ((coords.0 - 1 + -(time as i32)).rem_euclid(max_x as i32 - 1) + 1),
                coords.1,
            ),
            BlizzardDirection::East => (
                ((coords.0 - 1 + (time as i32)).rem_euclid(max_x as i32 - 1) + 1),
                coords.1,
            ),
        };

        blizzard_map.insert(p);
    }

    blizzard_map
}

fn find(input: &Input, start: Coords, time: usize, end: Coords) -> usize {
    let mut blizzard_tiles = HashMap::default();
    let mut queue = BinaryHeap::new();

    let heuristic = |p: Coords| ((p.0).abs_diff(end.0) + (p.1).abs_diff(end.1)) as usize;

    queue.push(SearchPoint {
        coords: start,
        time,
        heuristic: heuristic(start),
    });

    let mut visited = HashSet::default();
    visited.insert((start, time));

    while let Some(item) = queue.pop() {
        if item.coords == end {
            return item.time;
        }

        //Gen blizzards
        let blizzards = blizzard_tiles.entry(item.time + 1).or_insert_with(|| {
            get_blizzard_tiles(&input.blizzards, item.time + 1, input.max_x, input.max_y)
        });

        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0), (0, 0)] {
            let p = (item.coords.0 + dx, item.coords.1 + dy);

            if !input.open.contains(&p) {
                continue;
            }

            if blizzards.contains(&p) {
                continue;
            }

            let sp = SearchPoint {
                coords: p,
                time: item.time + 1,
                heuristic: heuristic(p),
            };

            if visited.insert((sp.coords, sp.time)) {
                queue.push(sp);
            }
        }
    }

    panic!("No path found")
}

pub fn part_one(input: &str) -> usize {
    let state = parse_input(input);

    let start = (1, 0);
    let end = (state.max_x as i32 - 1, state.max_y as i32);

    find(&state, start, 0, end)
}

pub fn part_two(input: &str) -> usize {
    let state = parse_input(input);

    let start = (1, 0);
    let end = (state.max_x as i32 - 1, state.max_y as i32);

    let part1 = find(&state, start, 0, end);
    let part2 = find(&state, end, part1, start);
    find(&state, start, part2, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 24);
        assert_eq!(part_one(&input), 18);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 24);
        assert_eq!(part_two(&input), 54);
    }
}
