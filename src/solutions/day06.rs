fn get_number_of_strategies(time: u64, distance: u64) -> u64 {
    let square = (time * time - 4 * (distance + 1)) as f64;
    let root = square.sqrt();
    let min = (0.5 * (time as f64 - root)).ceil() as u64;
    let max = (0.5 * (time as f64 + root)).floor() as u64;

    max - min + 1
}

fn parse_input(input: &str) -> impl Iterator<Item = (u64, u64)> + '_ {
    let mut lines = input.lines();

    let mut parse_line = |name: &str| {
        lines
            .next()
            .unwrap()
            .strip_prefix(name)
            .unwrap()
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
    };

    let times = parse_line("Time: ");
    let distances = parse_line("Distance: ");

    times.zip(distances)
}

pub fn part_one(input: &str) -> u32 {
    parse_input(input)
        .map(|(t, d)| get_number_of_strategies(t, d))
        .product::<u64>() as u32
}

fn parse_input_2(input: &str) -> (u64, u64) {
    let mut lines = input.lines();

    let mut parse_line = |name: &str| -> u64 {
        let line = lines.next().unwrap().strip_prefix(name).unwrap();

        let mut n: u64 = 0;

        for c in line.chars() {
            if c.is_ascii_whitespace() {
                continue;
            }

            n *= 10;
            n += c.to_digit(10).unwrap() as u64;
        }

        n
    };

    let times = parse_line("Time: ");
    let distances = parse_line("Distance: ");

    (times, distances)
}

pub fn part_two(input: &str) -> u32 {
    let (times, distance) = parse_input_2(input);
    get_number_of_strategies(times, distance) as u32
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("examples", 6);
        assert_eq!(part_one(&input), 288);

        let input = read_file("inputs", 6);
        assert_eq!(part_one(&input), 4811940);
    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 6);
        assert_eq!(part_two(&input), 71503);

        let input = read_file("inputs", 6);
        assert_eq!(part_two(&input), 30077773);
    }
}
