fn get_digits_part1(s: &str) -> (u8, u8) {
    let mut first = None;
    let mut last = None;

    for c in s.chars() {
        if c.is_ascii_digit() {
            if first.is_none() {
                first = Some(c.to_digit(10).unwrap() as u8);
            }

            last = Some(c.to_digit(10).unwrap() as u8);
        }
    }

    (first.unwrap(), last.unwrap())
}

fn end_matches(window: u64, s: &str) -> bool {
    let s_bytes = s.as_bytes();
    let s_len = s_bytes.len();

    // Convert s to a u64, aligned to the right
    let mut s_val: u64 = 0;
    for &b in s_bytes {
        s_val = (s_val << 8) | (b as u64);
    }

    let mask = (1u64 << (8 * s_len)) - 1;
    (window & mask) == s_val
}

fn get_digits_part2(s: &str) -> (u8, u8) {
    let mut first = None;
    let mut last = None;

    let get_digit = |c: u8, window: u64| -> Option<u8> {
        if c == b'1' || end_matches(window, "one") {
            Some(1)
        } else if c == b'2' || end_matches(window, "two") {
            Some(2)
        } else if c == b'3' || end_matches(window, "three") {
            Some(3)
        } else if c == b'4' || end_matches(window, "four") {
            Some(4)
        } else if c == b'5' || end_matches(window, "five") {
            Some(5)
        } else if c == b'6' || end_matches(window, "six") {
            Some(6)
        } else if c == b'7' || end_matches(window, "seven") {
            Some(7)
        } else if c == b'8' || end_matches(window, "eight") {
            Some(8)
        } else if c == b'9' || end_matches(window, "nine") {
            Some(9)
        } else {
            None
        }
    };

    let mut window: u64 = 0;

    for c in s.bytes() {
        window = (window << 8) | (c as u64);

        if let Some(d) = get_digit(c, window) {
            if first.is_none() {
                first = Some(d)
            }

            last = Some(d);
        }
    }

    (first.unwrap(), last.unwrap())
}

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (f, l) = get_digits_part1(line);
            f as u32 * 10 + l as u32
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (f, l) = get_digits_part2(line);
            f as u32 * 10 + l as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{read_file, read_file_with_name};

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file_with_name("examples", "01_1");
        assert_eq!(part_one(&input), 142);

        let input = read_file("inputs", 1);
        assert_eq!(part_one(&input), 54968);
    }

    #[test]
    fn test_part_two() {
        let input = read_file_with_name("examples", "01_2");
        assert_eq!(part_two(&input), 281);

        let input = read_file("inputs", 1);
        assert_eq!(part_two(&input), 54094);
    }
}
