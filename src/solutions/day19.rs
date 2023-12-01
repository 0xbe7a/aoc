use std::{
    cmp::max,
    ops::{Add, Sub},
};

use rustc_hash::FxHashSet as HashSet;

use nom::{
    bytes::complete::tag, character::complete::u16, combinator::opt, sequence::terminated, IResult,
};

use rayon::prelude::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Ressources(u16, u16, u16, u16);

#[derive(Debug, Clone)]
struct Blueprint {
    id: u16,
    ore: Ressources,
    clay: Ressources,
    obsidian: Ressources,
    geode: Ressources,
}

impl Ressources {
    fn available(self, price: Ressources) -> bool {
        self.0 >= price.0 && self.1 >= price.1 && self.2 >= price.2 && self.3 >= price.3
    }
}

impl Sub for Ressources {
    type Output = Ressources;

    fn sub(self, rhs: Self) -> Self::Output {
        Ressources(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl Add for Ressources {
    type Output = Ressources;

    fn add(self, rhs: Self) -> Self::Output {
        Ressources(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

fn parse_price(i: &str) -> IResult<&str, Ressources> {
    let (i, ore) = opt(terminated(u16, tag(" ore")))(i)?;
    let (i, _) = opt(tag(" and "))(i)?;
    let (i, clay) = opt(terminated(u16, tag(" clay")))(i)?;
    let (i, _) = opt(tag(" and "))(i)?;
    let (i, obsidian) = opt(terminated(u16, tag(" obsidian")))(i)?;
    Ok((
        i,
        Ressources(
            ore.unwrap_or(0),
            clay.unwrap_or(0),
            obsidian.unwrap_or(0),
            0,
        ),
    ))
}

fn parse_line(i: &str) -> IResult<&str, Blueprint> {
    let (i, _) = tag("Blueprint ")(i)?;
    let (i, id) = u16(i)?;
    let (i, _) = tag(": Each ore robot costs ")(i)?;
    let (i, ore) = parse_price(i)?;
    let (i, _) = tag(". Each clay robot costs ")(i)?;
    let (i, clay) = parse_price(i)?;
    let (i, _) = tag(". Each obsidian robot costs ")(i)?;
    let (i, obsidian) = parse_price(i)?;
    let (i, _) = tag(". Each geode robot costs ")(i)?;
    let (i, geode) = parse_price(i)?;

    Ok((
        i,
        Blueprint {
            id,
            ore,
            clay,
            obsidian,
            geode,
        },
    ))
}

fn parse_input(input: &str) -> impl Iterator<Item = Blueprint> + '_ {
    input.lines().map(|line| parse_line(line).unwrap().1)
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct QueueItem {
    ressources: Ressources,
    robots: Ressources,
    time: u16,
}

fn sim_blueprint(blueprint: &Blueprint, time: u16) -> u16 {
    let mut queue = Vec::new();
    queue.push(QueueItem {
        ressources: Ressources(0, 0, 0, 0),
        robots: Ressources(1, 0, 0, 0),
        time: 0,
    });

    let mut max_geodes = 0;
    let mut visited_states = HashSet::default();
    let max_ore_costs = [blueprint.ore.0, blueprint.clay.0, blueprint.obsidian.0, blueprint.geode.0].iter().copied().max().unwrap();
    let max_clay_costs = [blueprint.ore.1, blueprint.clay.1, blueprint.obsidian.1, blueprint.geode.1].iter().copied().max().unwrap();
    let max_obsidian_costs = [blueprint.ore.2, blueprint.clay.2, blueprint.obsidian.2, blueprint.geode.2].iter().copied().max().unwrap();

    while let Some(mut item) = queue.pop() {
        if !visited_states.insert(item.clone()) {
            continue;
        }

        item.time += 1;

        if item.time == time {
            max_geodes = max(max_geodes, item.ressources.3);
            continue;
        }

        if item.ressources.available(blueprint.ore) && item.robots.0 < max_ore_costs {
            let mut ore_build = item.clone();
            ore_build.robots.0 += 1;
            ore_build.ressources = ore_build.ressources - blueprint.ore + item.robots;
            queue.push(ore_build)
        }

        if item.ressources.available(blueprint.clay) && item.robots.1 < max_clay_costs {
            let mut clay_build = item.clone();
            clay_build.robots.1 += 1;
            clay_build.ressources = clay_build.ressources - blueprint.clay + item.robots;
            queue.push(clay_build)
        }

        if item.ressources.available(blueprint.obsidian) && item.robots.2 < max_obsidian_costs {
            let mut obsidian_build = item.clone();
            obsidian_build.robots.2 += 1;
            obsidian_build.ressources =
                obsidian_build.ressources - blueprint.obsidian + item.robots;
            queue.push(obsidian_build)
        }

        if item.ressources.available(blueprint.geode) {
            let mut geode_build = item.clone();
            geode_build.robots.3 += 1;
            geode_build.ressources = geode_build.ressources - blueprint.geode + item.robots;
            queue.push(geode_build)
        }

        //No action
        item.ressources = item.ressources + item.robots;
        queue.push(item);
    }

    max_geodes
}

pub fn part_one(input: &str) -> u32 {
    let blueprints: Vec<_> = parse_input(input).collect();

    blueprints.into_par_iter().map(|blueprint| {
        let max_geodes = sim_blueprint(&blueprint, 25);
        dbg!(max_geodes, blueprint.id);

        max_geodes as u32 * blueprint.id as u32
    }).sum()
}

pub fn part_two(input: &str) -> u32 {
    let blueprints: Vec<_> = parse_input(input).collect();

    blueprints.iter().take(3).map(|blueprint| {
        let max_geodes = sim_blueprint(&blueprint, 33);
        dbg!(max_geodes, blueprint.id);

        max_geodes as u32
    }).product()
}

/* 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 19);
        assert_eq!(part_one(&input), 33);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 19);
        assert_eq!(part_two(&input), 1707);
    }
}
 */