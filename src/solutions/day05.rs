use nom::{
    branch::alt,
    bytes::complete::{tag},
    character::{
        complete::{char, digit1, newline, satisfy},
    },
    combinator::{map, map_res, recognize},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

fn read_bay_item(input: &str) -> IResult<&str, Option<char>> {
    let item = map(
        delimited(char('['), satisfy(|c| c.is_alphabetic()), char(']')),
        Some,
    );
    let space = map(tag("   "), |_| None);
    alt((item, space))(input)
}

fn read_bay_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list1(tag(" "), read_bay_item)(input)
}

fn read_bay(mut input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let mut read_line = map(read_bay_line, Some);
    let row_num = delimited(char(' '), satisfy(|c| c.is_numeric()), char(' '));
    let mut end_line = map(separated_list1(tag(" "), row_num), |_| None);

    let mut rows = Vec::new();

    loop {
        let (rest, row) = alt((&mut read_line, &mut end_line))(input)?;
        let (rest, _) = newline(rest)?;

        input = rest;

        match row {
            Some(cols) => {
                if rows.is_empty() {
                    for _ in 0..cols.len() {
                        rows.push(Vec::new());
                    }
                }

                for (x, col) in cols.into_iter().zip(rows.iter_mut()) {
                    if let Some(i) = x {
                        col.push(i);
                    }
                }
            }
            None => {
                for v in rows.iter_mut() {
                    v.reverse();
                }
                break;
            }
        };
    }

    Ok((input, rows))
}

struct Instruction {
    from: usize,
    to: usize,
    count: usize,
}

fn read_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("move ")(input)?;
    let (input, count) = map_res(recognize(digit1), str::parse)(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = map_res(recognize(digit1), str::parse)(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = map_res(recognize(digit1), str::parse)(input)?;
    Ok((input, Instruction { from, to, count }))
}

struct Input {
    bay: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

fn read_input(input: &str) -> IResult<&str, Input> {
    let (input, bay) = read_bay(input)?;
    let (input, _) = newline(input)?;
    let (input, instructions) = separated_list1(newline, read_instruction)(input)?;

    Ok((input, Input { bay, instructions }))
}

fn get_output(bay: Vec<Vec<char>>) -> String {
    let mut output = String::new();
    for mut col in bay {
        let top = col.pop().expect("No item on top");
        output.push(top);
    }

    output
}

pub fn part_one(input: &str) -> String {
    let Input {
        mut bay,
        instructions,
    } = read_input(input).unwrap().1;

    for command in instructions {
        for _ in 0..command.count {
            let item = bay[command.from - 1].pop().expect("No item");
            bay[command.to - 1].push(item);
        }
    }

    get_output(bay)
}

pub fn part_two(input: &str) -> String {
    let Input {
        mut bay,
        instructions,
    } = read_input(input).unwrap().1;

    for command in instructions {
        let col = &mut bay[command.from - 1];
        let items = col.split_off(col.len() - command.count);
        bay[command.to - 1].extend_from_slice(&items);
    }

    get_output(bay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_parser() {
        assert_eq!(read_bay_item("[A]"), Ok(("", Some('A'))));
        assert_eq!(
            read_bay_line("[N] [C]    "),
            Ok(("", vec![Some('N'), Some('C'), None]))
        );
    }

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 5);
        assert_eq!(part_one(&input), "CMZ");
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 5);
        assert_eq!(part_two(&input), "MCD");
    }
}
