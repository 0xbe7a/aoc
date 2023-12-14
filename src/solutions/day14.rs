use grid::Grid;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PartialOrd, Ord, Hash)]
enum Symbol {
    #[default]
    Empty,
    Round,
    Cube,
}

impl Symbol {
    fn from_char(c: char) -> Self {
        match c {
            'O' => Self::Round,
            '#' => Self::Cube,
            '.' => Self::Empty,
            _ => panic!("invalid symbol: {}", c),
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

enum Direction {
    North,
    East,
    South,
    West,
}

fn tilt_platform(grid: &mut Grid<Symbol>, direction: Direction) {
    match direction {
        Direction::North => grid.rotate_right(),
        Direction::East => {}
        Direction::South => grid.rotate_left(),
        Direction::West => grid.rotate_half(),
    };

    let mut new_grid_data = Vec::with_capacity(grid.rows() * grid.cols());

    for mut row in grid.iter_rows() {
        loop {
            let mut empty = 0;
            let mut round = 0;
            let mut has_cube = false;

            for sym in row.by_ref() {
                match sym {
                    Symbol::Empty => empty += 1,
                    Symbol::Round => round += 1,
                    Symbol::Cube => {
                        has_cube = true;
                        break;
                    }
                }
            }

            for _ in 0..empty {
                new_grid_data.push(Symbol::Empty);
            }

            for _ in 0..round {
                new_grid_data.push(Symbol::Round);
            }

            if has_cube {
                new_grid_data.push(Symbol::Cube);
            } else {
                break
            }
        }
    }

    let mut new_grid = Grid::from_vec(new_grid_data, grid.cols());

    match direction {
        Direction::North => new_grid.rotate_left(),
        Direction::East => {}
        Direction::South => new_grid.rotate_right(),
        Direction::West => new_grid.rotate_half(),
    };

    *grid = new_grid;
}

fn get_weight(grid: &Grid<Symbol>) -> usize {
    grid.iter_rows()
        .enumerate()
        .map(|(i, row)| {
            let multiplier = grid.rows() - i;
            let weight = row.filter(|x| matches!(x, Symbol::Round)).count();

            weight * multiplier
        })
        .sum()
}

pub fn part_one(input: &str) -> usize {
    let mut grid = parse_input(input);
    tilt_platform(&mut grid, Direction::North);
    get_weight(&grid)
}

pub fn part_two(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut seen = HashMap::new();
    let mut i = 0;

    let tilt_cycle = |grid: &mut Grid<_>| {
        tilt_platform(grid, Direction::North);
        tilt_platform(grid, Direction::West);
        tilt_platform(grid, Direction::South);
        tilt_platform(grid, Direction::East);
    };

    loop {
        tilt_cycle(&mut grid);
        i += 1;

        match seen.get(&grid) {
            Some(&prev_i) => {
                let loop_size = i - prev_i;
                let remaining_loops = (1_000_000_000 - i) % loop_size;
                for _ in 0..remaining_loops {
                    tilt_cycle(&mut grid);
                }
                break;
            }
            None => {
                seen.insert(grid.clone(), i);
            }
        }
    }

    get_weight(&grid)
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("examples", 14);
        assert_eq!(part_one(&input), 136);
    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 14);
        assert_eq!(part_two(&input), 64);

        let input = read_file("inputs", 14);
        assert_eq!(part_two(&input), 102509);
    }
}
