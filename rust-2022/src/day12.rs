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

type Input = HashMap<(isize, isize), Square>;

#[derive(Clone, Eq, PartialEq)]
struct Path {
    cost: usize,
    distance_to_finish: usize,
    //Could make this a non empty by making it a tuple and that would make the comparison not use unwrap
    history: Vec<(isize, isize)>,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.distance_to_finish.cmp(&self.distance_to_finish))

        // other
        //     .distance_to_finish
        //     .cmp(&self.distance_to_finish)
        //     .then_with(|| other.cost.cmp(&self.cost))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
    Plot(usize),
    Start,
    Finish,
}

pub fn part1() -> usize {
    let input = include_str!("../../data/2022/12.txt");

    match input_parser(input) {
        Ok((remaining_input, map)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let (&finish, _) = map.iter().find(|(_, &s)| s == Square::Finish).unwrap();
            let (&start, _) = map.iter().find(|(_, &s)| s == Square::Start).unwrap();

            let mut path_attempts: BinaryHeap<Path> = BinaryHeap::new();
            let mut have_shortest_path: HashSet<(isize, isize)> = HashSet::new();

            path_attempts.push(Path {
                cost: 0,
                history: vec![start],
                distance_to_finish: manhattan_distance(start, finish),
            });
            have_shortest_path.insert(start);

            let mut attempts = 0;

            while let Some(path) = path_attempts.pop() {
                attempts += 1;
                let options = next_steps(&path, &map);
                println!(
                    "current_cost: {}, attempts: {} remaining: {}",
                    path.cost,
                    attempts,
                    path_attempts.len()
                );
                for option in options {
                    //if we have a solution
                    if let Some(&square) = map.get(&option) {
                        if square == Square::Finish {
                            // println!("path: {:?}", path.history);
                            return path.cost + 1;
                        }
                    }

                    if have_shortest_path.contains(&option) {
                        continue;
                    }

                    have_shortest_path.insert(option);

                    let mut new_history = path.history.clone();
                    new_history.push(option);

                    path_attempts.push(Path {
                        cost: path.cost + 1,
                        history: new_history,
                        distance_to_finish: manhattan_distance(option, finish),
                    })
                }
            }

            println!("could not find a solution after {} tries", attempts);
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

pub fn part2() -> usize {
    let input = include_str!("../../data/2022/12.txt");

    match input_parser(input) {
        Ok((remaining_input, map)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let (&finish, _) = map.iter().find(|(_, &s)| s == Square::Finish).unwrap();

            let mut path_attempts: BinaryHeap<Path> = BinaryHeap::new();
            let mut have_shortest_path: HashSet<(isize, isize)> = HashSet::new();

            for (&possible_start, _) in map
                .iter()
                .filter(|(_, &value)| value == Square::Start || value == Square::Plot(0))
            {
                path_attempts.push(Path {
                    cost: 0,
                    history: vec![possible_start],
                    distance_to_finish: manhattan_distance(possible_start, finish),
                });
                have_shortest_path.insert(possible_start);
            }

            let mut attempts = 0;

            while let Some(path) = path_attempts.pop() {
                attempts += 1;
                let options = next_steps(&path, &map);
                println!(
                    "current_cost: {}, attempts: {} remaining: {}",
                    path.cost,
                    attempts,
                    path_attempts.len()
                );
                for option in options {
                    //if we have a solution
                    if let Some(&square) = map.get(&option) {
                        if square == Square::Finish {
                            // println!("path: {:?}", path.history);
                            return path.cost + 1;
                        }
                    }

                    if have_shortest_path.contains(&option) {
                        continue;
                    }

                    have_shortest_path.insert(option);

                    let mut new_history = path.history.clone();
                    new_history.push(option);

                    path_attempts.push(Path {
                        cost: path.cost + 1,
                        history: new_history,
                        distance_to_finish: manhattan_distance(option, finish),
                    })
                }
            }

            println!("could not find a solution after {} tries", attempts);
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

fn next_steps(path: &Path, map: &HashMap<(isize, isize), Square>) -> Vec<(isize, isize)> {
    let &location = path.history.last().unwrap();
    let left = (location.0 - 1, location.1);
    let right = (location.0 + 1, location.1);
    let top = (location.0, location.1 + 1);
    let bottom = (location.0, location.1 - 1);

    let maybe_options = vec![left, right, top, bottom];
    let mut options = vec![];
    for option in maybe_options {
        if let Some(square) = map.get(&option) {
            if !path.history.contains(&option) && can_reach(*map.get(&location).unwrap(), *square) {
                options.push(option);
            }
        }
    }

    // println!(
    //     "found {} options {:?} for location {:?}",
    //     options.len(),
    //     options,
    //     location
    // );

    options
}

fn can_reach(location: Square, to: Square) -> bool {
    match (location, to) {
        (Square::Start, Square::Plot(height)) => height <= 1,
        (Square::Plot(this_height), Square::Finish) => this_height >= 24,
        (Square::Start, Square::Finish) => false,
        (Square::Finish, _) => true,
        (_, Square::Start) => true,
        (Square::Plot(this_height), Square::Plot(other_height)) => this_height + 1 >= other_height,
    }
}

fn manhattan_distance(a: (isize, isize), b: (isize, isize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn start_parser(input: &str) -> IResult<&str, Square> {
    let (input, _) = tag("S")(input)?;

    Ok((input, Square::Start))
}

fn finish_parser(input: &str) -> IResult<&str, Square> {
    let (input, _) = tag("E")(input)?;

    Ok((input, Square::Finish))
}

fn plot_parser(input: &str) -> IResult<&str, Square> {
    let (input, c) = nom::character::complete::none_of("\n")(input)?;

    let value = c as usize - 'a' as usize;

    Ok((input, Square::Plot(value)))
}

fn square_parser(input: &str) -> IResult<&str, Square> {
    nom::branch::alt((start_parser, finish_parser, plot_parser))(input)
}

fn row_parser(input: &str) -> IResult<&str, Vec<Square>> {
    many1(square_parser)(input)
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, rows) = separated_list1(nom::character::complete::newline, row_parser)(input)?;

    let mut map: Input = HashMap::new();
    for (y, row) in rows.iter().enumerate() {
        for (x, square) in row.iter().enumerate() {
            map.insert((x as isize, y as isize), *square);
        }
    }

    Ok((input, map))
}
