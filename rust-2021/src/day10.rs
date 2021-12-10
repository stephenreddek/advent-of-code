use core::num;
use itertools::Itertools;
use nom::{bytes::complete::tag, multi::separated_list0, sequence::separated_pair, IResult};
use std::collections::{HashMap, HashSet};

pub fn part1() -> usize {
    let input = include_str!("../../data/2021/10.txt");

    match input_parser(input) {
        Ok((remaining_input, input)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            input.iter().map(|line| part_1_score_line(line)).sum()
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
    let input = include_str!("../../data/2021/10.txt");

    match input_parser(input) {
        Ok((remaining_input, input)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let scores = input
                .iter()
                .filter_map(|line| score_line_complete(line))
                .sorted()
                .collect_vec();
            let middle_pos = scores.len() / 2;
            scores[middle_pos]
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

fn part_1_score_line(chars: &Vec<char>) -> usize {
    let mut stack: Vec<char> = Vec::new();
    let mut input = chars.to_owned();

    input.reverse();

    while !input.is_empty() {
        match input.pop() {
            Some('[') => {
                stack.push('[');
            }
            Some('(') => {
                stack.push('(');
            }
            Some('{') => {
                stack.push('{');
            }
            Some('<') => {
                stack.push('<');
            }
            Some(']') => {
                match stack.pop() {
                    Some('[') => {
                        //ok
                    }
                    Some(x) => return 57,
                    None => return 0,
                }
            }
            Some(')') => {
                match stack.pop() {
                    Some('(') => {
                        //ok
                    }
                    Some(x) => return 3,
                    None => return 0,
                }
            }
            Some('}') => {
                match stack.pop() {
                    Some('{') => {
                        //ok
                    }
                    Some(x) => return 1197,
                    None => return 0,
                }
            }
            Some('>') => {
                match stack.pop() {
                    Some('<') => {
                        //ok
                    }
                    Some(x) => return 25137,
                    None => return 0,
                }
            }
            Some(x) => {
                panic!("invalid input {:?}", x);
            }
            None => {
                break;
            }
        }
    }

    0
}

fn score_line_complete(chars: &Vec<char>) -> Option<usize> {
    let mut stack: Vec<char> = Vec::new();
    let mut input = chars.to_owned();

    input.reverse();

    while !input.is_empty() {
        match input.pop() {
            Some('[') => {
                stack.push('[');
            }
            Some('(') => {
                stack.push('(');
            }
            Some('{') => {
                stack.push('{');
            }
            Some('<') => {
                stack.push('<');
            }
            Some(']') => {
                match stack.pop() {
                    Some('[') => {
                        //ok
                    }
                    Some(x) => return None,
                    None => break,
                }
            }
            Some(')') => {
                match stack.pop() {
                    Some('(') => {
                        //ok
                    }
                    Some(x) => return None,
                    None => break,
                }
            }
            Some('}') => {
                match stack.pop() {
                    Some('{') => {
                        //ok
                    }
                    Some(x) => return None,
                    None => break,
                }
            }
            Some('>') => {
                match stack.pop() {
                    Some('<') => {
                        //ok
                    }
                    Some(x) => return None,
                    None => break,
                }
            }
            Some(x) => {
                panic!("invalid input {:?}", x);
            }
            None => {
                break;
            }
        }
    }

    if stack.is_empty() {
        return None;
    }

    //whatever is left on the stack is what needs to be completed.
    let mut score: usize = 0;
    loop {
        match stack.pop() {
            Some('(') => {
                score = score * 5 + 1;
            }
            Some('[') => {
                score = score * 5 + 2;
            }
            Some('{') => {
                score = score * 5 + 3;
            }
            Some('<') => {
                score = score * 5 + 4;
            }
            Some(c) => {
                panic!("invalid value on stack {:?}", c);
            }
            None => return Some(score),
        }
    }
}

fn line_parser(input: &str) -> IResult<&str, Vec<char>> {
    let (input, string) = nom::character::complete::not_line_ending(input)?;
    Ok((input, string.chars().collect_vec()))
}

fn input_parser(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (input, lines) = separated_list0(nom::character::complete::newline, line_parser)(input)?;

    // let (input, _) = nom::character::complete::newline(input)?;

    Ok((input, lines))
}
