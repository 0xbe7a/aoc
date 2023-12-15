fn calculate_hash(input: &str) -> u32 {
    let mut v = 0;

    for c in input.chars() {
        v += c as u32;
        v *= 17;
        v %= 256;
    }

    v
}

enum Operation {
    Insert(u8),
    Remove,
}

struct Instruction<'a> {
    operation: Operation,
    label: &'a str,
}

fn parse_instruction(input: &str) -> Instruction<'_> {
    let (label, focal_length) = input.split_once(['=', '-']).expect("invalid instruction");

    let operation = match focal_length {
        "" => Operation::Remove,
        len => {
            let len = len.parse().expect("invalid focal length");
            Operation::Insert(len)
        }
    };

    Instruction { operation, label }
}

pub fn part_one(input: &str) -> u32 {
    input.split(',').map(calculate_hash).sum()
}

#[derive(Default, Debug, Clone)]
struct HashMap<'a> {
    bucket: Vec<(&'a str, u8)>,
}

impl<'a> HashMap<'a> {
    fn new() -> Self {
        Self { bucket: Vec::new() }
    }

    fn insert(&mut self, key: &'a str, value: u8) {
        for (k, v) in self.bucket.iter_mut() {
            if k == &key {
                *v = value;
                return;
            }
        }

        self.bucket.push((key, value));
    }

    fn remove(&mut self, key: &str) {
        self.bucket.retain(|(k, _)| k != &key);
    }
}

pub fn part_two(input: &str) -> usize {
    let mut boxes = vec![HashMap::new(); 256];

    for instruction in input.split(',').map(parse_instruction) {
        let hash = calculate_hash(instruction.label);

        let lens_box = &mut boxes[hash as usize];

        match instruction.operation {
            Operation::Insert(len) => {
                lens_box.insert(instruction.label, len);
            }
            Operation::Remove => {
                lens_box.remove(instruction.label);
            }
        }
    }

    boxes
        .into_iter()
        .enumerate()
        .map(|(box_idx, lens_box)| {
            lens_box
                .bucket
                .into_iter()
                .enumerate()
                .map(|(lens_idx, (_, focal))| (box_idx + 1) * (lens_idx + 1) * focal as usize)
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("examples", 15);
        assert_eq!(part_one(&input), 1320);
    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 15);
        assert_eq!(part_two(&input), 145);
    }
}
