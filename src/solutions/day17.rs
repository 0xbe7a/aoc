use std::{
    cmp::Reverse,
    collections::BinaryHeap,
};

use rustc_hash::FxHashMap as HashMap;

use grid::Grid;

fn parse_input(input: &str) -> Grid<u8> {
    let mut cols: Option<usize> = None;

    let grid_data: Vec<u8> = input
        .lines()
        .enumerate()
        .flat_map(|(index, line)| {
            if index == 0 {
                cols = Some(line.len());
            }
            line.chars()
                .map(|c| c.to_digit(10).expect("invalid digit") as u8)
        })
        .collect();

    Grid::from_vec(grid_data, cols.expect("no data"))
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(&self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }

    fn turn_back(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct State {
    f_cost: usize,
    g_cost: usize,
    prev_direction: Direction,
    moves_since_last_turn: usize,
    position: (usize, usize),
}

fn find_min_path(
    grid: &Grid<u8>,
    is_valid_move: impl Fn(Direction, Direction, usize) -> bool,
) -> usize {
    let mut dist = HashMap::default();
    let mut heap = BinaryHeap::new();

    let goal = (grid.rows() - 1, grid.cols() - 1);

    let manhattan_distance = |pos: (usize, usize)| (goal.0 - pos.0 + goal.1 - pos.1);

    let state_key = |state: &State| {
        (
            state.position,
            state.prev_direction,
            state.moves_since_last_turn,
        )
    };

    let start_state = State {
        f_cost: manhattan_distance((0, 0)),
        g_cost: 0,
        prev_direction: Direction::Right,
        moves_since_last_turn: 0,
        position: (0, 0),
    };

    dist.insert(state_key(&start_state), 0);
    heap.push(Reverse(start_state));

    while let Some(Reverse(state)) = heap.pop() {
        if state.position == goal {
            return dist[&state_key(&state)];
        }

        for &next_dir in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if !is_valid_move(state.prev_direction, next_dir, state.moves_since_last_turn) {
                continue;
            }

            let (dy, dx) = next_dir.delta();

            if (dy == -1 && state.position.0 == 0)
                || (dx == -1 && state.position.1 == 0)
                || (dy == 1 && state.position.0 == grid.rows() - 1)
                || (dx == 1 && state.position.1 == grid.cols() - 1)
            {
                continue;
            }

            let next_position = (
                state.position.0.saturating_add_signed(dy),
                state.position.1.saturating_add_signed(dx),
            );

            let moves_count = if next_dir == state.prev_direction {
                state.moves_since_last_turn + 1
            } else {
                1
            };

            let new_g_cost = state.g_cost + grid[next_position] as usize;
            let new_f_cost = new_g_cost + manhattan_distance(next_position);

            let next_state = State {
                f_cost: new_f_cost,
                g_cost: new_g_cost,
                prev_direction: next_dir,
                moves_since_last_turn: moves_count,
                position: next_position,
            };

            let key = state_key(&next_state);
            if new_g_cost < *dist.entry(key).or_insert(usize::MAX) {
                heap.push(Reverse(next_state));
                dist.insert(key, new_g_cost);
            }
        }
    }

    panic!("no path found");
}

pub fn part_one(input: &str) -> usize {
    let grid = parse_input(input);

    find_min_path(
        &grid,
        |prev_direction, next_direction, moves_since_last_turn| {
            if next_direction == prev_direction.turn_back() {
                return false;
            }

            if moves_since_last_turn == 3 && next_direction == prev_direction {
                return false;
            }

            true
        },
    )
}

pub fn part_two(input: &str) -> usize {
    let grid = parse_input(input);

    find_min_path(
        &grid,
        |prev_direction, next_direction, moves_since_last_turn| {
            if next_direction == prev_direction.turn_back() {
                return false;
            }

            if moves_since_last_turn == 10 && next_direction == prev_direction {
                return false;
            }

            if moves_since_last_turn < 4 && next_direction != prev_direction {
                return false;
            }

            true
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("examples", 17);
        assert_eq!(part_one(&input), 102);
    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 17);
        assert_eq!(part_two(&input), 94);
    }
}
