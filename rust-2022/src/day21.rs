use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map_res, opt},
    multi::separated_list0,
    multi::{many1, separated_list1},
    IResult,
};
use std::{cmp::Ordering, hash::Hash};
use std::{collections::BinaryHeap, time::SystemTime};
use std::{
    collections::{HashMap, HashSet},
    ptr,
};
use std::{fmt, ops};

type Input = Vec<(String, MonkeyOutput)>;

#[derive(Clone)]
enum MonkeyOutput {
    Literal(isize),
    Addition(String, String),
    Subtraction(String, String),
    Multiplication(String, String),
    Division(String, String),
}

pub fn part1() -> isize {
    // let input = include_str!("../../data/2022/21-example.txt");
    let input = include_str!("../../data/2022/21.txt");

    match input_parser(input) {
        Ok((remaining_input, monkeys)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut monkey_lookup = HashMap::new();

            for monkey in monkeys.iter() {
                monkey_lookup.insert(monkey.0.clone(), monkey.1.clone());
            }

            resolve("root", &monkey_lookup)
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
    let input = include_str!("../../data/2022/21.txt");
    // let input = include_str!("../../data/2022/21-example.txt");

    match input_parser(input) {
        Ok((remaining_input, monkeys)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut monkey_lookup = HashMap::new();

            for monkey in monkeys.iter() {
                monkey_lookup.insert(monkey.0.clone(), monkey.1.clone());
            }

            let root = monkey_lookup.get("root").unwrap();

            let (left, right) = match root {
                MonkeyOutput::Literal(_) => {
                    panic!("It doesn't make sense for the root to be literal")
                }
                MonkeyOutput::Addition(left, right) => (left, right),
                MonkeyOutput::Subtraction(left, right) => (left, right),
                MonkeyOutput::Multiplication(left, right) => (left, right),
                MonkeyOutput::Division(left, right) => (left, right),
            };

            let left_value = maybe_resolve(left, &monkey_lookup);
            let right_value = maybe_resolve(right, &monkey_lookup);

            let (comparison, hole_side) = if let Some(value) = left_value {
                (value, right)
            } else {
                (right_value.unwrap(), left)
            };

            reduce_to_answer(comparison, hole_side, &monkey_lookup)
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

fn reduce_to_answer(
    result: isize,
    top_of_tree: &str,
    monkey_lookup: &HashMap<String, MonkeyOutput>,
) -> isize {
    if top_of_tree == "humn" {
        return result;
    }

    let root = monkey_lookup.get(top_of_tree).unwrap();
    match root {
        MonkeyOutput::Literal(_value) => {
            panic!("this isn't how it should end")
        }
        MonkeyOutput::Addition(left, right) => {
            //left + right = result
            let left_value = maybe_resolve(left, monkey_lookup);
            let right_value = maybe_resolve(right, monkey_lookup);

            if let Some(value) = left_value {
                //right side has the hole and must equal result - left
                reduce_to_answer(result - value, right, monkey_lookup)
            } else {
                //left side has the hole and must equal result - right
                reduce_to_answer(result - right_value.unwrap(), left, monkey_lookup)
            }
        }
        MonkeyOutput::Subtraction(left, right) => {
            //left - right = result
            let left_value = maybe_resolve(left, monkey_lookup);
            let right_value = maybe_resolve(right, monkey_lookup);

            if let Some(value) = left_value {
                //right side has the hole and must equal left - result
                reduce_to_answer(value - result, right, monkey_lookup)
            } else {
                //left side has the hole and must equal result + right
                reduce_to_answer(result + right_value.unwrap(), left, monkey_lookup)
            }
        }
        MonkeyOutput::Multiplication(left, right) => {
            //left * right = result
            let left_value = maybe_resolve(left, monkey_lookup);
            let right_value = maybe_resolve(right, monkey_lookup);

            if let Some(value) = left_value {
                //right side has the hole and must equal result / left
                reduce_to_answer(result / value, right, monkey_lookup)
            } else {
                //left side has the hole and must equal result / right
                reduce_to_answer(result / right_value.unwrap(), left, monkey_lookup)
            }
        }
        MonkeyOutput::Division(left, right) => {
            //left / right = result
            let left_value = maybe_resolve(left, monkey_lookup);
            let right_value = maybe_resolve(right, monkey_lookup);

            if let Some(value) = left_value {
                //right side has the hole and must equal left / result
                reduce_to_answer(value / result, right, monkey_lookup)
            } else {
                //left side has the hole and must equal result * right
                reduce_to_answer(result * right_value.unwrap(), left, monkey_lookup)
            }
        }
    }
}

fn resolve(monkey_name: &str, lookup: &HashMap<String, MonkeyOutput>) -> isize {
    let monkey_output = lookup.get(monkey_name).unwrap();

    match monkey_output {
        MonkeyOutput::Literal(value) => *value,
        MonkeyOutput::Addition(left, right) => resolve(left, lookup) + resolve(right, lookup),
        MonkeyOutput::Subtraction(left, right) => resolve(left, lookup) - resolve(right, lookup),
        MonkeyOutput::Multiplication(left, right) => resolve(left, lookup) * resolve(right, lookup),
        MonkeyOutput::Division(left, right) => resolve(left, lookup) / resolve(right, lookup),
    }
}

fn maybe_resolve(monkey_name: &str, lookup: &HashMap<String, MonkeyOutput>) -> Option<isize> {
    if monkey_name == "humn" {
        return None;
    }

    let monkey_output = lookup.get(monkey_name).unwrap();

    match monkey_output {
        MonkeyOutput::Literal(value) => Some(*value),
        MonkeyOutput::Addition(left, right) => {
            let left_value = maybe_resolve(left, lookup);
            let right_value = maybe_resolve(right, lookup);
            if left_value.is_none() || right_value.is_none() {
                None
            } else {
                Some(left_value.unwrap() + right_value.unwrap())
            }
        }
        MonkeyOutput::Subtraction(left, right) => {
            let left_value = maybe_resolve(left, lookup);
            let right_value = maybe_resolve(right, lookup);
            if left_value.is_none() || right_value.is_none() {
                None
            } else {
                Some(left_value.unwrap() - right_value.unwrap())
            }
        }
        MonkeyOutput::Multiplication(left, right) => {
            let left_value = maybe_resolve(left, lookup);
            let right_value = maybe_resolve(right, lookup);
            if left_value.is_none() || right_value.is_none() {
                None
            } else {
                Some(left_value.unwrap() * right_value.unwrap())
            }
        }
        MonkeyOutput::Division(left, right) => {
            let left_value = maybe_resolve(left, lookup);
            let right_value = maybe_resolve(right, lookup);
            if left_value.is_none() || right_value.is_none() {
                None
            } else {
                Some(left_value.unwrap() / right_value.unwrap())
            }
        }
    }
}

fn division_parser(input: &str) -> IResult<&str, MonkeyOutput> {
    let (input, left) = nom::character::complete::alpha1(input)?;
    let (input, _) = tag(" / ")(input)?;
    let (input, right) = nom::character::complete::alpha1(input)?;

    Ok((
        input,
        MonkeyOutput::Division(left.to_owned(), right.to_owned()),
    ))
}

fn multiplication_parser(input: &str) -> IResult<&str, MonkeyOutput> {
    let (input, left) = nom::character::complete::alpha1(input)?;
    let (input, _) = tag(" * ")(input)?;
    let (input, right) = nom::character::complete::alpha1(input)?;

    Ok((
        input,
        MonkeyOutput::Multiplication(left.to_owned(), right.to_owned()),
    ))
}

fn addition_parser(input: &str) -> IResult<&str, MonkeyOutput> {
    let (input, left) = nom::character::complete::alpha1(input)?;
    let (input, _) = tag(" + ")(input)?;
    let (input, right) = nom::character::complete::alpha1(input)?;

    Ok((
        input,
        MonkeyOutput::Addition(left.to_owned(), right.to_owned()),
    ))
}

fn subtraction_parser(input: &str) -> IResult<&str, MonkeyOutput> {
    let (input, left) = nom::character::complete::alpha1(input)?;
    let (input, _) = tag(" - ")(input)?;
    let (input, right) = nom::character::complete::alpha1(input)?;

    Ok((
        input,
        MonkeyOutput::Subtraction(left.to_owned(), right.to_owned()),
    ))
}

fn literal_parser(input: &str) -> IResult<&str, MonkeyOutput> {
    let (input, num) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<isize>()
    })(input)?;

    Ok((input, MonkeyOutput::Literal(num)))
}

fn monkey_output_parser(input: &str) -> IResult<&str, MonkeyOutput> {
    let (input, output) = nom::branch::alt((
        literal_parser,
        division_parser,
        multiplication_parser,
        addition_parser,
        subtraction_parser,
    ))(input)?;
    Ok((input, output))
}

fn monkey_parser(input: &str) -> IResult<&str, (String, MonkeyOutput)> {
    let (input, monkey_name) = nom::character::complete::alpha1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, output) = monkey_output_parser(input)?;

    Ok((input, (monkey_name.to_owned(), output)))
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, formations) =
        separated_list1(nom::character::complete::newline, monkey_parser)(input)?;

    Ok((input, formations))
}

// #[cfg(test)]
// mod day21_tests {
//     use std::ptr;

//     use crate::day21;

//     #[test]
//     fn test_() {

//     }
// }
