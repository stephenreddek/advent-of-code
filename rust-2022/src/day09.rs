use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use nom::{
    bytes::complete::tag, combinator::map_res, multi::separated_list0, multi::separated_list1,
    IResult,
};

type Input = Vec<Move>;

#[derive(Debug, Clone, Copy)]
enum Move {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

pub fn part1() -> usize {
    let input = include_str!("../../data/2022/09.txt");

    match input_parser(input) {
        Ok((remaining_input, moves)) if remaining_input.is_empty() => {
            println!("parsed entire input");
            let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();

            let mut head_coordinate = (0, 0);
            let mut tail_coordinate = (0, 0);
            tail_positions.insert(tail_coordinate);
            for step in moves.iter().flat_map(expand_move) {
                simulate_movement(&mut head_coordinate, &mut tail_coordinate, step);
                tail_positions.insert(tail_coordinate);
            }

            tail_positions.len()
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
    let input = include_str!("../../data/2022/09.txt");

    match input_parser(input) {
        Ok((remaining_input, moves)) if remaining_input.is_empty() => {
            println!("parsed entire input");
            let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();

            //0 will be the head, 8 will be the tail
            let mut rope_positions = vec![(0, 0); 10];

            // print_map(&rope_positions);
            tail_positions.insert((0, 0));
            for (_step_num, step) in moves.iter().flat_map(expand_move).enumerate() {
                let mut head_coordinate = *rope_positions.get(0).unwrap();
                let mut next_coordinate = *rope_positions.get(1).unwrap();
                simulate_movement(&mut head_coordinate, &mut next_coordinate, step);
                rope_positions[0] = head_coordinate;
                rope_positions[1] = next_coordinate;

                for index in 1..(rope_positions.len() - 1) {
                    let head_coordinate = *rope_positions.get(index).unwrap();
                    let mut next_coordinate = *rope_positions.get(index + 1).unwrap();
                    pull_tail(&head_coordinate, &mut next_coordinate);
                    rope_positions[index + 1] = next_coordinate;
                }
                tail_positions.insert(*rope_positions.last().unwrap());

                // println!("{:?}", *rope_positions.get(8).unwrap());
                // println!("After Step {}:", step_num);
                // print_map(&rope_positions);
            }

            // print_map(&rope_positions);
            tail_positions.len()
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

fn expand_move(m: &Move) -> Vec<Move> {
    match m {
        Move::Up(amount) => vec![Move::Up(1); *amount],
        Move::Down(amount) => vec![Move::Down(1); *amount],
        Move::Left(amount) => vec![Move::Left(1); *amount],
        Move::Right(amount) => vec![Move::Right(1); *amount],
    }
}

fn simulate_movement(
    head_coordinate: &mut (i32, i32),
    tail_coordinate: &mut (i32, i32),
    movement: Move,
) {
    match movement {
        Move::Up(_) => {
            head_coordinate.1 += 1;
            pull_tail(head_coordinate, tail_coordinate)
        }
        Move::Down(_) => {
            head_coordinate.1 -= 1;
            pull_tail(head_coordinate, tail_coordinate)
        }
        Move::Left(_) => {
            head_coordinate.0 -= 1;
            pull_tail(head_coordinate, tail_coordinate)
        }
        Move::Right(_) => {
            head_coordinate.0 += 1;
            pull_tail(head_coordinate, tail_coordinate)
        }
    }
}

fn pull_tail(head_coordinate: &(i32, i32), tail_coordinate: &mut (i32, i32)) {
    if !too_far(head_coordinate, tail_coordinate) {
        return;
    }

    if tail_coordinate.0 == head_coordinate.0 {
        //then fix the y
        if tail_coordinate.1 < head_coordinate.1 {
            tail_coordinate.1 += 1;
        } else {
            tail_coordinate.1 -= 1;
        }
    } else if tail_coordinate.1 == head_coordinate.1 {
        //fix the x
        if tail_coordinate.0 < head_coordinate.0 {
            tail_coordinate.0 += 1;
        } else {
            tail_coordinate.0 -= 1;
        }
    } else {
        //diagonal move
        if tail_coordinate.1 < head_coordinate.1 {
            tail_coordinate.1 += 1;
        } else {
            tail_coordinate.1 -= 1;
        }
        if tail_coordinate.0 < head_coordinate.0 {
            tail_coordinate.0 += 1;
        } else {
            tail_coordinate.0 -= 1;
        }
    }
}

fn print_map(rope: &[(i32, i32)]) {
    let mut grid = HashMap::<(i32, i32), char>::new();

    for (index, position) in rope.iter().enumerate() {
        print!("{}:{:?},", index, position);
        if index == 0 {
            grid.insert(*position, 'H');
        } else {
            match grid.get(position) {
                Some(_) => {
                    continue;
                }
                None => {
                    grid.insert(*position, index.to_string().chars().next().unwrap());
                }
            }
        }
    }
    println!();

    for y in (-5..16).rev() {
        for x in -11..12 {
            match grid.get(&(x, y)) {
                Some(c) => {
                    print!("{}", c);
                }
                None => {
                    print!(".");
                }
            }
        }

        println!();
    }
}

fn too_far(a: &(i32, i32), b: &(i32, i32)) -> bool {
    let x_gap = a.0.abs_diff(b.0);
    let y_gap = a.1.abs_diff(b.1);

    x_gap > 1 || y_gap > 1
}

fn up_parser(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("U ")(input)?;
    let (input, amount) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<usize>()
    })(input)?;

    Ok((input, Move::Up(amount)))
}

fn down_parser(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("D ")(input)?;
    let (input, amount) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<usize>()
    })(input)?;

    Ok((input, Move::Down(amount)))
}

fn left_parser(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("L ")(input)?;
    let (input, amount) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<usize>()
    })(input)?;

    Ok((input, Move::Left(amount)))
}

fn right_parser(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("R ")(input)?;
    let (input, amount) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<usize>()
    })(input)?;

    Ok((input, Move::Right(amount)))
}

fn move_parser(input: &str) -> IResult<&str, Move> {
    nom::branch::alt((up_parser, down_parser, left_parser, right_parser))(input)
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, lines) = separated_list1(nom::character::complete::newline, move_parser)(input)?;

    Ok((input, lines))
}
