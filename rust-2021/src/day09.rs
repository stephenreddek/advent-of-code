use core::num;
use itertools::Itertools;
use nom::{bytes::complete::tag, multi::separated_list0, sequence::separated_pair, IResult};
use std::collections::{HashMap, HashSet};

type Input = HashMap<(i32, i32), u32>;

pub fn part1() -> u32 {
    let input = include_str!("../../data/2021/9.txt");

    match input_parser(input) {
        Ok((remaining_input, cave_map)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            cave_map.iter().fold(0, |acc, (position, position_value)| {
                if is_local_minimum(&cave_map, position, position_value) {
                    acc + *position_value + 1
                } else {
                    acc
                }
            })
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
    let input = include_str!("../../data/2021/9.txt");

    match input_parser(input) {
        Ok((remaining_input, cave_map)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut sets = cave_map
                .iter()
                .filter_map(|(position, &value)| {
                    if value < 9 {
                        Some(HashSet::from([*position]))
                    } else {
                        None
                    }
                })
                .collect_vec();

            let valid_spaces: Vec<(i32, i32)> = cave_map
                .iter()
                .filter_map(
                    |(position, &value)| {
                        if value < 9 {
                            Some(*position)
                        } else {
                            None
                        }
                    },
                )
                .collect_vec();

            for &position in valid_spaces.iter() {
                if let Some(my_set_index) = index_of_set(position, &sets) {
                    let mut other = None;
                    for set_index in 0..sets.len() {
                        if set_index != my_set_index && adjacent_to_set(position, &sets[set_index])
                        {
                            other = Some(set_index);
                            break;
                        }
                    }

                    if let Some(set_index) = other {
                        let other = sets[set_index].clone();
                        sets[my_set_index].extend(other);
                        sets.swap_remove(set_index);
                    }
                }
            }

            for position in valid_spaces {
                if let Some(my_set_index) = index_of_set(position, &sets) {
                    let mut other = None;
                    for set_index in 0..sets.len() {
                        if set_index != my_set_index && adjacent_to_set(position, &sets[set_index])
                        {
                            other = Some(set_index);
                            break;
                        }
                    }

                    if let Some(set_index) = other {
                        let other = sets[set_index].clone();
                        sets[my_set_index].extend(other);
                        sets.swap_remove(set_index);
                    }
                }
            }

            sets.iter()
                .map(|s| TryInto::<u32>::try_into(s.len()).unwrap())
                .sorted()
                .rev()
                .take(3)
                .product()
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

fn index_of_set(position: (i32, i32), sets: &Vec<HashSet<(i32, i32)>>) -> Option<usize> {
    for (set_index, set) in sets.iter().enumerate() {
        if set.contains(&position) {
            return Some(set_index);
        }
    }

    None
}

fn adjacent_to_set(position: (i32, i32), set: &HashSet<(i32, i32)>) -> bool {
    let adjacent_spaces = [(-1, 0), (0, 1), (0, -1), (1, 0)];
    for (x_offset, y_offset) in adjacent_spaces {
        if set.contains(&(position.0 + x_offset, position.1 + y_offset)) {
            return true;
        }
    }

    false
}

fn is_local_minimum(
    map: &HashMap<(i32, i32), u32>,
    position: &(i32, i32),
    position_value: &u32,
) -> bool {
    let adjacent_spaces = [(-1, 0), (0, 1), (0, -1), (1, 0)];
    for (x_offset, y_offset) in adjacent_spaces {
        if let Some(value) = map.get(&(position.0 + x_offset, position.1 + y_offset)) {
            if *value <= *position_value {
                return false;
            }
        }
    }

    true
}

fn belongs_to_set(set: &HashSet<(i32, i32)>, position: &(i32, i32)) -> bool {
    let adjacent_spaces = [(-1, 0), (0, 1), (0, -1), (1, 0)];
    for (x_offset, y_offset) in adjacent_spaces {
        if set.contains(&(position.0 + x_offset, position.1 + y_offset)) {
            return true;
        }
    }

    false
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
