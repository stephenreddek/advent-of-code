use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    bytes::complete::tag, combinator::map_res, multi::separated_list0, multi::separated_list1,
    IResult,
};

type Input<'a> = Vec<TerminalLine<'a>>;

enum TerminalLine<'a> {
    Directory(&'a str),
    File(&'a str, usize),
    ListFiles,
    MoveUpDirectory,
    GoDownDirectory(&'a str),
    GoToRoot,
}

pub fn part1() -> usize {
    let input = include_str!("../../data/2022/07.txt");

    match input_parser(input) {
        Ok((remaining_input, input)) if remaining_input.is_empty() => {
            println!("parsed entire input");
            let mut model: HashMap<String, usize> = HashMap::new();
            sum_directories(&input, &mut model);
            let mut sum = 0;

            for (_name, &size) in model.iter() {
                if size <= 100000 {
                    // println!("{}: {}", name, size);
                    sum += size
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
    let input = include_str!("../../data/2022/07.txt");

    match input_parser(input) {
        Ok((remaining_input, input)) if remaining_input.is_empty() => {
            println!("parsed entire input");
            let mut model: HashMap<String, usize> = HashMap::new();
            sum_directories(&input, &mut model);

            let total_space = 70000000;
            let required_space = 30000000;
            let used_space = *model.get("/").unwrap();
            let currently_free = total_space - used_space;
            let to_free_up = required_space - currently_free;

            println!("used space: {}", used_space);
            println!("currently free: {}", currently_free);
            println!("to free up: {}", to_free_up);

            *model
                .iter()
                .sorted_by(|a, b| Ord::cmp(a.1, b.1))
                .find(|(name, size)| **size >= to_free_up)
                .unwrap()
                .1
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

fn sum_directories(input: &[TerminalLine], model: &mut HashMap<String, usize>) {
    let mut current_directory: Vec<&str> = Vec::new();
    for line in input {
        match line {
            TerminalLine::GoToRoot => {
                current_directory.clear();
                current_directory.push("/");
            }
            TerminalLine::Directory(dir_name) => {}
            TerminalLine::File(_, size) => {
                let mut qualified_directory_name = String::new();
                for directory in current_directory.iter() {
                    qualified_directory_name.push_str(directory);
                    match model.get(qualified_directory_name.as_str()) {
                        Some(current_size) => {
                            model.insert(qualified_directory_name.clone(), current_size + size);
                        }
                        None => {
                            model.insert(qualified_directory_name.clone(), *size);
                        }
                    }
                }
            }
            TerminalLine::ListFiles => {}
            TerminalLine::MoveUpDirectory => {
                current_directory.pop();
            }
            TerminalLine::GoDownDirectory(dir_name) => {
                current_directory.push(dir_name);
            }
        }
    }
}

fn file_description_parser(input: &str) -> IResult<&str, TerminalLine> {
    let (input, file_size) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<usize>()
    })(input)?;
    let (input, _) = nom::character::complete::char(' ')(input)?;
    let (input, file_name) = nom::bytes::complete::take_while(|c| c != '\n')(input)?;
    Ok((input, TerminalLine::File(file_name, file_size)))
}

fn directory_name_parser(input: &str) -> IResult<&str, TerminalLine> {
    let (input, _) = tag("dir ")(input)?;
    let (input, dir_name) = nom::bytes::complete::take_while(|c| c != '\n')(input)?;
    Ok((input, TerminalLine::Directory(dir_name)))
}

fn change_directory_parser(input: &str) -> IResult<&str, TerminalLine> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir_name) = nom::bytes::complete::take_while(|c| c != '\n')(input)?;

    match dir_name {
        ".." => Ok((input, TerminalLine::MoveUpDirectory)),
        "/" => Ok((input, TerminalLine::GoToRoot)),
        _ => Ok((input, TerminalLine::GoDownDirectory(dir_name))),
    }
}

fn list_files_parser(input: &str) -> IResult<&str, TerminalLine> {
    let (input, _) = tag("$ ls")(input)?;

    Ok((input, TerminalLine::ListFiles))
}

fn terminal_line_parser(input: &str) -> IResult<&str, TerminalLine> {
    nom::branch::alt((
        file_description_parser,
        directory_name_parser,
        change_directory_parser,
        list_files_parser,
    ))(input)
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, lines) =
        separated_list1(nom::character::complete::newline, terminal_line_parser)(input)?;

    Ok((input, lines))
}
