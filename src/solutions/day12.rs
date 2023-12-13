use rustc_hash::FxHashMap as HashMap;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

#[derive(Debug, Hash)]
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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct CheckState<'a> {
    last_item: Option<SpringState>,
    current_group: Option<usize>,
    next_group: usize,
    groups: &'a [usize],
}

impl<'a> CheckState<'a> {
    fn new(groups: &'a [usize]) -> Self {
        Self {
            last_item: None,
            current_group: groups.first().copied(),
            next_group: 1,
            groups,
        }
    }

    fn check_next_item_compatible(&mut self, item: SpringState) -> bool {
        let last_item = self.last_item;
        self.last_item = Some(item);

        match (last_item, &item, &mut self.current_group) {
            (_, SpringState::Broken, Some(rem)) if *rem > 0 => {
                *rem -= 1;
                true
            }
            (None, SpringState::Operational, _) => true,
            (Some(SpringState::Broken), SpringState::Operational, Some(0)) => {
                self.current_group = self.groups.get(self.next_group).copied();
                self.next_group += 1;
                true
            }
            (Some(SpringState::Operational), SpringState::Operational, _) => true,
            _ => false,
        }
    }

    fn is_finished(&mut self) -> bool {
        if self.current_group == Some(0) {
            self.current_group = self.groups.get(self.next_group).copied();
            self.next_group += 1;
        }

        self.current_group.is_none()
    }
}

fn check_combinations(row: &[SpringState], groups: &[usize]) -> usize {
    let mut current_states = HashMap::default();
    current_states.insert(CheckState::new(groups), 1);


    for item in row {
        let mut new_state = HashMap::default();

        for (mut state, count) in current_states {
            match item {
                SpringState::Operational | SpringState::Broken => {
                    if state.check_next_item_compatible(*item) {
                        *new_state.entry(state).or_default() += count;
                    }
                }
                SpringState::Unknown => {
                    for item in [SpringState::Operational, SpringState::Broken] {
                        let mut forked_state = state.clone();
                        if forked_state.check_next_item_compatible(item) {
                            *new_state.entry(forked_state).or_default() += count;
                        }
                    }
                }
            }
        }

        current_states = new_state;
    }

    let mut valid = 0;

    for (mut state, count) in current_states {
        if state.is_finished() {
            valid += count;
        }
    }

    valid
}

pub fn part_one(input: &str) -> usize {
    parse_input(input)
        .map(|row| check_combinations(&row.row, &row.broken_groups))
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let inputs = parse_input(input).collect::<Vec<_>>();

    inputs.into_iter().map(|row| {
        let mut new_row = Vec::new();
        let mut new_groups = Vec::new();

        for k in 0..5 {
            if k != 0 {
                new_row.push(SpringState::Unknown);
            }

            new_row.extend(row.row.iter().cloned());
            new_groups.extend(row.broken_groups.iter().copied());
        };

        check_combinations(&new_row, &new_groups)
    }).sum()
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
