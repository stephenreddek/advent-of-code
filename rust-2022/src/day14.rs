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

type Input = Vec<Formation>;

type Formation = Vec<(isize, isize)>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Spot {
    Sand,
    Rock,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SandResult {
    Settled,
    Abyss,
}

pub fn part1() -> usize {
    let input = include_str!("../../data/2022/14.txt");

    match input_parser(input) {
        Ok((remaining_input, formations)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut map = HashMap::new();

            fill_map(&formations, &mut map);

            let deepest = map.iter().max_by(|x, y| x.0 .1.cmp(&y.0 .1)).unwrap().0 .1;

            while simulate_sand_part1(&mut map, deepest + 2) == SandResult::Settled {}

            map.iter().filter(|(_, &spot)| spot == Spot::Sand).count()
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
    let input = include_str!("../../data/2022/14.txt");

    match input_parser(input) {
        Ok((remaining_input, formations)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut map = HashMap::new();

            fill_map(&formations, &mut map);

            let deepest = map.iter().max_by(|x, y| x.0 .1.cmp(&y.0 .1)).unwrap().0 .1;

            while simulate_sand_part2(&mut map, deepest + 1) == SandResult::Settled {}

            map.iter().filter(|(_, &spot)| spot == Spot::Sand).count()
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

fn simulate_sand_part1(map: &mut HashMap<(isize, isize), Spot>, abyss_start: isize) -> SandResult {
    let mut sand_position: (isize, isize) = (500, 0);

    while let Some(position) = find_next_position(map, sand_position) {
        sand_position = position;
        if sand_position.1 >= abyss_start {
            return SandResult::Abyss;
        }
    }

    map.insert(sand_position, Spot::Sand);

    SandResult::Settled
}

fn simulate_sand_part2(map: &mut HashMap<(isize, isize), Spot>, floor_start: isize) -> SandResult {
    let mut sand_position: (isize, isize) = (500, 0);

    while let Some(position) = find_next_position_part2(map, sand_position, floor_start) {
        sand_position = position;
    }

    map.insert(sand_position, Spot::Sand);

    //not really accurate since no abyss, but 🤷‍♂️
    if sand_position.1 == 0 {
        SandResult::Abyss
    } else {
        SandResult::Settled
    }
}

fn find_next_position(
    map: &HashMap<(isize, isize), Spot>,
    sand_position: (isize, isize),
) -> Option<(isize, isize)> {
    let down = (sand_position.0, sand_position.1 + 1);
    let diagonal_left = (sand_position.0 - 1, sand_position.1 + 1);
    let diagonal_right = (sand_position.0 + 1, sand_position.1 + 1);

    match map.get(&down) {
        Some(_) => {}
        None => return Some(down),
    }

    match map.get(&diagonal_left) {
        Some(_) => {}
        None => return Some(diagonal_left),
    }

    match map.get(&diagonal_right) {
        Some(_) => {}
        None => return Some(diagonal_right),
    }

    None
}

fn find_next_position_part2(
    map: &HashMap<(isize, isize), Spot>,
    sand_position: (isize, isize),
    floor_start: isize,
) -> Option<(isize, isize)> {
    let down = (sand_position.0, sand_position.1 + 1);
    let diagonal_left = (sand_position.0 - 1, sand_position.1 + 1);
    let diagonal_right = (sand_position.0 + 1, sand_position.1 + 1);

    match map.get(&down) {
        Some(_) => {}
        None => {
            if down.1 <= floor_start {
                return Some(down);
            }
        }
    }

    match map.get(&diagonal_left) {
        Some(_) => {}
        None => {
            if diagonal_left.1 <= floor_start {
                return Some(diagonal_left);
            }
        }
    }

    match map.get(&diagonal_right) {
        Some(_) => {}
        None => {
            if diagonal_right.1 <= floor_start {
                return Some(diagonal_right);
            }
        }
    }

    return None;
}

fn fill_map(formations: &[Formation], map: &mut HashMap<(isize, isize), Spot>) {
    for formation in formations {
        for line in formation.windows(2) {
            let end_1 = line[0];
            let end_2 = line[1];

            if end_1.0 == end_2.0 {
                //vertical line
                let (start, end) = if end_1.1 < end_2.1 {
                    (end_1, end_2)
                } else {
                    (end_2, end_1)
                };

                for y in start.1..(end.1 + 1) {
                    map.insert((end_1.0, y), Spot::Rock);
                }
            } else {
                //horizontal line
                let (start, end) = if end_1.0 < end_2.0 {
                    (end_1, end_2)
                } else {
                    (end_2, end_1)
                };

                for x in start.0..(end.0 + 1) {
                    map.insert((x, end_1.1), Spot::Rock);
                }
            }
        }
    }
}

fn coordinate_parser(input: &str) -> IResult<&str, (isize, isize)> {
    let (input, x) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<isize>()
    })(input)?;
    let (input, _) = nom::character::complete::char(',')(input)?;
    let (input, y) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<isize>()
    })(input)?;

    Ok((input, (x, y)))
}

fn formation_parser(input: &str) -> IResult<&str, Formation> {
    let (input, formation) = separated_list1(tag(" -> "), coordinate_parser)(input)?;

    Ok((input, formation))
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, formations) =
        separated_list1(nom::character::complete::newline, formation_parser)(input)?;

    Ok((input, formations))
}
