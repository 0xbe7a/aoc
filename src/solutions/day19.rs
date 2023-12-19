use rustc_hash::FxHashMap as HashMap;

use once_cell::sync::Lazy;
use regex::Regex;
use smallvec::{smallvec, SmallVec};

static WORKFLOW_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w+)\{(.+)\}").unwrap());
static INPUT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap());

#[derive(Debug)]
struct Step {
    condition: Option<Condition>,
    action: Action,
}

impl Step {
    fn from_str(s: &str) -> Self {
        match s.split_once(':') {
            Some((condition, action)) => {
                let mut chars = condition.chars();
                let variable = Variable::from_char(chars.next().unwrap());
                let relation = Relation::from_char(chars.next().unwrap());
                let value = chars.as_str().parse::<u16>().unwrap();

                Step {
                    condition: Some(Condition {
                        variable,
                        relation,
                        value,
                    }),
                    action: Action::from_str(action),
                }
            }
            None => Step {
                condition: None,
                action: Action::from_str(s),
            },
        }
    }
}

#[derive(Debug)]
struct Condition {
    variable: Variable,
    relation: Relation,
    value: u16,
}

impl Condition {
    fn eval(&self, input: &Input) -> bool {
        let val = match self.variable {
            Variable::Cool => input.cool,
            Variable::Musical => input.musical,
            Variable::Aerodynamic => input.aerodynamic,
            Variable::Shiny => input.shiny,
        };

        match self.relation {
            Relation::Less => val < self.value,
            Relation::Greater => val > self.value,
        }
    }

    fn eval_range(&self, input: &InputRange) -> SmallVec<[(InputRange, bool); 2]> {
        let (lower, upper) = match self.variable {
            Variable::Cool => input.cool,
            Variable::Musical => input.musical,
            Variable::Aerodynamic => input.aerodynamic,
            Variable::Shiny => input.shiny,
        };

        let (lower_eval, upper_eval, lower_end, upper_start) = match self.relation {
            Relation::Less => (true, false, self.value - 1, self.value),
            Relation::Greater => (false, true, self.value, self.value + 1),
        };

        if self.value >= upper {
            return smallvec![(input.clone(), lower_eval)];
        }

        if self.value <= lower {
            return smallvec![(input.clone(), upper_eval)];
        }

        let mut lower_range = input.clone();
        let mut upper_range = input.clone();

        match self.variable {
            Variable::Cool => {
                lower_range.cool = (lower, lower_end);
                upper_range.cool = (upper_start, upper);
            }
            Variable::Musical => {
                lower_range.musical = (lower, lower_end);
                upper_range.musical = (upper_start, upper);
            }
            Variable::Aerodynamic => {
                lower_range.aerodynamic = (lower, lower_end);
                upper_range.aerodynamic = (upper_start, upper);
            }
            Variable::Shiny => {
                lower_range.shiny = (lower, lower_end);
                upper_range.shiny = (upper_start, upper);
            }
        }

        smallvec![(lower_range, lower_eval), (upper_range, upper_eval),]
    }
}

#[derive(Debug)]
enum Relation {
    Less,
    Greater,
}

impl Relation {
    fn from_char(c: char) -> Self {
        match c {
            '<' => Self::Less,
            '>' => Self::Greater,
            _ => panic!("invalid relation"),
        }
    }
}

#[derive(Debug)]
enum Variable {
    Cool,
    Musical,
    Aerodynamic,
    Shiny,
}

impl Variable {
    fn from_char(c: char) -> Self {
        match c {
            'x' => Self::Cool,
            'm' => Self::Musical,
            'a' => Self::Aerodynamic,
            's' => Self::Shiny,
            _ => panic!("invalid variable"),
        }
    }
}

#[derive(Debug)]
enum Action {
    Accept,
    Reject,
    Workflow(String),
}

impl Action {
    fn from_str(s: &str) -> Self {
        match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::Workflow(s.to_string()),
        }
    }
}

fn parse_workflows(input: &str) -> HashMap<String, Vec<Step>> {
    WORKFLOW_REGEX
        .captures_iter(input)
        .map(|m| {
            let workflow_name = m.get(1).unwrap().as_str();
            let workflow_steps = m.get(2).unwrap().as_str();

            let steps = workflow_steps
                .split(',')
                .map(Step::from_str)
                .collect::<Vec<_>>();

            (workflow_name.to_string(), steps)
        })
        .collect()
}

struct Input {
    cool: u16,
    musical: u16,
    aerodynamic: u16,
    shiny: u16,
}

fn parse_input(input: &str) -> impl Iterator<Item = Input> + '_ {
    INPUT_REGEX.captures_iter(input).map(|m| {
        let cool = m.get(1).unwrap().as_str().parse::<u16>().unwrap();
        let musical = m.get(2).unwrap().as_str().parse::<u16>().unwrap();
        let aerodynamic = m.get(3).unwrap().as_str().parse::<u16>().unwrap();
        let shiny = m.get(4).unwrap().as_str().parse::<u16>().unwrap();

        Input {
            cool,
            musical,
            aerodynamic,
            shiny,
        }
    })
}

fn process(workflows: &HashMap<String, Vec<Step>>, input: &Input) -> bool {
    let mut current_workflow = "in";

    loop {
        let steps = workflows.get(current_workflow).unwrap();

        for step in steps {
            if let Some(condition) = &step.condition {
                if !condition.eval(input) {
                    continue;
                }
            }

            match &step.action {
                Action::Accept => return true,
                Action::Reject => return false,
                Action::Workflow(workflow) => {
                    current_workflow = workflow;
                    break;
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct InputRange {
    cool: (u16, u16),
    musical: (u16, u16),
    aerodynamic: (u16, u16),
    shiny: (u16, u16),
}

#[derive(Clone, Debug)]
struct InputState<'a> {
    range: InputRange,
    step: u16,
    workflow: &'a str,
}

fn find_accepting_ranges(workflows: &HashMap<String, Vec<Step>>) -> Vec<InputRange> {
    let state = InputState {
        range: InputRange {
            cool: (1, 4000),
            musical: (1, 4000),
            aerodynamic: (1, 4000),
            shiny: (1, 4000),
        },
        step: 0,
        workflow: "in",
    };

    let mut accepted_ranges = vec![];
    let mut unterminated_inputs = vec![state];

    while let Some(input) = unterminated_inputs.pop() {
        let step = &workflows.get(input.workflow).unwrap()[input.step as usize];

        let mut apply_action = |range: InputRange, unterminated: &mut Vec<_>| match &step.action {
            Action::Accept => accepted_ranges.push(range),
            Action::Reject => {}
            Action::Workflow(workflow) => {
                let new_input = InputState {
                    range,
                    step: 0,
                    workflow,
                };

                unterminated.push(new_input);
            }
        };

        if let Some(condition) = &step.condition {
            for (range, eval) in condition.eval_range(&input.range) {
                if eval {
                    apply_action(range, &mut unterminated_inputs);
                } else {
                    unterminated_inputs.push(InputState {
                        range,
                        step: input.step + 1,
                        workflow: input.workflow,
                    });
                }
            }
        } else {
            apply_action(input.range, &mut unterminated_inputs);
        }
    }

    accepted_ranges
}

pub fn part_one(input: &str) -> usize {
    let (workflows, inputs) = input.split_once("\n\n").unwrap();

    let inputs = parse_input(inputs);
    let workflows = parse_workflows(workflows);

    inputs
        .filter(|input| process(&workflows, input))
        .flat_map(|input| [input.cool, input.musical, input.aerodynamic, input.shiny])
        .map(|x| x as usize)
        .sum::<usize>()
}

pub fn part_two(input: &str) -> u128 {
    let (workflows, _) = input.split_once("\n\n").unwrap();
    let workflows = parse_workflows(workflows);

    let mut combinations = 0;

    for range in find_accepting_ranges(&workflows) {
        combinations += (range.cool.1 - range.cool.0 + 1) as u128
            * (range.musical.1 - range.musical.0 + 1) as u128
            * (range.aerodynamic.1 - range.aerodynamic.0 + 1) as u128
            * (range.shiny.1 - range.shiny.0 + 1) as u128;
    }

    combinations
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("examples", 19);
        assert_eq!(part_one(&input), 19114);
    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 19);
        assert_eq!(part_two(&input), 167409079868000);
    }
}
