use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

type Offset = (u8, u8);
type Coordinates = (usize, usize);

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Down,
}

#[derive(Clone, Copy, Debug)]
enum Shape {
    Minus,
    Plus,
    Angle,
    Pillar,
    Square,
}

impl Shape {
    fn fields(&self) -> &[Offset] {
        match self {
            Self::Minus => &[(0, 0), (1, 0), (2, 0), (3, 0)],
            Self::Plus => &[(0, 1), (1, 2), (1, 1), (1, 0), (2, 1)],
            Self::Angle => &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Self::Pillar => &[(0, 0), (0, 1), (0, 2), (0, 3)],
            Self::Square => &[(0, 0), (1, 0), (0, 1), (1, 1)],
        }
    }

    fn iter() -> impl Iterator<Item = Self> + Clone {
        [
            Shape::Minus,
            Shape::Plus,
            Shape::Angle,
            Shape::Pillar,
            Shape::Square,
        ]
        .iter()
        .copied()
    }
}

fn read_jets(input: &str) -> impl Iterator<Item = Direction> + Clone + '_ {
    input.chars().map(|c| match c {
        '<' => Direction::Left,
        '>' => Direction::Right,
        _ => panic!("Cant parse jet"),
    })
}

fn run_tetris(input: &str, count: usize) -> usize {
    let mut jets = read_jets(input).enumerate().cycle().peekable();
    let mut shape_iter = Shape::iter().enumerate().cycle();
    let mut fields: HashSet<Coordinates> = HashSet::new();
    let mut dropped: usize = 0;

    let mut return_delta = 0;
    let mut highest_y = 0;
    let mut levels = [0; 7];

    let mut states = HashMap::new();
    
    loop {
        let (shape_i, shape) = shape_iter.next().unwrap();

        if dropped == count {
            return return_delta + highest_y;
        }

        if return_delta == 0 {
            let (jet_i, _) = *jets.peek().unwrap();
            let lowest = levels.iter().min().unwrap();

            let mut relative_heights = levels.clone();
            for c in relative_heights.iter_mut() {
                *c -= lowest
            }

            let key = (shape_i, jet_i, relative_heights);

            if let Some((dropped_cycle, y_cycle)) = states.get(&key) {
                let skip_size = dropped - dropped_cycle;
                let skip_count = (count - dropped).div_floor(skip_size);
                dropped += skip_count * skip_size;
                return_delta = (highest_y - y_cycle) * skip_count;
            } else {
                states.insert(key, (dropped, highest_y));
            }
        }

        dropped += 1;

        let mut origin: Coordinates = (3, highest_y + 4);
        let mut jet = true;

        loop {
            let direction = if jet {
                let (_, direction) = jets.next().unwrap();
                direction
            } else {
                Direction::Down
            };

            let new_origin = match direction {
                Direction::Down => (origin.0, origin.1 - 1),
                Direction::Left => (origin.0 - 1, origin.1),
                Direction::Right => (origin.0 + 1, origin.1),
            };

            let mut collides = false;

            for offset in shape.fields() {
                let p = (
                    new_origin.0 + offset.0 as usize,
                    new_origin.1 + offset.1 as usize,
                );

                //Walls
                if p.0 == 0 || p.0 == 8 {
                    collides = true;
                };

                //Bottom
                if p.1 == 0 {
                    collides = true;
                }

                if fields.contains(&p) {
                    collides = true;
                }
            }

            if collides && !jet {
                break;
            } else if !collides {
                origin = new_origin;
            }

            jet = !jet;
        }

        for offset in shape.fields() {
            let p = (origin.0 + offset.0 as usize, origin.1 + offset.1 as usize);
            fields.insert(p);

            levels[p.0 - 1] = max(levels[p.0 - 1], p.1);
            highest_y = max(p.1, highest_y);
        }
    }
}

pub fn part_one(input: &str) -> usize {
    run_tetris(input, 2022)
}
pub fn part_two(input: &str) -> usize {
    run_tetris(input, 1000000000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 17);
        assert_eq!(part_one(&input), 3068);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 17);
        assert_eq!(part_two(&input), 1514285714288);
    }
}
