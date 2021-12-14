use itertools::Itertools;
use nom::{bytes::complete::tag, multi::separated_list0, sequence::separated_pair, IResult};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

type Input<'a> = (&'a str, Vec<(&'a str, char)>);

pub fn part1() -> usize {
    let input = include_str!("../../data/2021/14.txt");

    match input_parser(input) {
        Ok((remaining_input, (starting_polymer_template, substitutions)))
            if remaining_input.is_empty() =>
        {
            println!("parsed entire input");

            let mut polymer_template = starting_polymer_template.to_owned();

            for i in 0..10 {
                let mut next_polymer = String::with_capacity(polymer_template.len() * 2);
                next_polymer.push(polymer_template.chars().next().unwrap());
                for (prev, next) in polymer_template.chars().into_iter().tuple_windows() {
                    if let Some(&(_, new)) = substitutions.iter().find(|&(to_replace, _)| {
                        // to_replace.starts_with(&String::from_iter(vec![prev, next]))
                        to_replace.starts_with(prev) && to_replace.chars().nth(1) == Some(next)
                    }) {
                        next_polymer.push(new);
                    }

                    next_polymer.push(next);
                }
                polymer_template = next_polymer.to_owned();
            }

            score(&polymer_template)
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
    let input = include_str!("../../data/2021/14.txt");

    match input_parser(input) {
        Ok((remaining_input, (starting_polymer_template, substitutions)))
            if remaining_input.is_empty() =>
        {
            println!("parsed entire input");

            let mut polymer_template: HashMap<(char, char), usize> = HashMap::new();
            for pair in starting_polymer_template
                .chars()
                .into_iter()
                .tuple_windows()
            {
                upsert(&mut polymer_template, pair, 1);
            }

            for _ in 0..40 {
                let mut next_polymer: HashMap<(char, char), usize> =
                    HashMap::with_capacity(polymer_template.len() * 2);
                for (&(prev, next), count) in polymer_template.iter() {
                    if let Some(&(_, new)) = substitutions.iter().find(|&(to_replace, _)| {
                        // to_replace.starts_with(&String::from_iter(vec![prev, next]))
                        to_replace.starts_with(prev) && to_replace.chars().nth(1) == Some(next)
                    }) {
                        upsert(&mut next_polymer, (prev, new), *count);
                        upsert(&mut next_polymer, (new, next), *count);
                    } else {
                        upsert(&mut next_polymer, (prev, next), *count);
                    }
                }

                polymer_template = next_polymer.to_owned();
            }

            let mut frequencies: HashMap<char, usize> = HashMap::new();
            for ((first, second), count) in polymer_template.iter() {
                upsert_c(&mut frequencies, *first, *count);
                upsert_c(&mut frequencies, *second, *count);
            }
            upsert_c(
                &mut frequencies,
                starting_polymer_template.chars().next().unwrap(),
                1,
            );
            upsert_c(
                &mut frequencies,
                starting_polymer_template.chars().last().unwrap(),
                1,
            );

            let mut sorted_frequencies = frequencies
                .iter()
                .sorted_by(|&(_, val1), &(_, val2)| Ord::cmp(&val1, &val2));
            let fewest = sorted_frequencies.next().unwrap();
            let most = sorted_frequencies.last().unwrap();

            println!(
                "most {} {} fewest {} {}",
                most.0, most.1, fewest.0, fewest.1
            );

            most.1 / 2 - fewest.1 / 2
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

fn upsert(map: &mut HashMap<(char, char), usize>, key: (char, char), val: usize) {
    match map.get(&key) {
        Some(&current_val) => {
            map.insert(key, current_val + val);
        }
        None => {
            map.insert(key, val);
        }
    }
}

fn upsert_c(map: &mut HashMap<char, usize>, key: char, val: usize) {
    match map.get(&key) {
        Some(&current_val) => {
            map.insert(key, current_val + val);
        }
        None => {
            map.insert(key, val);
        }
    }
}

fn score(result: &String) -> usize {
    let mut frequencies: HashMap<char, usize> = HashMap::new();
    for c in result.chars() {
        match frequencies.get(&c) {
            Some(count) => {
                frequencies.insert(c, *count + 1);
            }
            None => {
                frequencies.insert(c, 1);
            }
        }
    }

    let mut sorted_frequencies = frequencies
        .iter()
        .sorted_by(|&(_, val1), &(_, val2)| Ord::cmp(&val1, &val2));
    let fewest = sorted_frequencies.next().unwrap();
    let most = sorted_frequencies.last().unwrap();

    most.1 - fewest.1
}

fn substitution_parser(input: &str) -> IResult<&str, (&str, char)> {
    separated_pair(
        nom::character::complete::alpha1,
        tag(" -> "),
        nom::character::complete::anychar,
    )(input)
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, polymer_template) = nom::character::complete::alpha1(input)?;
    let (input, _) = nom::character::complete::newline(input)?;
    let (input, _) = nom::character::complete::newline(input)?;

    let (input, substitutions) =
        separated_list0(nom::character::complete::newline, substitution_parser)(input)?;

    let (input, _) = nom::character::complete::newline(input)?;

    Ok((input, (polymer_template, substitutions)))
}
