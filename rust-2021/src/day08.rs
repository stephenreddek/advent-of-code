use nom::{bytes::complete::tag, multi::separated_list0, sequence::separated_pair, IResult};
use std::collections::HashSet;

type InputLine<'a, 'b> = (Vec<&'a str>, Vec<&'b str>);

pub fn part1() -> usize {
    let input = include_str!("../../data/2021/8.txt");

    match input_parser(input) {
        Ok((remaining_input, display_logs)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            display_logs.iter().fold(0, |acc, (_inputs, outputs)| {
                acc + count_recognizable(outputs)
            })
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
    let input = include_str!("../../data/2021/8.txt");

    match input_parser(input) {
        Ok((remaining_input, display_logs)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            display_logs.iter().fold(0, |acc, (inputs, outputs)| {
                let connections = infer_connections(inputs);

                let decoded = decode(outputs, &connections);
                acc + decoded
            })
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

fn decode(values: &[&str], connections: &[(&str, usize)]) -> usize {
    values.iter().fold(0, |acc, value| {
        (acc * 10) + find_value(connections, *value).unwrap()
    })
}

fn find_value(connections: &[(&str, usize)], value: &str) -> Option<usize> {
    connections
        .iter()
        .find(|(key, _val)| {
            HashSet::<char>::from_iter(key.chars()) == HashSet::from_iter(value.chars())
        })
        .map(|opt| opt.1)
}

fn infer_connections<'a>(input: &[&'a str]) -> Vec<(&'a str, usize)> {
    let connection_1 = find_length(input, 2);
    let connection_4 = find_length(input, 4);
    let connection_7 = find_length(input, 3);
    let connection_8 = find_length(input, 7);

    let chars_in_7: HashSet<char> = HashSet::from_iter(connection_7.chars());
    let chars_in_4: HashSet<char> = HashSet::from_iter(connection_4.chars());

    let connection_0 = input
        .iter()
        .filter(|val| val.len() == 6)
        .filter(|val| HashSet::from_iter(val.chars()).is_superset(&chars_in_7))
        .find(|val| !HashSet::from_iter(val.chars()).is_superset(&chars_in_4))
        .unwrap();

    let connection_3 = input
        .iter()
        .filter(|val| val.len() == 5)
        .filter(|val| HashSet::from_iter(val.chars()).is_superset(&chars_in_7))
        .find(|val| !HashSet::from_iter(val.chars()).is_superset(&chars_in_4))
        .unwrap();

    let connection_6 = input
        .iter()
        .filter(|val| val.len() == 6)
        .find(|val| !HashSet::from_iter(val.chars()).is_superset(&chars_in_7))
        .unwrap();

    let connection_9 = input
        .iter()
        .filter(|val| val.len() == 6)
        .find(|val| HashSet::from_iter(val.chars()).is_superset(&chars_in_4))
        .unwrap();

    let chars_in_6: HashSet<char> = HashSet::from_iter(connection_6.chars());

    let connection_5 = input
        .iter()
        .filter(|val| val.len() == 5)
        .find(|val| HashSet::from_iter(val.chars()).is_subset(&chars_in_6))
        .unwrap();

    let chars_in_3: HashSet<char> = HashSet::from_iter(connection_3.chars());
    let chars_in_5: HashSet<char> = HashSet::from_iter(connection_5.chars());

    let connection_2 = input
        .iter()
        .filter(|val| val.len() == 5)
        .filter(|val| !HashSet::from_iter(val.chars()).is_subset(&chars_in_3))
        .find(|val| !HashSet::from_iter(val.chars()).is_subset(&chars_in_5))
        .unwrap();

    vec![
        (*connection_0, 0),
        (connection_1, 1),
        (*connection_2, 2),
        (*connection_3, 3),
        (connection_4, 4),
        (*connection_5, 5),
        (*connection_6, 6),
        (connection_7, 7),
        (connection_8, 8),
        (*connection_9, 9),
    ]
}

fn find_length<'a>(input: &[&'a str], length: usize) -> &'a str {
    input.iter().find(|val| val.len() == length).unwrap()
}

fn count_recognizable(values: &[&str]) -> usize {
    values
        .iter()
        .filter(|value| is_recognizable(*value))
        .count()
}

fn is_recognizable(value: &&str) -> bool {
    matches!(value.len(), 2 | 3 | 4 | 7)
}

fn chars_parser(input: &str) -> IResult<&str, &str> {
    let (input, chars) = nom::character::complete::alpha1(input)?;
    // let set_of_chars: &str = chars.chars().collect();
    // Ok((input, set_of_chars))
    Ok((input, chars))
}

fn space_separated_strings(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(tag(" "), chars_parser)(input)
}

fn line_parser(input: &str) -> IResult<&str, InputLine> {
    separated_pair(space_separated_strings, tag(" | "), space_separated_strings)(input)
}

fn input_parser(input: &str) -> IResult<&str, Vec<InputLine>> {
    let (input, lines) = separated_list0(nom::character::complete::newline, line_parser)(input)?;

    let (input, _) = nom::character::complete::newline(input)?;

    Ok((input, lines))
}
