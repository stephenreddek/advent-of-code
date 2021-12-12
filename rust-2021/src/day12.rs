use itertools::Itertools;
use nom::{bytes::complete::tag, multi::separated_list0, sequence::separated_pair, IResult};
use std::collections::{HashMap, HashSet};

type Input<'a, 'b> = Vec<(&'a str, &'b str)>;

pub fn part1() -> usize {
    let input = include_str!("../../data/2021/12.txt");

    match input_parser(input) {
        Ok((remaining_input, connections)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let starting_place = "start";
            let current_path: Vec<String> = vec![starting_place.to_owned()];

            let paths = expand_path(&current_path, &connections);

            // print_paths(&paths);

            paths.len()
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
    let input = include_str!("../../data/2021/12.txt");

    match input_parser(input) {
        Ok((remaining_input, connections)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let starting_place = "start";
            let current_path: Vec<String> = vec![starting_place.to_owned()];

            let paths = part_2_expand_path(&current_path, &connections);

            // print_paths(&paths);

            paths.len()
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

fn print_paths(paths: &[Vec<String>]) {
    for path in paths {
        println!("{:?}", path.join("->"));
        println!();
    }
}

fn expand_path(path: &[String], connections: &[(&str, &str)]) -> Vec<Vec<String>> {
    let place = path.last().unwrap();
    let possible_connections = connections
        .iter()
        .filter_map(|&(end1, end2)| {
            if end1 == place {
                Some((end1, end2))
            } else if end2 == place {
                Some((end2, end1))
            } else {
                None
            }
        })
        .filter(|&(_end1, end2)| {
            if end2.chars().next().unwrap().is_uppercase() {
                true
            } else {
                !path.contains(&end2.to_owned())
            }
        });

    let mut all_paths: Vec<Vec<String>> = Vec::new();
    for connection in possible_connections {
        let mut current_path: Vec<String> = path.to_owned();
        current_path.push(connection.1.to_owned());

        if connection.1 == "end" {
            all_paths.push(current_path.to_owned());
            continue;
        }

        let paths = expand_path(&current_path, connections);
        all_paths.extend(paths);
    }

    all_paths
}

fn part_2_expand_path(path: &[String], connections: &[(&str, &str)]) -> Vec<Vec<String>> {
    let place = path.last().unwrap();
    let possible_connections = connections
        .iter()
        .filter_map(|&(end1, end2)| {
            if end1 == place {
                Some((end1, end2))
            } else if end2 == place {
                Some((end2, end1))
            } else {
                None
            }
        })
        .filter(|&(_end1, end2)| {
            if end2 == "start" {
                return false;
            }

            if end2.chars().next().unwrap().is_uppercase() {
                true
            } else {
                !contains_small_duplicates(path) || !path.contains(&end2.to_owned())
            }
        });

    let mut all_paths: Vec<Vec<String>> = Vec::new();
    for connection in possible_connections {
        let mut current_path: Vec<String> = path.to_owned();
        current_path.push(connection.1.to_owned());

        if connection.1 == "end" {
            all_paths.push(current_path.to_owned());
            continue;
        }

        let paths = part_2_expand_path(&current_path, connections);
        all_paths.extend(paths);
    }

    all_paths
}

fn contains_small_duplicates(path: &[String]) -> bool {
    let mut seen: HashSet<String> = HashSet::new();
    for cave in path {
        if cave != "star" && cave.chars().next().unwrap().is_lowercase() {
            if seen.get(cave).is_some() {
                return true;
            }

            seen.insert(cave.to_owned());
        }
    }

    false
}

fn line_parser(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(
        nom::character::complete::alpha1,
        tag("-"),
        nom::character::complete::alpha1,
    )(input)
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, paths) = separated_list0(nom::character::complete::newline, line_parser)(input)?;

    let (input, _) = nom::character::complete::newline(input)?;

    Ok((input, paths))
}
