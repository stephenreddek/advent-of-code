use core::num;
use itertools::Itertools;
use nom::{bytes::complete::tag, multi::separated_list0, sequence::separated_pair, IResult};
use std::{
    collections::{HashMap, HashSet},
    ops::RangeBounds,
};

type Input = HashMap<(i32, i32), u32>;

pub fn part1() -> usize {
    let input = include_str!("../../data/2021/11.txt");

    match input_parser(input) {
        Ok((remaining_input, mut state)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut total_flashes = 0;
            // print_state(&state);
            for _ in 0..100 {
                step(&mut state);
                total_flashes += state.iter().filter(|(_, &value)| value == 0).count();
                // print_state(&state);
            }

            total_flashes
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
    let input = include_str!("../../data/2021/11.txt");

    match input_parser(input) {
        Ok((remaining_input, mut state)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut steps = 1;
            // print_state(&state);att
            loop {
                step(&mut state);
                if state.iter().filter(|(_, &value)| value == 0).count() == 100 {
                    break;
                }
                // print_state(&state);
                steps += 1;
            }

            steps
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

fn print_state(state: &Input) {
    for y in 0..10 {
        for x in 0..10 {
            let value = state.get(&(x, y)).unwrap();
            print!("{}", *value);
        }

        println!();
    }

    println!();
    println!();
}

fn step(state: &mut Input) {
    for val in state.values_mut() {
        *val += 1;
    }

    flash(state);

    for val in state.values_mut() {
        if *val > 9 {
            *val = 0;
        }
    }
}

fn flash(state: &mut Input) {
    let mut flashed: HashSet<(i32, i32)> = HashSet::new();
    let to_visit: Vec<(i32, i32)> = state
        .iter()
        .filter_map(|(&position, &value)| if value > 9 { Some(position) } else { None })
        .collect_vec();
    flash_help(state, &to_visit, &mut flashed);
}

fn flash_help(state: &mut Input, to_visit: &Vec<(i32, i32)>, flashed: &mut HashSet<(i32, i32)>) {
    let mut next_visit = Vec::new();
    for position in to_visit.iter() {
        if flashed.contains(position) {
            continue;
        }
        if let Some(value) = state.get_mut(position) {
            *value += 1;
            if *value > 9 {
                push_neighbors(position, &mut next_visit);
                flashed.insert(*position);
            }
        }
    }

    if next_visit.is_empty() {
        return;
    }

    flash_help(state, &next_visit, flashed);
}

fn push_neighbors(position: &(i32, i32), to_visit: &mut Vec<(i32, i32)>) {
    for (x_offset, y_offset) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (1, 0),
        (1, 1),
        (0, 1),
        (1, -1),
        (0, -1),
    ] {
        to_visit.push((position.0 + x_offset, position.1 + y_offset));
    }
}

fn one_digit_parser(input: &str) -> IResult<&str, u32> {
    let (input, digit_char) = nom::character::complete::satisfy(|c| c.is_ascii_digit())(input)?;
    let digit = digit_char.to_digit(10).unwrap();
    Ok((input, digit))
}

fn line_parser(input: &str) -> IResult<&str, Vec<u32>> {
    nom::multi::many0(one_digit_parser)(input)
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, lines) = separated_list0(nom::character::complete::newline, line_parser)(input)?;

    // let (input, _) = nom::character::complete::newline(input)?;

    let mut cave_map: HashMap<(i32, i32), u32> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, value) in line.iter().enumerate() {
            cave_map.insert((x.try_into().unwrap(), y.try_into().unwrap()), *value);
        }
    }

    Ok((input, cave_map))
}
