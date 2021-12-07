use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    combinator::{map_res, opt},
    multi::separated_list0,
    IResult,
};
use std::collections::HashMap;

enum LineSegment {
    Horizontal(((u32, u32), u32)),
    Vertical((u32, (u32, u32))),
    Diagonal(((u32, u32), (u32, u32))),
}

pub fn part1() -> u32 {
    let input = include_str!("../../data/2021/5.txt");

    match input_parser(input) {
        Ok((remaining_input, line_segments)) if remaining_input.len() == 0 => {
            println!("parsed entire input");
            let occupied_spaces = line_segments
                .iter()
                .filter(|line_segment| match line_segment {
                    LineSegment::Horizontal(_) => true,
                    LineSegment::Vertical(_) => true,
                    LineSegment::Diagonal(_) => false,
                })
                .fold(
                    HashMap::new(),
                    |mut acc: HashMap<(u32, u32), u32>, line_segment| match line_segment {
                        &LineSegment::Horizontal(((starting_x, ending_x), y)) => {
                            for x in starting_x..(ending_x + 1) {
                                let cell = (x, y);
                                match acc.get(&cell) {
                                    Some(current_count) => {
                                        acc.insert(cell, current_count + 1);
                                    }
                                    None => {
                                        acc.insert(cell, 1);
                                    }
                                }
                            }
                            acc
                        }
                        &LineSegment::Vertical((x, (starting_y, ending_y))) => {
                            for y in starting_y..(ending_y + 1) {
                                let cell = (x, y);
                                match acc.get(&cell) {
                                    Some(current_count) => {
                                        acc.insert(cell, current_count + 1);
                                    }
                                    None => {
                                        acc.insert(cell, 1);
                                    }
                                }
                            }
                            acc
                        }
                        &LineSegment::Diagonal(_) => acc,
                    },
                );
            occupied_spaces
                .iter()
                .filter(|(_key, count)| **count > 1)
                .count()
                .try_into()
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

pub fn part2() -> u32 {
    let input = include_str!("../../data/2021/5.txt");

    match input_parser(input) {
        Ok((remaining_input, line_segments)) if remaining_input.len() == 0 => {
            println!("parsed entire input");
            let occupied_spaces = line_segments.iter().fold(
                HashMap::new(),
                |mut acc: HashMap<(u32, u32), u32>, line_segment| match line_segment {
                    &LineSegment::Horizontal(((starting_x, ending_x), y)) => {
                        for x in starting_x..(ending_x + 1) {
                            let cell = (x, y);
                            match acc.get(&cell) {
                                Some(current_count) => {
                                    acc.insert(cell, current_count + 1);
                                }
                                None => {
                                    acc.insert(cell, 1);
                                }
                            }
                        }
                        acc
                    }
                    &LineSegment::Vertical((x, (starting_y, ending_y))) => {
                        for y in starting_y..(ending_y + 1) {
                            let cell = (x, y);
                            match acc.get(&cell) {
                                Some(current_count) => {
                                    acc.insert(cell, current_count + 1);
                                }
                                None => {
                                    acc.insert(cell, 1);
                                }
                            }
                        }
                        acc
                    }
                    &LineSegment::Diagonal(((starting_x, starting_y), (ending_x, ending_y))) => {
                        let mut cell = (starting_x, starting_y);
                        while cell != (ending_x, ending_y) {
                            match acc.get(&cell) {
                                Some(current_count) => {
                                    acc.insert(cell, current_count + 1);
                                }
                                None => {
                                    acc.insert(cell, 1);
                                }
                            }

                            cell = (
                                if cell.0 > ending_x {
                                    cell.0 - 1
                                } else {
                                    cell.0 + 1
                                },
                                if cell.1 > ending_y {
                                    cell.1 - 1
                                } else {
                                    cell.1 + 1
                                },
                            );
                        }
                        //This is not good... Off by one since we need to calculate once the cell is on the ending spot
                        match acc.get(&cell) {
                            Some(current_count) => {
                                acc.insert(cell, current_count + 1);
                            }
                            None => {
                                acc.insert(cell, 1);
                            }
                        }

                        acc
                    }
                },
            );
            occupied_spaces
                .iter()
                .filter(|(_key, count)| **count > 1)
                .count()
                .try_into()
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

fn coordinate_parser(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, x) = map_res(nom::character::complete::digit0, |s: &str| s.parse::<u32>())(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = map_res(nom::character::complete::digit0, |s: &str| s.parse::<u32>())(input)?;
    Ok((input, (x, y)))
}

fn line_segment_parser(input: &str) -> IResult<&str, LineSegment> {
    let (input, start) = coordinate_parser(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, end) = coordinate_parser(input)?;

    if start.0 == end.0 {
        let y_component = if start.1 < end.1 {
            (start.1, end.1)
        } else {
            (end.1, start.1)
        };
        return Ok((input, LineSegment::Vertical((start.0, y_component))));
    }

    if start.1 == end.1 {
        let x_component = if start.0 < end.0 {
            (start.0, end.0)
        } else {
            (end.0, start.0)
        };
        return Ok((input, LineSegment::Horizontal((x_component, end.1))));
    }

    Ok((input, LineSegment::Diagonal((end, start))))
}

fn input_parser(input: &str) -> IResult<&str, Vec<LineSegment>> {
    let (input, lines) =
        separated_list0(nom::character::complete::newline, line_segment_parser)(input)?;

    let (input, _) = nom::character::complete::newline(input)?;

    Ok((input, lines))
}
