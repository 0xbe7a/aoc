use std::cmp::{max, min};

use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

type NumberIdx = u16;

struct RawBoard {
    numbers: Vec<u32>,
    digits: Vec<Option<NumberIdx>>,
    symbols: Vec<(Coord, char)>,
    size: usize,
}

struct Board {
    numbers: Vec<u32>,
    adjacent_numbers_to_symbols: Vec<(char, SmallVec<[NumberIdx; 8]>)>,
}

fn parse_input(input: &str) -> RawBoard {
    let mut symbols = Vec::new();
    let mut numbers = Vec::new();
    let size = input.lines().count();

    let mut digits = vec![None; size * size];

    let end_current_number =
        |numbers: &mut Vec<u32>, current_number: &mut Option<u32>| {
            if let Some(num) = current_number.take() {
                numbers.push(num);
            }
        };

    for (y, line) in input.lines().enumerate() {
        let mut current_number = None;

        for (x, c) in line.chars().enumerate() {
            match c {
                '0'..='9' => {
                    let d = c.to_digit(10).unwrap();

                    let num = current_number.get_or_insert(0);

                    *num *= 10;
                    *num += d;

                    digits[y * size + x] = Some(numbers.len() as NumberIdx);
                }
                _ => {
                    // end current number parsing, if any
                    end_current_number(&mut numbers, &mut current_number);

                    if c != '.' {
                        symbols.push((Coord { x, y }, c));
                    }
                }
            }
        }

        // end current number parsing, if any
        end_current_number(&mut numbers, &mut current_number);
    }

    RawBoard {
        digits,
        numbers,
        symbols,
        size,
    }
}

fn process_board(input: &str) -> Board {
    let RawBoard {
        digits,
        numbers,
        symbols,
        size,
    } = parse_input(input);

    // build up adjacency map for symbols
    let mut adjacent_numbers_to_symbols = Vec::new();

    for (symbol_cord, symbol) in symbols.into_iter() {
        let mut adjacent_numbers = SmallVec::<[NumberIdx; 8]>::new();

        let x_range =
            max(symbol_cord.x.saturating_sub(1), 0)..=min(symbol_cord.x + 1, size - 1);

        let y_range =
            max(symbol_cord.y.saturating_sub(1), 0)..=min(symbol_cord.y + 1, size - 1);

        for (x, y) in x_range.flat_map(|x| y_range.clone().map(move |y| (x, y))) {
            if symbol_cord.x == x && symbol_cord.y == y {
                continue;
            }

            if let Some(number_cord) = digits[y * size + x] {
                if adjacent_numbers.contains(&number_cord) {
                    continue;
                }
                adjacent_numbers.push(number_cord);
            }
        }

        if !adjacent_numbers.is_empty() {
            adjacent_numbers_to_symbols.push((symbol, adjacent_numbers));
        }
    }

    Board {
        adjacent_numbers_to_symbols,
        numbers,
    }
}

pub fn part_one(input: &str) -> u32 {
    let Board {
        numbers,
        adjacent_numbers_to_symbols,
    } = process_board(input);

    let mut is_part_number = vec![false; numbers.len()];

    for (_, numbers) in adjacent_numbers_to_symbols.into_iter() {
        for start_cord in numbers {
            is_part_number[start_cord as usize] = true;
        }
    }

    is_part_number
        .into_iter()
        .enumerate()
        .filter(|(_, is_part)| *is_part)
        .map(|(c, _)| numbers.get(c).unwrap())
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let Board {
        numbers,
        adjacent_numbers_to_symbols,
    } = process_board(input);

    let mut total_gear_ratios = 0;

    for (c, adjacent_numbers) in adjacent_numbers_to_symbols.into_iter() {
        if c != '*' || adjacent_numbers.len() != 2 {
            continue;
        }

        let gear_ratio = adjacent_numbers
            .iter()
            .map(|c| numbers.get(*c as usize).unwrap())
            .product::<u32>();

        total_gear_ratios += gear_ratio;
    }

    total_gear_ratios
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("examples", 3);
        assert_eq!(part_one(&input), 4361);

        let input = read_file("inputs", 3);
        assert_eq!(part_one(&input), 557705);
    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 3);
        assert_eq!(part_two(&input), 467835);

        let input = read_file("inputs", 3);
        assert_eq!(part_two(&input), 84266818);
    }
}
