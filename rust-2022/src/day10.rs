use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    combinator::{map_res, opt},
    multi::separated_list0,
    multi::separated_list1,
    IResult,
};

type Input = Vec<Instruction>;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    AddX(i32),
    NoOp,
}

pub fn part1() -> i32 {
    let input = include_str!("../../data/2022/10.txt");

    match input_parser(input) {
        Ok((remaining_input, instructions)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut instructions_iter = instructions.iter().flat_map(expand_add);

            let mut sum = 0;
            let mut register_x: i32 = 1;
            for cycle in 1..221 {
                match cycle {
                    20 => sum += 20 * register_x,
                    60 => sum += 60 * register_x,
                    100 => sum += 100 * register_x,
                    140 => sum += 140 * register_x,
                    180 => sum += 180 * register_x,
                    220 => sum += 220 * register_x,
                    _ => {}
                }

                let instruction = instructions_iter.next().unwrap();

                match instruction {
                    Instruction::AddX(amount) => register_x += amount,
                    Instruction::NoOp => {}
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
    let input = include_str!("../../data/2022/10.txt");

    match input_parser(input) {
        Ok((remaining_input, instructions)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut instructions_iter = instructions.iter().flat_map(expand_add);
            let mut register_x: i32 = 1;

            for _ow in 0..6 {
                for column in 0..40 {
                    if (column == register_x - 1
                        || column == register_x
                        || column == register_x + 1)
                    {
                        print!("#");
                    } else {
                        print!(".");
                    }

                    let instruction = instructions_iter.next().unwrap();

                    match instruction {
                        Instruction::AddX(amount) => register_x += amount,
                        Instruction::NoOp => {}
                    }
                }
                println!();
            }

            0
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

fn expand_add(instruction: &Instruction) -> Vec<Instruction> {
    match instruction {
        Instruction::AddX(_) => vec![Instruction::NoOp, *instruction],
        Instruction::NoOp => vec![*instruction],
    }
}

fn add_parser(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("addx ")(input)?;
    let (input, negative_sign) = opt(nom::character::complete::char('-'))(input)?;
    let (input, amount) =
        map_res(nom::character::complete::digit1, |s: &str| s.parse::<i32>())(input)?;

    Ok((
        input,
        Instruction::AddX(if negative_sign.is_some() {
            -amount
        } else {
            amount
        }),
    ))
}

fn noop_parser(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("noop")(input)?;

    Ok((input, Instruction::NoOp))
}

fn move_parser(input: &str) -> IResult<&str, Instruction> {
    nom::branch::alt((add_parser, noop_parser))(input)
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, lines) = separated_list1(nom::character::complete::newline, move_parser)(input)?;

    Ok((input, lines))
}
