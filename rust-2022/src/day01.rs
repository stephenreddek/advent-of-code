use itertools::Itertools;
use nom::{combinator::map_res, multi::separated_list0, IResult};

pub fn part1() -> usize {
    let input = include_str!("../../data/2022/01.txt");

    match input_parser(input) {
        Ok((remaining_input, elves)) if remaining_input.is_empty() => {
            println!("parsed entire input");
            elves
                .iter()
                .max_by(|&x, &y| x.iter().sum::<usize>().cmp(&(y.iter().sum::<usize>())))
                .unwrap_or(&vec![])
                .iter()
                .sum::<usize>()
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
    let input = include_str!("../../data/2022/01.txt");

    match input_parser(input) {
        Ok((remaining_input, elves)) if remaining_input.is_empty() => {
            println!("parsed entire input");
            elves
                .iter()
                .sorted_by(|&x, &y| y.iter().sum::<usize>().cmp(&(x.iter().sum::<usize>())))
                .take(3)
                .map(|elf| elf.iter().sum::<usize>())
                .sum::<usize>()
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

fn food_item_calorie_parser(input: &str) -> IResult<&str, usize> {
    let (input, calories) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<usize>()
    })(input)?;
    Ok((input, calories))
}

fn elf_pack_parser(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, elves) =
        separated_list0(nom::character::complete::newline, food_item_calorie_parser)(input)?;
    Ok((input, elves))
}

fn input_parser(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    let (input, lines) =
        separated_list0(nom::character::complete::newline, elf_pack_parser)(input)?;

    Ok((input, lines))
}
