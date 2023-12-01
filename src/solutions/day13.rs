use std::cmp::Ordering;

use nom::{
    branch::alt,
    character::complete::{char, u16},
    combinator::map,
    multi::separated_list0,
    IResult,
};

#[derive(Debug, Clone)]
enum Element {
    List(Vec<Element>),
    Integer(u16),
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Element::Integer(a), Element::Integer(b)) => a.cmp(b),
            (Element::List(a), Element::List(b)) => a.cmp(b),
            (Element::List(a), b @ Element::Integer(_)) => {
                a.as_slice().cmp([b.clone()].as_slice())
            }
            (a @ Element::Integer(_), Element::List(b)) => {
                [a.clone()].as_slice().cmp(b.as_slice())
            }
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Element {}

fn parse_list(input: &str) -> IResult<&str, Element> {
    let (input, _) = char('[')(input)?;
    let (input, elements) = separated_list0(char(','), parse_element)(input)?;
    let (input, _) = char(']')(input)?;

    Ok((input, Element::List(elements)))
}

fn parse_element(input: &str) -> IResult<&str, Element> {
    let parse_integer = map(u16, Element::Integer);
    alt((parse_list, parse_integer))(input)
}

fn read_pairs(input: &str) -> impl Iterator<Item = (Element, Element)> + '_ {
    input.split("\n\n").map(|pair_str| {
        let (mut left_str, mut right_str) = pair_str.split_once('\n').expect("Cant split pair");
        (
            parse_element(&mut left_str).unwrap().1,
            parse_element(&mut right_str).unwrap().1,
        )
    })
}

pub fn part_one(input: &str) -> usize {
    let mut total = 0;
    for (idx, (left, right)) in read_pairs(input).enumerate() {
        if left < right {
            total += idx + 1;
        }
    }

    total
}

pub fn part_two(input: &str) -> usize {
    let mut elements: Vec<_> = read_pairs(input).flat_map(|(a, b)| [a, b]).collect();

    let (d1, d2) = (
        Element::List(vec![Element::List(vec![Element::Integer(2)])]),
        Element::List(vec![Element::List(vec![Element::Integer(6)])]),
    );

    elements.push(d1.clone());
    elements.push(d2.clone());

    elements.sort();

    let idx_1 = elements.binary_search(&d1).unwrap();
    let idx_2 = elements.binary_search(&d2).unwrap();

    (idx_1 + 1) * (idx_2 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            parse_element(&mut "[[1],[2,3,4]]"),
            Ok((
                "",
                Element::List(vec![
                    Element::List(vec![Element::Integer(1)]),
                    Element::List(vec![
                        Element::Integer(2),
                        Element::Integer(3),
                        Element::Integer(4)
                    ])
                ])
            ))
        );
    }

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 13);
        assert_eq!(part_one(&input), 13);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 13);
        assert_eq!(part_two(&input), 140);
    }
}
