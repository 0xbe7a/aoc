use std::{convert::identity, mem::swap};

fn parse_nums(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|c| c.parse().expect("Cant parse num"))
        .collect()
}

fn solve(nums: &[i64], rounds: usize) -> i64 {
    let mut ans = (0..nums.len()).collect::<Vec<_>>();
    for _ in 0..rounds {
        for (i, &x) in nums.iter().enumerate() {
            let pos = ans.iter().position(|&y| y == i).unwrap();
            ans.remove(pos);
            let new_i = (pos as i64 + x).rem_euclid(ans.len() as i64) as usize;
            ans.insert(new_i, i);
        }
    }

    let orig_zero_i = nums.iter().position(|&i| i == 0).unwrap();
    let zero_i = ans.iter().position(|&i| i == orig_zero_i).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| nums[ans[(zero_i + i) % ans.len()]])
        .sum()
}

pub fn part_one(input: &str) -> i64 {
    let nums: Vec<_> = parse_nums(input);
    solve(&nums, 1)
}

pub fn part_two(input: &str) -> i64 {
    let nums: Vec<_> = parse_nums(input).into_iter().map(|x| x * 811589153).collect();
    solve(&nums, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 20);
        assert_eq!(part_one(&input), 3);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 20);
        assert_eq!(part_two(&input), 1623178306);
    }
}
