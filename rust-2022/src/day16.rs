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
use std::collections::{BinaryHeap, VecDeque};
use std::collections::{HashMap, HashSet};
use std::{cmp::Ordering, hash::Hash};

type Input = Vec<ValveDescription>;

type Location = (char, char);

#[derive(Debug, Clone, Eq, PartialEq)]
struct ValveDescription {
    label: Location,
    flow_rate: usize,
    connections: Vec<Location>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Step {
    Move(Location),
    OpenValve(Location),
}

struct Opportunity {
    moves: VecDeque<Step>,
    value: usize,
}

pub fn part1() -> usize {
    let input = include_str!("../../data/2022/16-example.txt");

    match input_parser(input) {
        Ok((remaining_input, valves)) if remaining_input.is_empty() => {
            println!("parsed entire input");
            let mut time_remaining = 30;

            let mut location = ('A', 'A');
            let mut pressure_relieved_per_minute = 0;
            let mut total_pressure_released = 0;
            let mut open_valves = HashSet::new();
            let mut next_steps: VecDeque<Step> = VecDeque::new();
            while time_remaining > 0 {
                total_pressure_released += pressure_relieved_per_minute;
                print!("releasing {} pressure. ", pressure_relieved_per_minute);

                if next_steps.is_empty() {
                    next_steps = find_next_steps(&valves, &open_valves, location, time_remaining);
                    println!("plan length: {}", next_steps.len());
                }

                match next_steps.pop_front().unwrap() {
                    Step::Move(new_location) => {
                        location = new_location;
                        println!("Moving to {:?}", new_location);
                    }
                    Step::OpenValve(room_label) => {
                        let flow_rate = valves
                            .iter()
                            .find(|r| r.label == room_label)
                            .unwrap()
                            .flow_rate;
                        pressure_relieved_per_minute += flow_rate;
                        open_valves.insert(location);
                        println!("Opening valve worth {}", flow_rate);
                    }
                }

                time_remaining -= 1;
            }

            total_pressure_released
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
    let input = include_str!("../../data/2022/16-example.txt");

    match input_parser(input) {
        Ok((remaining_input, valves)) if remaining_input.is_empty() => {
            println!("parsed entire input");

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

fn find_next_steps(
    valves: &[ValveDescription],
    open_valves: &HashSet<Location>,
    current_location: Location,
    time_remaining: usize,
) -> VecDeque<Step> {
    let opportunities =
        // greedy_build_opportunities(valves, open_valves, current_location, time_remaining);
        build_opportunities(valves, open_valves, current_location, time_remaining, 17);
    let best_move = opportunities
        .iter()
        .max_by(|a, b| {
            a.value
                .cmp(&b.value)
                .then_with(|| b.moves.len().cmp(&a.moves.len()))
        })
        .unwrap();

    best_move.moves.to_owned()
}

fn greedy_build_opportunities(
    valves: &[ValveDescription],
    open_valves: &HashSet<Location>,
    current_location: Location,
    time_remaining: usize,
) -> Vec<Opportunity> {
    let mut visited: HashSet<Location> = HashSet::new();
    let mut to_visit: Vec<(Location, Vec<Location>)> = vec![(current_location, vec![])];

    let mut opportunities = vec![];
    while let Some((room_label, path)) = to_visit.pop() {
        if path.len() > time_remaining {
            continue;
        }

        let room = valves.iter().find(|r| r.label == room_label).unwrap();
        let value = if open_valves.contains(&room_label) {
            0
        } else {
            (time_remaining - path.len()) * room.flow_rate
        };

        let mut moves = VecDeque::from_iter(path.iter().map(|&to_room| Step::Move(to_room)));
        moves.push_back(Step::OpenValve(room_label));

        opportunities.push(Opportunity { value, moves });

        for connection in room.connections.iter() {
            if !visited.contains(connection) {
                visited.insert(*connection);
                let mut this_path = path.clone();
                this_path.push(*connection);
                to_visit.push((*connection, this_path))
            }
        }
    }

    opportunities
}

fn build_opportunities(
    valves: &[ValveDescription],
    open_valves: &HashSet<Location>,
    current_location: Location,
    time_remaining: usize,
    max_size: usize,
) -> Vec<Opportunity> {
    let mut to_visit: Vec<(usize, Location, VecDeque<Step>)> =
        vec![(0, current_location, VecDeque::new())];

    let mut opportunities = vec![];
    while let Some((current_value, room_label, moves)) = to_visit.pop() {
        // println!(
        //     "oppoertunities: {} to_visit: {}",
        //     opportunities.len(),
        //     to_visit.len()
        // );
        if moves.len() > time_remaining || moves.len() == max_size {
            opportunities.push(Opportunity {
                value: current_value,
                moves,
            });
            continue;
        }

        let room = valves.iter().find(|r| r.label == room_label).unwrap();
        let room_value = if open_valves.contains(&room_label) {
            0
        } else {
            (time_remaining - moves.len()) * room.flow_rate
        };

        //create possible moves

        //generate open
        if !open_valves.contains(&room_label) && !moves.contains(&Step::OpenValve(room_label)) {
            let mut next_moves = moves.clone();
            next_moves.push_back(Step::OpenValve(room_label));
            to_visit.push((current_value + room_value, room_label, next_moves))
        }

        //generate keep moving
        for connection in room.connections.iter() {
            let mut next_moves = moves.clone();
            next_moves.push_back(Step::Move(*connection));
            to_visit.push((current_value, *connection, next_moves))
        }
    }

    opportunities
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
