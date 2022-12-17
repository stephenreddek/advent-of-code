use itertools::Itertools;
use nom::{
    branch::alt,
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

type Input = Vec<Direction>;

type Formation = Vec<(isize, isize)>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum StepResult {
    Settled(Formation),
    Moved(Formation),
}

fn get_formations() -> Vec<Formation> {
    let minus = vec![(2, 0), (3, 0), (4, 0), (5, 0)];
    let plus = vec![(3, 2), (2, 1), (3, 1), (4, 1), (3, 0)];
    let corner = vec![(4, 2), (4, 1), (4, 0), (3, 0), (2, 0)];
    let pipe = vec![(2, 0), (2, 1), (2, 2), (2, 3)];
    let square = vec![(2, 0), (3, 0), (2, 1), (3, 1)];
    vec![minus, plus, corner, pipe, square]
}

pub fn part1() -> isize {
    let input = include_str!("../../data/2022/17.txt");

    match input_parser(input) {
        Ok((remaining_input, shifts)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let formations = get_formations();
            let mut grid: HashSet<(isize, isize)> = HashSet::new();
            let mut rock = next_rock(&grid, &(formations[0]));
            let mut round = 0;
            let mut total_rock_count = 1;
            while total_rock_count <= 2022 {
                let shift = shifts[round % shifts.len()];
                match step(&grid, &rock, &shift) {
                    StepResult::Settled(new_position) => {
                        grid.extend(new_position.iter());
                        total_rock_count += 1;
                        rock = next_rock(&grid, &(formations[(total_rock_count - 1) % 5]));
                    }
                    StepResult::Moved(new_position) => rock = new_position,
                }

                // print_grid(&grid, Some(&rock));
                // println!("\n\n");
                round += 1;
            }

            // print_grid(&grid, None);

            max_height(&grid).unwrap_or(0) + 1
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

pub fn part2() -> isize {
    let input = include_str!("../../data/2022/17.txt");

    match input_parser(input) {
        Ok((remaining_input, shifts)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let formations = get_formations();
            let mut grid: HashSet<(isize, isize)> = HashSet::new();
            let mut total_rock_count: u64 = 1;
            let mut floor_height = 0;
            let mut memory: HashMap<(usize, usize), (u64, isize)> = HashMap::new();

            let mut shift_infinite_iter = shifts.iter().enumerate().cycle();
            let mut formation_infinite_iter = formations.iter().enumerate().cycle();

            let mut rock = next_rock(&grid, formation_infinite_iter.next().unwrap().1);
            let goal_rocks = 1_000_000_000_000;
            // let goal_rocks = 2022;
            while total_rock_count <= goal_rocks {
                // if round % total_shifts == 0 && (total_rock_count - 1) % total_formations == 0 {
                //     // println!("{}%", total_rock_count * 100 / 1_000_000_000_000);
                //     println!(
                //         "\n\n\ndone with round. height: {}\n\n\n",
                //         max_height(&grid).unwrap_or(0) + floor_height
                //     );
                // }

                let (shift_index, shift) = shift_infinite_iter.next().unwrap();
                match step(&grid, &rock, shift) {
                    StepResult::Settled(new_position) => {
                        grid.extend(new_position.iter());
                        total_rock_count += 1;
                        let (rock_index, next_formation) = formation_infinite_iter.next().unwrap();
                        if let Some(new_floor) = find_new_floor(&grid, &new_position) {
                            // println!("found new floor {}", new_floor);
                            grid = shift_grid(&grid, new_floor);
                            floor_height += new_floor;

                            if let Some(&previous) = memory.get(&(shift_index, rock_index)) {
                                println!("found a repeat! {:?}", previous);
                                let rock_diff = total_rock_count - previous.0;
                                let floor_diff = floor_height - previous.1;
                                let remaining_rocks_to_try = goal_rocks - total_rock_count;
                                let multiplier = remaining_rocks_to_try / rock_diff;
                                total_rock_count += rock_diff * multiplier;
                                floor_height += floor_diff * multiplier as isize;
                                memory.clear();
                            }

                            memory.insert(
                                (shift_index, rock_index),
                                (total_rock_count, floor_height),
                            );
                        }

                        rock = next_rock(&grid, next_formation);
                    }
                    StepResult::Moved(new_position) => rock = new_position,
                }
            }

            max_height(&grid).unwrap_or(0) + floor_height + 1
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

fn find_new_floor(grid: &HashSet<(isize, isize)>, latest_rock: &[(isize, isize)]) -> Option<isize> {
    let bottom_of_rock = latest_rock.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let top_of_rock = latest_rock.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    for y in bottom_of_rock..(top_of_rock + 1) {
        if (0..7).all(|x| grid.contains(&(x, y))) {
            return Some(y);
        }
    }

    None
}

fn shift_grid(grid: &HashSet<(isize, isize)>, new_floor: isize) -> HashSet<(isize, isize)> {
    // println!("shifting by {}", new_floor);

    let mut shifted_grid = HashSet::new();

    for &(x, y) in grid.iter() {
        if y >= new_floor {
            shifted_grid.insert((x, y - new_floor));
        }
    }

    shifted_grid
}

fn print_grid(grid: &HashSet<(isize, isize)>, rock: Option<&Formation>) {
    let formation = rock.map_or(HashSet::new(), |v| HashSet::from_iter(v.iter()));
    for y in (0..20).rev() {
        for x in 0..7 {
            if grid.contains(&(x, y)) || formation.contains(&(x, y)) {
                print!("{}", "#");
            } else {
                print!("{}", ".");
            }
        }
        println!();
    }
}

//this doesn't have to be a calculated value
fn max_height(grid: &HashSet<(isize, isize)>) -> Option<isize> {
    grid.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|a| a.1)
}

fn next_rock(grid: &HashSet<(isize, isize)>, base_formation: &Formation) -> Formation {
    let height_of_settled = max_height(grid);
    //there needs to be 3 spaces
    let bottom_of_formation = height_of_settled.map_or(3, |height| height + 4);
    let formation = base_formation
        .iter()
        .map(|&(x, y)| (x, y + bottom_of_formation))
        .collect_vec();

    formation
}

fn step(grid: &HashSet<(isize, isize)>, rock: &[(isize, isize)], shift: &Direction) -> StepResult {
    let falling_rock = shift_direction(shift, rock, grid);
    match shift_formation_down(&falling_rock, grid) {
        Some(new_position) => StepResult::Moved(new_position),
        None => StepResult::Settled(falling_rock),
    }
}

fn shift_direction(
    direction: &Direction,
    rock: &[(isize, isize)],
    grid: &HashSet<(isize, isize)>,
) -> Vec<(isize, isize)> {
    match direction {
        Direction::Left => shift_formation_left(rock, grid),
        Direction::Right => shift_formation_right(rock, grid),
    }
}

fn shift_formation_left(
    rock: &[(isize, isize)],
    grid: &HashSet<(isize, isize)>,
) -> Vec<(isize, isize)> {
    let mut result = vec![(0, 0); rock.len()];
    for (index, &(x, y)) in rock.iter().enumerate() {
        //formation is already on the left side
        if x == 0 || grid.contains(&(x - 1, y)) {
            return rock.to_owned();
        }

        result[index] = (x - 1, y);
    }

    result
}

fn shift_formation_right(
    rock: &[(isize, isize)],
    grid: &HashSet<(isize, isize)>,
) -> Vec<(isize, isize)> {
    let mut result = vec![(0, 0); rock.len()];
    for (index, &(x, y)) in rock.iter().enumerate() {
        //formation is already on the right side
        if x == 6 || grid.contains(&(x + 1, y)) {
            return rock.to_owned();
        }

        result[index] = (x + 1, y);
    }

    result
}

fn shift_formation_down(
    rock: &[(isize, isize)],
    grid: &HashSet<(isize, isize)>,
) -> Option<Vec<(isize, isize)>> {
    let mut result = vec![(0, 0); rock.len()];
    for (index, &(x, y)) in rock.iter().enumerate() {
        //formation is already on the ground or hits another piece
        if y == 0 || grid.contains(&(x, y - 1)) {
            return None;
        }

        result[index] = (x, y - 1);
    }

    Some(result)
}

fn left_parser(input: &str) -> IResult<&str, Direction> {
    let (input, _) = nom::character::complete::char('<')(input)?;

    Ok((input, Direction::Left))
}

fn right_parser(input: &str) -> IResult<&str, Direction> {
    let (input, _) = nom::character::complete::char('>')(input)?;

    Ok((input, Direction::Right))
}

fn direction_parser(input: &str) -> IResult<&str, Direction> {
    let (input, direction) = alt((left_parser, right_parser))(input)?;

    Ok((input, direction))
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, moves) = many1(direction_parser)(input)?;

    Ok((input, moves))
}
