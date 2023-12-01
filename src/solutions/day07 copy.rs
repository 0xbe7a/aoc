use std::{cell::RefCell, cmp::min, collections::HashMap, rc::Rc, slice::SliceIndex};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline, not_line_ending, u64},
    multi::separated_list0,
    IResult,
};

type Node<'a> = Rc<RefCell<IONode<'a>>>;

#[derive(Debug, Clone)]
enum Filetype<'a> {
    Directory(HashMap<&'a str, Node<'a>>),
    File(u64),
}

#[derive(Debug, Clone)]
struct IONode<'a> {
    filetype: Filetype<'a>,
    parrent: Option<Node<'a>>,
}

impl<'a> IONode<'a> {
    fn get_content(&self) -> Option<&HashMap<&'a str, Node<'a>>> {
        match &self.filetype {
            Filetype::File(_) => None,
            Filetype::Directory(dir) => Some(dir),
        }
    }

    fn get_content_mut(&mut self) -> Option<&mut HashMap<&'a str, Node<'a>>> {
        match &mut self.filetype {
            Filetype::File(_) => None,
            Filetype::Directory(dir) => Some(dir),
        }
    }

    fn size(&self) -> u64 {
        match &self.filetype {
            Filetype::File(size) => *size,
            Filetype::Directory(dirs) => dirs
                .iter()
                .map(|(_, x)| {
                    let val: &IONode = &(**x).borrow();
                    val.size()
                })
                .sum(),
        }
    }

    fn part1(&self) -> u64 {
        match &self.filetype {
            Filetype::Directory(dirs) => {
                let size = self.size();

                let output = if size <= 100000 { size } else { 0 };

                return output
                    + dirs
                        .iter()
                        .map(|(_, x)| {
                            let val: &IONode = &(**x).borrow();
                            let size = val.part1();
                            size
                        })
                        .sum::<u64>();
            }
            _ => return 0,
        }
    }

    fn part2(&self, required: u64) -> Option<u64> {
        match &self.filetype {
            Filetype::Directory(dirs) => {
                let size = self.size();

                let output = if size >= required { Some(size) } else { None };

                let subfolder_min = dirs
                    .iter()
                    .filter_map(|(_, x)| {
                        let val: &IONode = &(**x).borrow();
                        val.part2(required)
                    })
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

fn apply_log<'a>(input: &'a str) -> Node<'a> {
    let log = read_log(input);

    let mut current_node = Rc::new(RefCell::new(IONode {
        filetype: Filetype::Directory(HashMap::new()),
        parrent: None,
    }));

    let root_dir = current_node.clone();

    for command in log {
        match command {
            Command::CD(new_path) => match new_path {
                "/" => current_node = root_dir.clone(),
                ".." => {
                    current_node = {
                        let current = current_node.as_ref().borrow();
                        current.parrent.as_ref().expect("No parrent").clone()
                    };
                }
                path => {
                    current_node = {
                        let node = current_node.as_ref().borrow();
                        node.get_content()
                            .unwrap()
                            .get(path)
                            .expect("No such file or directory")
                            .clone()
                    };
                }
            },
            Command::LS(outputs) => {
                for output in outputs {
                    let node: &mut IONode = &mut (&current_node).borrow_mut();

                    match output {
                        LSOutput::File(name, size) => {
                            node.get_content_mut().unwrap().insert(
                                name,
                                Rc::new(RefCell::new(IONode {
                                    filetype: Filetype::File(size),
                                    parrent: Some(current_node.clone()),
                                })),
                            );
                        }
                        LSOutput::Dir(name) => {
                            node.get_content_mut()
                                .unwrap()
                                .entry(name)
                                .or_insert(Rc::new(RefCell::new(IONode {
                                    filetype: Filetype::Directory(HashMap::new()),
                                    parrent: Some(current_node.clone()),
                                })));
                        }
                    }
                }
            }
        }
    }

    root_dir
}

pub fn part_one(input: &str) -> u64 {
    let root_dir = apply_log(input);
    let node: &mut IONode = &mut (*root_dir).borrow_mut();

    return node.part1();
}

pub fn part_two(input: &str) -> u64 {
    let root_dir = apply_log(input);
    let node: &mut IONode = &mut (*root_dir).borrow_mut();

    let required_space = 30_000_000u64.saturating_sub(70_000_000 - node.size());
    return node.part2(required_space).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 7);
        assert_eq!(part_one(&input), 95437);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 7);
        assert_eq!(part_two(&input), 24933642);
    }
}
