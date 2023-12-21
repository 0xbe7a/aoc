use std::collections::HashSet;

use grid::Grid;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Cell {
    Starting,
    Plots,
    Rocks,
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            'S' => Self::Starting,
            '.' => Self::Plots,
            '#' => Self::Rocks,
            _ => panic!("invalid cell"),
        }
    }
}

fn parse_grid(input: &str) -> (Grid<Cell>, (usize, usize)) {
    let mut cols: Option<usize> = None;
    let mut starting_pos: Option<(usize, usize)> = None;
    let mut data = Vec::new();

    for (y, line) in input.lines().enumerate() {
        if y == 0 {
            cols = Some(line.len());
        }

        for (x, c) in line.chars().enumerate() {
            let s = Cell::from_char(c);

            if matches!(s, Cell::Starting) {
                starting_pos = Some((y, x));
            }

            data.push(s);
        }
    }

    (
        Grid::from_vec(data, cols.expect("no data")),
        starting_pos.expect("no starting pos"),
    )
}

pub fn part_one(input: &str) -> usize {
    let (grid, starting_pos) = parse_grid(input);
    let mut frontier = HashSet::new();
    frontier.insert(starting_pos);

    for _ in 0..64 {
        let mut next_frontier = HashSet::new();

        for (y, x) in frontier {
            for (dy, dx) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (Some(ny), Some(nx)) = (y.checked_add_signed(*dy), x.checked_add_signed(*dx))
                else {
                    continue;
                };

                match grid.get(ny, nx) {
                    Some(Cell::Rocks) => continue,
                    None => continue,
                    _ => (),
                }

                next_frontier.insert((ny, nx));
            }
        }

        frontier = next_frontier;
    }

    frontier.len()
}

pub fn part_two(input: &str) -> usize {
    let (grid, (sy, sx)) = parse_grid(input);

    let mut frontier = HashSet::new();
    frontier.insert((sy as isize, sx as isize));

    let mut y = Vec::new();

    for i in 1..=(65 + 2 * grid.cols()) {
        let mut next_frontier = HashSet::new();

        for (y, x) in frontier {
            for (dy, dx) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (ny, nx) = (y + dy, x + dx);

                let (iy, ix) = (
                    (ny.rem_euclid(grid.rows() as isize)) as usize,
                    (nx.rem_euclid(grid.cols() as isize)) as usize,
                );

                if matches!(grid[(iy, ix)], Cell::Rocks) {
                    continue;
                }

                next_frontier.insert((ny, nx));
            }
        }

        frontier = next_frontier;

        if i % grid.cols() == 65 {
            y.push(frontier.len());
        }
    }

    let x = (26501365 - 65) / grid.cols();
    let y = interpolate(y[0] as f64, y[1] as f64, y[2] as f64, x as f64);

    y as usize
    
}


fn interpolate(y0: f64, y1: f64, y2: f64, x: f64) -> f64 {
    let a = y0 / 2.0 - y1 + y2 / 2.0;
    let b = -3.0 * y0 / 2.0 + 2.0 * y1 - y2 / 2.0;
    let c = y0;

    a * x.powf(2.) + b * x + c
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("inputs", 21);
        assert_eq!(part_one(&input), 3666);
    }
}
