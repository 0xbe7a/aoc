use std::cmp::max;

use itertools::Itertools;
use rustc_hash::FxHashMap as HashMap;

type Coord = (usize, usize, usize);

fn parse_cube(line: &str) -> Brick {
    let (start, end) = line.split_once('~').expect("cant parse cube");

    let parse_coord = |s: &str| {
        let mut parts = s.split(',');

        let mut parse_digit = || {
            parts
                .next()
                .expect("cant parse coord")
                .parse::<usize>()
                .expect("cant parse coord")
        };

        (parse_digit(), parse_digit(), parse_digit())
    };

    Brick {
        start: parse_coord(start),
        end: parse_coord(end),
    }
}

#[derive(Debug, Clone)]
struct Brick {
    start: Coord,
    end: Coord,
}

impl Brick {
    fn iter(&self) -> impl Iterator<Item = Coord> + '_ {
        let (x1, y1, z1) = self.start;
        let (x2, y2, z2) = self.end;

        (x1..=x2).flat_map(move |x| (y1..=y2).flat_map(move |y| (z1..=z2).map(move |z| (x, y, z))))
    }
}

type ColumnIdx = (usize, usize);
type CubeIdx = usize;
type ColumnPosition = usize;

#[derive(Debug, Clone)]
struct Universe {
    bricks: Vec<Brick>,
    column_idxs: HashMap<CubeIdx, Vec<(ColumnIdx, ColumnPosition)>>,
    column_vectors: HashMap<ColumnIdx, Vec<CubeIdx>>,
}

impl Universe {
    fn new(bricks: Vec<Brick>) -> Self {
        let mut raw_intersection_map: HashMap<ColumnIdx, Vec<_>> = HashMap::default();

        for (idx, cube) in bricks.iter().enumerate() {
            for coord in cube.iter() {
                raw_intersection_map
                    .entry((coord.0, coord.1))
                    .or_default()
                    .push((coord.2, idx));
            }
        }

        // pieces that share a common (x, y) column can never "overtake" each other
        // as this would require a piece to move through another piece in at least one location

        // A map of cube idx -> Vec<(x, y, position inside column vec)>
        let mut column_idxs: HashMap<CubeIdx, Vec<(ColumnIdx, ColumnPosition)>> =
            HashMap::default();

        // The column vecs for each (x, y) are sorted by z (descending)
        // As pieces in the same column can never overtake each other, we can
        // iterate over the column vecs and only need to check the distance to the next piece.
        // `column_idxs` allows us to quickly our position and the next piece in a column.
        let mut column_vectors: HashMap<ColumnIdx, Vec<CubeIdx>> = HashMap::default();

        for (column, v) in raw_intersection_map.iter_mut() {
            v.sort_unstable();
            v.reverse();

            let mut cube_idx_sorted_by_z = Vec::with_capacity(v.len());

            for (column_position, cube_idx) in v.iter().enumerate() {
                column_idxs
                    .entry(cube_idx.1)
                    .or_default()
                    .push(((column.0, column.1), column_position));
                cube_idx_sorted_by_z.push(cube_idx.1);
            }

            column_vectors.insert(*column, cube_idx_sorted_by_z);
        }

        Universe {
            bricks,
            column_idxs,
            column_vectors,
        }
    }

    fn check_gravity(&self, idx: usize, filter: Option<usize>) -> usize {
        let brick = &self.bricks[idx];

        let mut min_distance = None;
        for ((x, y), column_position) in self.column_idxs.get(&idx).unwrap() {
            let column_cube_idxs = self.column_vectors.get(&(*x, *y)).unwrap();

            for other_cube_idx in column_cube_idxs[*column_position..].iter() {
                if idx == *other_cube_idx {
                    continue;
                }

                if Some(*other_cube_idx) == filter {
                    continue;
                }

                let other_cube = &self.bricks[*other_cube_idx];

                assert!(brick.start.2 <= brick.end.2);
                assert!(other_cube.start.2 < brick.start.2);

                let distance = brick.start.2 - other_cube.end.2 - 1;

                if min_distance.is_none() || distance < min_distance.unwrap() {
                    min_distance = Some(distance);
                }

                break;
            }
        }

        min_distance.unwrap_or(brick.start.2 - 1)
    }

    fn get_cubes_by_z(&self) -> Vec<(usize, usize, usize)> {
        self.bricks
            .iter()
            .enumerate()
            .sorted_unstable_by_key(|(_, c)| c.start.2)
            .map(|(i, c)| (i, c.start.2, c.end.2))
            .collect()
    }

    fn settle(&mut self) {
        let cubes_by_z: Vec<_> = self.get_cubes_by_z();

        for (idx, _, _) in cubes_by_z {
            let distance = self.check_gravity(idx, None);
            if distance > 0 {
                let brick = &mut self.bricks[idx];

                brick.start.2 -= distance;
                brick.end.2 -= distance;
            }
        }
    }

    fn non_critical_1(&self) -> usize {
        let cubes_by_z: Vec<_> = self.get_cubes_by_z();

        let mut non_critical = 0;

        for (scan_idx, (idx, _, last_z)) in cubes_by_z.iter().enumerate() {
            let mut critical = false;

            for (other_idx, other_first_z, _) in cubes_by_z[scan_idx + 1..].iter() {
                if self.check_gravity(*other_idx, Some(*idx)) > 0 {
                    critical = true;
                }

                if (last_z + 1) < *other_first_z {
                    break;
                }
            }

            if !critical {
                non_critical += 1;
            }
        }

        non_critical
    }

    fn non_critical_2(&self) -> usize {
        let cubes_by_z: Vec<_> = self.get_cubes_by_z();
        let mut total_moved = 0;

        for (start_pos, (removed, _, mut last_z)) in cubes_by_z.iter().enumerate() {
            let mut moved = 0;
            let mut other_universe = self.clone();

            for (idx, start_z, end_z) in cubes_by_z[(start_pos + 1)..].iter() {
                if (last_z + 1) < *start_z {
                    break;
                }

                let distance = other_universe.check_gravity(*idx, Some(*removed));
                if distance > 0 {
                    let brick = &mut other_universe.bricks[*idx];

                    last_z = max(*end_z, last_z);
                    moved += 1;
                    brick.start.2 -= distance;
                    brick.end.2 -= distance;
                }
            }

            total_moved += moved;
        }

        total_moved
    }
}

pub fn part_one(input: &str) -> usize {
    let bricks = input.lines().map(parse_cube).collect::<Vec<_>>();
    let mut universe = Universe::new(bricks);

    universe.settle();
    universe.non_critical_1()
}

pub fn part_two(input: &str) -> usize {
    let bricks = input.lines().map(parse_cube).collect::<Vec<_>>();
    let mut universe = Universe::new(bricks);

    universe.settle();
    universe.non_critical_2()
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("examples", 22);
        assert_eq!(part_one(&input), 5);

        let input = read_file("inputs", 22);
        assert_eq!(part_one(&input), 465);
    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 22);
        assert_eq!(part_two(&input), 7);

        let input = read_file("inputs", 22);
        assert_eq!(part_two(&input), 79042);
    }
}
