use std::vec::Vec;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, not_line_ending, u64},
    multi::separated_list0,
    IResult,
};
use std::cell::RefCell;

type Item = u64;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    Addition(Item),
    Multiplication(Item),
    Square,
}

impl Operation {
    fn op(&self, x: Item) -> Item {
        match self {
            Self::Addition(y) => x + y,
            Self::Multiplication(y) => x * y,
            Self::Square => x * x,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    mod_num: Item,
    jmp_true: u64,
    jmp_false: u64,
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let parse_addition = |i| {
        let (i, _) = tag(" + ")(i)?;
        let (i, num) = u64(i)?;
        Ok((i, Operation::Addition(num)))
    };

    let parse_multipliction = |i| {
        let (i, _) = tag(" * ")(i)?;
        let (i, num) = u64(i)?;
        Ok((i, Operation::Multiplication(num)))
    };

    let parse_square = |i| {
        let (i, _) = tag(" * old")(i)?;
        Ok((i, Operation::Square))
    };

    let (i, _) = tag("Operation: new = old")(input)?;
    alt((parse_addition, parse_multipliction, parse_square))(i)
}

fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
    let (i, _) = tag("Monkey ")(i)?;
    let (i, _) = not_line_ending(i)?;
    let (i, _) = newline(i)?;
    let (i, _) = tag("  Starting items: ")(i)?;
    let (i, items) = separated_list0(tag(", "), u64)(i)?;
    let (i, _) = newline(i)?;
    let (i, _) = tag("  ")(i)?;
    let (i, operation) = parse_operation(i)?;
    let (i, _) = newline(i)?;
    let (i, _) = tag("  Test: divisible by ")(i)?;
    let (i, mod_num) = u64(i)?;
    let (i, _) = newline(i)?;
    let (i, _) = tag("    If true: throw to monkey ")(i)?;
    let (i, jmp_true) = u64(i)?;
    let (i, _) = newline(i)?;
    let (i, _) = tag("    If false: throw to monkey ")(i)?;
    let (i, jmp_false) = u64(i)?;

    Ok((
        i,
        Monkey {
            items,
            operation,
            mod_num,
            jmp_true,
            jmp_false,
        },
    ))
}

fn read_input(i: &str) -> Vec<Monkey> {
    let (i, monkeys) = separated_list0(tag("\n\n"), parse_monkey)(i).expect("Cant parse monkeys");
    assert!(i.is_empty());
    monkeys
}

fn run_monkeys(monkeys: Vec<RefCell<Monkey>>, rounds: usize, worry_op: impl Fn(Item) -> Item) -> usize {
    let mut monkey_counts = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for (count, monkey) in monkey_counts.iter_mut().zip(monkeys.iter()) {
            let mut monkey = monkey.borrow_mut();
            for item in monkey.items.iter() {
                *count += 1;
                let new_level = worry_op(monkey.operation.op(*item));
                let jmp_target = if new_level.rem_euclid(monkey.mod_num) == 0 {
                    monkey.jmp_true as usize
                } else {
                    monkey.jmp_false as usize
                };

                // RefCell Safety: A monkey never throws items at itself
                monkeys[jmp_target].borrow_mut().items.push(new_level);
            }

            monkey.items.clear()
        }
    }

    monkey_counts.sort_by(|a, b| b.cmp(a));
    monkey_counts[0] * monkey_counts[1]
}

pub fn part_one(input: &str) -> usize {
    let monkeys = read_input(input).into_iter().map(RefCell::from).collect();
    run_monkeys(monkeys, 20, |x| x.div_floor(3))
}

pub fn part_two(input: &str) -> usize {
    let monkeys: Vec<_> = read_input(input).into_iter().map(RefCell::from).collect();
    let modproduct: Item = monkeys.iter().map(|m| m.borrow().mod_num).product();
    run_monkeys(monkeys, 10_000, |x| x % modproduct)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            parse_operation("Operation: new = old * 13"),
            Ok(("", Operation::Multiplication(13)))
        )
    }

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 11);
        assert_eq!(part_one(&input), 10605);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 11);
        assert_eq!(part_two(&input), 2713310158);
    }
}
