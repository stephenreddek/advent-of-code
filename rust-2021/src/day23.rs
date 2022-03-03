use itertools::Itertools;
use nom::{
    bytes::complete::tag, combinator::map_res, multi::separated_list1, sequence::separated_pair,
    IResult,
};
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::ops;
use std::os::macos::raw::stat;
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
};

#[derive(Eq, PartialEq, Clone, Debug)]
struct Move {
    to_move: char,
    start: (usize, usize),
    end: (usize, usize),
}

impl Move {
    fn cost(&self) -> usize {
        if self.start.1 == 1 || self.end.1 == 1 {
            //going to or from the hallway means manhattan can work
            manhattan_distance(self.start, self.end) * Move::cost_modifier(self.to_move)
        } else {
            //going from a room to a room.
            // we need to calculate the cost to go up to the hallway too
            let lateral_cost = (self.start.0 as isize - self.end.0 as isize).abs() as usize;
            let cost_to_hallway =
                (self.start.1 as isize - 1) as usize + (self.end.1 as isize - 1) as usize;
            (lateral_cost + cost_to_hallway) * Move::cost_modifier(self.to_move)
        }
    }

    fn cost_modifier(c: char) -> usize {
        match c {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => {
                panic!("cannot move a {}", c)
            }
        }
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost().cmp(&other.cost())
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq)]
struct UnstructuredState {
    moves: usize,
    history: Vec<Move>,
    map: HashMap<(usize, usize), char>,
}

impl UnstructuredState {
    fn from_map(map: HashMap<(usize, usize), char>) -> Self {
        UnstructuredState {
            moves: 0,
            history: vec![],
            map,
        }
    }

    fn is_complete(&self) -> bool {
        for (&position, &c) in self.map.iter() {
            if !self.is_in_final_destination(position, c) {
                return false;
            }
        }

        true
    }

    fn apply_move(&self, m: Move) -> Self {
        let mut new_map = self.map.clone();

        new_map.insert(m.start, '.');
        new_map.insert(m.end, m.to_move);

        let mut new_history = self.history.clone();
        new_history.push(m.clone());

        UnstructuredState {
            moves: self.moves + m.cost(),
            history: new_history,
            map: new_map,
        }
    }

    fn is_in_final_destination(&self, (x, y): (usize, usize), c: char) -> bool {
        if !(('A'..'E').contains(&c)) {
            return true;
        }

        let expected_x = (c as usize - 'A' as usize) * 2 + 3;

        if x != expected_x {
            return false;
        }

        if y == 5 {
            //bottom spot
            true
        } else if y > 1 {
            //check all the spots below
            for to_check in (y + 1)..6 {
                if *self.map.get(&(x, to_check)).unwrap() != c {
                    return false;
                }
                // && self.is_in_final_destination((x, to_check), c)
            }

            true
        } else {
            //not in a final room
            false
        }
    }

    fn is_in_hallway(&self, (x, y): (usize, usize)) -> bool {
        y == 1
    }

    fn is_move_blocked(&self, m: &Move) -> bool {
        //it's blocked if there's another item in the hallway that's between current and destination in the x direction

        //if starting in the bottom row
        if m.start.1 > 2 {
            for y in 1..(m.start.1) {
                if self.map.get(&(m.start.0, y)) != Some(&'.') {
                    return true;
                }
            }
        }

        //if ending in the bottom row
        if m.end.1 > 2 {
            for y in 1..(m.end.1) {
                if self.map.get(&(m.end.0, y)) != Some(&'.') {
                    return true;
                }
            }
        }

        let x_start = m.start.0.min(m.end.0);
        let x_end = m.start.0.max(m.end.0);

        //these modifiers may be wrong
        for x in (x_start + 1)..x_end {
            if self.map.get(&(x, 1)) != Some(&'.') {
                return true;
            }
        }

        false
    }

    fn is_in_room_opening(&(x, y): &(usize, usize)) -> bool {
        y == 1 && (x == 3 || x == 5 || x == 7 || x == 9)
    }

    fn is_valid_move(&self, possible_move: &Move) -> bool {
        if UnstructuredState::is_in_room_opening(&possible_move.end) {
            return false;
        }

        if self.is_in_hallway(possible_move.start) {
            if self.is_in_hallway(possible_move.end) {
                return false;
            }

            if !self.is_in_final_destination(possible_move.end, possible_move.to_move) {
                return false;
            }
        } else if !self.is_in_hallway(possible_move.end)
            && !self.is_in_final_destination(possible_move.end, possible_move.to_move)
        {
            return false;
        }

        if self.is_move_blocked(possible_move) {
            return false;
        }

        true
    }

    fn print(&self) -> String {
        let mut output = String::new();
        for y in 0..7 {
            for x in 0..13 {
                if let Some(c) = self.map.get(&(x, y)) {
                    output.push(*c);
                }
            }

            output.push('\n');
        }

        output
    }
}

impl Ord for UnstructuredState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.moves.cmp(&other.moves)
    }
}

impl PartialOrd for UnstructuredState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1() -> usize {
    return 0;
    let input = include_str!("../../data/2021/23.txt");

    let initial_state = input_parser(input);

    println!("Parsed! running");

    let mut states_to_consider: BinaryHeap<Reverse<UnstructuredState>> = BinaryHeap::new();
    states_to_consider.push(Reverse(initial_state));

    //hacky
    let mut states_seen_so_far: HashMap<String, usize> = HashMap::new();

    loop {
        if let Some(Reverse(state_to_try_next)) = states_to_consider.pop() {
            if state_to_try_next.is_complete() {
                println!("complete");
                println!("{}", state_to_try_next.print());
                println!("how we got here: {:?}", state_to_try_next.history);
                return state_to_try_next.moves;
            }

            if let Some(&minimum_score) = states_seen_so_far.get(&state_to_try_next.print()) {
                if state_to_try_next.moves >= minimum_score {
                    println!("skipping dupe");
                    continue;
                }
            }

            states_seen_so_far.insert(state_to_try_next.print(), state_to_try_next.moves);

            let possible_moves = generate_possible_moves(&state_to_try_next);
            for possible_move in possible_moves {
                states_to_consider.push(Reverse(state_to_try_next.apply_move(possible_move)))
            }

            println!();
            println!("{}", state_to_try_next.print());
            println!();

            println!(
                "current cost {} states to consider: {}",
                state_to_try_next.moves,
                states_to_consider.len()
            );

            println!();
            println!();
        } else {
            panic!("failed to have a state to consider before completing")
        }
    }
}

pub fn part2() -> usize {
    let input = include_str!("../../data/2021/23.txt");

    let initial_state = input_parser(input);

    println!("Parsed! running");

    let mut states_to_consider: BinaryHeap<Reverse<UnstructuredState>> = BinaryHeap::new();
    states_to_consider.push(Reverse(initial_state));

    //hacky
    let mut states_seen_so_far: HashMap<String, usize> = HashMap::new();

    loop {
        if let Some(Reverse(state_to_try_next)) = states_to_consider.pop() {
            if state_to_try_next.is_complete() {
                println!("complete");
                println!("{}", state_to_try_next.print());
                println!("how we got here: {:?}", state_to_try_next.history);
                return state_to_try_next.moves;
            }

            if let Some(&minimum_score) = states_seen_so_far.get(&state_to_try_next.print()) {
                if state_to_try_next.moves >= minimum_score {
                    println!("skipping dupe");
                    continue;
                }
            }

            states_seen_so_far.insert(state_to_try_next.print(), state_to_try_next.moves);

            let possible_moves = generate_possible_moves(&state_to_try_next);
            for possible_move in possible_moves {
                states_to_consider.push(Reverse(state_to_try_next.apply_move(possible_move)))
            }

            println!();
            println!("{}", state_to_try_next.print());
            println!();

            println!(
                "current cost {} states to consider: {}",
                state_to_try_next.moves,
                states_to_consider.len()
            );

            println!();
            println!();
        } else {
            panic!("failed to have a state to consider before completing")
        }
    }
}

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    (a.0 as isize - b.0 as isize).abs() as usize + (a.1 as isize - b.1 as isize).abs() as usize
}

fn generate_possible_moves(state: &UnstructuredState) -> Vec<Move> {
    let mut moves = Vec::new();
    for (&position, &c) in state.map.iter() {
        if !(('A'..'E').contains(&c)) {
            continue;
        }

        if state.is_in_final_destination(position, c) {
            continue;
        }

        for (&possible_destination_position, _) in state
            .map
            .iter()
            .filter(|(_, &destination_c)| destination_c == '.')
        {
            let possible_move = Move {
                to_move: c,
                start: position,
                end: possible_destination_position,
            };

            if state.is_valid_move(&possible_move) {
                moves.push(possible_move)
            }
        }
    }

    moves
}

fn input_parser(input: &str) -> UnstructuredState {
    let mut state = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            state.insert((x, y), c);
        }
    }

    UnstructuredState::from_map(state)
}

#[cfg(test)]
mod day23_tests {
    use crate::day23::input_parser;
    use crate::day23::Move;
    use crate::day23::UnstructuredState;

    #[test]
    fn test_invalid_moves_are_marked_invalid() {
        let input = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";
        let state = input_parser(input);

        let test_move = Move {
            to_move: 'B',
            start: (3, 2), //far left, top B
            end: (4, 1),
        };

        assert!(state.is_valid_move(&test_move));

        let test_move = Move {
            to_move: 'B',
            start: (3, 2), //far left, top B
            end: (2, 1),
        };

        assert!(state.is_valid_move(&test_move));

        let test_move = Move {
            to_move: 'B',
            start: (3, 2), //far left, top B
            end: (3, 1),
        };

        assert!(!state.is_valid_move(&test_move));

        let test_move = Move {
            to_move: 'A',
            start: (3, 3), //far left, top A
            end: (4, 1),
        };

        assert!(!state.is_valid_move(&test_move));
    }

    #[test]
    fn test_moves_where_there_is_a_block() {
        let input = "#############
#...A.....AD#
###B#C#B#.###
  #.#D#C#.#
  #########
";
        let state = input_parser(input);

        let test_move = Move {
            to_move: 'B',
            start: (3, 2), //far left, top B
            end: (6, 1),
        };

        assert!(!state.is_valid_move(&test_move));

        let test_move = Move {
            to_move: 'A',
            start: (4, 4), //far left, top A
            end: (3, 3),
        };

        assert!(!state.is_valid_move(&test_move));

        let test_move = Move {
            to_move: 'D',
            start: (11, 4), //far right, top D
            end: (9, 3),
        };

        assert!(!state.is_valid_move(&test_move));

        let input = "#############
#.......A..D#
###B#C#B#.###
  #A#D#C#.#
  #########
";
        let state = input_parser(input);

        let test_move = Move {
            to_move: 'A',
            start: (3, 3), //bottom left A
            end: (1, 1),
        };

        assert!(!state.is_valid_move(&test_move));

        let test_move = Move {
            to_move: 'D',
            start: (11, 1), //top right D
            end: (9, 2),
        };

        assert!(!state.is_valid_move(&test_move));

        let input = "#############
#.A.B.C...D.#
###B#.#.#.###
  #A#D#C#.#
  #########
";
        let state = input_parser(input);

        let test_move = Move {
            to_move: 'D',
            start: (5, 3),
            end: (9, 3),
        };

        assert!(!state.is_valid_move(&test_move));
    }

    #[test]
    fn test_final_state_determination() {
        let input = "#############
#...A.....A.#
###B#C#B#D###
  #D#C#C#A#
  #D#B#A#C#
  #.#D#C#.#
  #########
";
        let state = input_parser(input);

        assert!(state.is_in_final_destination((7, 5), 'C'));
        assert!(!state.is_in_final_destination((7, 3), 'C'));
        assert!(!state.is_in_final_destination((5, 3), 'D'));
        assert!(!state.is_in_final_destination((9, 2), 'D'));

        let input = "#############
#...........#
###B#C#B#D###
  #A#C#B#A#
  #A#B#A#D#
  #A#D#C#A#
  #########
";
        let state = input_parser(input);

        assert!(state.is_in_final_destination((7, 5), 'C'));
        assert!(!state.is_in_final_destination((5, 5), 'D'));
        assert!(!state.is_in_final_destination((9, 4), 'D')); //because the one below it is wrong
        assert!(state.is_in_final_destination((3, 3), 'A'));
        assert!(!state.is_complete());

        let input = "#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #A#B#C#D#
  #A#B#C#D#
  #########
";
        let state = input_parser(input);

        assert!(state.is_complete());
    }

    #[test]
    fn test_move_cost() {
        assert_eq!(
            Move {
                to_move: 'A',
                start: (3, 3), //bottom left A
                end: (1, 1),
            }
            .cost(),
            4
        );
        assert_eq!(
            Move {
                to_move: 'B',
                start: (3, 3), //bottom left A
                end: (1, 1),
            }
            .cost(),
            40
        );
        assert_eq!(
            Move {
                to_move: 'D',
                start: (5, 3),
                end: (9, 3)
            }
            .cost(),
            8000
        );
    }
}
