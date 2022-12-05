use std::collections::VecDeque;

use itertools::Itertools;
use nom::{
    bytes::complete::tag, combinator::map_res, multi::separated_list0, multi::separated_list1,
    IResult,
};

type Input = (Vec<VecDeque<char>>, Vec<MoveInstruction>);

#[derive(Debug, PartialEq)]
struct MoveInstruction {
    from: usize,
    to: usize,
    count: usize,
}

pub fn part1() -> usize {
    let input = include_str!("../../data/2022/05.txt");

    match input_parser(input) {
        Ok((remaining_input, (mut stacks, instructions))) if remaining_input.is_empty() => {
            println!("parsed entire input");
            for instruction in instructions {
                perform_move_as_9000(&mut stacks, &instruction);
            }
            for mut stack in stacks {
                print!("{}", stack.pop_front().unwrap())
            }
            println!();
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

pub fn part2() -> usize {
    let input = include_str!("../../data/2022/05.txt");

    match input_parser(input) {
        Ok((remaining_input, (mut stacks, instructions))) if remaining_input.is_empty() => {
            println!("parsed entire input");
            for instruction in instructions {
                perform_move_as_9001(&mut stacks, &instruction);
            }
            for mut stack in stacks {
                print!("{}", stack.pop_front().unwrap())
            }
            println!();
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

fn perform_move_as_9000(stacks: &mut [VecDeque<char>], instruction: &MoveInstruction) {
    for _ in 0..instruction.count {
        let item = stacks
            .get_mut(instruction.from - 1)
            .unwrap()
            .pop_front()
            .unwrap();
        stacks.get_mut(instruction.to - 1).unwrap().push_front(item)
    }
}

fn perform_move_as_9001(stacks: &mut [VecDeque<char>], instruction: &MoveInstruction) {
    let mut temp = VecDeque::new();
    for _ in 0..instruction.count {
        let item = stacks
            .get_mut(instruction.from - 1)
            .unwrap()
            .pop_front()
            .unwrap();
        temp.push_front(item)
    }

    for &item in temp.iter() {
        stacks.get_mut(instruction.to - 1).unwrap().push_front(item)
    }
}

fn block_parser(input: &str) -> IResult<&str, char> {
    let (input, _) = nom::character::complete::char('[')(input)?;
    let (input, c) = nom::character::complete::anychar(input)?;
    let (input, _) = nom::character::complete::char(']')(input)?;
    Ok((input, c))
}

fn empty_block_parser(input: &str) -> IResult<&str, char> {
    let (input, _) = nom::character::complete::char(' ')(input)?;
    let (input, _) = nom::character::complete::char(' ')(input)?;
    let (input, _) = nom::character::complete::char(' ')(input)?;
    Ok((input, ' '))
}

fn stack_number_parser(input: &str) -> IResult<&str, usize> {
    let (input, _) = nom::character::complete::char(' ')(input)?;
    let (input, n) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<usize>()
    })(input)?;
    let (input, _) = nom::character::complete::char(' ')(input)?;
    Ok((input, n))
}

fn boxes_row_parser(input: &str) -> IResult<&str, Vec<char>> {
    let (input, lines) = separated_list1(
        nom::character::complete::char(' '),
        nom::branch::alt((block_parser, empty_block_parser)),
    )(input)?;

    Ok((input, lines))
}

fn identifier_row_parser(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, lines) =
        separated_list1(nom::character::complete::char(' '), stack_number_parser)(input)?;

    Ok((input, lines))
}

fn boxes_parser(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (input, lines) =
        separated_list1(nom::character::complete::newline, boxes_row_parser)(input)?;

    Ok((input, lines))
}

fn instruction_parser(input: &str) -> IResult<&str, MoveInstruction> {
    let (input, _) = tag("move ")(input)?;
    let (input, count) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<usize>()
    })(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<usize>()
    })(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<usize>()
    })(input)?;

    Ok((input, MoveInstruction { count, from, to }))
}

fn all_instructions_parser(input: &str) -> IResult<&str, Vec<MoveInstruction>> {
    let (input, instructions) =
        separated_list1(nom::character::complete::newline, instruction_parser)(input)?;

    Ok((input, instructions))
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, box_rows) = boxes_parser(input)?;
    let (input, _) = nom::character::complete::newline(input)?;
    let (input, identifiers) = identifier_row_parser(input)?;
    let (input, _) = nom::character::complete::newline(input)?;
    let (input, _) = nom::character::complete::newline(input)?;
    let (input, instructions) = all_instructions_parser(input)?;

    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); identifiers.len()];

    for row in box_rows {
        for (stack_num, &maybe_box) in row.iter().enumerate() {
            if maybe_box != ' ' {
                stacks.get_mut(stack_num).unwrap().push_back(maybe_box)
            }
        }
    }

    Ok((input, (stacks, instructions)))
}
