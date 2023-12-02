use std::cmp::max;

#[derive(Default)]
struct Observation {
    red: usize,
    green: usize,
    blue: usize,
}

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

fn parse_game(game: &str) -> (usize, impl Iterator<Item = Observation> + '_) {
    //Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

    let (game_id, contents) = game.split_once(": ").expect("Cant parse game line");
    let id = game_id
        .strip_prefix("Game ")
        .expect("Game id malformed")
        .parse::<usize>()
        .expect("Invalid game id");

    let iter = contents.split("; ").map(move |single_game| {
        let mut observation = Observation::default();

        for item in single_game.split(", ") {
            let (count_str, color) = item.split_once(' ').expect("Invalid item");
            let count = count_str.parse::<usize>().expect("Invalid item count");
            match color {
                "red" => observation.red = count,
                "green" => observation.green = count,
                "blue" => observation.blue = count,
                _ => panic!("Invalid color"),
            }
        }

        observation
    });

    (id, iter)
}

fn get_max_min_counts(iter: impl Iterator<Item = Observation>) -> Observation {
    iter.reduce(|mut acc, item| {
        acc.red = max(acc.red, item.red);
        acc.green = max(acc.green, item.green);
        acc.blue = max(acc.blue, item.blue);
        acc
    })
    .unwrap_or_default()
}

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(parse_game)
        .filter_map(|(id, items)| {
            let max_counts = get_max_min_counts(items);

            if max_counts.red <= MAX_RED
                && max_counts.green <= MAX_GREEN
                && max_counts.blue <= MAX_BLUE
            {
                Some(id)
            } else {
                None
            }
        })
        .sum::<usize>() as u32
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(parse_game)
        .map(|(_, items)| get_max_min_counts(items))
        .map(|counts| counts.red * counts.green * counts.blue)
        .sum::<usize>() as u32
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("examples", 2);
        assert_eq!(part_one(&input), 8);

        let input = read_file("inputs", 2);
        assert_eq!(part_one(&input), 2795)
    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 2);
        assert_eq!(part_two(&input), 2286);

        let input = read_file("inputs", 2);
        assert_eq!(part_two(&input), 75561);
    }
}
