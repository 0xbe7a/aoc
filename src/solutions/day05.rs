use std::cmp::min;

#[derive(Debug)]
struct Map {
    dst: usize,
    src: usize,
    len: usize,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let mut parts = input.split(' ');

        let mut parse_next_number = || parts.next().unwrap().parse().unwrap();

        let dst = parse_next_number();
        let src = parse_next_number();
        let len = parse_next_number();

        Map { dst, src, len }
    }
}

#[derive(Debug)]
struct Input {
    maps: Vec<Vec<Map>>,
    input_numbers: Vec<usize>,
}

fn parse_input(input: &str) -> Input {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let mut maps: Vec<Vec<Map>> = Vec::new();
    let mut input_numbers: Vec<usize> = Vec::new();

    for section in sections {
        if let Some(raw_input_numbers) = section.strip_prefix("seeds: ") {
            for n in raw_input_numbers.split(' ').map(|n| n.parse().unwrap()) {
                input_numbers.push(n);
            }
            continue;
        }

        let map_section: Vec<Map> = section.lines().skip(1).map(Map::from_str).collect();
        maps.push(map_section);
    }

    // prepare maps for binary search
    for section in maps.iter_mut() {
        section.sort_by(|a, b| a.src.cmp(&b.src));
    }

    Input {
        maps,
        input_numbers,
    }
}

fn find_containing_map(section: &[Map], n: usize) -> Option<&Map> {
    match section.binary_search_by_key(&n, |map| map.src) {
        Ok(index) => Some(&section[index]),
        Err(index) if (index > 0) => {
            let map = &section[index - 1];

            if map.src + map.len > n {
                Some(map)
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn part_one(input: &str) -> u32 {
    let Input {
        maps,
        mut input_numbers,
    } = parse_input(input);

    for section in maps {
        for number in input_numbers.iter_mut() {
            let map = find_containing_map(&section, *number);

            if let Some(map) = map {
                *number = map.dst + (*number - map.src);
            }
        }
    }

    input_numbers.into_iter().min().unwrap() as u32
}

pub fn part_two(input: &str) -> u32 {
    let Input {
        maps,
        input_numbers,
    } = parse_input(input);

    let mut ranges: Vec<(usize, usize)> = input_numbers.chunks(2).map(|c| (c[0], c[1])).collect();

    for section in maps {
        let mut next_ranges = Vec::new();

        for (mut start, mut len) in ranges {
            while len > 0 {
                let map = find_containing_map(&section, start);

                if let Some(map) = map {
                    let captured = min(len, map.src + map.len - start);
                    let new_start = map.dst + (start - map.src);
                    next_ranges.push((new_start, captured));
                    len -= captured;
                    start += captured;
                } else {
                    next_ranges.push((start, len));
                    break;
                }
            }
        }

        ranges = next_ranges;
    }

    ranges.into_iter().map(|(s, _)| s).min().unwrap() as u32
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("examples", 5);
        assert_eq!(part_one(&input), 35);

        let input = read_file("inputs", 5);
        assert_eq!(part_one(&input), 403695602);
    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 5);
        assert_eq!(part_two(&input), 46);

        let input = read_file("inputs", 5);
        assert_eq!(part_two(&input), 219529182);
    }
}
