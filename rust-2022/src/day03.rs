use itertools::Itertools;
use nom::{combinator::map_res, multi::many1, IResult};
use std::{collections::HashSet, hash::Hash};

fn intersection<T: Eq + Hash>(a: HashSet<T>, b: &HashSet<T>) -> HashSet<T> {
    a.into_iter().filter(|e| b.contains(e)).collect()
}

pub fn part1() -> u32 {
    let input = include_str!("../../data/2022/03.txt");

    match input_parser(input) {
        Ok((remaining_input, rucksacks)) if remaining_input.is_empty() => {
            println!("parsed entire input");
            rucksacks
                .iter()
                .map(|rucksack| {
                    rucksack
                        .0
                        .intersection(&rucksack.1)
                        .map(value_of_item)
                        .sum::<u32>()
                })
                .sum()
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
    let input = include_str!("../../data/2022/03.txt");

    match input_parser(input) {
        Ok((remaining_input, rucksacks)) if remaining_input.is_empty() => {
            println!("parsed entire input");
            let groups = rucksacks.chunks(3).collect_vec();

            let mut total_score: u32 = 0;

            for group in groups {
                let mut group_intersection = HashSet::new();
                group.into_iter().for_each(|sack| {
                    let mut entire_sack = HashSet::new();
                    entire_sack.extend(sack.0.iter());
                    entire_sack.extend(sack.1.iter());

                    if group_intersection.is_empty() {
                        group_intersection = entire_sack.clone()
                    } else {
                        group_intersection.retain(|item| entire_sack.contains(item))
                    }
                });

                let group_badge = group_intersection.into_iter().next().unwrap();
                let group_score = value_of_item(&group_badge);
                total_score += group_score
            }
            total_score
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

fn value_of_item(&item: &char) -> u32 {
    if item.is_ascii_lowercase() {
        item as u32 - 'a' as u32 + 1
    } else {
        item as u32 - 'A' as u32 + 27
    }
}

fn rucksack_parser(input: &str) -> IResult<&str, (HashSet<char>, HashSet<char>)> {
    let (input, items) = nom::character::complete::alpha1(input)?;
    let (input, _) = nom::character::complete::newline(input)?;
    let per_compartment = items.len() / 2;
    let left = HashSet::from_iter(items.chars().take(per_compartment));
    let right = HashSet::from_iter(items.chars().skip(per_compartment));
    // println!("left {:?} right {:?}", left, right);
    // println!("left len {} right len {}", left.len(), right.len());
    Ok((input, (left, right)))
}

fn input_parser(input: &str) -> IResult<&str, Vec<(HashSet<char>, HashSet<char>)>> {
    let (input, lines) = many1(rucksack_parser)(input)?;

    Ok((input, lines))
}
