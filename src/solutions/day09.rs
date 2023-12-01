use std::{collections::HashSet};

enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

struct Movement {
    direction: Direction,
    steps: usize,
}

fn read_knots(input: &str) -> impl Iterator<Item = Movement> + '_ {
    input
        .lines()
        .map(|line| {
            let (dir_str, steps_str) = line.split_once(' ').expect("Cant parse line");
            let direction = match dir_str {
                "U" => Direction::UP,
                "R" => Direction::RIGHT,
                "D" => Direction::DOWN,
                "L" => Direction::LEFT,
                _ => panic!("Cant parse direction"),
            };

            let steps = steps_str.parse().expect("Cant parse step size");

            Movement { steps, direction }
        })
}

pub fn part_one(input: &str) -> usize {
    move_rope::<1>(input)
}

pub fn move_rope<const KNOTS: usize>(input: &str) -> usize {
    let steps = read_knots(input);
    let (mut head_x, mut head_y) = (0i32, 0i32);
    let mut positions = [(0, 0); KNOTS];
    let mut visited_positions = HashSet::new();

    for movement in steps {
        for _ in 0..movement.steps {
            match movement.direction {
                Direction::UP => head_y += 1,
                Direction::RIGHT => head_x += 1,
                Direction::DOWN => head_y -= 1,
                Direction::LEFT => head_x -= 1,
            };

            let (mut prev_x, mut prev_y) = (head_x, head_y);

            for (knot_x, knot_y) in positions.iter_mut() {
                let (dx, dy) = (prev_x - *knot_x, prev_y - *knot_y);
                if dx.abs() > 1 || dy.abs() > 1 {
                    *knot_x += dx.signum();
                    *knot_y += dy.signum();
                }
                (prev_x, prev_y) = (*knot_x, *knot_y);
            }

            visited_positions.insert((prev_x, prev_y));
        }
    }

    visited_positions.len()
}

pub fn part_two(input: &str) -> usize {
    move_rope::<9>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 9);
        assert_eq!(part_one(&input), 13);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("inputs", 9);
        assert_eq!(part_two(&input), 2593);
    }
}
