use bitvec::array::BitArray;
use std::{cmp::min, ops::BitAnd};

fn get_winning_count(card: &str) -> usize {
    let (_, numbers) = card.split_once(':').expect("invalid input");
    let (winning_numbers, card_numbers) = numbers.split_once('|').expect("invalid input");

    let parse_numbers = |numbers: &str| {
        let mut bits: BitArray<[usize; 2]> = BitArray::ZERO;

        for n in numbers.split_ascii_whitespace() {
            let n = n.parse::<usize>().unwrap();
            bits.set(n, true);
        }

        bits
    };

    let winning_numbers = parse_numbers(winning_numbers);
    let card_numbers = parse_numbers(card_numbers);

    winning_numbers.bitand(card_numbers).count_ones()
}

pub fn part_one(input: &str) -> u32 {
    input.lines().map(get_winning_count).fold(0, |acc, count| {
        acc + match count {
            0 => 0,
            m => 1 << (m - 1),
        }
    })
}

pub fn part_two(input: &str) -> u32 {
    let win_counts = input.lines().map(get_winning_count).collect::<Vec<_>>();
    let mut instances = vec![1; win_counts.len()];

    win_counts
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, count)| {
            for k in (i + 1)..min(i + 1 + count, instances.len()) {
                instances[k] += instances[i];
            }

            acc + instances[i]
        })
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("examples", 4);
        assert_eq!(part_one(&input), 13);

        let input = read_file("inputs", 4);
        assert_eq!(part_one(&input), 26443);
    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 4);
        assert_eq!(part_two(&input), 30);

        let input = read_file("inputs", 4);
        assert_eq!(part_two(&input), 6284877);
    }
}
