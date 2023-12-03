use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Symbol {
    Dot,
    Digit(u8),
    Other(char),
}

impl Symbol {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Symbol::Dot,
            '0'..='9' => Symbol::Digit(c.to_digit(10).unwrap() as u8),
            _ => Symbol::Other(c),
        }
    }
}

#[derive(Debug)]
struct ContingousNumber {
    x: usize,
    y: usize,
    length: u8,
    value: u32,
}

struct Board {
    symbols: HashMap<(usize, usize), char>,
    numbers: HashMap<(usize, usize), ContingousNumber>,
    size: (usize, usize),
}

fn extract_board_information(input: &str) -> Board {
    let mut symbols = HashMap::new();
    let mut numbers = HashMap::new();

    let mut store_number = |current_number: &mut Option<ContingousNumber>| {
        if let Some(num) = current_number.take() {
            numbers.insert((num.x, num.y), num);
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
            let s = Symbol::from_char(c);

            match s {
                Symbol::Digit(d) => {
                    let num = current_number.get_or_insert(ContingousNumber {
                        x,
                        y,
                        length: 0,
                        value: 0,
                    });

                    num.length += 1;
                    num.value = num.value * 10 + d as u32;
                }
                _ => {
                    // end current number parsing, if any
                    store_number(&mut current_number);

                    if let Symbol::Other(c) = s {
                        symbols.insert((x, y), c);
                    }
                }
            }
        }

        // end current number parsing, if any
        store_number(&mut current_number);
    }

    Board {
        symbols,
        numbers,
        size: (size.0.unwrap_or(0), size.1),
    }
}

/// Iterate over all number digits and call the action function for each digit.
/// if action returns true, the iteration will be stopped.
fn iterate_number_digits<F>(
    numbers: &HashMap<(usize, usize), ContingousNumber>,
    width: usize,
    height: usize,
    mut action: F,
) where
    F: FnMut(&ContingousNumber, usize, usize) -> bool,
{
    for number in numbers.values() {
        for x in number.x..number.x + number.length as usize {
            for sx in x.saturating_sub(1)..=std::cmp::min(x + 1, width - 1) {
                for sy in number.y.saturating_sub(1)..=std::cmp::min(number.y + 1, height - 1) {
                    if sx == x && sy == number.y {
                        continue;
                    }
                    if action(number, sx, sy) {
                        break;
                    }
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> u32 {
    let Board {
        symbols,
        numbers,
        size: (width, height),
    } = extract_board_information(input);
    let mut part_numbers = HashMap::new();

    iterate_number_digits(&numbers, width, height, |number, sx, sy| {
        if symbols.get(&(sx, sy)).is_some() {
            part_numbers.insert((number.x, number.y), number.value);
            true // break the loop
        } else {
            false
        }
    });

    part_numbers.values().sum()
}

pub fn part_two(input: &str) -> u32 {
    let Board {
        symbols,
        numbers,
        size: (width, height),
    } = extract_board_information(input);
    let mut adjacent_numbers_of_gear = HashMap::new();

    iterate_number_digits(&numbers, width, height, |number, sx, sy| {
        if let Some('*') = symbols.get(&(sx, sy)) {
            adjacent_numbers_of_gear
                .entry((sx, sy))
                .or_insert_with(HashSet::new)
                .insert((number.x, number.y));
        }
        false // keep going
    });

    adjacent_numbers_of_gear
        .iter()
        .filter(|(_, cords)| cords.len() == 2)
        .map(|(_, cords)| {
            cords
                .iter()
                .map(|(x, y)| numbers.get(&(*x, *y)).unwrap().value)
                .product::<u32>()
        })
        .sum()
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
