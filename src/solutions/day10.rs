enum Instruction {
    NOOP,
    ADDV(i32),
}

impl Instruction {
    fn get_cycles(&self) -> usize {
        match self {
            Self::NOOP => 1,
            Self::ADDV(_) => 2,
        }
    }
}

type Register = i32;

struct CPU<I> {
    register: Register,
    rem_cycles: usize,
    prev_instruction: Option<Instruction>,
    instructions: I,
}

impl<I> CPU<I> {
    fn new(instructions: I) -> Self {
        Self {
            register: 1,
            rem_cycles: 0,
            prev_instruction: None,
            instructions,
        }
    }
}

impl<I> Iterator for CPU<I>
where
    I: Iterator<Item = Instruction>,
{
    type Item = Register;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rem_cycles == 0 {
            match self.prev_instruction {
                Some(Instruction::ADDV(v)) => self.register += v,
                _ => (),
            };

            let next_instruction = match self.instructions.next() {
                Some(instruction) => instruction,
                None => return None,
            };

            self.rem_cycles = next_instruction.get_cycles();
            self.prev_instruction = Some(next_instruction);
        }

        self.rem_cycles -= 1;

        Some(self.register)
    }
}

fn read_instructions(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.lines().map(|line| {
        let mut instr = line.split(' ');
        match instr.next().expect("Missing opcode") {
            "addx" => Instruction::ADDV(
                instr
                    .next()
                    .expect("Missing argument")
                    .parse()
                    .expect("Cant parse argument"),
            ),
            "noop" => Instruction::NOOP,
            _ => panic!("Cant parse instruction"),
        }
    })
}

pub fn part_one(input: &str) -> i32 {
    let instructions = read_instructions(&input);

    let products = CPU::new(instructions)
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(i, r)| (i + 1) as i32 * r)
        .sum();

    products
}

pub fn part_two(input: &str) -> String {
    let instructions = read_instructions(&input);
    let mut screen = String::new();

    for (idx, register) in CPU::new(instructions).enumerate() {
        if (idx % 40) == 0 && idx != 0 {
            screen.push('\n');
        }

        let pixel = if ((idx % 40) as i32 - register).abs() <= 1 {
            '#'
        } else {
            '.'
        };

        screen.push(pixel);
    }

    screen
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 10);
        assert_eq!(part_one(&input), 13140);
    }
}
