use std::{cell::RefCell, cmp::min, collections::HashMap, rc::Rc};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline, not_line_ending, u64},
    multi::separated_list0,
    IResult,
};

type Directory<'a> = Rc<RefCell<HashMap<&'a str, IONode<'a>>>>;

#[derive(Debug, Clone)]
enum IONode<'a> {
    Directory(Directory<'a>),
    File(u64),
}

impl<'a> IONode<'a> {
    fn size(&self) -> u64 {
        match &self {
            Self::File(size) => *size,
            Self::Directory(dirs) => dirs.borrow().iter().map(|(_, x)| x.size()).sum(),
        }
    }

    fn part1(&self) -> u64 {
        match &self {
            Self::Directory(dirs) => {
                let size = self.size();

                let output = if size <= 100000 { size } else { 0 };

                return output + dirs.borrow().iter().map(|(_, x)| x.part1()).sum::<u64>();
            }
            _ => return 0,
        }
    }

    fn part2(&self, required: u64) -> Option<u64> {
        match &self {
            Self::Directory(dirs) => {
                let size = self.size();

                let output = if size >= required { Some(size) } else { None };

                let subfolder_min = dirs
                    .borrow()
                    .iter()
                    .filter_map(|(_, x)| x.part2(required))
                    .min();

                match (output, subfolder_min) {
                    (Some(a), Some(b)) => Some(min(a, b)),
                    (Some(a), None) => Some(a),
                    (None, Some(b)) => Some(b),
                    _ => None,
                }
            }
            _ => return None,
        }
    }
}

#[derive(Debug, Clone)]
enum LSOutput<'a> {
    Dir(&'a str),
    File(&'a str, u64),
}

#[derive(Debug, Clone)]
enum Command<'a> {
    CD(&'a str),
    LS(Vec<LSOutput<'a>>),
}

fn parse_dir<'a>(input: &'a str) -> IResult<&str, LSOutput<'a>> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = not_line_ending(input)?;

    Ok((input, LSOutput::Dir(name)))
}

fn parse_file(input: &str) -> IResult<&str, LSOutput> {
    let (input, size) = u64(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, name) = not_line_ending(input)?;

    Ok((input, LSOutput::File(name, size)))
}

fn parse_ls_command(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ls\n")(input)?;
    let (input, outputs) = separated_list0(newline, alt((parse_dir, parse_file)))(input)?;

    Ok((input, Command::LS(outputs)))
}

fn parse_cd_command(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, path) = not_line_ending(input)?;

    Ok((input, Command::CD(path)))
}

fn read_log(input: &str) -> Vec<Command> {
    let (rem, commands) =
        separated_list0(newline, alt((parse_ls_command, parse_cd_command)))(input).unwrap();
    assert!(rem.is_empty());
    commands
}

fn apply_log<'a>(input: &'a str) -> IONode<'a> {
    let log = read_log(input);

    let root_dir = Directory::default();
    let mut current_node = root_dir.clone();
    let mut path = Vec::new();

    for command in log {
        match command {
            Command::CD(new_path) => match new_path {
                "/" => {
                    path.clear();
                    current_node = root_dir.clone();
                }
                ".." => current_node = path.pop().unwrap_or(root_dir.clone()),
                directory => {
                    let next_node = {
                        match current_node
                            .borrow_mut()
                            .entry(directory)
                            .or_insert(IONode::Directory(Directory::default()))
                        {
                            IONode::Directory(dir) => dir.clone(),
                            IONode::File(_) => panic!("Cant cd into file"),
                        }
                    };

                    path.push(current_node);
                    current_node = next_node;
                }
            },
            Command::LS(outputs) => {
                for output in outputs {
                    match output {
                        LSOutput::File(name, size) => {
                            current_node.borrow_mut().insert(name, IONode::File(size));
                        }
                        LSOutput::Dir(name) => {
                            current_node
                                .borrow_mut()
                                .insert(name, IONode::Directory(Directory::default()));
                        }
                    }
                }
            }
        }
    }

    IONode::Directory(root_dir)
}

pub fn part_one(input: &str) -> u64 {
    let root_dir = apply_log(input);
    root_dir.part1()
}

pub fn part_two(input: &str) -> u64 {
    let root_dir = apply_log(input);
    let required_space = 30_000_000u64.saturating_sub(70_000_000 - root_dir.size());
    root_dir.part2(required_space).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 7);
        assert_eq!(part_one(&input), 95437);

        let input = read_file("inputs", 7);
        assert_eq!(part_one(&input), 1908462);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 7);
        assert_eq!(part_two(&input), 24933642);

        let input = read_file("inputs", 7);
        assert_eq!(part_two(&input), 3979145);
    }
}
