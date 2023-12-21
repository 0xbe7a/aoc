use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction(Vec<PulseType>),
    Broadcast,
}

impl ModuleType {
    fn process(&mut self, pulse: PulseType, input_idx: usize) -> Option<PulseType> {
        match self {
            ModuleType::FlipFlop(s) => match pulse {
                PulseType::Low => {
                    *s = !*s;

                    if *s {
                        Some(PulseType::High)
                    } else {
                        Some(PulseType::Low)
                    }
                }
                PulseType::High => None,
            },
            ModuleType::Conjunction(last_signals) => {
                last_signals[input_idx] = pulse;

                if last_signals.iter().all(|p| matches!(p, PulseType::High)) {
                    Some(PulseType::Low)
                } else {
                    Some(PulseType::High)
                }
            }
            ModuleType::Broadcast => Some(pulse),
        }
    }
}

#[derive(Debug)]
struct ModuleIO {
    output: Vec<String>,
    input: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
enum PulseType {
    Low,
    High,
}

#[derive(Debug)]
struct Circuit {
    modules: HashMap<String, ModuleType>,
    io: HashMap<String, ModuleIO>,
}

impl Circuit {
    fn from_input(input: &str) -> Self {
        let mut modules_map = HashMap::new();
        let mut io_map = HashMap::new();

        for line in input.lines() {
            let (name, outputs) = line.split_once(" -> ").unwrap();

            let (r#type, name) = {
                let first_char = name.chars().next().unwrap();

                match first_char {
                    '%' => (ModuleType::FlipFlop(false), name[1..].to_string()),
                    '&' => (ModuleType::Conjunction(Vec::new()), name[1..].to_string()),
                    _ => (ModuleType::Broadcast, name.to_string()),
                }
            };

            let output = outputs.split(", ").map(|s| s.to_string()).collect();

            io_map.insert(
                name.clone(),
                ModuleIO {
                    output,
                    input: Vec::new(),
                },
            );

            modules_map.insert(name, r#type);
        }

        let keys = io_map.keys().cloned().collect::<Vec<_>>();

        for key in keys {
            let outputs = io_map[&key].output.clone();

            for output in outputs {
                match io_map.get_mut(&output) {
                    Some(module) => {
                        module.input.push(key.clone());

                        let mut r#type = modules_map.get_mut(&output).unwrap();
                        if let ModuleType::Conjunction(s) = &mut r#type {
                            s.push(PulseType::Low); // init with low
                        }
                    }
                    None => continue,
                }
            }
        }

        io_map
            .get_mut("broadcaster")
            .unwrap()
            .input
            .push("button".to_string());

        Self {
            modules: modules_map,
            io: io_map,
        }
    }

    fn push_button(&mut self) -> (usize, usize, [usize; 4]) {
        let mut actions = VecDeque::new();
        actions.push_back(("broadcaster", "button", PulseType::Low));

        let mut low_pulses = 0;
        let mut high_pulses = 0;

        let mut target_triggers = [0, 0, 0, 0];

        while let Some((name, sender, pulse)) = actions.pop_front() {
            match pulse {
                PulseType::Low => low_pulses += 1,
                PulseType::High => high_pulses += 1,
            };

            let module = match self.modules.get_mut(name) {
                Some(m) => m,
                None => continue,
            };

            // if name == "rx" && matches!(pulse, PulseType::Low) {
            //     rx_triggers += 1;
            // }

            if name == "hf" {
                if let ModuleType::Conjunction(s) = module {
                    for (i, p) in s.iter().enumerate() {
                        if matches!(p, PulseType::High) {
                            target_triggers[i] += 1;
                        }
                    }
                }
            }

            let io = &self.io[name];

            let input_idx = io.input.iter().position(|s| s == sender).unwrap_or(0);
            let output = module.process(pulse, input_idx);

            if let Some(output) = output {
                for recv in io.output.iter() {
                    actions.push_back((recv, name, output));
                }
            }
        }

        (low_pulses, high_pulses, target_triggers)
    }
}

pub fn part_one(input: &str) -> usize {
    let mut circuit = Circuit::from_input(input);
    println!("{:?}", circuit);
    let (mut total_low, mut total_high) = (0, 0);

    for _ in 0..1000 {
        let (low, high, _) = circuit.push_button();
        total_high += high;
        total_low += low;
    }

    println!("total_low: {}, total_high: {}", total_low, total_high);

    total_high * total_low
}

pub fn part_two(input: &str) -> usize {
    let mut circuit = Circuit::from_input(input);
    println!("{:?}", circuit);
    let mut presses = 0;

    let mut cycles = [None; 4];

    loop {
        presses += 1;

        if presses % 100_000 == 0 {
            println!("i: {}", presses);
        }

        let (_, _, rx_triggers) = circuit.push_button();

        for (i, t) in rx_triggers.iter().enumerate() {
            if *t > 0 && cycles[i].is_none() {
                cycles[i] = Some(presses);
            }
        }

        println!("rx_triggers: {:?}", rx_triggers);
        println!("cycles: {:?}", cycles);

        if cycles.iter().all(|c| c.is_some()) {
            let mut cycle = 1;

            for c in cycles.iter() {
                cycle = num::integer::lcm(cycle, c.unwrap());
            }

            return cycle;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("examples", 20);
        assert_eq!(part_one(&input), 32000000);
    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 20);
        assert_eq!(part_two(&input), 0);
    }
}
