use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map_res, opt},
    multi::separated_list0,
    multi::{many1, separated_list1},
    IResult,
};
use std::collections::{HashMap, HashSet};
use std::{cmp::Ordering, hash::Hash};
use std::{
    collections::{BinaryHeap, VecDeque},
    hash::Hasher,
};

type Input = Vec<ValveDescription>;

type Location = (char, char);

#[derive(Debug, Clone, Eq, PartialEq)]
struct ValveDescription {
    label: Location,
    flow_rate: usize,
    connections: Vec<Location>,
}

#[derive(Debug, Clone, Eq)]
struct State {
    total_pressure_released: usize,
    pressure_per_minute: usize,
    location: (char, char),
    open_valves: HashSet<(char, char)>,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, h: &mut H) {
        Hash::hash(
            &(
                self.total_pressure_released,
                self.pressure_per_minute,
                self.location,
            ),
            h,
        );
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.total_pressure_released == other.total_pressure_released
            && self.pressure_per_minute == other.pressure_per_minute
            && self.location == other.location
    }
}

#[derive(Debug, Clone, Eq)]
struct TwoActorState {
    total_pressure_released: usize,
    pressure_per_minute: usize,
    location_1: (char, char),
    location_2: (char, char),
    open_valves: HashSet<(char, char)>,
}

impl Hash for TwoActorState {
    fn hash<H: Hasher>(&self, h: &mut H) {
        Hash::hash(
            &(
                self.total_pressure_released,
                self.pressure_per_minute,
                self.location_1.min(self.location_2),
                self.location_1.max(self.location_2),
            ),
            h,
        );
    }
}

impl PartialEq for TwoActorState {
    fn eq(&self, other: &Self) -> bool {
        self.total_pressure_released == other.total_pressure_released
            && self.pressure_per_minute == other.pressure_per_minute
            && ((self.location_1 == other.location_1 && self.location_2 == other.location_2)
                || self.location_2 == other.location_1 && self.location_1 == other.location_2)
    }
}

pub fn part1() -> usize {
    let input = include_str!("../../data/2022/16.txt");

    match input_parser(input) {
        Ok((remaining_input, valves)) if remaining_input.is_empty() => {
            println!("parsed entire input");
            // let mut time_remaining = 30;

            // let mut states = HashSet::new();
            // states.insert(State {
            //     location: ('A', 'A'),
            //     total_pressure_released: 0,
            //     pressure_per_minute: 0,
            //     open_valves: HashSet::new(),
            // });

            // let mut next_states: HashSet<State> = HashSet::new();

            // while time_remaining > 0 {
            //     println!("time remaining: {}", time_remaining);
            //     println!("{} states to evaluate", states.len());

            //     for state in states {
            //         let room = valves.iter().find(|r| r.label == state.location).unwrap();

            //         if room.flow_rate > 0 && !state.open_valves.contains(&state.location) {
            //             let mut next_open_valves = state.open_valves.clone();
            //             next_open_valves.insert(state.location);
            //             let flow_rate = room.flow_rate;

            //             let next_state = State {
            //                 location: state.location,
            //                 total_pressure_released: state.total_pressure_released
            //                     + state.pressure_per_minute,
            //                 pressure_per_minute: state.pressure_per_minute + flow_rate,
            //                 open_valves: next_open_valves,
            //             };

            //             next_states.insert(next_state);
            //         }

            //         for connection in room.connections.iter() {
            //             let next_state = State {
            //                 location: *connection,
            //                 total_pressure_released: state.total_pressure_released
            //                     + state.pressure_per_minute,
            //                 pressure_per_minute: state.pressure_per_minute,
            //                 open_valves: state.open_valves.clone(),
            //             };

            //             next_states.insert(next_state);
            //         }
            //     }

            //     states = next_states;
            //     next_states = HashSet::new();
            //     time_remaining -= 1;
            // }

            // states
            //     .iter()
            //     .max_by(|a, b| a.total_pressure_released.cmp(&b.total_pressure_released))
            //     .unwrap()
            //     .total_pressure_released
            println!("skpping part 1");
            0
        }
        Ok((remaining, _)) => {
            println!("remaining unparsed \"{}\"", remaining);
            0
        }
        Err(e) => {
            println!("error parsing \"{}\"", e);
            0
        }
    }
}

pub fn part2() -> usize {
    let input = include_str!("../../data/2022/16.txt");

    match input_parser(input) {
        Ok((remaining_input, valves)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut time_remaining = 26;

            let mut states = HashSet::new();
            states.insert(TwoActorState {
                location_1: ('A', 'A'),
                location_2: ('A', 'A'),
                total_pressure_released: 0,
                pressure_per_minute: 0,
                open_valves: HashSet::new(),
            });

            let mut next_states: HashSet<TwoActorState> = HashSet::new();

            let total_amount_of_flow = valves.iter().map(|v| v.flow_rate).sum1().unwrap();

            let mut final_states: Vec<TwoActorState> = vec![];

            let mut cutoff = 0;

            while time_remaining > 0 {
                println!("time remaining: {}", time_remaining);
                println!("{} states to evaluate", states.len());
                println!("{} already finished", final_states.len());
                if !final_states.is_empty() {
                    println!(
                        "Best so far: {}",
                        final_states
                            .iter()
                            .max_by(|a, b| a
                                .total_pressure_released
                                .cmp(&b.total_pressure_released))
                            .unwrap()
                            .total_pressure_released
                    );
                }

                let mut min = states.iter().next().unwrap().total_pressure_released;
                let mut max = states.iter().next().unwrap().total_pressure_released;

                for state in states {
                    if state.pressure_per_minute == total_amount_of_flow {
                        //nothing more they can do
                        final_states.push(TwoActorState {
                            location_1: state.location_1,
                            location_2: state.location_2,
                            total_pressure_released: state.total_pressure_released
                                + (state.pressure_per_minute * time_remaining),
                            pressure_per_minute: state.pressure_per_minute,
                            open_valves: state.open_valves,
                        });
                        continue;
                    }

                    if time_remaining < 20
                        && time_remaining > 2
                        && state.total_pressure_released < cutoff
                    {
                        continue;
                    }

                    min = min.min(state.total_pressure_released);
                    max = max.max(state.total_pressure_released);

                    let room_1 = valves.iter().find(|r| r.label == state.location_1).unwrap();
                    let room_2 = valves.iter().find(|r| r.label == state.location_2).unwrap();

                    let room_1_should_open =
                        room_1.flow_rate > 0 && !state.open_valves.contains(&state.location_1);
                    let room_2_should_open =
                        room_2.flow_rate > 0 && !state.open_valves.contains(&state.location_2);

                    if room_1.label == room_2.label {
                        if room_1_should_open {
                            //actor 1 stays, actor 2 moves
                            for connection in room_2.connections.iter() {
                                let mut next_open_valves = state.open_valves.clone();
                                next_open_valves.insert(state.location_1);
                                let next_state = TwoActorState {
                                    location_1: state.location_1,
                                    location_2: *connection,
                                    total_pressure_released: state.total_pressure_released
                                        + state.pressure_per_minute,
                                    pressure_per_minute: state.pressure_per_minute
                                        + room_1.flow_rate,
                                    open_valves: next_open_valves,
                                };

                                next_states.insert(next_state);
                            }
                        }

                        for connection_1 in room_1.connections.iter() {
                            for connection_2 in room_2.connections.iter() {
                                let next_state = TwoActorState {
                                    location_1: *connection_1,
                                    location_2: *connection_2,
                                    total_pressure_released: state.total_pressure_released
                                        + state.pressure_per_minute,
                                    pressure_per_minute: state.pressure_per_minute,
                                    open_valves: state.open_valves.clone(),
                                };

                                next_states.insert(next_state);
                            }
                        }
                    } else {
                        //if separate rooms
                        // both open
                        if room_1_should_open && room_2_should_open {
                            let mut next_open_valves = state.open_valves.clone();
                            next_open_valves.insert(state.location_1);
                            next_open_valves.insert(state.location_2);
                            let next_state = TwoActorState {
                                location_1: state.location_1,
                                location_2: state.location_2,
                                total_pressure_released: state.total_pressure_released
                                    + state.pressure_per_minute,
                                pressure_per_minute: state.pressure_per_minute
                                    + room_1.flow_rate
                                    + room_2.flow_rate,
                                open_valves: next_open_valves,
                            };

                            next_states.insert(next_state);
                        }

                        // one open and other move
                        if room_1_should_open && !room_2_should_open {
                            for connection in room_2.connections.iter() {
                                let mut next_open_valves = state.open_valves.clone();
                                next_open_valves.insert(state.location_1);
                                let next_state = TwoActorState {
                                    location_1: state.location_1,
                                    location_2: *connection,
                                    total_pressure_released: state.total_pressure_released
                                        + state.pressure_per_minute,
                                    pressure_per_minute: state.pressure_per_minute
                                        + room_1.flow_rate,
                                    open_valves: next_open_valves,
                                };

                                next_states.insert(next_state);
                            }
                        }

                        // one move and other open
                        if !room_1_should_open && room_2_should_open {
                            for connection_1 in room_1.connections.iter() {
                                let mut next_open_valves = state.open_valves.clone();
                                next_open_valves.insert(state.location_2);
                                let next_state = TwoActorState {
                                    location_1: *connection_1,
                                    location_2: state.location_2,
                                    total_pressure_released: state.total_pressure_released
                                        + state.pressure_per_minute,
                                    pressure_per_minute: state.pressure_per_minute
                                        + room_2.flow_rate,
                                    open_valves: next_open_valves,
                                };

                                next_states.insert(next_state);
                            }
                        }

                        //both move
                        for connection_1 in room_1.connections.iter() {
                            for connection_2 in room_2.connections.iter() {
                                let next_state = TwoActorState {
                                    location_1: *connection_1,
                                    location_2: *connection_2,
                                    total_pressure_released: state.total_pressure_released
                                        + state.pressure_per_minute,
                                    pressure_per_minute: state.pressure_per_minute,
                                    open_valves: state.open_valves.clone(),
                                };

                                next_states.insert(next_state);
                            }
                        }
                    }
                }

                cutoff = max - ((max - min) / 4);
                states = next_states;
                next_states = HashSet::new();
                time_remaining -= 1;
            }

            states
                .iter()
                .max_by(|a, b| a.total_pressure_released.cmp(&b.total_pressure_released))
                .unwrap()
                .total_pressure_released
        }
        Ok((remaining, _)) => {
            println!("remaining unparsed \"{}\"", remaining);
            0
        }
        Err(e) => {
            println!("error parsing \"{}\"", e);
            0
        }
    }
}

pub fn part2_old() -> usize {
    let input = include_str!("../../data/2022/16-example.txt");

    match input_parser(input) {
        Ok((remaining_input, valves)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut time_remaining = 26;

            let mut states = HashSet::new();
            states.insert(TwoActorState {
                location_1: ('A', 'A'),
                location_2: ('A', 'A'),
                total_pressure_released: 0,
                pressure_per_minute: 0,
                open_valves: HashSet::new(),
            });

            let mut next_states: HashSet<TwoActorState> = HashSet::new();

            let total_amount_of_flow = valves.iter().map(|v| v.flow_rate).sum1().unwrap();

            let mut final_states: Vec<TwoActorState> = vec![];

            while time_remaining > 0 {
                println!("time remaining: {}", time_remaining);
                println!("{} states to evaluate", states.len());
                println!("{} already finished", final_states.len());
                if !final_states.is_empty() {
                    println!(
                        "Best so far: {}",
                        final_states
                            .iter()
                            .max_by(|a, b| a
                                .total_pressure_released
                                .cmp(&b.total_pressure_released))
                            .unwrap()
                            .total_pressure_released
                    );
                }

                for state in states {
                    if state.pressure_per_minute == total_amount_of_flow {
                        //nothing more they can do
                        final_states.push(TwoActorState {
                            location_1: state.location_1,
                            location_2: state.location_2,
                            total_pressure_released: state.total_pressure_released
                                + (state.pressure_per_minute * time_remaining),
                            pressure_per_minute: state.pressure_per_minute,
                            open_valves: state.open_valves,
                        });
                        continue;
                    }

                    let room_1 = valves.iter().find(|r| r.label == state.location_1).unwrap();
                    let room_2 = valves.iter().find(|r| r.label == state.location_2).unwrap();

                    let room_1_should_open =
                        room_1.flow_rate > 0 && !state.open_valves.contains(&state.location_1);
                    let room_2_should_open =
                        room_2.flow_rate > 0 && !state.open_valves.contains(&state.location_2);

                    if room_1.label == room_2.label {
                        if room_1_should_open {
                            //actor 1 stays, actor 2 moves
                            for connection in room_2.connections.iter() {
                                let mut next_open_valves = state.open_valves.clone();
                                next_open_valves.insert(state.location_1);
                                let next_state = TwoActorState {
                                    location_1: state.location_1,
                                    location_2: *connection,
                                    total_pressure_released: state.total_pressure_released
                                        + state.pressure_per_minute,
                                    pressure_per_minute: state.pressure_per_minute
                                        + room_1.flow_rate,
                                    open_valves: next_open_valves,
                                };

                                next_states.insert(next_state);
                            }
                        } else {
                            //both move
                            for connection_1 in room_1.connections.iter() {
                                for connection_2 in room_2.connections.iter() {
                                    let next_state = TwoActorState {
                                        location_1: *connection_1,
                                        location_2: *connection_2,
                                        total_pressure_released: state.total_pressure_released
                                            + state.pressure_per_minute,
                                        pressure_per_minute: state.pressure_per_minute,
                                        open_valves: state.open_valves.clone(),
                                    };

                                    next_states.insert(next_state);
                                }
                            }
                        }
                    } else if room_1_should_open {
                        // actor 1 stays
                        if room_2_should_open {
                            // both stay and open
                            let mut next_open_valves = state.open_valves.clone();
                            next_open_valves.insert(state.location_1);
                            next_open_valves.insert(state.location_2);
                            let next_state = TwoActorState {
                                location_1: state.location_1,
                                location_2: state.location_2,
                                total_pressure_released: state.total_pressure_released
                                    + state.pressure_per_minute,
                                pressure_per_minute: state.pressure_per_minute
                                    + room_1.flow_rate
                                    + room_2.flow_rate,
                                open_valves: next_open_valves,
                            };

                            next_states.insert(next_state);
                        } else {
                            for connection in room_2.connections.iter() {
                                let mut next_open_valves = state.open_valves.clone();
                                next_open_valves.insert(state.location_1);
                                let next_state = TwoActorState {
                                    location_1: state.location_1,
                                    location_2: *connection,
                                    total_pressure_released: state.total_pressure_released
                                        + state.pressure_per_minute,
                                    pressure_per_minute: state.pressure_per_minute
                                        + room_1.flow_rate,
                                    open_valves: next_open_valves,
                                };

                                next_states.insert(next_state);
                            }
                        }
                    } else {
                        //actor 1 moves
                        if room_2_should_open {
                            // actor 1 moves actor 2 stays
                            for connection_1 in room_1.connections.iter() {
                                let mut next_open_valves = state.open_valves.clone();
                                next_open_valves.insert(state.location_2);
                                let next_state = TwoActorState {
                                    location_1: *connection_1,
                                    location_2: state.location_2,
                                    total_pressure_released: state.total_pressure_released
                                        + state.pressure_per_minute,
                                    pressure_per_minute: state.pressure_per_minute
                                        + room_2.flow_rate,
                                    open_valves: next_open_valves,
                                };

                                next_states.insert(next_state);
                            }
                        } else {
                            //both move
                            for connection_1 in room_1.connections.iter() {
                                for connection_2 in room_2.connections.iter() {
                                    let next_state = TwoActorState {
                                        location_1: *connection_1,
                                        location_2: *connection_2,
                                        total_pressure_released: state.total_pressure_released
                                            + state.pressure_per_minute,
                                        pressure_per_minute: state.pressure_per_minute,
                                        open_valves: state.open_valves.clone(),
                                    };

                                    next_states.insert(next_state);
                                }
                            }
                        }
                    }
                }

                states = next_states;
                next_states = HashSet::new();
                time_remaining -= 1;
            }

            final_states
                .iter()
                .max_by(|a, b| a.total_pressure_released.cmp(&b.total_pressure_released))
                .unwrap()
                .total_pressure_released
        }
        Ok((remaining, _)) => {
            println!("remaining unparsed \"{}\"", remaining);
            0
        }
        Err(e) => {
            println!("error parsing \"{}\"", e);
            0
        }
    }
}

fn location_name_parser(input: &str) -> IResult<&str, Location> {
    let (input, label_a) = nom::character::complete::anychar(input)?;
    let (input, label_b) = nom::character::complete::anychar(input)?;
    Ok((input, (label_a, label_b)))
}

fn valve_description_parser(input: &str) -> IResult<&str, ValveDescription> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, label) = location_name_parser(input)?;
    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, flow_rate) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<usize>()
    })(input)?;
    let (input, _) = alt((
        tag("; tunnel leads to valve "),
        tag("; tunnels lead to valves "),
    ))(input)?;
    let (input, connections) = separated_list1(tag(", "), location_name_parser)(input)?;

    Ok((
        input,
        ValveDescription {
            label,
            flow_rate,
            connections,
        },
    ))
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, formations) =
        separated_list1(nom::character::complete::newline, valve_description_parser)(input)?;

    Ok((input, formations))
}
