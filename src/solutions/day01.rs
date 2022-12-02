use std::mem::swap;

fn read_elves(input: &str) -> Vec<Vec<u32>> {
    let mut lines = input.lines();
    let mut elves = Vec::new();
    loop {
        let elf: Vec<_> = (&mut lines)
            .map_while(|line| line.parse::<u32>().ok())
            .collect();
        if elf.is_empty() {
            break;
        }

        elves.push(elf);
    }
    elves
}

pub fn part_one(input: &str) -> u32 {
    return read_elves(input)
        .iter()
        .map(|g| g.iter().sum::<u32>())
        .max()
        .unwrap();
}

pub fn part_two(input: &str) -> u32 {
    return read_elves(input)
        .iter()
        .map(|g| g.iter().sum::<u32>())
        .fold([0, 0, 0], |mut n_max, mut x| {
            for max_k in n_max.iter_mut() {
                if x >= *max_k {
                    swap(&mut x, max_k);
                }
            }
            n_max
        })
        .iter()
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 1);
        assert_eq!(part_one(&input), 24000);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 1);
        assert_eq!(part_two(&input), 45000);
    }
}
