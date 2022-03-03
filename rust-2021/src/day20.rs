use itertools::Itertools;
use nom::{
    bytes::complete::tag, combinator::map_res, multi::separated_list1, sequence::separated_pair,
    IResult,
};
use std::iter::Scan;
use std::ops;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

type Input = (Vec<bool>, Image);

struct Image {
    bounds: ((i32, i32), (i32, i32)),
    pixels: HashMap<(i32, i32), bool>,
}

pub fn part1() -> i32 {
    let input = include_str!("../../data/2021/20-test.txt");

    match input_parser(input) {
        Ok((remaining_input, (algorithm, image))) if remaining_input.is_empty() => {
            println!("parsed entire input");

            // print(&image);

            let first_enhance = enhance(&image, &algorithm, 1);

            // println!();
            // print(&first_enhance);

            let second_enhance = enhance(&first_enhance, &algorithm, 2);

            // println!();
            // print(&second_enhance);

            second_enhance
                .pixels
                .iter()
                .fold(0, |acc, (_, pixel)| if *pixel { acc + 1 } else { acc })
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

pub fn part2() -> i32 {
    let input = include_str!("../../data/2021/20.txt");

    match input_parser(input) {
        Ok((remaining_input, (algorithm, image))) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut enhanced = image;
            for generation in 1..51 {
                enhanced = enhance(&enhanced, &algorithm, generation);
            }

            enhanced
                .pixels
                .iter()
                .fold(0, |acc, (_, pixel)| if *pixel { acc + 1 } else { acc })
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

fn print(image: &Image) {
    let y_min = (image.bounds.1 .0) - 1;
    let y_max = (image.bounds.1 .1) + 1;

    let x_min = (image.bounds.0 .0) - 1;
    let x_max = (image.bounds.0 .1) + 1;

    for y in y_min..y_max {
        for x in x_min..x_max {
            let pixel = read_pixel(image, &(x, y), false);
            print!("{}", if pixel { '#' } else { '.' })
        }

        println!();
    }
}

fn enhance(image: &Image, algorithm: &[bool], generation: i32) -> Image {
    let y_min = (image.bounds.1 .0) - 1;
    let y_max = (image.bounds.1 .1) + 1;

    let x_min = (image.bounds.0 .0) - 1;
    let x_max = (image.bounds.0 .1) + 1;

    let mut new_pixels = HashMap::new();

    for x in x_min..x_max {
        for y in y_min..y_max {
            let new_pixel = enhance_pixel(image, algorithm, &(x, y), generation);
            new_pixels.insert((x, y), new_pixel);
        }
    }

    Image {
        bounds: ((x_min, x_max), (y_min, y_max)),
        pixels: new_pixels,
    }
}

fn enhance_pixel(
    image: &Image,
    algorithm: &[bool],
    location: &(i32, i32),
    generation: i32,
) -> bool {
    let default = algorithm[0] && generation % 2 == 0;

    let all_considered_pixel_offsets = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (0, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut algorithm_lookup = 0;

    for (x_offset, y_offset) in all_considered_pixel_offsets {
        algorithm_lookup <<= 1;
        algorithm_lookup += if read_pixel(
            image,
            &(location.0 + x_offset, location.1 + y_offset),
            default,
        ) {
            1
        } else {
            0
        };
    }

    algorithm[algorithm_lookup]
}

fn read_pixel(image: &Image, location: &(i32, i32), default: bool) -> bool {
    *image.pixels.get(location).unwrap_or(&default)
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, algorithm_string) = nom::character::complete::not_line_ending(input)?;
    let (input, _) = nom::character::complete::newline(input)?;
    let (input, _) = nom::character::complete::newline(input)?;

    let (input, image_lines) = separated_list1(
        nom::character::complete::newline,
        nom::character::complete::not_line_ending,
    )(input)?;

    let algorithm = algorithm_string.chars().map(|c| c == '#').collect_vec();

    let mut pixels: HashMap<(i32, i32), bool> = HashMap::new();

    for (y, line) in image_lines.iter().enumerate() {
        let y_val: i32 = y.try_into().unwrap();
        for (x, c) in line.chars().enumerate() {
            let x_val: i32 = x.try_into().unwrap();
            pixels.insert((x_val, y_val), c == '#');
        }
    }

    let image = Image {
        bounds: (
            (0, image_lines[0].len().try_into().unwrap()),
            (0, image_lines.len().try_into().unwrap()),
        ),
        pixels,
    };

    Ok((input, (algorithm, image)))
}
