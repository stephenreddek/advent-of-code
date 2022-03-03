use itertools::Itertools;
use nom::character::complete::newline;
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

#[derive(Clone)]
struct GameState {
    player_1: PlayerState,
    player_2: PlayerState,
    rolls: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct PlayerState {
    position: usize,
    score: usize,
}

impl GameState {
    fn new(player_1_start: usize, player_2_start: usize) -> Self {
        GameState {
            player_1: PlayerState::new(player_1_start),
            player_2: PlayerState::new(player_2_start),
            rolls: 0,
        }
    }

    fn play_deterministic_round(&mut self) {
        let is_player_1_turn = self.rolls % 2 == 0;
        let rolls = self.roll() + self.roll() + self.roll();
        if is_player_1_turn {
            self.player_1.advance(rolls);
        } else {
            self.player_2.advance(rolls);
        }
    }

    fn roll(&mut self) -> usize {
        self.rolls += 1;
        (self.rolls - 1) % 100 + 1
    }
}

impl PlayerState {
    fn new(start_position: usize) -> Self {
        PlayerState {
            position: start_position,
            score: 0,
        }
    }

    fn from_state(position: usize, score: usize) -> Self {
        PlayerState { position, score }
    }

    fn advance(&mut self, rolls: usize) {
        self.position = (self.position + rolls - 1) % 10 + 1;
        self.score += self.position;
    }
}

#[derive(Clone)]
struct DiracGameState {
    ongoing_games: HashMap<(PlayerState, PlayerState), usize>,
    completed_games: HashMap<(PlayerState, PlayerState), usize>,
    rolls: usize,
}

impl DiracGameState {
    fn new(player_1_start: usize, player_2_start: usize) -> Self {
        let player_1 = PlayerState::new(player_1_start);
        let player_2 = PlayerState::new(player_2_start);
        DiracGameState {
            ongoing_games: vec![((player_1, player_2), 1)].into_iter().collect(),
            completed_games: HashMap::new(),

            rolls: 0,
        }
    }

    fn roll() -> Vec<usize> {
        (1..4)
            .cartesian_product(1..4)
            .cartesian_product(1..4)
            .map(|((a, b), c)| a + b + c)
            .collect_vec()
    }

    fn play_round(&mut self) {
        let is_player_1_turn = self.rolls % 2 == 0;

        self.rolls += 3;

        let rolls = DiracGameState::roll();

        let mut next_ongoing_games = HashMap::new();

        for (((player_1, player_2), count), roll) in
            self.ongoing_games.iter().cartesian_product(rolls)
        {
            if is_player_1_turn {
                let next_position = (player_1.position + roll - 1) % 10 + 1;
                let next_score = player_1.score + next_position;

                if next_score >= 21 {
                    upsert(
                        &mut self.completed_games,
                        (
                            PlayerState::from_state(next_position, next_score),
                            *player_2,
                        ),
                        *count,
                    );
                } else {
                    upsert(
                        &mut next_ongoing_games,
                        (
                            PlayerState::from_state(next_position, next_score),
                            *player_2,
                        ),
                        *count,
                    );
                }
            } else {
                let next_position = (player_2.position + roll - 1) % 10 + 1;
                let next_score = player_2.score + next_position;

                if next_score >= 21 {
                    upsert(
                        &mut self.completed_games,
                        (
                            *player_1,
                            PlayerState::from_state(next_position, next_score),
                        ),
                        *count,
                    );
                } else {
                    upsert(
                        &mut next_ongoing_games,
                        (
                            *player_1,
                            PlayerState::from_state(next_position, next_score),
                        ),
                        *count,
                    );
                }
            }
        }

        self.ongoing_games = next_ongoing_games;
    }
}

fn upsert(
    map: &mut HashMap<(PlayerState, PlayerState), usize>,
    key: (PlayerState, PlayerState),
    val: usize,
) {
    match map.get(&key) {
        Some(&current_val) => {
            map.insert(key, current_val + val);
        }
        None => {
            map.insert(key, val);
        }
    }
}

pub fn part1() -> usize {
    let input = include_str!("../../data/2021/21.txt");

    match input_parser(input) {
        Ok((remaining_input, (player_1_start, player_2_start))) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut state = GameState::new(player_1_start, player_2_start);

            while state.player_1.score < 1000 && state.player_2.score < 1000 {
                state.play_deterministic_round()
            }

            let losing_score = state.player_1.score.min(state.player_2.score);

            losing_score * state.rolls
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
    let input = include_str!("../../data/2021/21.txt");

    match input_parser(input) {
        Ok((remaining_input, (player_1_start, player_2_start))) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut state = DiracGameState::new(player_1_start, player_2_start);

            while !state.ongoing_games.is_empty() {
                state.play_round()
            }

            let (count_of_1_wins, count_of_2_wins) = state.completed_games.iter().fold(
                (0, 0),
                |(player_1_wins, player_2_wins), ((player_1, player_2), count)| {
                    if player_1.score > player_2.score {
                        (player_1_wins + *count, player_2_wins)
                    } else {
                        (player_1_wins, player_2_wins + *count)
                    }
                },
            );

            count_of_1_wins.max(count_of_2_wins)
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

fn parse_number(input: &str) -> IResult<&str, usize> {
    let (input, val) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<usize>()
    })(input)?;

    Ok((input, val))
}

fn input_parser(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, _) = tag("Player 1 starting position: ")(input)?;
    let (input, player_1_start) = parse_number(input)?;

    let (input, _) = newline(input)?;

    let (input, _) = tag("Player 2 starting position: ")(input)?;
    let (input, player_2_start) = parse_number(input)?;

    let (input, _) = newline(input)?;

    Ok((input, (player_1_start, player_2_start)))
}
