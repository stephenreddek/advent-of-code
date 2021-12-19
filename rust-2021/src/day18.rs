use itertools::Itertools;
use nom::{
    bytes::complete::tag, combinator::map_res, multi::separated_list1, sequence::separated_pair,
    IResult,
};
use std::ops;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Clone, Debug, PartialEq)]
enum SnailfishNumber {
    Literal(i32),
    Pair(Box<(SnailfishNumber, SnailfishNumber)>),
}

pub fn part1() -> i32 {
    let input = include_str!("../../data/2021/18.txt");

    match input_parser(input) {
        Ok((remaining_input, input_numbers)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let result = sum(input_numbers);

            magnitude(result)
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

pub fn part2() -> i32 {
    let input = include_str!("../../data/2021/18.txt");

    match input_parser(input) {
        Ok((remaining_input, input_numbers)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut max_magnitude = 0;
            for mut pair in input_numbers.into_iter().combinations(2) {
                let in_order = magnitude(sum(pair.clone()));
                if in_order > max_magnitude {
                    max_magnitude = in_order;
                }

                pair.reverse();
                let reversed = magnitude(sum(pair));
                if reversed > max_magnitude {
                    max_magnitude = reversed;
                }
            }

            max_magnitude
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

fn sum(numbers: Vec<SnailfishNumber>) -> SnailfishNumber {
    let result = numbers.into_iter().reduce(|a, b| add(a, b));

    result.unwrap()
}

fn magnitude(number: SnailfishNumber) -> i32 {
    match number {
        SnailfishNumber::Literal(x) => x,
        SnailfishNumber::Pair(pair) => 3 * magnitude(pair.0) + 2 * magnitude(pair.1),
    }
}

fn add(left: SnailfishNumber, right: SnailfishNumber) -> SnailfishNumber {
    reduce(SnailfishNumber::Pair(Box::new((left, right))))
}

fn reduce(number: SnailfishNumber) -> SnailfishNumber {
    let mut result = number;
    loop {
        if let Some(explode_result) = try_explode(result.clone()) {
            result = explode_result;
        } else if let Some(split_result) = try_split(result.clone()) {
            result = split_result;
        } else {
            return result;
        }
    }
}

enum ExplodeResult {
    Exploded(i32, i32),
    LeftExplodedReplaceRightWith(i32),
    RightExplodedReplaceLeftWith(i32),
    NoExplosion,
    ExplosionHandled,
}

fn build_pair(left: SnailfishNumber, right: SnailfishNumber) -> SnailfishNumber {
    SnailfishNumber::Pair(Box::new((left, right)))
}

fn try_explode(number: SnailfishNumber) -> Option<SnailfishNumber> {
    match try_explode_help(number, 0) {
        (_, ExplodeResult::NoExplosion) => None,
        (result, _) => Some(result),
    }
}

fn try_explode_help(number: SnailfishNumber, depth: usize) -> (SnailfishNumber, ExplodeResult) {
    match number {
        SnailfishNumber::Literal(value) => (number, ExplodeResult::NoExplosion),
        SnailfishNumber::Pair(pair) => {
            if depth == 4 {
                //explode
                //panic if the left or right is a pair
                match *pair {
                    (SnailfishNumber::Literal(left), SnailfishNumber::Literal(right)) => (
                        SnailfishNumber::Literal(0),
                        ExplodeResult::Exploded(left, right),
                    ),
                    _ => {
                        panic!("At depth 4, the left or right value was not a literal")
                    }
                }
            } else {
                let left_result = try_explode_help(pair.0.clone(), depth + 1);
                match left_result.1 {
                    ExplodeResult::Exploded(exploded_left, exploded_right) => (
                        build_pair(
                            SnailfishNumber::Literal(0),
                            add_to_far_left(pair.1, exploded_right),
                        ),
                        ExplodeResult::LeftExplodedReplaceRightWith(exploded_left),
                    ),
                    ExplodeResult::LeftExplodedReplaceRightWith(exploded_left) => (
                        //cannot handle
                        build_pair(left_result.0, pair.1),
                        ExplodeResult::LeftExplodedReplaceRightWith(exploded_left),
                    ),
                    ExplodeResult::RightExplodedReplaceLeftWith(exploded_right) => (
                        //can handle
                        build_pair(left_result.0, add_to_far_left(pair.1, exploded_right)),
                        ExplodeResult::ExplosionHandled,
                    ),
                    ExplodeResult::ExplosionHandled => (
                        build_pair(left_result.0, pair.1),
                        ExplodeResult::ExplosionHandled,
                    ),
                    ExplodeResult::NoExplosion => {
                        let right_result = try_explode_help(pair.1.clone(), depth + 1);
                        match right_result.1 {
                            ExplodeResult::Exploded(exploded_left, exploded_right) => (
                                build_pair(
                                    add_to_far_right(pair.0, exploded_left),
                                    SnailfishNumber::Literal(0),
                                ),
                                ExplodeResult::RightExplodedReplaceLeftWith(exploded_right),
                            ),
                            ExplodeResult::LeftExplodedReplaceRightWith(old_left) => {
                                //can handle
                                (
                                    build_pair(add_to_far_right(pair.0, old_left), right_result.0),
                                    ExplodeResult::ExplosionHandled,
                                )
                            }
                            ExplodeResult::RightExplodedReplaceLeftWith(old_right) => {
                                //cannot handle
                                (
                                    build_pair(pair.0, right_result.0),
                                    ExplodeResult::RightExplodedReplaceLeftWith(old_right),
                                )
                            }
                            ExplodeResult::ExplosionHandled => (
                                build_pair(pair.0, right_result.0),
                                ExplodeResult::ExplosionHandled,
                            ),
                            ExplodeResult::NoExplosion => (
                                SnailfishNumber::Pair(Box::new(*pair)),
                                ExplodeResult::NoExplosion,
                            ),
                        }
                    }
                }
            }
        }
    }
}

fn add_to_far_left(number: SnailfishNumber, value: i32) -> SnailfishNumber {
    match number {
        SnailfishNumber::Literal(x) => SnailfishNumber::Literal(x + value),
        SnailfishNumber::Pair(pair) => build_pair(add_to_far_left(pair.0, value), pair.1),
    }
}

fn add_to_far_right(number: SnailfishNumber, value: i32) -> SnailfishNumber {
    match number {
        SnailfishNumber::Literal(x) => SnailfishNumber::Literal(x + value),
        SnailfishNumber::Pair(pair) => build_pair(pair.0, add_to_far_right(pair.1, value)),
    }
}

fn try_split(number: SnailfishNumber) -> Option<SnailfishNumber> {
    match number {
        SnailfishNumber::Literal(value) if value > 9 => Some(SnailfishNumber::Pair(Box::new((
            SnailfishNumber::Literal((value as f32 / 2_f32).floor() as i32),
            SnailfishNumber::Literal((value as f32 / 2_f32).ceil() as i32),
        )))),
        SnailfishNumber::Literal(value) => None,
        SnailfishNumber::Pair(pair) => match (try_split(pair.0.clone()), try_split(pair.1.clone()))
        {
            (Some(left), _) => Some(SnailfishNumber::Pair(Box::new((left, pair.1)))),
            (None, Some(right)) => Some(SnailfishNumber::Pair(Box::new((pair.0, right)))),
            (None, None) => None,
        },
    }
}

fn parse_number(input: &str) -> IResult<&str, SnailfishNumber> {
    let (input, maybe_negative) = nom::combinator::opt(tag("-"))(input)?;
    let (input, val) =
        map_res(nom::character::complete::digit1, |s: &str| s.parse::<i32>())(input)?;

    Ok((
        input,
        if maybe_negative.is_some() {
            SnailfishNumber::Literal(-val)
        } else {
            SnailfishNumber::Literal(val)
        },
    ))
}

fn nested_pair_parser(input: &str) -> IResult<&str, SnailfishNumber> {
    let (input, _) = tag("[")(input)?;
    let (input, result) =
        separated_pair(snailfish_value_parser, tag(","), snailfish_value_parser)(input)?;
    let (input, _) = tag("]")(input)?;

    Ok((input, SnailfishNumber::Pair(Box::new(result))))
}

fn snailfish_value_parser(input: &str) -> IResult<&str, SnailfishNumber> {
    nom::branch::alt((parse_number, nested_pair_parser))(input)
}

fn input_parser(input: &str) -> IResult<&str, Vec<SnailfishNumber>> {
    let (input, numbers) =
        separated_list1(nom::character::complete::newline, nested_pair_parser)(input)?;

    let (input, _) = nom::character::complete::newline(input)?;

    Ok((input, numbers))
}

#[cfg(test)]
mod day18_tests {
    use crate::day18::snailfish_value_parser;
    use crate::day18::SnailfishNumber;

    #[test]
    fn can_parse_literal_value() {
        let test_string = "[1,2]";
        let result = snailfish_value_parser(test_string);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().1,
            SnailfishNumber::Pair(Box::new((
                SnailfishNumber::Literal(1),
                SnailfishNumber::Literal(2)
            )))
        );
    }

    #[test]
    fn can_parse_nested_value() {
        let test_string = "[[9,3],[[9,9],[6,[4,9]]]]";
        let result = snailfish_value_parser(test_string);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().1,
            SnailfishNumber::Pair(Box::new((
                SnailfishNumber::Pair(Box::new((
                    SnailfishNumber::Literal(9),
                    SnailfishNumber::Literal(3)
                ))),
                SnailfishNumber::Pair(Box::new((
                    SnailfishNumber::Pair(Box::new((
                        SnailfishNumber::Literal(9),
                        SnailfishNumber::Literal(9)
                    ))),
                    SnailfishNumber::Pair(Box::new((
                        SnailfishNumber::Literal(6),
                        SnailfishNumber::Pair(Box::new((
                            SnailfishNumber::Literal(4),
                            SnailfishNumber::Literal(9)
                        )))
                    )))
                )))
            )))
        );
    }
}
