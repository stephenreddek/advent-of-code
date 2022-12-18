use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map_res, opt},
    multi::separated_list0,
    multi::{many1, separated_list1},
    IResult,
};
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};
use std::{cmp::Ordering, hash::Hash};

type Coordinate = (isize, isize, isize);

type Input = HashSet<Coordinate>;

pub fn part1() -> usize {
    let input = include_str!("../../data/2022/18.txt");

    match input_parser(input) {
        Ok((remaining_input, blocks)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            blocks
                .iter()
                .map(|&b| count_exposed_sides(&blocks, b))
                .sum1()
                .unwrap()
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
    let input = include_str!("../../data/2022/18.txt");

    match input_parser(input) {
        Ok((remaining_input, reference)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let (mut carved, starting_point) = create_bounding_box(&reference);

            let mut to_carve = vec![starting_point];
            while let Some(point) = to_carve.pop() {
                let carve_next = carve_from_point(&mut carved, &reference, point);
                to_carve.extend(carve_next);
            }

            carved
                .iter()
                .map(|&b| count_exposed_sides(&carved, b))
                .sum1()
                .unwrap()
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

fn count_exposed_sides(grid: &HashSet<Coordinate>, spot: Coordinate) -> usize {
    let mut count = 0;
    for x in [-1, 1] {
        if !grid.contains(&(spot.0 + x, spot.1, spot.2)) {
            count += 1;
        }
    }

    for y in [-1, 1] {
        if !grid.contains(&(spot.0, spot.1 + y, spot.2)) {
            count += 1;
        }
    }

    for z in [-1, 1] {
        if !grid.contains(&(spot.0, spot.1, spot.2 + z)) {
            count += 1;
        }
    }

    count
}

fn create_bounding_box(reference: &HashSet<Coordinate>) -> (HashSet<Coordinate>, Coordinate) {
    let min_x = reference.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_x = reference.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;

    let min_y = reference.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let max_y = reference.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    let min_z = reference.iter().min_by(|a, b| a.2.cmp(&b.2)).unwrap().2;
    let max_z = reference.iter().max_by(|a, b| a.2.cmp(&b.2)).unwrap().2;

    let mut block = HashSet::new();

    //create it bigger by 1 in every direction so we can start carving in one area and have a spot to carve all around
    for x in (min_x - 1)..(max_x + 2) {
        for y in (min_y - 1)..(max_y + 2) {
            for z in (min_z - 1)..(max_z + 2) {
                block.insert((x, y, z));
            }
        }
    }

    (block, (min_x - 1, min_y - 1, min_z - 1))
}

fn carve_from_point(
    to_carve: &mut HashSet<Coordinate>,
    reference: &HashSet<Coordinate>,
    point: Coordinate,
) -> Vec<Coordinate> {
    //if point does not exist, carve it out and return new points to carve from
    // don't carve points that are already carved out
    let mut next = vec![];

    if !reference.contains(&point) {
        to_carve.remove(&point);

        for x in [-1, 1] {
            if to_carve.contains(&(point.0 + x, point.1, point.2)) {
                next.push((point.0 + x, point.1, point.2));
            }
        }

        for y in [-1, 1] {
            if to_carve.contains(&(point.0, point.1 + y, point.2)) {
                next.push((point.0, point.1 + y, point.2));
            }
        }

        for z in [-1, 1] {
            if to_carve.contains(&(point.0, point.1, point.2 + z)) {
                next.push((point.0, point.1, point.2 + z));
            }
        }
    }

    next
}

fn coordinate_parser(input: &str) -> IResult<&str, Coordinate> {
    let (input, x) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<isize>()
    })(input)?;
    let (input, _) = nom::character::complete::char(',')(input)?;
    let (input, y) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<isize>()
    })(input)?;
    let (input, _) = nom::character::complete::char(',')(input)?;
    let (input, z) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<isize>()
    })(input)?;

    Ok((input, (x, y, z)))
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, formations) =
        separated_list1(nom::character::complete::newline, coordinate_parser)(input)?;

    Ok((input, HashSet::from_iter(formations)))
}
