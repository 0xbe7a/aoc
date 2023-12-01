use std::{
    cmp::max,
    collections::{HashMap, HashSet, VecDeque},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, u16},
    multi::separated_list0,
    IResult,
};

type FlowRate = u16;

#[derive(Debug, Clone)]
struct Valve<'a> {
    flow_rate: FlowRate,
    name: &'a str,
    neighbors: Vec<&'a str>,
}

fn parse_line(i: &str) -> IResult<&str, Valve> {
    let parse_valve_name = alpha1;

    let (i, _) = tag("Valve ")(i)?;
    let (i, name) = parse_valve_name(i)?;
    let (i, _) = tag(" has flow rate=")(i)?;
    let (i, flow_rate) = u16(i)?;
    let (i, _) = alt((
        tag("; tunnels lead to valves "),
        tag("; tunnel leads to valve "),
    ))(i)?;
    let (i, neighbors) = separated_list0(tag(", "), parse_valve_name)(i)?;

    Ok((
        i,
        Valve {
            name,
            flow_rate,
            neighbors,
        },
    ))
}

fn parse_input(input: &str) -> impl Iterator<Item = Valve> {
    input.lines().map(|line| parse_line(line).unwrap().1)
}

fn build_paths<'a>(
    valves: &'a HashMap<&'a str, Valve>,
) -> HashMap<&'a str, HashMap<&'a str, usize>> {
    let mut neighbors: HashMap<&str, HashMap<&str, usize>> = HashMap::new();
    for node in valves.values() {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((node, 0));
        visited.insert(node.name);

        while let Some((current_node, costs)) = queue.pop_front() {
            if current_node.flow_rate > 0 {
                neighbors
                    .entry(node.name)
                    .or_default()
                    .insert(&current_node.name, costs);
            }

            for neighbor in &current_node.neighbors {
                if !visited.insert(neighbor) {
                    continue;
                }

                queue.push_back((valves.get(neighbor).unwrap(), costs + 1))
            }
        }
    }
    neighbors
}

pub fn part_one(input: &str) -> i32 {
    let valves: HashMap<&str, Valve> = parse_input(input)
        .map(|valve| (valve.name, valve))
        .collect();
    let paths = build_paths(&valves);
    let valve_index: HashMap<&str, usize> =
        valves.keys().enumerate().map(|(k, v)| (*v, k)).collect();

    assert!(valves.len() <= 64);

    let mut open = Vec::new();
    open.push(("AA", 0, 30, 0u64));

    dbg!(paths.get("AA").unwrap());

    let mut max_release = 0;

    while let Some((node, total, budget, visited)) = open.pop() {
        if total > max_release {
            max_release = max(max_release, total);
        }

        for (target, move_costs) in paths.get(node).unwrap() {
            let node_idx = valve_index.get(target).unwrap();
            let mask = 1u64 << node_idx;

            let rem_budget = budget - *move_costs as i32 - 1;
            let gain = rem_budget * valves.get(target).unwrap().flow_rate as i32;

            if rem_budget >= 0 && (visited & mask) == 0 {
                open.push((target, total + gain, rem_budget, visited | mask));
            }
        }
    }

    dbg!(valve_index);

    max_release
}

pub fn part_two(input: &str) -> i32 {
    let valves: HashMap<&str, Valve> = parse_input(input)
        .map(|valve| (valve.name, valve))
        .collect();
    let paths = build_paths(&valves);
    let valve_index: HashMap<&str, usize> =
        valves.keys().enumerate().map(|(k, v)| (*v, k)).collect();

    assert!(valves.len() <= 64);

    let mut open = Vec::new();
    open.push((("AA", 26), ("AA", 26), 0, 0u64));

    dbg!(paths.get("AA").unwrap());

    let mut max_release = 0;
    let mut nodes = 0;

    while let Some(((human, h_arrival), (elpehant, e_arrival), total, visited)) = open.pop() {
        if total > max_release {
            max_release = max(max_release, total);
            dbg!(((human, h_arrival), (elpehant, e_arrival), total, nodes));
        }

        nodes += 1;

        let ts = max(h_arrival, e_arrival);

        let position = if ts == h_arrival { human } else { elpehant };

        for (target, move_costs) in paths.get(position).unwrap() {
            let node_idx = valve_index.get(target).unwrap();
            let mask = 1u64 << node_idx;

            let rem_budget = ts - *move_costs as i32 - 1;
            let gain = rem_budget * valves.get(target).unwrap().flow_rate as i32;

            if rem_budget >= 0 && (visited & mask) == 0 {
                if ts == h_arrival {
                    open.push((
                        (target, rem_budget),
                        (elpehant, e_arrival),
                        total + gain,
                        visited | mask,
                    ));
                } else {
                    open.push((
                        (human, h_arrival),
                        (target, rem_budget),
                        total + gain,
                        visited | mask,
                    ));
                };
            }
        }
    }

    dbg!(nodes);
    max_release
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 16);
        assert_eq!(part_one(&input), 1651);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 16);
        assert_eq!(part_two(&input), 1707);
    }
}
