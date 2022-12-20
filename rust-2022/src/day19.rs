use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map_res, opt},
    multi::separated_list0,
    multi::{many1, separated_list1},
    IResult,
};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::{cmp::Ordering, hash::Hash};
use std::{collections::BinaryHeap, time::SystemTime};

type Input = Vec<Blueprint>;

struct Blueprint {
    ore_robot_cost: Cost,
    clay_robot_cost: Cost,
    obsidian_robot_cost: Cost,
    geode_robot_cost: Cost,
}

enum Cost {
    Ore(usize),
    OreAndClay(usize, usize),
    OreAndObsidian(usize, usize),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
    factory_use: FactoryUse,
    time: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum FactoryUse {
    Nothing,
    BuildingOreRobot,
    BuildingClayRobot,
    BuildingObsidianRobot,
    BuildingGeodeRobot,
}

pub fn part1() -> usize {
    let input = include_str!("../../data/2022/19.txt");

    match input_parser(input) {
        Ok((remaining_input, blueprints)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut sum = 0;

            // for (index, blueprint) in blueprints.iter().enumerate() {
            //     let start = SystemTime::now();
            //     let max_geodes = blueprint.max_geodes(24);
            //     let end = SystemTime::now();
            //     println!(
            //         "ID {}: {} in {} seconds",
            //         index + 1,
            //         max_geodes,
            //         end.duration_since(start).unwrap().as_secs()
            //     );
            //     sum += (index + 1) * max_geodes;
            // }

            println!("part 1 result: {sum}");

            sum
        }
        Ok((remaining, _)) => {
            println!("remaining unparsed \"{remaining}\"");
            0
        }
        Err(e) => {
            println!("error parsing \"{e}\"");
            0
        }
    }
}

pub fn part2() -> usize {
    let input = include_str!("../../data/2022/19.txt");

    match input_parser(input) {
        Ok((remaining_input, blueprints)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut result = 1;

            for (index, blueprint) in blueprints.iter().take(1).enumerate() {
                let start = SystemTime::now();
                let max_geodes = blueprint.max_geodes(32);
                let end = SystemTime::now();
                println!(
                    "ID {}: {} in {} seconds",
                    index + 1,
                    max_geodes,
                    end.duration_since(start).unwrap().as_secs()
                );
                result *= max_geodes;
            }

            result
        }
        Ok((remaining, _)) => {
            println!("remaining unparsed \"{remaining}\"");
            0
        }
        Err(e) => {
            println!("error parsing \"{e}\"");
            0
        }
    }
}

fn blueprint_parser(input: &str) -> IResult<&str, Blueprint> {
    let (input, _) = tag("Blueprint ")(input)?;
    let (input, _) = nom::character::complete::digit1(input)?;
    let (input, _) = tag(": Each ore robot costs ")(input)?;
    let (input, ore_robot_ore_cost) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<usize>()
    })(input)?;
    let (input, _) = tag(" ore. Each clay robot costs ")(input)?;
    let (input, clay_robot_ore_cost) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<usize>()
    })(input)?;
    let (input, _) = tag(" ore. Each obsidian robot costs ")(input)?;
    let (input, obsidian_robot_ore_cost) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<usize>()
    })(input)?;
    let (input, _) = tag(" ore and ")(input)?;
    let (input, obsidian_robot_clay_cost) =
        map_res(nom::character::complete::digit1, |s: &str| {
            s.parse::<usize>()
        })(input)?;
    let (input, _) = tag(" clay. Each geode robot costs ")(input)?;
    let (input, geode_robot_ore_cost) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<usize>()
    })(input)?;
    let (input, _) = tag(" ore and ")(input)?;
    let (input, geode_robot_obsidian_cost) =
        map_res(nom::character::complete::digit1, |s: &str| {
            s.parse::<usize>()
        })(input)?;
    let (input, _) = tag(" obsidian.")(input)?;

    Ok((
        input,
        Blueprint {
            ore_robot_cost: Cost::Ore(ore_robot_ore_cost),
            clay_robot_cost: Cost::Ore(clay_robot_ore_cost),
            obsidian_robot_cost: Cost::OreAndClay(
                obsidian_robot_ore_cost,
                obsidian_robot_clay_cost,
            ),
            geode_robot_cost: Cost::OreAndObsidian(geode_robot_ore_cost, geode_robot_obsidian_cost),
        },
    ))
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, formations) =
        separated_list1(nom::character::complete::newline, blueprint_parser)(input)?;

    Ok((input, formations))
}

impl Blueprint {
    fn max_geodes(&self, minutes: usize) -> usize {
        let mut states = HashSet::new();
        let mut best_state = State::new();
        let mut count_final_states = 0;
        states.insert(State::new());

        // let mut min = states.iter().next().unwrap().geodes;
        // let mut max = states.iter().next().unwrap().geodes;
        // let mut cutoff = 0;

        while !states.is_empty() {
            // println!("time remaining: {}", time_remaining);
            println!("{} states to evaluate", states.len());
            println!("{} final states", count_final_states);
            println!("{} best so far", best_state.geodes);
            let mut next_states = HashSet::new();

            for state in states {
                // if time_remaining < 10 && state.geodes <= cutoff {
                //     continue;
                // }

                // if state.is_building_too_much_supply(self) {
                //     continue;
                // }

                // min = min.min(state.geodes);
                // max = max.max(state.geodes);

                // next_states.extend(self.generate_possible_states(&state));
                for next_state in self.generate_possible_states(&state, minutes) {
                    if next_state.time < minutes {
                        next_states.insert(next_state);
                    } else if next_state.geodes > best_state.geodes {
                        best_state = next_state;
                        count_final_states += 1;
                    } else {
                        count_final_states += 1;
                    }
                }
            }

            // println!(
            //     "{:?}",
            //     next_states
            //         .iter()
            //         .max_by(|a, b| a.geodes.cmp(&b.geodes))
            //         .unwrap_or(&State::new()),
            // );

            // cutoff = max - ((max - min) / 2);
            states = next_states;
        }

        println!("found {best_state:?}");

        best_state.geodes
    }

    fn generate_possible_states(&self, current_state: &State, minutes: usize) -> Vec<State> {
        //options:
        //build nothing
        //build one ore robot if possible
        //build one clay robot if possible
        //build one obsidian robot if possible
        //build one geode robot if possible
        let mut possibilities = vec![];

        if let Some(time_to_afford_ore) = current_state.time_to_afford(&self.ore_robot_cost) {
            if current_state.time + time_to_afford_ore < minutes {
                let new_state = current_state
                    .advance_time(time_to_afford_ore + 1)
                    .pay(&self.ore_robot_cost)
                    .start_factory(FactoryUse::BuildingOreRobot)
                    .resolve_factory();
                possibilities.push(new_state);
            }
        }

        if let Some(time_to_afford_clay) = current_state.time_to_afford(&self.clay_robot_cost) {
            if current_state.time + time_to_afford_clay < minutes {
                let new_state = current_state
                    .advance_time(time_to_afford_clay + 1)
                    .pay(&self.clay_robot_cost)
                    .start_factory(FactoryUse::BuildingClayRobot)
                    .resolve_factory();
                possibilities.push(new_state);
            }
        }

        if let Some(time_to_afford_obsidian) =
            current_state.time_to_afford(&self.obsidian_robot_cost)
        {
            if current_state.time + time_to_afford_obsidian < minutes {
                let new_state = (*current_state)
                    .advance_time(time_to_afford_obsidian + 1)
                    .pay(&self.obsidian_robot_cost)
                    .start_factory(FactoryUse::BuildingObsidianRobot)
                    .resolve_factory();
                possibilities.push(new_state);
            }
        }

        if let Some(time_to_afford_geode) = current_state.time_to_afford(&self.geode_robot_cost) {
            if current_state.time + time_to_afford_geode < minutes {
                let new_state = current_state
                    .advance_time(time_to_afford_geode + 1)
                    .pay(&self.geode_robot_cost)
                    .start_factory(FactoryUse::BuildingGeodeRobot)
                    .resolve_factory();
                possibilities.push(new_state);
            }
        }

        if possibilities.is_empty() {
            //must be out of time
            possibilities.push((*current_state).advance_time(minutes - current_state.time))
        }

        possibilities
    }

    fn max_ore_cost(&self) -> usize {
        self.ore_robot_cost
            .ore_cost()
            .max(self.clay_robot_cost.ore_cost())
            .max(self.obsidian_robot_cost.ore_cost())
            .max(self.geode_robot_cost.ore_cost())
    }
}

impl Cost {
    fn ore_cost(&self) -> usize {
        match self {
            Cost::Ore(ore) => *ore,
            Cost::OreAndClay(ore, _) => *ore,
            Cost::OreAndObsidian(ore, _) => *ore,
        }
    }

    fn clay_cost(&self) -> usize {
        match self {
            Cost::Ore(_) => 0,
            Cost::OreAndClay(_, clay) => *clay,
            Cost::OreAndObsidian(_, _) => 0,
        }
    }

    fn obsidian_cost(&self) -> usize {
        match self {
            Cost::Ore(_) => 0,
            Cost::OreAndClay(_, _) => 0,
            Cost::OreAndObsidian(_, obsidian) => *obsidian,
        }
    }
}

// impl fmt::Debug for State {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "ore: {} clay: {}, obsidian: {}, geodes: {}\nore_robots: {} clay_robots: {}, obsidian_robots: {}, geode_robots: {}\n",
//             self.ore, self.clay, self.obsidian, self.geodes,self.ore_robots, self.clay_robots, self.obsidian_robots, self.geode_robots
//         )
//     }
// }

impl State {
    fn new() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            factory_use: FactoryUse::Nothing,
            time: 0,
        }
    }

    fn resolve_factory(mut self) -> Self {
        match self.factory_use {
            FactoryUse::Nothing => {}
            FactoryUse::BuildingOreRobot => self.ore_robots += 1,
            FactoryUse::BuildingClayRobot => self.clay_robots += 1,
            FactoryUse::BuildingObsidianRobot => self.obsidian_robots += 1,
            FactoryUse::BuildingGeodeRobot => self.geode_robots += 1,
        };

        self.factory_use = FactoryUse::Nothing;

        self
    }

    fn advance_time(mut self, time: usize) -> Self {
        self.ore += self.ore_robots * time;
        self.clay += self.clay_robots * time;
        self.obsidian += self.obsidian_robots * time;
        self.geodes += self.geode_robots * time;
        self.time += time;

        self
    }

    fn start_factory(mut self, how: FactoryUse) -> Self {
        self.factory_use = how;

        self
    }

    fn time_to_afford(&self, cost: &Cost) -> Option<usize> {
        match cost {
            Cost::Ore(ore) => {
                if self.ore >= *ore {
                    Some(0)
                } else {
                    Some((*ore - self.ore).div_ceil(self.ore_robots))
                }
            }
            Cost::OreAndClay(ore, clay) => {
                if self.clay_robots > 0 {
                    if self.ore >= *ore && self.clay >= *clay {
                        Some(0)
                    } else {
                        Some(
                            (*ore as isize - self.ore as isize)
                                .div_ceil(self.ore_robots as isize)
                                .max(
                                    (*clay as isize - self.clay as isize)
                                        .div_ceil(self.clay_robots as isize),
                                ) as usize,
                        )
                    }
                } else {
                    None
                }
            }
            Cost::OreAndObsidian(ore, obsidian) => {
                if self.obsidian_robots > 0 {
                    if self.ore >= *ore && self.obsidian >= *obsidian {
                        Some(0)
                    } else {
                        Some(
                            (*ore as isize - self.ore as isize)
                                .div_ceil(self.ore_robots as isize)
                                .max(
                                    (*obsidian as isize - self.obsidian as isize)
                                        .div_ceil(self.obsidian_robots as isize),
                                ) as usize,
                        )
                    }
                } else {
                    None
                }
            }
        }
    }

    fn pay(&mut self, cost: &Cost) -> &Self {
        match cost {
            Cost::Ore(ore) => self.ore -= ore,
            Cost::OreAndClay(ore, clay) => {
                self.ore -= ore;
                self.clay -= clay;
            }
            Cost::OreAndObsidian(ore, obsidian) => {
                self.ore -= ore;
                self.obsidian -= obsidian;
            }
        };

        self
    }
}
