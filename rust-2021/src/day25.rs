use itertools::Itertools;
use nom::{
    bytes::complete::tag, combinator::map_res, multi::separated_list1, sequence::separated_pair,
    IResult,
};
use std::{
    collections::{HashMap, HashSet},
    fmt,
    hash::Hash,
};

#[derive(Copy, Clone)]
enum Spot {
    EastMoving,
    SouthMoving,
    Empty,
}

impl fmt::Debug for Spot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Spot::Empty => {
                write!(f, ".")
            }
            Spot::EastMoving => {
                write!(f, ">")
            }
            Spot::SouthMoving => {
                write!(f, "v")
            }
        }
    }
}

#[derive(Clone)]
struct FloorMap {
    x_len: usize,
    y_len: usize,
    map: HashMap<(usize, usize), Spot>,
}

impl FloorMap {
    fn from_lines(lines: Vec<Vec<Spot>>) -> Self {
        let mut map: HashMap<(usize, usize), Spot> = HashMap::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, value) in line.iter().enumerate() {
                map.insert((x, y), *value);
            }
        }

        FloorMap {
            x_len: lines[0].len(),
            y_len: lines.len(),
            map,
        }
    }

    fn with_maximums(map: &FloorMap) -> Self {
        FloorMap {
            x_len: map.x_len,
            y_len: map.y_len,
            map: HashMap::new(),
        }
    }

    fn is_occupied(&self, location: &(usize, usize)) -> bool {
        match self.map.get(location) {
            None => false,
            Some(Spot::Empty) => false,
            Some(Spot::SouthMoving) => true,
            Some(Spot::EastMoving) => true,
        }
    }

    fn print(&self) {
        for y in 0..self.y_len {
            for x in 0..self.x_len {
                print!("{:?}", self.map.get(&(x, y)).unwrap_or(&Spot::Empty))
            }
            println!();
        }
        println!();
    }
}

pub fn part1() -> usize {
    let input = include_str!("../../data/2021/25.txt");

    match input_parser(input) {
        Ok((remaining_input, initial_floor_map)) if remaining_input.is_empty() => {
            // println!("parsed entire input");

            initial_floor_map.print();

            let mut floor_map: FloorMap = initial_floor_map;
            let mut counter: usize = 0;
            loop {
                let (next_floor_map, changed) = step(&floor_map);
                counter += 1;

                if !changed {
                    return counter;
                }

                floor_map = next_floor_map;

                // println!("after {}, {}", counter, changed);
                // floor_map.print();
            }
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
    let input = include_str!("../../data/2021/25-test.txt");

    match input_parser(input) {
        Ok((remaining_input, floor_map)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            0
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

fn step(floor_map: &FloorMap) -> (FloorMap, bool) {
    let mut after_east_move: FloorMap = FloorMap::with_maximums(floor_map);
    let mut changed = false;

    for (&(x, y), &spot) in floor_map.map.iter() {
        match spot {
            Spot::EastMoving => {
                let next_x = (x + 1) % floor_map.x_len;
                if floor_map.is_occupied(&(next_x, y)) {
                    after_east_move.map.insert((x, y), spot);
                } else {
                    after_east_move.map.insert((next_x, y), spot);
                    changed = true;
                }
            }
            Spot::SouthMoving => {
                after_east_move.map.insert((x, y), spot);
            }
            Spot::Empty => {
                //do nothing
            }
        }
    }

    let mut result: FloorMap = FloorMap::with_maximums(floor_map);

    for (&(x, y), &spot) in after_east_move.map.iter() {
        match spot {
            Spot::EastMoving => {
                result.map.insert((x, y), spot);
            }
            Spot::SouthMoving => {
                let next_y = (y + 1) % after_east_move.y_len;
                if after_east_move.is_occupied(&(x, next_y)) || result.is_occupied(&(x, next_y)) {
                    result.map.insert((x, y), spot);
                } else {
                    result.map.insert((x, next_y), spot);
                    changed = true;
                }
            }
            Spot::Empty => {
                //do nothing
            }
        }
    }

    (result, changed)
}

fn line_parser(input: &str) -> IResult<&str, Vec<Spot>> {
    let (input, line) = nom::character::complete::not_line_ending(input)?;
    let spots = line
        .chars()
        .filter_map(|c| match c {
            '.' => Some(Spot::Empty),
            '>' => Some(Spot::EastMoving),
            'v' => Some(Spot::SouthMoving),
            _ => None,
        })
        .collect_vec();

    Ok((input, spots))
}

fn input_parser(input: &str) -> IResult<&str, FloorMap> {
    let (input, lines) = separated_list1(nom::character::complete::newline, line_parser)(input)?;

    // let (input, _) = nom::character::complete::newline(input)?;

    let floor_map = FloorMap::from_lines(lines.into_iter().dropping_back(1).collect_vec());

    Ok((input, floor_map))
}
