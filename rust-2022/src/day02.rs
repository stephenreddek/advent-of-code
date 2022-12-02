use itertools::Itertools;
use nom::{
    branch::alt, bytes::complete::tag, combinator::map_res, multi::separated_list0, IResult,
};

#[derive(Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone)]
enum RoundResult {
    Loss,
    Win,
    Draw,
}

pub fn part1() -> usize {
    let input = include_str!("../../data/2022/02.txt");

    match part1_input_parser(input) {
        Ok((remaining_input, rounds)) if remaining_input.is_empty() => {
            println!("parsed entire input");
            rounds.iter().map(score_round).sum::<usize>()
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
    let input = include_str!("../../data/2022/02.txt");

    match part2_input_parser(input) {
        Ok((remaining_input, rounds)) if remaining_input.is_empty() => {
            println!("parsed entire input");
            rounds
                .iter()
                .map(|&(opponent, result)| -> (Shape, Shape) {
                    (opponent, select_reaction(&(opponent, result)))
                })
                .map(|(opponent, reaction)| score_round(&(opponent, reaction)))
                .sum::<usize>()
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

fn score_round(selections: &(Shape, Shape)) -> usize {
    let result = match &selections {
        (Shape::Rock, Shape::Paper) => RoundResult::Win,
        (Shape::Rock, Shape::Rock) => RoundResult::Draw,
        (Shape::Rock, Shape::Scissors) => RoundResult::Loss,

        (Shape::Paper, Shape::Paper) => RoundResult::Draw,
        (Shape::Paper, Shape::Rock) => RoundResult::Loss,
        (Shape::Paper, Shape::Scissors) => RoundResult::Win,

        (Shape::Scissors, Shape::Paper) => RoundResult::Loss,
        (Shape::Scissors, Shape::Rock) => RoundResult::Win,
        (Shape::Scissors, Shape::Scissors) => RoundResult::Draw,
    };

    let result_score = match result {
        RoundResult::Loss => 0,
        RoundResult::Draw => 3,
        RoundResult::Win => 6,
    };

    result_score + score_selection(&selections.1)
}

fn score_selection(selection: &Shape) -> usize {
    match &selection {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn select_reaction(input: &(Shape, RoundResult)) -> Shape {
    match &input {
        (Shape::Rock, RoundResult::Draw) => Shape::Rock,
        (Shape::Rock, RoundResult::Win) => Shape::Paper,
        (Shape::Rock, RoundResult::Loss) => Shape::Scissors,

        (Shape::Paper, RoundResult::Draw) => Shape::Paper,
        (Shape::Paper, RoundResult::Win) => Shape::Scissors,
        (Shape::Paper, RoundResult::Loss) => Shape::Rock,

        (Shape::Scissors, RoundResult::Draw) => Shape::Scissors,
        (Shape::Scissors, RoundResult::Win) => Shape::Rock,
        (Shape::Scissors, RoundResult::Loss) => Shape::Paper,
    }
}

fn opponent_parser(input: &str) -> IResult<&str, Shape> {
    let rock_parser = map_res(
        nom::character::complete::char('A'),
        |_| -> Result<Shape, ()> { Ok(Shape::Rock) },
    );
    let paper_parser = map_res(
        nom::character::complete::char('B'),
        |_| -> Result<Shape, ()> { Ok(Shape::Paper) },
    );
    let scissors_parser = map_res(
        nom::character::complete::char('C'),
        |_| -> Result<Shape, ()> { Ok(Shape::Scissors) },
    );
    alt((rock_parser, paper_parser, scissors_parser))(input)
}

fn reaction_parser(input: &str) -> IResult<&str, Shape> {
    let rock_parser = map_res(
        nom::character::complete::char('X'),
        |_| -> Result<Shape, ()> { Ok(Shape::Rock) },
    );
    let paper_parser = map_res(
        nom::character::complete::char('Y'),
        |_| -> Result<Shape, ()> { Ok(Shape::Paper) },
    );
    let scissors_parser = map_res(
        nom::character::complete::char('Z'),
        |_| -> Result<Shape, ()> { Ok(Shape::Scissors) },
    );
    alt((rock_parser, paper_parser, scissors_parser))(input)
}

fn result_parser(input: &str) -> IResult<&str, RoundResult> {
    let loss_parser = map_res(
        nom::character::complete::char('X'),
        |_| -> Result<RoundResult, ()> { Ok(RoundResult::Loss) },
    );
    let draw_parser = map_res(
        nom::character::complete::char('Y'),
        |_| -> Result<RoundResult, ()> { Ok(RoundResult::Draw) },
    );
    let win_parser = map_res(
        nom::character::complete::char('Z'),
        |_| -> Result<RoundResult, ()> { Ok(RoundResult::Win) },
    );
    alt((loss_parser, draw_parser, win_parser))(input)
}

fn part1_round_parser(input: &str) -> IResult<&str, (Shape, Shape)> {
    let (input, x) = opponent_parser(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, y) = reaction_parser(input)?;
    Ok((input, (x, y)))
}

fn part2_round_parser(input: &str) -> IResult<&str, (Shape, RoundResult)> {
    let (input, x) = opponent_parser(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, y) = result_parser(input)?;
    Ok((input, (x, y)))
}

fn part1_input_parser(input: &str) -> IResult<&str, Vec<(Shape, Shape)>> {
    let (input, lines) =
        separated_list0(nom::character::complete::newline, part1_round_parser)(input)?;

    Ok((input, lines))
}

fn part2_input_parser(input: &str) -> IResult<&str, Vec<(Shape, RoundResult)>> {
    let (input, lines) =
        separated_list0(nom::character::complete::newline, part2_round_parser)(input)?;

    Ok((input, lines))
}
