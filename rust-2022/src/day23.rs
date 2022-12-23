use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map_res, opt},
    multi::separated_list0,
    multi::{many1, separated_list1},
    IResult,
};
use std::{cmp::Ordering, hash::Hash};
use std::{collections::BinaryHeap, time::SystemTime};
use std::{
    collections::{HashMap, HashSet},
    ptr,
};
use std::{fmt, ops};

type Input = HashSet<ElfSpot>;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct ElfSpot {
    x: isize,
    y: isize,
}

struct BoundingBox {
    min_x: isize,
    min_y: isize,
    max_x: isize,
    max_y: isize,
}

pub fn part1() -> isize {
    let input = include_str!("../../data/2022/23.txt");
    // let input = include_str!("../../data/2022/23-example.txt");

    match input_parser(input) {
        Ok((remaining_input, mut map)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            for round in 0..10 {
                let mut proposed_moves = vec![];
                for elf in map.iter() {
                    if elf.has_neighbors(&map) {
                        if let Some(proposed_spot) = elf.propose_new_spot(&map, round) {
                            proposed_moves.push((*elf, proposed_spot));
                        }
                    }
                }

                for (new_spot, mut proposals_to_spot) in proposed_moves
                    .iter()
                    .sorted_by(|a, b| a.1.cmp(&b.1))
                    .group_by(|(_, spot)| spot)
                    .into_iter()
                {
                    let elf_proposal = proposals_to_spot.next().unwrap();
                    if proposals_to_spot.count() == 0 {
                        //then move
                        map.remove(&elf_proposal.0);
                        map.insert(*new_spot);
                    }
                }
            }

            let smallest_boundary = bounding_box(&map);
            let mut empty_counter: isize = 0;
            for y in smallest_boundary.min_y..(smallest_boundary.max_y + 1) {
                for x in smallest_boundary.min_x..(smallest_boundary.max_x + 1) {
                    if !map.contains(&ElfSpot { x, y }) {
                        empty_counter += 1;
                        // print!(".");
                    } else {
                        // print!("#");
                    }
                }
                // println!();
            }

            empty_counter
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

pub fn part2() -> usize {
    let input = include_str!("../../data/2022/23.txt");
    // let input = include_str!("../../data/2022/23-example.txt");

    match input_parser(input) {
        Ok((remaining_input, mut map)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut round: usize = 0;
            loop {
                let mut moves_occurred = false;

                let mut proposed_moves = vec![];
                for elf in map.iter() {
                    if elf.has_neighbors(&map) {
                        if let Some(proposed_spot) = elf.propose_new_spot(&map, round) {
                            proposed_moves.push((*elf, proposed_spot));
                        }
                    }
                }

                for (new_spot, mut proposals_to_spot) in proposed_moves
                    .iter()
                    .sorted_by(|a, b| a.1.cmp(&b.1))
                    .group_by(|(_, spot)| spot)
                    .into_iter()
                {
                    let elf_proposal = proposals_to_spot.next().unwrap();
                    if proposals_to_spot.count() == 0 {
                        //then move
                        map.remove(&elf_proposal.0);
                        map.insert(*new_spot);
                        moves_occurred = true;
                    }
                }

                if !moves_occurred {
                    return round + 1;
                }

                round += 1;
            }
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

fn bounding_box(elves: &HashSet<ElfSpot>) -> BoundingBox {
    BoundingBox {
        min_x: elves.iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x,
        min_y: elves.iter().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y,
        max_x: elves.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x,
        max_y: elves.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y,
    }
}

impl ElfSpot {
    fn has_neighbors(&self, elves: &HashSet<ElfSpot>) -> bool {
        for y_diff in -1..2 {
            for x_diff in -1..2 {
                if x_diff == 0 && y_diff == 0 {
                    continue;
                }

                if elves.contains(&ElfSpot {
                    x: self.x + x_diff,
                    y: self.y + y_diff,
                }) {
                    return true;
                }
            }
        }

        false
    }

    fn propose_new_spot(&self, elves: &HashSet<ElfSpot>, round_number: usize) -> Option<Self> {
        // If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step.
        // If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
        // If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step.
        // If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes moving east one step.
        for i in round_number..(round_number + 4) {
            if i % 4 == 0 && self.is_north_open(elves) {
                return Some(self.to_north());
            } else if i % 4 == 1 && self.is_south_open(elves) {
                return Some(self.to_south());
            } else if i % 4 == 2 && self.is_west_open(elves) {
                return Some(self.to_west());
            } else if i % 4 == 3 && self.is_east_open(elves) {
                return Some(self.to_east());
            }
        }

        None
    }

    fn is_north_open(&self, elves: &HashSet<ElfSpot>) -> bool {
        for x_diff in -1..2 {
            if elves.contains(&ElfSpot {
                x: self.x + x_diff,
                y: self.y - 1,
            }) {
                return false;
            }
        }

        true
    }

    fn is_south_open(&self, elves: &HashSet<ElfSpot>) -> bool {
        for x_diff in -1..2 {
            if elves.contains(&ElfSpot {
                x: self.x + x_diff,
                y: self.y + 1,
            }) {
                return false;
            }
        }

        true
    }

    fn is_east_open(&self, elves: &HashSet<ElfSpot>) -> bool {
        for y_diff in -1..2 {
            if elves.contains(&ElfSpot {
                x: self.x + 1,
                y: self.y + y_diff,
            }) {
                return false;
            }
        }

        true
    }

    fn is_west_open(&self, elves: &HashSet<ElfSpot>) -> bool {
        for y_diff in -1..2 {
            if elves.contains(&ElfSpot {
                x: self.x - 1,
                y: self.y + y_diff,
            }) {
                return false;
            }
        }

        true
    }

    fn to_north(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn to_south(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn to_east(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn to_west(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
}

fn map_line_parser(input: &str) -> IResult<&str, Vec<char>> {
    let (input, output) =
        many1(nom::character::complete::satisfy(|c| c == '.' || c == '#'))(input)?;
    Ok((input, output))
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, lines) =
        separated_list1(nom::character::complete::newline, map_line_parser)(input)?;

    let mut map = HashSet::new();
    for (y, row) in lines.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                map.insert(ElfSpot {
                    x: x as isize,
                    y: y as isize,
                });
            }
        }
    }

    Ok((input, map))
}

// #[cfg(test)]
// mod day23_tests {
//     use crate::day23;

//     #[test]
//     fn test_1() {}
// }
