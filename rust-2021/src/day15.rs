use itertools::Itertools;
use nom::{bytes::complete::tag, multi::separated_list1, sequence::separated_pair, IResult};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

type Input = ((i32, i32), HashMap<(i32, i32), u32>);

pub fn part1() -> u32 {
    let input = include_str!("../../data/2021/15.txt");

    match input_parser(input) {
        Ok((remaining_input, ((width, height), grid))) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut cost_to_location: HashMap<(i32, i32), u32> = HashMap::with_capacity(grid.len());

            cost_to_location.insert((0, 0), 0);

            let mut to_visit: HashSet<(i32, i32)> = HashSet::new();

            push_neighbors(&(0, 0), &mut to_visit);

            build_cost(grid, &mut cost_to_location, &to_visit);

            let cost_of_ending_position = cost_to_location.get(&(width - 1, height - 1)).unwrap();
            *cost_of_ending_position
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

pub fn part2() -> u32 {
    let input = include_str!("../../data/2021/15.txt");

    match input_parser(input) {
        Ok((remaining_input, ((width, height), grid))) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut expanded_grid: HashMap<(i32, i32), u32> =
                HashMap::with_capacity(grid.len() * 25);
            for expanded_x in 0..5 {
                for expanded_y in 0..5 {
                    for (&position, &value) in grid.iter() {
                        let new_value = (value + expanded_x + expanded_y - 1) % 9 + 1;

                        let x_offset: i32 = expanded_x.try_into().unwrap();
                        let y_offset: i32 = expanded_y.try_into().unwrap();

                        let new_x: i32 = position.0 + (width * x_offset);
                        let new_y: i32 = position.1 + (height * y_offset);
                        expanded_grid.insert((new_x, new_y), new_value);
                    }
                }
            }

            let mut cost_to_location: HashMap<(i32, i32), u32> =
                HashMap::with_capacity(expanded_grid.len());

            cost_to_location.insert((0, 0), 0);

            let mut to_visit: HashSet<(i32, i32)> = HashSet::new();

            push_neighbors(&(0, 0), &mut to_visit);

            build_cost(expanded_grid, &mut cost_to_location, &to_visit);

            let cost_of_ending_position = cost_to_location
                .get(&(width * 5 - 1, height * 5 - 1))
                .unwrap();
            *cost_of_ending_position
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

fn build_cost(
    map: HashMap<(i32, i32), u32>,
    cost: &mut HashMap<(i32, i32), u32>,
    to_visit: &HashSet<(i32, i32)>,
) {
    let mut next_visit = HashSet::new();
    for position in to_visit.iter() {
        if let Some(value) = map.get(position) {
            if let Some(cost_to_get_here) = lesser_path_to(position, cost) {
                let prev_value = cost.get(position);
                let calculated_cost = *cost_to_get_here + *value;

                match prev_value {
                    Some(&previous_cost) => {
                        if calculated_cost < previous_cost {
                            push_neighbors(position, &mut next_visit);
                        }
                    }
                    None => {
                        push_neighbors(position, &mut next_visit);
                    }
                }

                cost.insert(*position, calculated_cost);
            }
        }
    }

    if next_visit.is_empty() {
        return;
    }

    build_cost(map, cost, &next_visit);
}

fn lesser_path_to<'a>(
    position: &(i32, i32),
    cost: &'a HashMap<(i32, i32), u32>,
) -> Option<&'a u32> {
    let neighbors = [(-1, 0), (1, 0), (0, 1), (0, -1)];

    neighbors
        .iter()
        .filter_map(|&(x_offset, y_offset)| {
            cost.get(&(position.0 + x_offset, position.1 + y_offset))
        })
        .min()
}

fn push_neighbors(position: &(i32, i32), to_visit: &mut HashSet<(i32, i32)>) {
    for (x_offset, y_offset) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
        to_visit.insert((position.0 + x_offset, position.1 + y_offset));
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
    let (input, lines) = separated_list1(nom::character::complete::newline, line_parser)(input)?;

    // let (input, _) = nom::character::complete::newline(input)?;

    let mut cave_map: HashMap<(i32, i32), u32> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, value) in line.iter().enumerate() {
            cave_map.insert((x.try_into().unwrap(), y.try_into().unwrap()), *value);
        }
    }

    let width: i32 = lines[0].len().try_into().unwrap();
    let height: i32 = lines.len().try_into().unwrap();

    //there's an extra, empty line at the bottom
    Ok((input, ((width, height - 1), cave_map)))
}
