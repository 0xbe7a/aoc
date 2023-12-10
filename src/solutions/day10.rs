use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Starting,
}

#[derive(Debug)]
enum Port {
    North,
    East,
    South,
    West,
}

impl Port {
    fn offset(&self) -> (isize, isize) {
        match self {
            Self::North => (-1, 0),
            Self::East => (0, 1),
            Self::South => (1, 0),
            Self::West => (0, -1),
        }
    }
}

impl Pipe {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '|' => Some(Self::Vertical),
            '-' => Some(Self::Horizontal),
            'L' => Some(Self::NorthEast),
            'J' => Some(Self::NorthWest),
            '7' => Some(Self::SouthWest),
            'F' => Some(Self::SouthEast),
            'S' => Some(Self::Starting),
            '.' => None,
            _ => panic!("invalid pipe char: {}", c),
        }
    }

    fn connections(&self) -> (bool, bool, bool, bool) {
        match self {
            Self::Vertical => (true, false, true, false),
            Self::Horizontal => (false, true, false, true),
            Self::NorthEast => (true, true, false, false),
            Self::NorthWest => (true, false, false, true),
            Self::SouthWest => (false, false, true, true),
            Self::SouthEast => (false, true, true, false),
            Self::Starting => (true, true, true, true),
        }
    }

    fn is_connected_to(&self, next: &Self, port: &Port) -> bool {
        let (north, east, south, west) = self.connections();
        let (next_north, next_east, next_south, next_west) = next.connections();

        match port {
            Port::North => north && next_south,
            Port::East => east && next_west,
            Port::South => south && next_north,
            Port::West => west && next_east,
        }
    }
}

struct Input {
    grid: Vec<Option<Pipe>>,
    size: (usize, usize),
    starting_point: (usize, usize),
}

fn parse_input(input: &str) -> Input {
    let mut starting_point = None;
    let mut grid = Vec::new();

    let mut no_lines = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                starting_point = Some((y, x));
            }

            grid.push(Pipe::from_char(c));
        }

        no_lines += 1;
    }

    let size = (no_lines, grid.len() / no_lines);

    Input {
        grid,
        starting_point: starting_point.expect("No starting point found"),
        size,
    }
}

fn flood_loop(input: &Input) -> HashMap<(usize, usize), u32> {
    let Input {
        grid,
        size,
        starting_point,
    } = input;

    let mut frontier = VecDeque::new();
    frontier.push_back((*starting_point, 0, *starting_point));

    let mut distances: HashMap<(usize, usize), u32> = HashMap::new();

    while let Some((pipe, distance, origin)) = frontier.pop_front() {
        if distances.contains_key(&pipe) {
            break;
        }

        distances.insert(pipe, distance);
        let current_pipe_shape = grid[pipe.0 * size.1 + pipe.1].as_ref().unwrap();

        for port in [Port::North, Port::East, Port::South, Port::West].iter() {
            let (dy, dx) = port.offset();
            let next_y = pipe.0.saturating_add_signed(dy).min(size.0 - 1);
            let next_x = pipe.1.saturating_add_signed(dx).min(size.1 - 1);

            if let Some(next_pipe_shape) = &grid[next_y * size.1 + next_x] {
                if current_pipe_shape.is_connected_to(next_pipe_shape, port) {
                    if (next_y, next_x) == origin {
                        continue;
                    }

                    frontier.push_back(((next_y, next_x), distance + 1, pipe));
                }
            }
        }
    }

    distances
}

pub fn part_one(input: &str) -> u32 {
    let inputs = parse_input(input);
    let distances = flood_loop(&inputs);

    distances.into_values().max().unwrap()
}

pub fn part_two(input: &str) -> u32 {
    let input = parse_input(input);
    let distances = flood_loop(&input);

    let mut inside = 0;

    for y in 0..input.size.0 {
        let mut is_inside = false;

        for x in 0..input.size.1 {
            let shape = input.grid[y * input.size.1 + x].as_ref();

            if distances.contains_key(&(y, x)) {
                if shape.as_ref().unwrap().connections().0 {
                    is_inside = !is_inside;
                }

                continue;
            }

            if is_inside {
                inside += 1;
            }
        }
    }

    inside
}

#[cfg(test)]
mod tests {
    use crate::{read_file, read_file_with_name};

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("examples", 10);
        assert_eq!(part_one(&input), 8);

        let input = read_file("inputs", 10);
        assert_eq!(part_one(&input), 6599);
    }

    #[test]
    fn test_part_two() {
        let input = read_file_with_name("examples", "10_2");
        assert_eq!(part_two(&input), 10);

        let input = read_file("inputs", 10);
        assert_eq!(part_two(&input), 477);
    }
}
