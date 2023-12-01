use std::{cmp::Ordering, collections::HashSet, hash::Hash};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Copy, Hash, Default)]
pub struct Item(char);

impl Item {
    pub fn priority(&self) -> u32 {
        if self.0.is_lowercase() {
            self.0 as u32 - 'a' as u32 + 1
        } else {
            self.0 as u32 - 'A' as u32 + 27
        }
    }
}

pub fn read_rucksacks(input: &str) -> Vec<Vec<Item>> {
    input
        .lines()
        .map(|l| l.chars().map(Item).collect())
        .collect()
}
pub fn get_non_exclusive_item(sacks: &mut [Vec<Item>]) -> Option<Item> {
    for rucksack in sacks.iter_mut() {
        rucksack.sort_by(|a, b| b.cmp(a));
    }

    loop {
        let mut all_equal = true;
        let mut current_lowest = sacks.first()?.last()?;
        let mut lowest_index = 0;

        for (idx, rucksack) in sacks.iter().enumerate() {
            match rucksack.last()?.cmp(current_lowest) {
                Ordering::Less => {
                    current_lowest = rucksack.last()?;
                    lowest_index = idx;
                    all_equal = false;
                }
                Ordering::Greater => all_equal = false,
                _ => continue,
            }
        }

        if all_equal {
            return Some(*current_lowest);
        } else {
            sacks.get_mut(lowest_index)?.pop();
        }
    }
}

pub fn part_one(input: &str) -> u32 {
    let mut total_priority = 0;
    for mut rucksack in read_rucksacks(input) {
        let compartment_b = rucksack.split_off(rucksack.len() / 2);
        let compartment_a = rucksack;

        let exclusive = get_non_exclusive_item(&mut [compartment_a, compartment_b]).unwrap();
        total_priority += exclusive.priority();
    }
    total_priority
}

pub fn part_two(input: &str) -> u32 {
    let mut total_priority = 0;
    for rucksacks in read_rucksacks(input).chunks_mut(3) {
        let exclusive = get_non_exclusive_item(rucksacks).unwrap();
        total_priority += exclusive.priority();
    }
    total_priority
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 3);
        assert_eq!(part_one(&input), 157);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 3);
        assert_eq!(part_two(&input), 70);
    }
}
