use itertools::Itertools;
use nom::{bytes::complete::tag, combinator::map_res, multi::separated_list0, IResult};
use std::collections::HashMap;

pub fn part1() -> isize {
    let input = include_str!("../../data/2021/7.txt");

    match input_parser(input) {
        Ok((remaining_input, crab_positions)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let max = crab_positions.iter().max().unwrap();
            let first_check = 0;
            let mut min_fuel = part1_cost_to_move_to_position(&first_check, &crab_positions);
            for x in 1..(*max + 1) {
                let fuel_cost = part1_cost_to_move_to_position(&x, &crab_positions);
                min_fuel = if fuel_cost < min_fuel {
                    fuel_cost
                } else {
                    min_fuel
                };
            }

            min_fuel
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

pub fn part2() -> isize {
    let input = include_str!("../../data/2021/7.txt");

    match input_parser(input) {
        Ok((remaining_input, crab_positions)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let max = crab_positions.iter().max().unwrap();
            let first_check = 0;
            let mut min_fuel = part2_cost_to_move_to_position(&first_check, &crab_positions);
            for x in 1..(*max + 1) {
                let fuel_cost = part2_cost_to_move_to_position(&x, &crab_positions);
                min_fuel = if fuel_cost < min_fuel {
                    fuel_cost
                } else {
                    min_fuel
                };
            }

            min_fuel
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

fn part1_cost_to_move_to_position(destination: &isize, crab_positions: &[isize]) -> isize {
    crab_positions
        .iter()
        .fold(0, |acc, position| acc + (*position - *destination).abs())
}

fn part2_cost_to_move_to_position(destination: &isize, crab_positions: &[isize]) -> isize {
    crab_positions.iter().fold(0, |acc, position| {
        let distance = (*position - *destination).abs();
        let cost = (distance * (distance + 1)) / 2;
        acc + cost
    })
}

fn input_parser(input: &str) -> IResult<&str, Vec<isize>> {
    let (input, timers) = separated_list0(
        tag(","),
        map_res(nom::character::complete::digit0, |s: &str| {
            s.parse::<isize>()
        }),
    )(input)?;

    let (input, _) = nom::character::complete::newline(input)?;

    Ok((input, timers))
}
