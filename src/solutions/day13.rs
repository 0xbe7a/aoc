use grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Symbol {
    Ash,
    Rocks,
}

const VERTICAL_MULTIPLIER: usize = 1;
const HORIZONTAL_MULTIPLIER: usize = 100;

impl Symbol {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Ash,
            '#' => Self::Rocks,
            _ => panic!("invalid symbol: {}", c),
        }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Grid<Symbol>> + '_ {
    input.split("\n\n").map(|pattern| {
        let mut grid_data = Vec::new();
        let mut width = 0;

        for line in pattern.lines() {
            width = line.len();

            for c in line.chars() {
                grid_data.push(Symbol::from_char(c));
            }
        }
        Grid::from_vec(grid_data, width)
    })
}

fn check_mirror(
    grid: &Grid<Symbol>,
    mirror_line: usize,
    is_vertical: bool,
    allow_smudge: bool,
) -> bool {
    let mut has_smudge = false;

    let primary_dim = if is_vertical {
        grid.rows()
    } else {
        grid.cols()
    };

    for primary in 0..primary_dim {
        for secondary in 0..=mirror_line {
            let mapped_secondary = (mirror_line - secondary) + mirror_line + 1;
            let (original, mapped) = if is_vertical {
                (
                    grid.get(primary, secondary).unwrap(),
                    grid.get(primary, mapped_secondary),
                )
            } else {
                (
                    grid.get(secondary, primary).unwrap(),
                    grid.get(mapped_secondary, primary),
                )
            };

            match mapped {
                Some(symbol) if original != symbol => {
                    if !allow_smudge || has_smudge {
                        return false;
                    }
                    has_smudge = true;
                }
                _ => continue,
            }
        }
    }

    allow_smudge == has_smudge
}

fn process_grid(input: &str, allow_smudge: bool) -> usize {
    let mut total = 0;

    'pattern: for grid in parse_input(input) {
        for x in 0..(grid.cols() - 1) {
            if check_mirror(&grid, x, true, allow_smudge) {
                total += (x + 1) * VERTICAL_MULTIPLIER;
                continue 'pattern;
            }
        }

        for y in 0..(grid.rows() - 1) {
            if check_mirror(&grid, y, false, allow_smudge) {
                total += (y + 1) * HORIZONTAL_MULTIPLIER;
                continue 'pattern;
            }
        }
    }

    total
}

pub fn part_one(input: &str) -> usize {
    process_grid(input, false)
}

pub fn part_two(input: &str) -> usize {
    process_grid(input, true)
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("examples", 13);
        assert_eq!(part_one(&input), 405);
    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 13);
        assert_eq!(part_two(&input), 400);
    }
}
