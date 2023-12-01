use std::cmp::{max, min};

use nom::{
    bytes::complete::tag,
    character::complete::{char, u16},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};
use rustc_hash::FxHashSet;

type Coord = (u16, u16);
type Line = Vec<Coord>;

#[derive(Clone, Default)]
struct Cave {
    deepest_y: u16,
    blocked: FxHashSet<Coord>,
    path: Vec<Coord>,
    finite: bool,
}

impl Cave {
    fn is_blocked(&self, idx: &Coord) -> bool {
        self.blocked.contains(idx) || (self.finite && idx.1 == self.deepest_y)
    }

    fn set_blocked(&mut self, idx: Coord) {
        self.blocked.insert(idx);
    }

    fn set_rock(&mut self, idx: Coord) {
        if idx.1 > self.deepest_y {
            self.deepest_y = idx.1;
        }

        self.set_blocked(idx);
    }

    fn set_finite(&mut self) {
        self.finite = true;
        self.deepest_y += 2;
    }

    #[inline]
    fn gravity(&mut self, (x, y): Coord) -> Option<Coord> {
        for p in [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)] {
            if !self.is_blocked(&p) {
                return Some(p);
            }
        }

        return None;
    }

    fn spawn_sand(&mut self) -> Option<Coord> {
        let mut p = (500, 0);

        if self.is_blocked(&p) {
            return None;
        }

        //Warmstart
        p = self.path.pop().unwrap_or(p);

        loop {
            let next_point = match self.gravity(p) {
                Some(np) => np,
                None => break,
            };

            self.path.push(p);

            if next_point.1 > self.deepest_y {
                return None;
            }

            p = next_point;
        }

        self.set_blocked(p);
        Some(p)
    }
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let parse_coord = separated_pair(u16, char(','), u16);
    separated_list0(tag(" -> "), parse_coord)(input)
}

fn parse_input(input: &str) -> Cave {
    let mut cave = Cave::default();

    let lines = input
        .lines()
        .map(|line| parse_line(line).expect("Cant parse line").1);

    for line in lines {
        for &[start, end] in line.array_windows::<2>() {
            //This is not correct in general but works since all lines are either horizontal or vertical
            for x in min(start.0, end.0)..=max(start.0, end.0) {
                for y in min(start.1, end.1)..=max(start.1, end.1) {
                    cave.set_rock((x, y));
                }
            }
        }
    }

    cave
}

pub fn part_one(input: &str) -> u32 {
    let mut cave = parse_input(input);

    let mut count = 0;
    while cave.spawn_sand().is_some() {
        count += 1;
    }
    count
}

pub fn part_two(input: &str) -> u32 {
    let mut cave = parse_input(input);

    cave.set_finite();

    let mut count = 0;
    while cave.spawn_sand().is_some() {
        count += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 14);
        assert_eq!(part_one(&input), 24);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 14);
        assert_eq!(part_two(&input), 93);
    }
}
