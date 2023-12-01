use std::{
    cmp::max,
    collections::HashSet,
};


use nom::{bytes::complete::tag, character::complete::i32, IResult};

type Coord = (i32, i32);

fn parse_coords(input: &str) -> IResult<&str, Coord> {
    let (input, _) = tag("x=")(input)?;
    let (input, x) = i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y) = i32(input)?;
    Ok((input, (x, y)))
}

fn parse_line(input: &str) -> IResult<&str, (Coord, Coord)> {
    let (input, _) = tag("Sensor at ")(input)?;
    let (input, sensor) = parse_coords(input)?;
    let (input, _) = tag(": closest beacon is at ")(input)?;
    let (input, beacon) = parse_coords(input)?;
    Ok((input, (sensor, beacon)))
}

fn parse_input(input: &str) -> Vec<(Coord, Coord)> {
    input.lines().map(|line| parse_line(line).unwrap().1).collect()
}

fn get_blocked_line(input: &[(Coord, Coord)], y: i32) -> Vec<(i32, i32)> {
    let mut ranges = Vec::new();

    for (sensor, beacon) in input.iter() {
        let radius = sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1);

        let dy = sensor.1.abs_diff(y);
        let d = radius.saturating_sub(dy);
        if radius >= dy {
            let x_min = sensor.0 - d as i32;
            let x_max = sensor.0 + d as i32;

            if x_min <= x_max {
                ranges.push((x_min, x_max));
            }
        }
    }

    ranges.sort_by_cached_key(|r| r.0);
    let mut merged_ranges = Vec::new();

    for range in ranges {
        let last = match merged_ranges.last_mut() {
            Some(last) => last,
            None => {
                merged_ranges.push(range);
                continue;
            }
        };

        if last.1 < range.0 {
            merged_ranges.push(range)
        } else {
            *last = (last.0, max(last.1, range.1));
        }
    }

    merged_ranges
}

pub fn part_one(input: &str) -> i32 {
    let input = parse_input(input);
    let merged_ranges = get_blocked_line(&input, 2000000);

    let mut total = 0;

    for (start, end) in merged_ranges {
        total += (end - start + 1) as i32;
    }

    let mut beacons = HashSet::new();

    for (_, beacon) in &input {
        if beacon.1 == 2000000 && beacons.insert(beacon) {
            total -= 1;
        }
    }

    total
}

pub fn part_two(input: &str) -> i64 {
    let input = parse_input(input);
    let range = 4000000;

    let mut a_coffs = HashSet::new();
    let mut b_coffs = HashSet::new();
    let mut scanners = HashSet::new();

    for (scanner, beacon) in input.iter() {
        let radius = (beacon.0 - scanner.0).abs() + (beacon.1 - scanner.1).abs();
        a_coffs.insert(scanner.1 - scanner.0 + radius + 1);
        a_coffs.insert(scanner.1 - scanner.0 - radius - 1);
        b_coffs.insert(scanner.0 + scanner.1 + radius + 1);
        b_coffs.insert(scanner.0 + scanner.1 - radius - 1);
        scanners.insert((scanner, radius));
    }

    for a in a_coffs.iter() {
        'outer: for b in b_coffs.iter() {
            let (x, y) = ((b - a) / 2, (b + a) / 2);
            if 0 > x || x > range || 0 > y || y > range {
                continue 'outer
            }

            for (mid, radius) in scanners.iter() {
                let d = (mid.0 - x).abs() + (mid.1 - y).abs();
                if d <= *radius {
                    continue 'outer
                }
            }

            return x as i64 * 4000000 + y as i64;
        }
    };

    panic!("No intersection found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("inputs", 15);
        assert_eq!(part_one(&input), 5299855);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("inputs", 15);
        assert_eq!(part_two(&input), 13615843289729);
    }
}
