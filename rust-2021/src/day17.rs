use itertools::Itertools;
use nom::{
    bytes::complete::tag, combinator::map_res, multi::separated_list1, sequence::separated_pair,
    IResult,
};
use std::cmp::Ordering;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, Clone, Copy)]
struct State {
    probe_position: (i32, i32),
    velocity: (i32, i32),
    peak_y: i32,
}

impl State {
    fn with_velocity(velocity: (i32, i32)) -> State {
        State {
            probe_position: (0, 0),
            velocity: velocity,
            peak_y: 0,
        }
    }
}

type TargetArea = ((i32, i32), (i32, i32));

enum ProbeResult {
    Hit(i32),
    Miss,
}

pub fn part1() -> i32 {
    let input = include_str!("../../data/2021/17.txt");

    match input_parser(input) {
        Ok((remaining_input, target_area)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut potential_velocities: Vec<(i32, i32)> = Vec::new();

            for x in 1..1000 {
                for y in 0..1000 {
                    potential_velocities.push((x, y))
                }
            }

            let mut velocities_that_hit: Vec<((i32, i32), i32)> = Vec::new();
            for velocity in potential_velocities.iter() {
                if let ProbeResult::Hit(peak) =
                    step_until_result(State::with_velocity(*velocity), target_area)
                {
                    velocities_that_hit.push((*velocity, peak))
                }
            }

            let velocity_with_max = velocities_that_hit.iter().max_by(|&a, &b| a.1.cmp(&b.1));

            velocity_with_max.unwrap().1
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
    let input = include_str!("../../data/2021/17.txt");

    match input_parser(input) {
        Ok((remaining_input, target_area)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut potential_velocities: Vec<(i32, i32)> = Vec::new();

            for x in 1..1000 {
                for y in -5000..5000 {
                    potential_velocities.push((x, y))
                }
            }
            // let mut velocities_that_hit = Vec::new();
            let mut velocities_that_hit: usize = 0;
            for velocity in potential_velocities.iter() {
                if let ProbeResult::Hit(peak) =
                    step_until_result(State::with_velocity(*velocity), target_area)
                {
                    // velocities_that_hit.push(*velocity)
                    velocities_that_hit += 1;
                }
            }

            // velocities_that_hit.len()
            velocities_that_hit
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

fn step_until_result(state: State, target_area: TargetArea) -> ProbeResult {
    let mut next_state = state;
    loop {
        next_state = step_probe(next_state);
        if is_in_target_area(next_state, target_area) {
            return ProbeResult::Hit(next_state.peak_y);
        } else if is_past_target_area(next_state, target_area) {
            return ProbeResult::Miss;
        }
    }
}

fn is_in_target_area(state: State, target_area: TargetArea) -> bool {
    state.probe_position.0 >= target_area.0 .0
        && state.probe_position.0 <= target_area.0 .1
        && state.probe_position.1 >= target_area.1 .0
        && state.probe_position.1 <= target_area.1 .1
}

fn is_past_target_area(state: State, target_area: TargetArea) -> bool {
    state.probe_position.0 > target_area.0 .1 || state.probe_position.1 < target_area.1 .0
}

fn step_probe(state: State) -> State {
    let next_position = (
        state.probe_position.0 + state.velocity.0,
        state.probe_position.1 + state.velocity.1,
    );

    let next_velocity = (
        match state.velocity.0.cmp(&0) {
            Ordering::Greater => state.velocity.0 - 1,
            Ordering::Equal => state.velocity.0,
            Ordering::Less => state.velocity.0 + 1,
        },
        state.velocity.1 - 1,
    );

    State {
        probe_position: next_position,
        velocity: next_velocity,
        peak_y: state.peak_y.max(next_position.1),
    }
}

fn parse_number(input: &str) -> IResult<&str, i32> {
    let (input, maybe_negative) = nom::combinator::opt(tag("-"))(input)?;
    let (input, val) =
        map_res(nom::character::complete::digit1, |s: &str| s.parse::<i32>())(input)?;

    Ok((input, if maybe_negative.is_some() { -val } else { val }))
}

fn input_parser(input: &str) -> IResult<&str, ((i32, i32), (i32, i32))> {
    let (input, _) = tag("target area: x=")(input)?;
    let (input, x_start) = parse_number(input)?;
    let (input, _) = tag("..")(input)?;
    let (input, x_end) = parse_number(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y_start) = parse_number(input)?;
    let (input, _) = tag("..")(input)?;
    let (input, y_end) = parse_number(input)?;

    let (input, _) = nom::character::complete::newline(input)?;

    Ok((input, ((x_start, x_end), (y_start, y_end))))
}
