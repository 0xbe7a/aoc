use std::{cmp, collections::VecDeque};

use bitvec::bitvec;
use grid::Grid;
use smallvec::{smallvec, SmallVec};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Symbol {
    Empty,
    UpwardMirror,
    DownwardMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Symbol {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '/' => Self::UpwardMirror,
            '\\' => Self::DownwardMirror,
            '|' => Self::VerticalSplitter,
            '-' => Self::HorizontalSplitter,
            _ => panic!("invalid symbol"),
        }
    }

    fn next_direction(&self, direction: Direction) -> SmallVec<[Direction; 2]> {
        match self {
            Self::Empty => smallvec![direction],
            Self::UpwardMirror => match direction {
                Direction::Up => smallvec![Direction::Right],
                Direction::Down => smallvec![Direction::Left],
                Direction::Left => smallvec![Direction::Down],
                Direction::Right => smallvec![Direction::Up],
            },
            Self::DownwardMirror => match direction {
                Direction::Up => smallvec![Direction::Left],
                Direction::Down => smallvec![Direction::Right],
                Direction::Left => smallvec![Direction::Up],
                Direction::Right => smallvec![Direction::Down],
            },
            Self::VerticalSplitter => match direction {
                Direction::Up => smallvec![Direction::Up],
                Direction::Down => smallvec![Direction::Down],
                Direction::Left => smallvec![Direction::Up, Direction::Down],
                Direction::Right => smallvec![Direction::Up, Direction::Down],
            },
            Self::HorizontalSplitter => match direction {
                Direction::Up => smallvec![Direction::Left, Direction::Right],
                Direction::Down => smallvec![Direction::Left, Direction::Right],
                Direction::Left => smallvec![Direction::Left],
                Direction::Right => smallvec![Direction::Right],
            },
        }
    }
}

fn parse_input(input: &str) -> Grid<Symbol> {
    let mut cols: Option<usize> = None;

    let grid_data: Vec<Symbol> = input
        .lines()
        .enumerate()
        .flat_map(|(index, line)| {
            if index == 0 {
                cols = Some(line.len());
            }
            line.chars().map(Symbol::from_char)
        })
        .collect();

    Grid::from_vec(grid_data, cols.expect("no data"))
}

fn run_from_tile(
    grid: &Grid<Symbol>,
    start_direction: Direction,
    position: (usize, usize),
) -> usize {
    let mut frontier = VecDeque::new();
    frontier.push_back((position, start_direction));

    let mut visited = bitvec![0; grid.cols() * grid.rows() * 4];

    let mut visit = |next_cell: (usize, usize), next_direction| {
        let idx = (next_cell.0 * grid.cols() + next_cell.1) * 4 + next_direction as usize;
        !visited.replace(idx, true)
    };

    let mut energized = Grid::new(grid.rows(), grid.cols());

    while let Some(((y, x), direction)) = frontier.pop_front() {
        *energized.get_mut(y, x).unwrap() = true;
        let symbol = grid.get(y, x).expect("invalid cell");
        let next_directions = symbol.next_direction(direction);

        for next_direction in next_directions {
            let next_cell = match next_direction {
                Direction::Up if y > 0 => (y - 1, x),
                Direction::Down if y < (grid.rows() - 1) => (y + 1, x),
                Direction::Left if x > 0 => (y, x - 1),
                Direction::Right if x < (grid.cols() - 1) => (y, x + 1),
                _ => continue,
            };

            if visit(next_cell, next_direction) {
                frontier.push_back((next_cell, next_direction));
            };
        }
    }

    energized.iter().filter(|x| **x).count()
}

pub fn part_one(input: &str) -> usize {
    let input = parse_input(input);
    run_from_tile(&input, Direction::Right, (0, 0))
}

pub fn part_two(input: &str) -> usize {
    let grid = parse_input(input);
    let mut max = 0;

    for x in 0..grid.cols() {
        max = cmp::max(max, run_from_tile(&grid, Direction::Down, (0, x)));
        max = cmp::max(
            max,
            run_from_tile(&grid, Direction::Up, (grid.rows() - 1, x)),
        );
    }

    for y in 0..grid.rows() {
        max = cmp::max(max, run_from_tile(&grid, Direction::Right, (y, 0)));
        max = cmp::max(
            max,
            run_from_tile(&grid, Direction::Left, (y, grid.cols() - 1)),
        );
    }

    max
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("examples", 16);
        assert_eq!(part_one(&input), 46);
    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 16);
        assert_eq!(part_two(&input), 51);
    }
}
