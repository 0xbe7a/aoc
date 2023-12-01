fn read_elves(input: &str) -> impl Iterator<Item = i64> + '_ {
    input.lines().map(|l| {
        let mut snafu = 0;
        let mut base = 1;

        for digit in l.chars().rev() {
            let d = match digit {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => panic!("unknown digit"),
            };
            snafu += d * base;
            base *= 5;
        }

        snafu
    })
}

pub fn part_one(input: &str) -> String {
    let mut s: i64 = read_elves(input).sum();
    let mut res = Vec::new();

    while s > 0 {
        match s % 5 {
            0 => res.push('0'),
            1 => res.push('1'),
            2 => res.push('2'),
            3 => {
                res.push('=');
                s += 2;
            }
            4 => {
                res.push('-');
                s += 1
            }
            _ => unreachable!(),
        };

        s /= 5;
    }

    res.into_iter().rev().collect()
}

pub fn part_two(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 25);
        assert_eq!(part_one(&input), "2=-1=0");
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 25);
        assert_eq!(part_two(&input), 0);
    }
}
