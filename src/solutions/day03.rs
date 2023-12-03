use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct ContingousNumber {
    coord: Coord,
    length: u8,
    value: u32,
}

struct RawBoard {
    digits: HashMap<Coord, Coord>,
    numbers: HashMap<Coord, ContingousNumber>,
    symbols: HashMap<Coord, char>,
    size: (usize, usize),
}

struct Board {
    numbers: HashMap<Coord, ContingousNumber>,
    adjacent_numbers_to_symbols: HashMap<Coord, (char, HashSet<Coord>)>,
}

fn parse_input(input: &str) -> RawBoard {
    let mut symbols = HashMap::new();
    let mut numbers = HashMap::new();
    let mut digits = HashMap::new();

    let mut store_number = |current_number: &mut Option<ContingousNumber>| {
        if let Some(num) = current_number.take() {
            numbers.insert(num.coord, num);
        }
    };

    let mut size = (None, 0);

    for (y, line) in input.lines().enumerate() {
        if size.0.is_none() {
            size.0 = Some(line.len());
        } else {
            assert_eq!(
                size.0.unwrap(),
                line.len(),
                "lines should have the same length"
            );
        }

        size.1 = y + 1;

        let mut current_number = None;

        for (x, c) in line.chars().enumerate() {
            match c {
                '0'..='9' => {
                    let d =  c.to_digit(10).unwrap() as u8;

                    let num = current_number.get_or_insert(ContingousNumber {
                        coord: Coord { x, y },
                        length: 0,
                        value: 0,
                    });

                    num.length += 1;
                    num.value = num.value * 10 + d as u32;
                    digits.insert(Coord { x, y }, num.coord);
                }
                _ => {
                    // end current number parsing, if any
                    store_number(&mut current_number);

                    if c != '.' {
                        symbols.insert(Coord { x, y }, c);
                    }
                }
            }
        }

        // end current number parsing, if any
        store_number(&mut current_number);
    }

    RawBoard {
        digits,
        numbers,
        symbols,
        size: (size.0.unwrap_or(0), size.1),
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
    let mut adjacent_numbers_to_symbols = HashMap::new();

    for (symbol_cord, symbol) in symbols.into_iter() {
        let mut adjacent_numbers = HashSet::new();

        let x_range =
            max(symbol_cord.x as isize - 1, 0) as usize..=min(symbol_cord.x + 1, size.0 - 1);

        let y_range =
            max(symbol_cord.y as isize - 1, 0) as usize..=min(symbol_cord.y + 1, size.1 - 1);

        for (x, y) in x_range.flat_map(|x| y_range.clone().map(move |y| (x, y))) {
            let lookup_coord = Coord { x, y };

            if lookup_coord == symbol_cord {
                continue;
            }

            if let Some(number_cord) = digits.get(&lookup_coord) {
                adjacent_numbers.insert(*number_cord);
            }
        }

        if !adjacent_numbers.is_empty() {
            adjacent_numbers_to_symbols.insert(symbol_cord, (symbol, adjacent_numbers));
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

    let mut part_numbers = HashSet::new();

    for (_, numbers) in adjacent_numbers_to_symbols.values() {
        for start_cord in numbers {
            part_numbers.insert(start_cord);
        }
    }

    part_numbers
        .iter()
        .map(|c| numbers.get(c).unwrap().value)
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let Board {
        numbers,
        adjacent_numbers_to_symbols,
    } = process_board(input);

    let mut total_gear_ratios = 0;

    for (c, adjacent_numbers) in adjacent_numbers_to_symbols.values() {
        if *c != '*' || adjacent_numbers.len() != 2 {
            continue;
        }

        let gear_ratio = adjacent_numbers
            .iter()
            .map(|c| numbers.get(c).unwrap().value)
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
