use grid::Grid;

fn parse_input(input: &str) -> Grid<bool> {
    let mut data = Vec::new();
    let mut lines = 0;

    for line in input.lines() {
        for c in line.chars() {
            data.push('#' == c);
        }
        lines += 1;
    }
    let cols = data.len() / lines;
    Grid::from_vec(data, cols)
}

fn expand(grid: &Grid<bool>, factor: u32) -> Vec<(usize, usize)> {
    let mut cols = vec![true; grid.cols()];
    let mut rows = vec![true; grid.rows()];

    for (coord, &value) in grid.indexed_iter() {
        if value {
            cols[coord.1] = false;
            rows[coord.0] = false;
        }
    }

    let empty_cols = cols
        .into_iter()
        .scan(0, |state, x| {
            *state += (x as u32) * (factor - 1);
            Some(*state)
        })
        .collect::<Vec<_>>();

    let empty_rows = rows
        .into_iter()
        .scan(0, |state, x| {
            *state += (x as u32) * (factor - 1);
            Some(*state)
        })
        .collect::<Vec<_>>();

    let mut mapped_stars = Vec::new();

    for (coord, &value) in grid.indexed_iter() {
        if !value {
            continue;
        }

        let (my, mx) = (
            coord.0 + empty_rows[coord.0] as usize,
            coord.1 + empty_cols[coord.1] as usize,
        );
        mapped_stars.push((my, mx));
    }

    mapped_stars
}

fn shortest_pairs(stars: &[(usize, usize)]) -> usize {
    let mut total = 0;

    for (i, x) in stars.iter().enumerate() {
        for y in stars.iter().skip(i + 1) {
            total += x.0.abs_diff(y.0) + x.1.abs_diff(y.1);
        }
    }

    total
}

pub fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    let mapped_stars = expand(&grid, 2);
    shortest_pairs(&mapped_stars)
}

pub fn part_two(input: &str) -> usize {
    let grid = parse_input(input);
    let mapped_stars = expand(&grid, 1000000);
    shortest_pairs(&mapped_stars)
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("examples", 11);
        let grid = parse_input(&input);

        let mapped_stars = expand(&grid, 2);
        assert_eq!(shortest_pairs(&mapped_stars), 374);

        let mapped_stars = expand(&grid, 10);
        assert_eq!(shortest_pairs(&mapped_stars), 1030);

        let mapped_stars = expand(&grid, 100);
        assert_eq!(shortest_pairs(&mapped_stars), 8410);
    }
}
