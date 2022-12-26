use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map_res, opt},
    multi::separated_list0,
    multi::{many1, separated_list1},
    IResult,
};
use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use std::{collections::BinaryHeap, time::SystemTime};
use std::{
    collections::{HashMap, HashSet},
    ptr,
};
use std::{fmt, ops};

type Input = Vec<SnafuNumber>;

type SnafuNumber = Vec<SnafuDigit>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum SnafuDigit {
    Positive(isize),
    Negative1,
    Negative2,
}

pub fn part1() -> isize {
    let input = include_str!("../../data/2022/25.txt");
    // let input = include_str!("../../data/2022/25-example.txt");

    match input_parser(input) {
        Ok((remaining_input, numbers)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut sum = 0;

            for snafu in numbers {
                let this_value = snafu_to_base_10(&snafu);
                // println!("{}", this_value);
                sum += this_value;
            }

            for number in base_10_to_snafu(sum) {
                match number {
                    SnafuDigit::Positive(n) => print!("{n}"),
                    SnafuDigit::Negative1 => print!("-"),
                    SnafuDigit::Negative2 => print!("="),
                }
            }
            println!();

            sum
        }
        Ok((remaining, _)) => {
            println!("remaining unparsed \"{remaining}\"");
            0
        }
        Err(e) => {
            println!("error parsing \"{e}\"");
            0
        }
    }
}

pub fn part2() -> isize {
    // let input = include_str!("../../data/2022/25.txt");
    let input = include_str!("../../data/2022/25-example.txt");

    match input_parser(input) {
        Ok((remaining_input, numbers)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            0
        }
        Ok((remaining, _)) => {
            println!("remaining unparsed \"{remaining}\"");
            0
        }
        Err(e) => {
            println!("error parsing \"{e}\"");
            0
        }
    }
}

fn snafu_to_base_10(snafu: &Vec<SnafuDigit>) -> isize {
    let mut position_value = 1;
    let mut value = 0;
    for digit in snafu.iter().rev() {
        match digit {
            SnafuDigit::Positive(number) => value += position_value * number,
            SnafuDigit::Negative1 => value += position_value * -1,
            SnafuDigit::Negative2 => value += position_value * -2,
        }

        position_value *= 5;
    }

    value
}

fn base_10_to_snafu(number: isize) -> Vec<SnafuDigit> {
    let mut snafu_number = vec![];
    let mut working_number = number;
    let mut carrying = 0;
    while working_number > 0 {
        match (working_number % 5) + carrying {
            0 => {
                carrying = 0;
                snafu_number.push(SnafuDigit::Positive(0));
            }
            1 => {
                carrying = 0;
                snafu_number.push(SnafuDigit::Positive(1));
            }
            2 => {
                carrying = 0;
                snafu_number.push(SnafuDigit::Positive(2));
            }
            3 => {
                carrying = 1;
                snafu_number.push(SnafuDigit::Negative2);
            }
            4 => {
                carrying = 1;
                snafu_number.push(SnafuDigit::Negative1);
            }
            5 => {
                carrying = 1;
                snafu_number.push(SnafuDigit::Positive(0));
            }
            _ => panic!(),
        }

        working_number /= 5;
    }

    if carrying > 0 {
        snafu_number.push(SnafuDigit::Positive(carrying))
    }

    snafu_number.reverse();

    snafu_number
}

fn negative1_parser(input: &str) -> IResult<&str, SnafuDigit> {
    let (input, _) = tag("-")(input)?;
    Ok((input, SnafuDigit::Negative1))
}

fn negative2_parser(input: &str) -> IResult<&str, SnafuDigit> {
    let (input, _) = tag("=")(input)?;
    Ok((input, SnafuDigit::Negative2))
}

fn positive_digit_parser(input: &str) -> IResult<&str, SnafuDigit> {
    let (input, digit) = map_res(
        nom::character::complete::satisfy(|c| c == '0' || c == '1' || c == '2'),
        |c: char| match c {
            '0' => Ok::<isize, &str>(0),
            '1' => Ok::<isize, &str>(1),
            '2' => Ok::<isize, &str>(2),
            _ => panic!(),
        },
    )(input)?;
    Ok((input, SnafuDigit::Positive(digit)))
}

fn snafu_parser(input: &str) -> IResult<&str, Vec<SnafuDigit>> {
    let (input, output) = many1(nom::branch::alt((
        negative1_parser,
        negative2_parser,
        positive_digit_parser,
    )))(input)?;
    Ok((input, output))
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, lines) = separated_list1(nom::character::complete::newline, snafu_parser)(input)?;
    Ok((input, lines))
}

#[cfg(test)]
mod day25_tests {
    use super::SnafuDigit;
    use crate::day25::base_10_to_snafu;
    use crate::day25::snafu_to_base_10;

    #[test]
    fn test_1() {
        let test_snafu = vec![SnafuDigit::Positive(1)];
        assert_eq!(snafu_to_base_10(&test_snafu), 1);

        let test_snafu = vec![
            SnafuDigit::Positive(1),
            SnafuDigit::Positive(2),
            SnafuDigit::Positive(1),
            SnafuDigit::Positive(1),
            SnafuDigit::Positive(1),
        ];
        assert_eq!(snafu_to_base_10(&test_snafu), 906);

        let test_snafu = vec![
            SnafuDigit::Positive(2),
            SnafuDigit::Negative1,
            SnafuDigit::Positive(1),
            SnafuDigit::Positive(2),
            SnafuDigit::Positive(1),
            SnafuDigit::Negative1,
            SnafuDigit::Negative2,
            SnafuDigit::Positive(0),
            SnafuDigit::Positive(0),
            SnafuDigit::Negative2,
            SnafuDigit::Positive(2),
            SnafuDigit::Negative1,
            SnafuDigit::Positive(0),
            SnafuDigit::Negative2,
            SnafuDigit::Negative2,
            SnafuDigit::Positive(2),
            SnafuDigit::Negative2,
            SnafuDigit::Negative2,
            SnafuDigit::Positive(2),
            SnafuDigit::Positive(1),
        ];
        assert_eq!(snafu_to_base_10(&test_snafu), 35422591760336);
    }

    #[test]
    fn test_2() {
        let test_snafu = vec![SnafuDigit::Positive(1), SnafuDigit::Positive(0)];
        assert_eq!(base_10_to_snafu(5), test_snafu);

        let test_snafu = vec![SnafuDigit::Positive(1), SnafuDigit::Positive(1)];
        assert_eq!(base_10_to_snafu(6), test_snafu);

        let test_snafu = vec![SnafuDigit::Positive(1), SnafuDigit::Negative1];
        assert_eq!(base_10_to_snafu(4), test_snafu);

        let test_snafu = vec![SnafuDigit::Positive(1), SnafuDigit::Negative2];
        assert_eq!(base_10_to_snafu(3), test_snafu);

        let test_snafu = vec![SnafuDigit::Positive(2), SnafuDigit::Negative1];
        assert_eq!(base_10_to_snafu(9), test_snafu);

        let test_snafu = vec![SnafuDigit::Positive(2), SnafuDigit::Negative2];
        assert_eq!(base_10_to_snafu(8), test_snafu);

        let test_snafu = vec![
            SnafuDigit::Positive(1),
            SnafuDigit::Negative2,
            SnafuDigit::Positive(0),
        ];
        assert_eq!(base_10_to_snafu(15), test_snafu);

        let test_snafu = vec![
            SnafuDigit::Positive(1),
            SnafuDigit::Positive(2),
            SnafuDigit::Positive(1),
            SnafuDigit::Positive(1),
            SnafuDigit::Positive(1),
        ];
        assert_eq!(base_10_to_snafu(906), test_snafu);

        let test_snafu = vec![
            SnafuDigit::Positive(1),
            SnafuDigit::Negative2,
            SnafuDigit::Positive(1),
            SnafuDigit::Positive(1),
            SnafuDigit::Negative1,
            SnafuDigit::Positive(2),
        ];
        assert_eq!(base_10_to_snafu(2022), test_snafu);

        let test_snafu = vec![
            SnafuDigit::Positive(1),
            SnafuDigit::Negative1,
            SnafuDigit::Positive(0),
            SnafuDigit::Negative1,
            SnafuDigit::Negative1,
            SnafuDigit::Negative1,
            SnafuDigit::Positive(0),
        ];
        assert_eq!(base_10_to_snafu(12345), test_snafu);

        let test_snafu = vec![
            SnafuDigit::Positive(1),
            SnafuDigit::Positive(1),
            SnafuDigit::Positive(2),
            SnafuDigit::Positive(1),
            SnafuDigit::Negative1,
            SnafuDigit::Positive(1),
            SnafuDigit::Positive(1),
            SnafuDigit::Positive(1),
            SnafuDigit::Positive(0),
            SnafuDigit::Negative1,
            SnafuDigit::Positive(1),
            SnafuDigit::Negative2,
            SnafuDigit::Positive(0),
        ];
        assert_eq!(base_10_to_snafu(314159265), test_snafu);
    }
}
