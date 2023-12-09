fn parse_input(input: &str) -> impl Iterator<Item = Vec<i64>> + '_ {
    input.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect()
    })
}

fn differentiate(input: &[i64]) -> Vec<i64> {
    input.windows(2).map(|w| w[1] - w[0]).collect()
}

pub fn part_one(input: &str) -> i64 {
    parse_input(input)
        .map(|mut measurement| {
            let mut last_values = Vec::new();

            loop {
                if measurement.iter().all(|v| *v == 0) {
                    break;
                }

                last_values.push(*measurement.last().unwrap());
                measurement = differentiate(&measurement);
            }

            last_values
                .into_iter()
                .rfold(0, |last_change, v| v + last_change)
        })
        .sum()
}

pub fn part_two(input: &str) -> i64 {
    parse_input(input)
        .map(|mut measurement| {
            let mut first_values = Vec::new();

            loop {
                if measurement.iter().all(|v| *v == 0) {
                    break;
                }

                first_values.push(*measurement.first().unwrap());
                measurement = differentiate(&measurement);
            }

            first_values
                .into_iter()
                .rfold(0, |last_change, v| v - last_change)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("examples", 9);
        assert_eq!(part_one(&input), 114);

        let input = read_file("inputs", 9);
        assert_eq!(part_one(&input), 1637452029);
    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 9);
        assert_eq!(part_two(&input), 2);

        let input = read_file("inputs", 9);
        assert_eq!(part_two(&input), 908);
    }
}
