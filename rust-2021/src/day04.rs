use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    combinator::{map_res, opt},
    multi::separated_list0,
    IResult,
};
use std::collections::HashMap;

pub fn part1() -> u32 {
    let input = include_str!("../../data/2021/4.txt");

    match input_parser(input) {
        Ok((remaining_input, (values, starting_boards))) if remaining_input.len() == 0 => {
            println!("parsed entire input");
            let mut boards = starting_boards;
            for value in values {
                boards = play_value(value, &boards);
                let winning_board = boards.iter().find_map(|board| score_board(board));
                if let Some(ref score) = winning_board {
                    return (*score) * value;
                }
            }

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

pub fn part2() -> u32 {
    let input = include_str!("../../data/2021/4.txt");

    match input_parser(input) {
        Ok((remaining_input, (values, starting_boards))) if remaining_input.len() == 0 => {
            println!("parsed entire input");
            let mut boards = starting_boards;
            for value in values {
                boards = play_value(value, &boards);
                if boards.len() > 1 {
                    boards = boards.iter().filter_map(|board|
                        //remove winning boards
                        if let Some(ref score) = score_board(board) {
                            None
                        } else {
                            Some(board.clone())
                        }
                    ).collect_vec();
                } else {
                    if let Some(ref score) = score_board(boards.first().unwrap()) {
                        return (*score) * value;
                    }
                }
            }

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

fn board_spot_parser(input: &str) -> IResult<&str, (u32, bool)> {
    let (input, _) = opt(tag(" "))(input)?;
    let (input, spot) =
        map_res(nom::character::complete::digit0, |s: &str| s.parse::<u32>())(input)?;

    Ok((input, (spot, false)))
}
fn board_line_parser(input: &str) -> IResult<&str, Vec<(u32, bool)>> {
    let (input, line) = separated_list0(tag(" "), board_spot_parser)(input)?;
    let (input, _) = nom::character::complete::newline(input)?;
    Ok((input, line))
}

fn board_parser(input: &str) -> IResult<&str, HashMap<(u32, u32), (u32, bool)>> {
    let (input, board) = nom::multi::count(board_line_parser, 5)(input)?;

    let mut card: HashMap<(u32, u32), (u32, bool)> = HashMap::new();
    for x in 0..board.len() {
        let line = &board[x];
        for y in 0..line.len() {
            card.insert((x.try_into().unwrap(), y.try_into().unwrap()), line[y]);
        }
    }

    Ok((input, card))
}

fn input_parser(input: &str) -> IResult<&str, (Vec<u32>, Vec<HashMap<(u32, u32), (u32, bool)>>)> {
    let (input, values) = separated_list0(
        tag(","),
        map_res(nom::character::complete::digit0, |s: &str| s.parse::<u32>()),
    )(input)?;

    let (input, _) = nom::character::complete::newline(input)?;
    let (input, _) = nom::character::complete::newline(input)?;

    let (input, boards) = separated_list0(nom::character::complete::newline, board_parser)(input)?;

    let parse_result = (values, boards);

    Ok((input, parse_result))
}

fn play_value(
    new_value: u32,
    boards: &Vec<HashMap<(u32, u32), (u32, bool)>>,
) -> Vec<HashMap<(u32, u32), (u32, bool)>> {
    boards
        .iter()
        .map(|board| play_value_on_board(new_value, board))
        .collect_vec()
}

fn play_value_on_board(
    new_value: u32,
    board: &HashMap<(u32, u32), (u32, bool)>,
) -> HashMap<(u32, u32), (u32, bool)> {
    board
        .iter()
        .map(|((x, y), (value, set))| ((*x, *y), (*value, *set || *value == new_value)))
        .collect()
}

fn sum_of_unset(board: &HashMap<(u32, u32), (u32, bool)>) -> u32 {
    board.iter().fold(
        0,
        |sum, ((_, _), (value, set))| if *set { sum } else { sum + *value },
    )
}

fn score_board(board: &HashMap<(u32, u32), (u32, bool)>) -> Option<u32> {
    for x in 0u32..5u32 {
        let is_winning = is_winning_column(x, board);
        if is_winning {
            return Some(sum_of_unset(board));
        };
    }

    for y in 0u32..5u32 {
        let is_winning = is_winning_row(y, board);
        if is_winning {
            return Some(sum_of_unset(board));
        };
    }

    None
}

fn is_winning_column(x: u32, board: &HashMap<(u32, u32), (u32, bool)>) -> bool {
    let mut all_are_set = true;
    for y in 0u32..5u32 {
        let cell = (x, y);
        if let Some((_, set)) = board.get(&cell) {
            all_are_set = all_are_set && *set;
        }
    }

    all_are_set
}

fn is_winning_row(y: u32, board: &HashMap<(u32, u32), (u32, bool)>) -> bool {
    let mut all_are_set = true;
    for x in 0u32..5u32 {
        let cell = (x, y);
        if let Some((_, set)) = board.get(&cell) {
            all_are_set = all_are_set && *set;
        }
    }

    all_are_set
}
