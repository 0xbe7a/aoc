struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn is_inclusive(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        (self.start <= other.start && other.start <= self.end)
            || (other.start <= self.start && self.start <= other.end)
    }
}

fn read_elves(input: &str) -> Vec<(Range, Range)> {
    input
        .lines()
        .map(|line| {
            let parse_range = |range_str: &str| {
                let (start, end) = range_str.split_once('-').expect("No range found");
                Range {
                    start: start.parse().expect("Cant parse number"),
                    end: end.parse().expect("Cant parse number"),
                }
            };
            let (fst, snd) = line.split_once(',').expect("No delimiter found");
            (parse_range(fst), parse_range(snd))
        })
        .collect()
}

pub fn part_one(input: &str) -> usize {
    return read_elves(input)
        .iter()
        .filter(|(fst, snd)| fst.is_inclusive(snd) || snd.is_inclusive(fst))
        .count();
}

pub fn part_two(input: &str) -> usize {
    return read_elves(input)
        .iter()
        .filter(|(fst, snd)| fst.overlaps(snd))
        .count();
}

//alternative solution
fn solve(input: &str) -> (usize, usize) {
    input
        .lines()
        .map(|line| {
            let (fst, snd) = line.split_once(',').expect("No delimiter found");
            let (a, b) = fst.split_once('-').expect("No range found");
            let (c, d) = snd.split_once('-').expect("No range found");
            (a, b, c, d)
        })
        .fold((0, 0), |(mut p1, mut p2), (a, b, c, d)| {
            //Check inclusive
            if (a <= c && d <= b) || (c <= a && d >= b) {
                p1 += 1
            }

            //Check overlap
            if (a <= c && c <= b) || (c <= a && a <= d) {
                p2 += 1
            }
            (p1, p2)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 4);
        assert_eq!(part_one(&input), 2);
    }

    #[test]
    fn test_alternative() {
        use crate::read_file;
        let input = read_file("examples", 4);
        assert_eq!(solve(&input), (2, 4));
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 4);
        assert_eq!(part_two(&input), 4);
    }
}
