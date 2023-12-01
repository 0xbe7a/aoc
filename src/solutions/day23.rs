use std::{
    cmp::{max, min},
    collections::{hash_map::Entry, HashMap, HashSet},
};

type Tiles = HashSet<(i32, i32)>;

#[derive(Clone, Debug, Copy)]
enum SearchDirection {
    North,
    South,
    West,
    East,
}

fn parse_input(input: &str) -> Tiles {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.char_indices().filter_map(move |(x, c)| match c {
                '.' => None,
                '#' => Some((x as i32, y as i32)),
                _ => panic!("Unknown char"),
            })
        })
        .collect()
}

fn step(state: &mut Tiles, directions: [SearchDirection; 4]) -> bool {
    let mut proposals = HashMap::new();

    for (x, y) in state.iter().copied() {
        let neighborhood = [
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ]
        .map(|(dx, dy)| state.contains(&(x + dx, y + dy)));

        let is_active = neighborhood.iter().copied().reduce(|a, b| a | b).unwrap();

        if !is_active {
            continue;
        }

        for d in directions {
            let blocked = match d {
                SearchDirection::North => neighborhood[7] | neighborhood[0] | neighborhood[1],
                SearchDirection::East => neighborhood[1] | neighborhood[2] | neighborhood[3],
                SearchDirection::South => neighborhood[3] | neighborhood[4] | neighborhood[5],
                SearchDirection::West => neighborhood[5] | neighborhood[6] | neighborhood[7],
            };

            if blocked {
                continue;
            }

            let p = match d {
                SearchDirection::North => (x, y - 1),
                SearchDirection::East => (x + 1, y),
                SearchDirection::South => (x, y + 1),
                SearchDirection::West => (x - 1, y),
            };

            match proposals.entry(p) {
                Entry::Occupied(mut e) => {
                    e.insert(None);
                }
                Entry::Vacant(e) => {
                    e.insert(Some((x, y)));
                }
            }

            break;
        }
    }

    let mut movement = false;

    //Apply new positions
    for (new_p, old) in proposals {
        if let Some(old_p) = old {
            movement = true;
            state.remove(&old_p);
            state.insert(new_p);
        }
    }

    movement
}

pub fn part_one(input: &str) -> i32 {
    let mut state = parse_input(input);

    let mut search = [
        SearchDirection::North,
        SearchDirection::South,
        SearchDirection::West,
        SearchDirection::East,
    ];

    for _ in 0..10 {
        step(&mut state, search);
        search.rotate_left(1);
    }

    //Calculate bounding rect
    let (min_x, max_x, min_y, max_y) = state.iter().fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(min_x, max_x, min_y, max_y), (x, y)| {
            (
                min(*x, min_x),
                max(*x, max_x),
                min(*y, min_y),
                max(*y, max_y),
            )
        },
    );

    (max_x - min_x + 1) * (max_y - min_y + 1) - state.len() as i32
}

pub fn part_two(input: &str) -> usize {
    let mut state = parse_input(input);
    let mut round = 0;

    let mut search = [
        SearchDirection::North,
        SearchDirection::South,
        SearchDirection::West,
        SearchDirection::East,
    ];

    loop {
        round += 1;

        let movement = step(&mut state, search);

        search.rotate_left(1);

        if !movement {
            break;
        }
    }
    round
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 23);
        assert_eq!(part_one(&input), 110);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 23);
        assert_eq!(part_two(&input), 20);
    }
}
