use itertools::Itertools;
use nom::{
    branch::alt, bytes::complete::tag, combinator::map_res, multi::separated_list0,
    sequence::separated_pair, IResult,
};
use std::collections::{HashMap, HashSet};

type Input = (Vec<(i32, i32)>, Vec<FoldInstruction>);

enum FoldInstruction {
    X(i32),
    Y(i32),
}

pub fn part1() -> usize {
    let input = include_str!("../../data/2021/13.txt");

    match input_parser(input) {
        Ok((remaining_input, (dots, fold_instructions))) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut plotted_dots: HashSet<(i32, i32)> = HashSet::new();
            plotted_dots.extend(dots);
            let fold_instruction = fold_instructions.get(0).unwrap();

            fold(fold_instruction, &mut plotted_dots);

            plotted_dots.len()
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
    let input = include_str!("../../data/2021/13.txt");

    match input_parser(input) {
        Ok((remaining_input, (dots, fold_instructions))) if remaining_input.is_empty() => {
            println!("parsed entire input");

            println!("parsed entire input");

            let mut plotted_dots: HashSet<(i32, i32)> = HashSet::new();
            plotted_dots.extend(dots);
            for fold_instruction in fold_instructions.iter() {
                fold(fold_instruction, &mut plotted_dots);
            }

            print_dots(&plotted_dots);

            plotted_dots.len()
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

fn print_dots(dots: &HashSet<(i32, i32)>) {
    let max_y = 100;
    let max_x = 100;

    for y in 0..max_y {
        for x in 0..max_x {
            print!("{}", if dots.contains(&(x, y)) { "#" } else { "." })
        }

        println!();
    }
}

fn fold(fold_instruction: &FoldInstruction, dots: &mut HashSet<(i32, i32)>) {
    match fold_instruction {
        FoldInstruction::X(x) => fold_x(*x, dots),
        FoldInstruction::Y(y) => fold_y(*y, dots),
    }
}

fn fold_x(x_line: i32, dots: &mut HashSet<(i32, i32)>) {
    let left: Vec<(i32, i32)> = dots
        .iter()
        .filter_map(|(x, y)| if *x < x_line { Some((*x, *y)) } else { None })
        .collect_vec();
    let right = dots
        .iter()
        .filter_map(|&(x, y)| {
            if x > x_line {
                let diff = x - x_line;
                Some(((x_line - diff), y))
            } else {
                None
            }
        })
        .collect_vec();
    dots.drain();
    dots.extend(left);
    dots.extend(right);
}

fn fold_y(y_line: i32, dots: &mut HashSet<(i32, i32)>) {
    let top: Vec<(i32, i32)> = dots
        .iter()
        .filter_map(|(x, y)| if *y < y_line { Some((*x, *y)) } else { None })
        .collect_vec();
    let bottom = dots
        .iter()
        .filter_map(|&(x, y)| {
            if y > y_line {
                let diff = y - y_line;
                Some((x, (y_line - diff)))
            } else {
                None
            }
        })
        .collect_vec();
    dots.drain();
    dots.extend(top);
    dots.extend(bottom);
}

fn dot_parser(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        map_res(nom::character::complete::digit0, |s: &str| s.parse::<i32>()),
        tag(","),
        map_res(nom::character::complete::digit0, |s: &str| s.parse::<i32>()),
    )(input)
}

fn fold_along_x_parser(input: &str) -> IResult<&str, FoldInstruction> {
    let (input, _) = tag("fold along x=")(input)?;
    let (input, value) =
        map_res(nom::character::complete::digit0, |s: &str| s.parse::<i32>())(input)?;
    Ok((input, FoldInstruction::X(value)))
}

fn fold_along_y_parser(input: &str) -> IResult<&str, FoldInstruction> {
    let (input, _) = tag("fold along y=")(input)?;
    let (input, value) =
        map_res(nom::character::complete::digit0, |s: &str| s.parse::<i32>())(input)?;
    Ok((input, FoldInstruction::Y(value)))
}

fn fold_instruction_parser(input: &str) -> IResult<&str, FoldInstruction> {
    alt((fold_along_x_parser, fold_along_y_parser))(input)
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, dots) = separated_list0(nom::character::complete::newline, dot_parser)(input)?;

    let (input, _) = nom::character::complete::newline(input)?;
    let (input, _) = nom::character::complete::newline(input)?;

    let (input, fold_instructions) =
        separated_list0(nom::character::complete::newline, fold_instruction_parser)(input)?;

    let (input, _) = nom::character::complete::newline(input)?;

    Ok((input, (dots, fold_instructions)))
}
