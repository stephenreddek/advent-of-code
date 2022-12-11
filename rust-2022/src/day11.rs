use ibig::{modular::ModuloRing, ops::DivRem, ubig, UBig};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    combinator::{map_res, opt},
    multi::separated_list0,
    multi::separated_list1,
    IResult,
};
use std::collections::{HashMap, HashSet};

type Input = Vec<Monkey>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    AddValue(usize),
    AddOld,
    MultiplyValue(usize),
    MultiplyOld,
}

#[derive(Debug, PartialEq)]
struct Monkey {
    items: Vec<UBig>,
    operation: Operation,
    divisible_test: UBig,
    positive_monkey: usize,
    negative_monkey: usize,
}

pub fn part1() -> usize {
    let input = include_str!("../../data/2022/11.txt");

    match input_parser(input) {
        Ok((remaining_input, mut monkeys)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut counts = vec![0; monkeys.len()];
            for _round in 0..20 {
                let mut holding: Vec<Vec<UBig>> = vec![vec![]; monkeys.len()];
                for (monkey_num, monkey) in monkeys.iter_mut().enumerate() {
                    monkey.items.extend(holding[monkey_num].clone());
                    holding[monkey_num].clear();

                    for item in monkey.items.iter() {
                        let (new_value, _) =
                            (do_operation(item, monkey.operation)).div_rem(ubig!(3));
                        if new_value.clone().div_rem(monkey.divisible_test.clone()).1 == ubig!(0) {
                            holding[monkey.positive_monkey].push(new_value);
                        } else {
                            holding[monkey.negative_monkey].push(new_value);
                        }
                    }

                    counts[monkey_num] += monkey.items.len();

                    monkey.items.clear();
                }

                for (monkey_num, monkey_holding) in holding.iter().enumerate() {
                    monkeys[monkey_num].items.extend(monkey_holding.clone())
                }

                // println!("round {:?}:", round);
                // for monkey in monkeys.iter() {
                //     println!("{:?}:", monkey.items);
                // }
            }

            println!("{:?}", counts);

            let mut most_active = counts.iter().sorted().rev();
            most_active.next().unwrap() * most_active.next().unwrap()
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
    let input = include_str!("../../data/2022/11.txt");

    match input_parser(input) {
        Ok((remaining_input, mut monkeys)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let divisor = monkeys
                .iter()
                .fold(ubig!(1), |acc, m| acc * m.divisible_test.clone());

            let mut counts = vec![0; monkeys.len()];
            for _round in 0..10_000 {
                let mut holding: Vec<Vec<UBig>> = vec![vec![]; monkeys.len()];
                for (monkey_num, monkey) in monkeys.iter_mut().enumerate() {
                    monkey.items.extend(holding[monkey_num].clone());
                    holding[monkey_num].clear();

                    for item in monkey.items.iter() {
                        let (_, new_value) =
                            (do_operation(item, monkey.operation)).div_rem(divisor.clone());
                        if new_value.clone().div_rem(monkey.divisible_test.clone()).1 == ubig!(0) {
                            holding[monkey.positive_monkey].push(new_value);
                        } else {
                            holding[monkey.negative_monkey].push(new_value);
                        }
                    }

                    counts[monkey_num] += monkey.items.len();

                    monkey.items.clear();
                }

                for (monkey_num, monkey_holding) in holding.iter().enumerate() {
                    monkeys[monkey_num].items.extend(monkey_holding.clone())
                }

                // println!("round {:?}:", _round);
                // for monkey in monkeys.iter() {
                //     println!("{:?}:", monkey.items);
                // }
            }

            println!("{:?}", counts);

            let mut most_active = counts.iter().sorted().rev();
            most_active.next().unwrap() * most_active.next().unwrap()
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

fn do_operation(old: &UBig, operation: Operation) -> UBig {
    match operation {
        Operation::AddValue(value) => old + value,
        Operation::AddOld => old + old,
        Operation::MultiplyValue(value) => old * value,
        Operation::MultiplyOld => old * old,
    }
}

fn mul_old_parser(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("* old")(input)?;
    Ok((input, Operation::MultiplyOld))
}

fn add_old_parser(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("+ old")(input)?;
    Ok((input, Operation::AddOld))
}

fn mul_value_parser(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("* ")(input)?;
    let (input, value) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<usize>()
    })(input)?;
    Ok((input, Operation::MultiplyValue(value)))
}

fn add_value_parser(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("+ ")(input)?;
    let (input, value) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<usize>()
    })(input)?;
    Ok((input, Operation::AddValue(value)))
}

fn operation_parser(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("  Operation: new = old ")(input)?;
    nom::branch::alt((
        add_old_parser,
        mul_old_parser,
        add_value_parser,
        mul_value_parser,
    ))(input)
}

fn monkey_parser(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = tag("Monkey ")(input)?;
    let (input, _) = nom::character::complete::digit1(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = nom::character::complete::newline(input)?;
    let (input, _) = tag("  Starting items: ")(input)?;
    let (input, items) = separated_list1(
        tag(", "),
        map_res(nom::character::complete::digit1, |s: &str| {
            s.parse::<UBig>()
        }),
    )(input)?;
    let (input, _) = nom::character::complete::newline(input)?;
    let (input, operation) = operation_parser(input)?;
    let (input, _) = nom::character::complete::newline(input)?;
    let (input, _) = tag("  Test: divisible by ")(input)?;
    let (input, divisible_test) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<UBig>()
    })(input)?;
    let (input, _) = nom::character::complete::newline(input)?;
    let (input, _) = tag("    If true: throw to monkey ")(input)?;
    let (input, positive_monkey) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<usize>()
    })(input)?;
    let (input, _) = nom::character::complete::newline(input)?;
    let (input, _) = tag("    If false: throw to monkey ")(input)?;
    let (input, negative_monkey) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<usize>()
    })(input)?;

    Ok((
        input,
        Monkey {
            items,
            operation,
            divisible_test,
            positive_monkey,
            negative_monkey,
        },
    ))
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, monkeys) = separated_list1(tag("\n\n"), monkey_parser)(input)?;

    Ok((input, monkeys))
}
