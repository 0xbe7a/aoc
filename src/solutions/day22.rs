use std::collections::VecDeque;

use itertools::{Itertools, MinMaxResult};
use nom::{
    branch::alt,
    character::complete::{char, u16},
    multi::many1,
    IResult, combinator::map,
};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tile {
    Blocked,
    Open,
    Outside,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

enum Instruction {
    Right,
    Left,
    Steps(u16)
}

struct Input {
    tiles: Vec<Vec<Tile>>,
    row_ranges: Vec<(usize, usize)>,
    col_ranges: Vec<(usize, usize)>,
    instructions: Vec<Instruction>,
}

fn parse_direction(i: &str) -> IResult<&str, Instruction> {
    let (i, instruction) = alt((char('L'), char('R')))(i)?;
    Ok((
        i,
        match instruction {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => unreachable!(),
        },
    ))
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    alt((parse_direction, map(u16, |s| Instruction::Steps(s))))(i)
}

fn parse_input(input: &str) -> Input {
    let (raw_tiles, raw_instructions) = input.split_once("\n\n").unwrap();
    let mut tiles: Vec<Vec<_>> = raw_tiles
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Tile::Open,
                    '#' => Tile::Blocked,
                    ' ' => Tile::Outside,
                    _ => panic!("Unknown char"),
                })
                .collect()
        })
        .collect();

    let max_row = tiles.iter().map(|l| l.len()).max().unwrap();
    tiles
        .iter_mut()
        .for_each(|l| l.resize(max_row, Tile::Outside));

    let col_ranges: Vec<_> = (0..max_row)
        .into_iter()
        .map(|c| {
            let minmax = tiles
                .iter()
                .enumerate()
                .filter_map(|(idx, r)| {
                    if r[c] != Tile::Outside {
                        Some(idx)
                    } else {
                        None
                    }
                })
                .minmax();

            match minmax {
                MinMaxResult::NoElements => panic!("Empty col"),
                MinMaxResult::OneElement(x) => (x, x),
                MinMaxResult::MinMax(x, y) => (x, y),
            }
        })
        .collect();

    let row_ranges: Vec<_> = tiles
        .iter()
        .map(|r| {
            let minmax = r
                .iter()
                .enumerate()
                .filter_map(|(idx, r)| if *r != Tile::Outside { Some(idx) } else { None })
                .minmax();

            match minmax {
                MinMaxResult::NoElements => panic!("Empty row"),
                MinMaxResult::OneElement(x) => (x, x),
                MinMaxResult::MinMax(x, y) => (x, y),
            }
        })
        .collect();

    let instructions = many1(parse_instruction)(raw_instructions).unwrap();
    println!("{}", instructions.0);
    assert!(instructions.0.is_empty());

    Input {
        tiles,
        row_ranges,
        col_ranges,
        instructions: instructions.1,
    }
}

pub fn part_one(input: &str) -> i32 {
    let input = parse_input(input);
    let mut position = (input.row_ranges[0].0 as i32, 0 as i32);
    let mut direction = Direction::Right;

    let mut dequeue = VecDeque::from(input.instructions);

    while let Some(instruction) = dequeue.pop_front() {
        let steps = match instruction {
            Instruction::Right => {
                direction = direction.right();
                continue
            }
            Instruction::Left => {
                direction = direction.left();
                continue
            },
            Instruction::Steps(s) => s,
        };

        for _ in 0..steps {
            let (row_range, col_range) = (
                input.row_ranges[position.1 as usize],
                input.col_ranges[position.0 as usize],
            );
            let (x, y) = match direction {
                Direction::Up => (position.0, position.1 - 1),
                Direction::Right => (position.0 + 1, position.1),
                Direction::Down => (position.0, position.1 + 1),
                Direction::Left => (position.0 - 1, position.1),
            };

            let wrapped_x =
                (x - row_range.0 as i32).rem_euclid((row_range.1 - row_range.0) as i32 + 1) + row_range.0 as i32;
            let wrapped_y =
                (y - col_range.0 as i32).rem_euclid((col_range.1 - col_range.0) as i32 + 1) + col_range.0 as i32;


            assert_ne!(input.tiles[wrapped_y as usize][wrapped_x as usize], Tile::Outside);

            if input.tiles[wrapped_y as usize][wrapped_x as usize] == Tile::Open {
                position = (wrapped_x, wrapped_y);
            } else {
                break;
            }
        }
    }

    let password = 1000 * (position.1 + 1)
        + 4 * (position.0 + 1)
        + match direction {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };

    password
}

#[derive(Debug)]
enum Side {
    Up,
    Right,
    Front,
    Down,
    Left,
    Back
}

pub fn part_two(input: &str) -> i32 {
    let input = parse_input(input);
    let mut position = (input.row_ranges[0].0 as i32, 0 as i32);
    let mut direction = Direction::Right;

    let mut dequeue = VecDeque::from(input.instructions);

    while let Some(instruction) = dequeue.pop_front() {
        let steps = match instruction {
            Instruction::Right => {
                direction = direction.right();
                continue
            }
            Instruction::Left => {
                direction = direction.left();
                continue
            },
            Instruction::Steps(s) => s,
        };

        for _ in 0..steps {
            

            let side = match (position.0 / 50, position.1 / 50) {
                (1, 0) => Side::Up,
                (2, 0) => Side::Right,
                (1, 1) => Side::Front,
                (1, 2) => Side::Down,
                (0, 2) => Side::Left,
                (0, 3) => Side::Back,
                _ => panic!("Unknown side")
            };


            let (row_range, col_range) = (
                input.row_ranges[position.1 as usize],
                input.col_ranges[position.0 as usize],
            );


            
            let (mut x, mut y) = position;
            let mut next_direction = direction.clone();

            if &direction == &Direction::Right && position.0 >= row_range.1 as i32 {
                match side {
                    Side::Right => {
                        (x, y) = (99, 149 - y);
                        next_direction = Direction::Left
                    },
                    Side::Front => {
                        (x, y) = (100 + (y - 50), 49);
                        next_direction = Direction::Up;
                    },
                    Side::Down => {
                        (x, y) = (149, 49 - (y - 100));
                        next_direction = Direction::Left;
                    },
                    Side::Back => {
                        (x, y) = ((y - 150) + 50, 149);
                        next_direction = Direction::Up;
                    },
                    _ => ()
                }
            } else if &direction == &Direction::Left && position.0 <= row_range.0 as i32 {
                match side {
                    Side::Up => {
                        (x, y) = (0, 149 - y);
                        next_direction = Direction::Right;
                    },
                    Side::Front => {
                        (x, y) = (y - 50, 100);
                        next_direction = Direction::Down;
                    },
                    Side::Left => {
                        (x, y) = (50, 49 - (y - 100));
                        next_direction = Direction::Right;
                    },
                    Side::Back => {
                        (x, y) = ((y - 150) + 50, 0);
                        next_direction = Direction::Down;
                    },
                    _ => ()
                }
            } else if &direction == &Direction::Down && position.1 >= col_range.1 as i32{
                match side {
                    Side::Right => {
                        (x, y) = (99, x - 100 + 50);
                        next_direction = Direction::Left;
                    },
                    Side::Down => {
                        (x, y) = (49, x + 100);
                        next_direction = Direction::Left;
                    },
                    Side::Back => {
                        (x, y) = (x + 100, 0);
                        next_direction = Direction::Down;
                    },
                    _ => ()
                }
            } else if &direction == &Direction::Up && position.1 <= col_range.0 as i32 {
                match side {
                    Side::Right => {
                        (x, y) = (x - 100, 199);
                        next_direction = Direction::Up;
                    },
                    Side::Up => {
                        (x, y) = (0, x + 100);
                        next_direction = Direction::Right;
                    },
                    Side::Left => {
                        (x, y) = (50, x + 50);
                        next_direction = Direction::Right;
                    },
                    _ => ()
                }
            } else {
                (x, y) = match direction {
                    Direction::Up => (position.0, position.1 - 1),
                    Direction::Right => (position.0 + 1, position.1),
                    Direction::Down => (position.0, position.1 + 1),
                    Direction::Left => (position.0 - 1, position.1),
                };
            }
                
            if input.tiles[y as usize][x as usize] == Tile::Open {
                position = (x, y);
                direction = next_direction;
            } else {
                break;
            }
        }
    }

    let password = 1000 * (position.1 + 1)
        + 4 * (position.0 + 1)
        + match direction {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };

    password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("inputs", 22);
        assert_eq!(part_one(&input), 117054);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("inputs", 22);
        assert_eq!(part_two(&input), 162096);
    }
}
