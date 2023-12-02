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

fn get_digits_part2(s: &str) -> (u8, u8) {
    let mut first = None;
    let mut last = None;

    let end_matches = |window: &[u8; 5], s: &str| &window[window.len() - s.len()..] == s.as_bytes();

    let get_digit = |window: &[u8; 5]| -> Option<u8> {
        if end_matches(window, "1") || end_matches(window, "one") {
            Some(1)
        } else if end_matches(window, "2") || end_matches(window, "two") {
            Some(2)
        } else if end_matches(window, "3") || end_matches(window, "three") {
            Some(3)
        } else if end_matches(window, "4") || end_matches(window, "four") {
            Some(4)
        } else if end_matches(window, "5") || end_matches(window, "five") {
            Some(5)
        } else if end_matches(window, "6") || end_matches(window, "six") {
            Some(6)
        } else if end_matches(window, "7") || end_matches(window, "seven") {
            Some(7)
        } else if end_matches(window, "8") || end_matches(window, "eight") {
            Some(8)
        } else if end_matches(window, "9") || end_matches(window, "nine") {
            Some(9)
        } else {
            None
        }
    };

    let mut window = [0; 5];

    for c in s.chars() {
        window[0] = c as u8;
        window.rotate_left(1);

        if let Some(d) = get_digit(&window) {
            if first.is_none() {
                first = Some(d)
            }

            last = Some(d);
        }
    }

    (first.unwrap(), last.unwrap())
}

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
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file_with_name;
        let input = read_file_with_name("examples", "01_1");
        assert_eq!(part_one(&input), 142);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file_with_name;
        let input = read_file_with_name("examples", "01_2");
        assert_eq!(part_two(&input), 281);
    }
}
