use itertools::Itertools;
use nom::{combinator::map_res, multi::separated_list0, IResult};

type SectionRange = (usize, usize);
type ElfPair = (SectionRange, SectionRange);

pub fn part1() -> usize {
    let input = include_str!("../../data/2022/04.txt");

    match input_parser(input) {
        Ok((remaining_input, pairings)) if remaining_input.is_empty() => {
            println!("parsed entire input");
            pairings
                .iter()
                .filter(|&pair| fully_overlapping(pair))
                .count()
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
    let input = include_str!("../../data/2022/04.txt");

    match input_parser(input) {
        Ok((remaining_input, pairings)) if remaining_input.is_empty() => {
            println!("parsed entire input");
            pairings.iter().filter(|&pair| overlapping(pair)).count()
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

fn fully_overlapping(pairing: &ElfPair) -> bool {
    let (lesser, greater) = if pairing.0 .0 < pairing.1 .0 {
        (pairing.0, pairing.1)
    } else if pairing.0 .0 == pairing.1 .0 {
        if pairing.0 .1 >= pairing.1 .1 {
            (pairing.0, pairing.1)
        } else {
            (pairing.1, pairing.0)
        }
    } else {
        (pairing.1, pairing.0)
    };

    lesser.0 <= greater.0 && lesser.1 >= greater.1
}

fn overlapping(pairing: &ElfPair) -> bool {
    let (lesser, greater) = if pairing.0 .0 < pairing.1 .0 {
        (pairing.0, pairing.1)
    } else if pairing.0 .0 == pairing.1 .0 {
        if pairing.0 .1 >= pairing.1 .1 {
            (pairing.0, pairing.1)
        } else {
            (pairing.1, pairing.0)
        }
    } else {
        (pairing.1, pairing.0)
    };

    lesser.1 >= greater.0
}

fn section_range_parser(input: &str) -> IResult<&str, SectionRange> {
    let mut number_parser = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<usize>()
    });
    let (input, min) = number_parser(input)?;
    let (input, _) = nom::character::complete::char('-')(input)?;
    let (input, max) = number_parser(input)?;
    Ok((input, (min, max)))
}

fn elf_pair_parser(input: &str) -> IResult<&str, ElfPair> {
    let (input, first) = section_range_parser(input)?;
    let (input, _) = nom::character::complete::char(',')(input)?;
    let (input, second) = section_range_parser(input)?;

    Ok((input, (first, second)))
}

fn input_parser(input: &str) -> IResult<&str, Vec<ElfPair>> {
    let (input, lines) =
        separated_list0(nom::character::complete::newline, elf_pair_parser)(input)?;

    Ok((input, lines))
}
