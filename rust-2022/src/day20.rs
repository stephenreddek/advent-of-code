use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map_res, opt},
    multi::separated_list0,
    multi::{many1, separated_list1},
    IResult,
};
use std::fmt;
use std::{cmp::Ordering, hash::Hash};
use std::{collections::BinaryHeap, time::SystemTime};
use std::{
    collections::{HashMap, HashSet},
    ptr,
};

type Input = Vec<isize>;

pub fn part1() -> isize {
    let input = include_str!("../../data/2022/20.txt");
    // let input = include_str!("../../data/2022/20-example.txt");

    match input_parser(input) {
        Ok((remaining_input, numbers)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut mixed = numbers.iter().collect_vec();
            // println!("{mixed:?}");
            let length = numbers.len();
            for number in numbers.iter() {
                shift_number(number, &mut mixed, length);
                // println!("{mixed:?}");
            }

            let index_of_0 = mixed.iter().position(|&n| *n == 0).unwrap();
            let first_index = (index_of_0 + 1000) % length;
            let second_index = (index_of_0 + 2000) % length;
            let third_index = (index_of_0 + 3000) % length;

            println!(
                "{}, {}, {}",
                *mixed[first_index], *mixed[second_index], *mixed[third_index]
            );
            *mixed[first_index] + *mixed[second_index] + *mixed[third_index]
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
    let input = include_str!("../../data/2022/20.txt");

    match input_parser(input) {
        Ok((remaining_input, encrypted_numbers)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let decryption_key = 811589153;
            let numbers = encrypted_numbers
                .iter()
                .map(|&n| n * decryption_key)
                .collect_vec();

            let mut mixed = numbers.iter().collect_vec();
            let length = numbers.len();

            for _ in 0..10 {
                for number in numbers.iter() {
                    shift_number(number, &mut mixed, length);
                }
            }

            let index_of_0 = mixed.iter().position(|&n| *n == 0).unwrap();
            let first_index = (index_of_0 + 1000) % length;
            let second_index = (index_of_0 + 2000) % length;
            let third_index = (index_of_0 + 3000) % length;

            println!(
                "{}, {}, {}",
                *mixed[first_index], *mixed[second_index], *mixed[third_index]
            );
            *mixed[first_index] + *mixed[second_index] + *mixed[third_index]
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

fn shift_number<'a>(number: &'a isize, mixed: &mut Vec<&'a isize>, length: usize) {
    let index_in_mixed = mixed.iter().position(|&n| ptr::eq(n, number)).unwrap();
    mixed.remove(index_in_mixed);
    let new_index_unadjusted = index_in_mixed as isize + *number;

    let new_index = if new_index_unadjusted <= 0 {
        ((new_index_unadjusted) % (length - 1) as isize + (length - 1) as isize) as usize
    } else if new_index_unadjusted > (length - 1) as isize {
        (new_index_unadjusted as usize) % (length - 1)
    } else {
        new_index_unadjusted as usize
    };

    mixed.insert(new_index, number);
}

fn number_parser(input: &str) -> IResult<&str, isize> {
    let (input, negative_sign) = opt(tag("-"))(input)?;
    let (input, num) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<isize>()
    })(input)?;

    Ok((input, if negative_sign.is_some() { -num } else { num }))
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, formations) =
        separated_list1(nom::character::complete::newline, number_parser)(input)?;

    Ok((input, formations))
}

#[cfg(test)]
mod day20_tests {
    use std::ptr;

    use crate::day20::shift_number;

    #[test]
    fn test_pointer_equality() {
        let x = 4;
        let y = 4;

        let x_1 = &x;
        let x_2 = &x;
        let y_1 = &y;

        assert!(ptr::eq(x_1, x_2));
        assert!(!ptr::eq(x_1, y_1));
    }

    // #[test]
    // fn test_shift_number() {
    //     let mut numbers = vec![1, -2, 3, 4];
    //     let len = numbers.len();
    //     shift_number(-2, &mut numbers, len);
    //     assert_eq!(numbers, vec![1, 3, -2, 4]);

    //     numbers = vec![1, 2, 3, 4];
    //     let len = numbers.len();
    //     shift_number(2, &mut numbers, len);
    //     assert_eq!(numbers, vec![1, 3, 4, 2]);

    //     numbers = vec![1, -1, 3, 4];
    //     let len = numbers.len();
    //     shift_number(-1, &mut numbers, len);
    //     assert_eq!(numbers, vec![1, 3, 4, -1]);

    //     numbers = vec![1, 8, 3, 4];
    //     let len = numbers.len();
    //     shift_number(8, &mut numbers, len);
    //     assert_eq!(numbers, vec![8, 1, 3, 4]);

    //     numbers = vec![1, 2, -2, -3, 0, 3, 4];
    //     let len = numbers.len();
    //     shift_number(-2, &mut numbers, len);
    //     assert_eq!(numbers, vec![1, 2, -3, 0, 3, 4, -2]);

    //     numbers = vec![4, -2, 5, 6, 7, 8, 9];
    //     let len = numbers.len();
    //     shift_number(-2, &mut numbers, len);
    //     assert_eq!(numbers, vec![4, 5, 6, 7, 8, -2, 9]);
    // }
}
