use std::collections::HashMap;

type Num = i64;

#[derive(Clone, Copy, Debug)]
enum OP {
    Addition,
    Subtraction,
    Multiplication,
    Divison,
}

#[derive(Clone, Copy, Debug)]
enum Input<'a> {
    Constant(Num),
    Operation(&'a str, &'a str, OP),
}

fn read_op(input: &str) -> impl Iterator<Item = (&str, Input)> + Clone + '_ {
    input.lines().map(|c| {
        let (name, command) = c.split_once(": ").expect("Cant parse line");
        let mut split = command.split_ascii_whitespace();
        let left = split.next().unwrap();

        match split.next() {
            None => (name, Input::Constant(left.parse().expect("Cant parse num"))),
            Some(op) => {
                let right = split.next().expect("Second operator expected");
                let parsed_op = match op {
                    "+" => Input::Operation(left, right, OP::Addition),
                    "-" => Input::Operation(left, right, OP::Subtraction),
                    "*" => Input::Operation(left, right, OP::Multiplication),
                    "/" => Input::Operation(left, right, OP::Divison),
                    _ => panic!("Unknown op"),
                };
                (name, parsed_op)
            }
        }
    })
}

struct ArithmeticNode {
    is_variable: bool,
    node: Box<Node>,
}

enum Node {
    Constant(Num),
    Op(ArithmeticNode, ArithmeticNode, OP),
}

fn transform(ops: &HashMap<&str, Input<'_>>, node: &str) -> ArithmeticNode {
    match ops.get(node).unwrap() {
        Input::Constant(i) if node == "humn" => ArithmeticNode {
            is_variable: true,
            node: Box::new(Node::Constant(*i)),
        },
        Input::Constant(i) => ArithmeticNode {
            is_variable: false,
            node: Box::new(Node::Constant(*i)),
        },
        Input::Operation(left, right, op) => {
            let (left, right) = (transform(ops, left), transform(ops, right));

            ArithmeticNode {
                is_variable: left.is_variable || right.is_variable,
                node: Box::new(Node::Op(left, right, *op)),
            }
        }
    }
}

impl ArithmeticNode {
    fn eval(&self) -> Num {
        match &*self.node {
            Node::Constant(i) => *i,
            Node::Op(a, b, OP::Addition) => a.eval() + b.eval(),
            Node::Op(a, b, OP::Subtraction) => a.eval() - b.eval(),
            Node::Op(a, b, OP::Multiplication) => a.eval() * b.eval(),
            Node::Op(a, b, OP::Divison) => a.eval() / b.eval(),
        }
    }
}

pub fn part_one(input: &str) -> Num {
    let ops: HashMap<_, _> = read_op(input).collect();
    let root = transform(&ops, "root");
    root.eval()
}

pub fn part_two(input: &str) -> Num {
    let ops: HashMap<_, _> = read_op(input).collect();
    let root = transform(&ops, "root").node;

    let (left, right) = match *root {
        Node::Op(left, right, _) => (left, right),
        _ => panic!("Unknown op"),
    };

    let (mut target, mut value) = match (left.is_variable, right.is_variable) {
        (true, false) => (*left.node, right.eval()),
        (false, true) => (*right.node, left.eval()),
        _ => panic!("One value must be fixed"),
    };

    loop {
        match target {
            Node::Constant(_) => return value,
            Node::Op(left, right, op) => match (op, left.is_variable) {
                (OP::Addition, true) => (target, value) = (*left.node, value - right.eval()),
                (OP::Addition, false) => (target, value) = (*right.node, value - left.eval()),
                (OP::Subtraction, true) => (target, value) = (*left.node, value + right.eval()),
                (OP::Subtraction, false) => (target, value) = (*right.node, left.eval() - value),
                (OP::Multiplication, true) => (target, value) = (*left.node, value / right.eval()),
                (OP::Multiplication, false) => (target, value) = (*right.node, value / left.eval()),
                (OP::Divison, true) => (target, value) = (*left.node, value * right.eval()),
                (OP::Divison, false) => (target, value) = (*right.node, left.eval() / value),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 21);
        assert_eq!(part_one(&input), 152);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 21);
        assert_eq!(part_two(&input), 301);
    }
}
