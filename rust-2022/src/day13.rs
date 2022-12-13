use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map_res, opt},
    multi::separated_list0,
    multi::{many1, separated_list1},
    IResult,
};
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};
use std::{cmp::Ordering, hash::Hash};

type Input = Vec<(Packet, Packet)>;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Value(usize),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::List(self_list), Packet::List(other_list)) => {
                for (left, right) in self_list.iter().zip(other_list) {
                    if left < right || right < left {
                        return left.cmp(right);
                    }
                }

                self_list.len().cmp(&other_list.len())
            }
            (Packet::List(_), Packet::Value(other_value)) => {
                self.cmp(&Packet::List(vec![Packet::Value(*other_value)]))
            }
            (Packet::Value(self_value), Packet::List(_)) => {
                Packet::List(vec![Packet::Value(*self_value)]).cmp(other)
            }
            (Packet::Value(self_value), Packet::Value(other_value)) => self_value.cmp(other_value),
        }
        // other
        //     .cost
        //     .cmp(&self.cost)
        //     .then_with(|| other.distance_to_finish.cmp(&self.distance_to_finish))
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1() -> usize {
    let input = include_str!("../../data/2022/13.txt");

    match part_1_input_parser(input) {
        Ok((remaining_input, packets)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut sum = 0;
            for (index, (left, right)) in packets.iter().enumerate() {
                // println!("{} : {}", index, left < right);
                if left < right {
                    sum += index + 1;
                }
            }

            sum
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
    let input = include_str!("../../data/2022/13.txt");

    match part2_input_parser(input) {
        Ok((remaining_input, mut packets)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let first_divider = Packet::List(vec![Packet::List(vec![Packet::Value(2)])]);
            let second_divider = Packet::List(vec![Packet::List(vec![Packet::Value(6)])]);

            packets.push(first_divider.clone());
            packets.push(second_divider.clone());

            let mut result = 1;

            for (index, packet) in packets.iter().sorted().enumerate() {
                if *packet == first_divider || *packet == second_divider {
                    result *= index + 1;
                }
            }

            result
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

fn list_parser(input: &str) -> IResult<&str, Packet> {
    let (input, _) = tag("[")(input)?;
    let (input, contents) =
        separated_list0(nom::character::complete::char(','), packet_parser)(input)?;
    let (input, _) = tag("]")(input)?;

    Ok((input, Packet::List(contents)))
}

fn value_parser(input: &str) -> IResult<&str, Packet> {
    let (input, value) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<usize>()
    })(input)?;

    Ok((input, Packet::Value(value)))
}

fn packet_parser(input: &str) -> IResult<&str, Packet> {
    nom::branch::alt((list_parser, value_parser))(input)
}

fn pair_parser(input: &str) -> IResult<&str, (Packet, Packet)> {
    let (input, first) = packet_parser(input)?;
    let (input, _) = nom::character::complete::newline(input)?;
    let (input, second) = packet_parser(input)?;

    Ok((input, (first, second)))
}

fn part_1_input_parser(input: &str) -> IResult<&str, Input> {
    let (input, rows) = separated_list1(tag("\n\n"), pair_parser)(input)?;

    Ok((input, rows))
}

fn part2_input_parser(input: &str) -> IResult<&str, Vec<Packet>> {
    let (input, pairs) = part_1_input_parser(input)?;

    let rows = pairs
        .iter()
        .flat_map(|(left, right)| vec![left.clone(), right.clone()])
        .collect_vec();

    Ok((input, rows))
}

#[cfg(test)]
mod day13_tests {
    use crate::day13::Packet;

    #[test]
    fn test_value_ordering() {
        assert!(Packet::Value(3) < Packet::Value(4));
    }

    #[test]
    fn test_list_length_ordering() {
        assert!(Packet::List(vec![]) < Packet::List(vec![Packet::Value(4)]));
    }

    #[test]
    fn test_list_length_ordering_backup_shorter() {
        assert!(
            Packet::List(vec![Packet::Value(4)])
                < Packet::List(vec![Packet::Value(4), Packet::Value(4)])
        );
    }

    #[test]
    fn test_list_length_ordering_backup_equal_length() {
        assert!(Packet::List(vec![Packet::Value(4)]) == Packet::List(vec![Packet::Value(4)]));
    }

    #[test]
    fn test_value_against_list() {
        assert!(Packet::Value(4) < Packet::List(vec![Packet::Value(5)]));
    }

    #[test]
    fn test_lists_of_different_length() {
        assert!(
            Packet::List(vec![Packet::Value(9)])
                > Packet::List(vec![Packet::List(vec![
                    Packet::Value(8),
                    Packet::Value(7),
                    Packet::Value(6)
                ])])
        );
    }
}
