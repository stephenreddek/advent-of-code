use itertools::Itertools;
use nom::{bytes::complete::tag, combinator::map_res, multi::separated_list0, IResult};
use std::collections::HashMap;

pub fn part1() -> usize {
    let input = include_str!("../../data/2021/6.txt");

    match input_parser(input) {
        Ok((remaining_input, initial_state)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut state: Vec<usize> = initial_state;
            for _ in 0..80 {
                state =
                    state
                        .iter()
                        .fold(Vec::with_capacity(state.capacity()), |mut acc, timer| {
                            if *timer == 0 {
                                acc.push(6);
                                acc.push(8);
                            } else {
                                acc.push(*timer - 1);
                            }
                            acc
                        });
            }

            state.len()
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
    let input = include_str!("../../data/2021/6.txt");

    match input_parser(input) {
        Ok((remaining_input, initial_state)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut state: HashMap<usize, usize> = initial_state
                .iter()
                .sorted()
                .group_by(|x| **x)
                .into_iter()
                .map(|(value, group)| (value, group.into_iter().count()))
                .collect();
            for _ in 0..256 {
                state = state
                    .iter()
                    .fold(HashMap::new(), |mut acc, (&value, &count)| {
                        if value == 0 {
                            upsert(&mut acc, 6, count, |current_val| current_val + count);
                            upsert(&mut acc, 8, count, |current_val| current_val + count);
                        } else {
                            upsert(&mut acc, value - 1, count, |current_val| {
                                current_val + count
                            });
                        }
                        acc
                    });
            }

            state.iter().fold(0, |acc, (_, &count)| acc + count)
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

fn upsert(map: &mut HashMap<usize, usize>, key: usize, val: usize, f: impl Fn(usize) -> usize) {
    match map.get(&key) {
        Some(&current_val) => {
            map.insert(key, f(current_val));
        }
        None => {
            map.insert(key, val);
        }
    }
}

fn input_parser(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, timers) = separated_list0(
        tag(","),
        map_res(nom::character::complete::digit0, |s: &str| {
            s.parse::<usize>()
        }),
    )(input)?;

    let (input, _) = nom::character::complete::newline(input)?;

    Ok((input, timers))
}
