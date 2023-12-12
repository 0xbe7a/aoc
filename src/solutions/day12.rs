use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Debug, Clone)]
enum SpringState {
    Operational,
    Broken,
    Unknown,
}

impl SpringState {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Self::Broken,
            '.' => Self::Operational,
            '?' => Self::Unknown,
            _ => panic!("invalid spring char: {}", c),
        }
    }
}

#[derive(Debug)]
struct SpringRow {
    row: Vec<SpringState>,
    broken_groups: Vec<usize>,
}

fn parse_input(input: &str) -> impl Iterator<Item = SpringRow> + '_ {
    input.lines().map(|line| {
        let (states, groups) = line.split_once(' ').expect("cant parse line");
        let row = states.chars().map(SpringState::from_char).collect();
        let broken_groups = groups.split(',').map(|g| g.parse().unwrap()).collect();

        SpringRow { row, broken_groups }
    })
}

fn check_valid_assignment(row: &[SpringState], groups: &[usize], assignments: u64) -> bool {
    let mut group_iter = groups.iter();
    let mut current_group = group_iter.next().copied();

    let mut last_state = None;
    let mut num_unknown: u64 = 0;

    let mut get_next_unknown = || {
        let state = {
            if (assignments >> num_unknown) & 1 == 1 {
                SpringState::Broken
            } else {
                SpringState::Operational
            }
        };

        num_unknown += 1;
        state
    };

    for state in row {
        let assigned_state = match state {
            SpringState::Broken => SpringState::Broken,
            SpringState::Operational => SpringState::Operational,
            SpringState::Unknown => get_next_unknown()
        };

        match (last_state, &assigned_state, &mut current_group) {
            (_, SpringState::Broken, Some(rem)) if *rem > 0 => {
                *rem -= 1;
            }
            (None, SpringState::Operational, _) => {}
            (Some(SpringState::Broken), SpringState::Operational, Some(0)) => {
                current_group = group_iter.next().copied();
            }
            (Some(SpringState::Operational), SpringState::Operational, _) => {}
            _ => {
                return false;
            }
        };

        last_state = Some(assigned_state);
    }

    if current_group == Some(0) {
        current_group = group_iter.next().copied();
    }

    current_group.is_none()
}

fn brute_force_row(row: &SpringRow) -> usize {
    let num_unknown = row
        .row
        .iter()
        .filter(|x| matches!(x, SpringState::Unknown))
        .count();

    (0u64..(1 << num_unknown))
        .into_par_iter()
        .filter(|assignments| {
            check_valid_assignment(
                row.row.as_slice(),
                row.broken_groups.as_slice(),
                *assignments,
            )
        })
        .count()
}

pub fn part_one(input: &str) -> usize {
    parse_input(input).map(|row| brute_force_row(&row)).sum()
}

pub fn part_two(input: &str) -> usize {
    // let mut total = 0;

    // for row in parse_input(input) {
    //     let mut new_row = Vec::new();
    //     let mut new_groups = Vec::new();

    //     for k in 0..5 {
    //         if k != 0 {
    //             new_row.push(SpringState::Unknown);
    //         }

    //         new_row.extend(row.row.iter().cloned());
    //         new_groups.extend(row.broken_groups.iter().copied());
    //     };

    //     let new_row = SpringRow {
    //         row: new_row,
    //         broken_groups: new_groups,
    //     };

    //     total += brute_force_row(&new_row);
    // }

    // total

    0
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "???.### 1,1,3";
        assert_eq!(part_one(input), 1);

        let input = ".??..??...?##. 1,1,3";
        assert_eq!(part_one(input), 4);

        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(part_one(input), 1);

        let input = "????.#...#... 4,1,1";
        assert_eq!(part_one(input), 1);

        let input = "????.######..#####. 1,6,5";
        assert_eq!(part_one(input), 4);

        let input = "?###???????? 3,2,1";
        assert_eq!(part_one(input), 10);

        let input = read_file("examples", 12);
        assert_eq!(part_one(&input), 21);
    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 12);
        assert_eq!(part_two(&input), 525152);
    }
}
