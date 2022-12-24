use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map_res, opt},
    multi::separated_list0,
    multi::{many1, separated_list1},
    IResult,
};
use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use std::{collections::BinaryHeap, time::SystemTime};
use std::{
    collections::{HashMap, HashSet},
    ptr,
};
use std::{fmt, ops};

type Input = HashMap<(isize, isize), Spot>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Spot {
    Open,
    MovingBlizzard(Vec<Blizzard>),
    Wall,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Blizzard {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Eq, PartialEq)]
struct Path {
    cost: usize,
    distance_to_finish: usize,
    location: (isize, isize),
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then(other.distance_to_finish.cmp(&self.distance_to_finish))
    }

    // fn cmp(&self, other: &Self) -> Ordering {
    //     other
    //         .distance_to_finish
    //         .cmp(&self.distance_to_finish)
    //         .then(other.cost.cmp(&self.cost))
    // }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1() -> usize {
    let input = include_str!("../../data/2022/24.txt");
    // let input = include_str!("../../data/2022/24-example.txt");

    match input_parser(input) {
        Ok((remaining_input, mut map)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            // let start = find_start(&map);
            // let end = find_end(&map);

            // let mut path_attempts: BinaryHeap<Path> = BinaryHeap::new();
            // let mut map_history: HashMap<usize, Input> = HashMap::new();
            // map_history.insert(0, map.clone());

            // path_attempts.push(Path {
            //     cost: 0,
            //     location: start,
            //     distance_to_finish: manhattan_distance(start, end),
            // });

            // let mut have_shortest_path: HashSet<((isize, isize), u64)> = HashSet::new();
            // let mut is_duplication_path: HashSet<((isize, isize), usize)> = HashSet::new();
            // let mut hash = DefaultHasher::new();
            // map.iter().collect_vec().hash(&mut hash);
            // have_shortest_path.insert((start, hash.finish()));
            // is_duplication_path.insert((start, 0));

            // while let Some(path) = path_attempts.pop() {
            //     println!("current_cost: {}", path.cost);
            //     let options = next_steps(&path, &mut map_history);

            //     let last_map = map_history.get(&path.cost).unwrap();

            //     for option in options {
            //         if option == end {
            //             return path.cost + 1;
            //         }

            //         let mut hash = DefaultHasher::new();
            //         last_map.iter().collect_vec().hash(&mut hash);
            //         let option_hash = (option, hash.finish());
            //         if have_shortest_path.contains(&option_hash) {
            //             continue;
            //         }
            //         have_shortest_path.insert(option_hash);

            //         if is_duplication_path.contains(&(option, path.cost)) {
            //             continue;
            //         }

            //         is_duplication_path.insert((option, path.cost));

            //         path_attempts.push(Path {
            //             cost: path.cost + 1,
            //             location: option,
            //             distance_to_finish: manhattan_distance(option, end),
            //         })
            //     }
            // }

            // println!("could not find a solution ");
            0
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
    let input = include_str!("../../data/2022/24.txt");
    // let input = include_str!("../../data/2022/24-example.txt");

    match input_parser(input) {
        Ok((remaining_input, map)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let start = find_start(&map);
            let end = find_end(&map);

            let (down, ending_map) = shortest_path(start, end, map);
            let (back, ending_map) = shortest_path(end, start, ending_map);
            let (down_again, _) = shortest_path(start, end, ending_map);

            down + back + down_again
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

fn shortest_path(
    start: (isize, isize),
    end: (isize, isize),
    starting_map: Input,
) -> (usize, Input) {
    let mut path_attempts: BinaryHeap<Path> = BinaryHeap::new();
    let mut map_history: HashMap<usize, Input> = HashMap::new();
    map_history.insert(0, starting_map.clone());

    path_attempts.push(Path {
        cost: 0,
        location: start,
        distance_to_finish: manhattan_distance(start, end),
    });

    let mut have_shortest_path: HashSet<((isize, isize), u64)> = HashSet::new();
    let mut hash = DefaultHasher::new();
    starting_map.iter().collect_vec().hash(&mut hash);
    have_shortest_path.insert((start, hash.finish()));

    while let Some(path) = path_attempts.pop() {
        // println!("current_cost: {}", path.cost);
        let options = next_steps(&path, &mut map_history);

        let this_map = map_history.get(&(path.cost + 1)).unwrap();

        for option in options {
            if option == end {
                return (path.cost + 1, this_map.clone());
            }

            let mut hash = DefaultHasher::new();
            this_map.iter().collect_vec().hash(&mut hash);
            let option_hash = (option, hash.finish());
            if have_shortest_path.contains(&option_hash) {
                continue;
            }
            have_shortest_path.insert(option_hash);

            path_attempts.push(Path {
                cost: path.cost + 1,
                location: option,
                distance_to_finish: manhattan_distance(option, end),
            })
        }
    }

    panic!("could not find a solution ");
}

fn print_map(map: &Input) {
    for y in 0..6 {
        for x in 0..8 {
            match map.get(&(x, y)) {
                Some(Spot::Open) => print!("."),
                Some(Spot::MovingBlizzard(blizzards)) => {
                    if blizzards.len() > 1 {
                        print!("{}", blizzards.len());
                    } else {
                        match blizzards[0] {
                            Blizzard::Left => print!("<"),
                            Blizzard::Right => print!(">"),
                            Blizzard::Up => print!("^"),
                            Blizzard::Down => print!("v"),
                        }
                    }
                }
                Some(Spot::Wall) => print!("#"),
                None => {
                    println!();
                    println!("Could not find x: {x}, y: {y}");
                    println!();
                    panic!();
                }
            }
        }
        println!();
    }
}

fn next_steps(path: &Path, map_history: &mut HashMap<usize, Input>) -> Vec<(isize, isize)> {
    let next_cost = path.cost + 1;
    if !map_history.contains_key(&next_cost) {
        progress_map_to_cost(next_cost, map_history);
    }

    let next_map = map_history.get(&next_cost).unwrap();
    let left = (path.location.0 - 1, path.location.1);
    let right = (path.location.0 + 1, path.location.1);
    let top = (path.location.0, path.location.1 + 1);
    let bottom = (path.location.0, path.location.1 - 1);
    let all_options = vec![left, right, top, bottom, path.location];

    let mut moves = vec![];
    for option in all_options {
        if let Some(square) = next_map.get(&option) {
            if *square == Spot::Open {
                moves.push(option);
            }
        }
    }

    moves
}

fn progress_map_to_cost(cost: usize, map_history: &mut HashMap<usize, Input>) {
    let (current_max_cost, map_at_max) = map_history.iter().max_by(|a, b| a.0.cmp(b.0)).unwrap();
    let mut last_map = map_at_max.to_owned();
    for index in (current_max_cost + 1)..(cost + 1) {
        let map_at_time = progress_map(&last_map);
        map_history.insert(index, map_at_time.clone());
        last_map = map_at_time;
    }
}

fn progress_map(map: &Input) -> Input {
    let mut new_map = HashMap::new();
    let width = map.iter().max_by(|a, b| a.0 .0.cmp(&b.0 .0)).unwrap().0 .0;
    let height = map.iter().max_by(|a, b| a.0 .1.cmp(&b.0 .1)).unwrap().0 .1;
    for (&location, spot) in map.iter() {
        match spot {
            Spot::Open => {
                //in case a blizzard moved there
                new_map.entry(location).or_insert(Spot::Open);
            }
            Spot::MovingBlizzard(blizzards) => {
                for blizzard in blizzards {
                    new_map.entry(location).or_insert(Spot::Open);

                    match blizzard {
                        Blizzard::Left => {
                            let next_spot = (location.0 - 1, location.1);
                            if map.get(&next_spot) == Some(&Spot::Wall) {
                                let wrapped = (width - 1, location.1);
                                upsert_blizzard(Blizzard::Left, wrapped, &mut new_map);
                            } else {
                                upsert_blizzard(Blizzard::Left, next_spot, &mut new_map);
                            }
                        }
                        Blizzard::Right => {
                            let next_spot = (location.0 + 1, location.1);
                            if map.get(&next_spot) == Some(&Spot::Wall) {
                                let wrapped = (1, location.1);
                                upsert_blizzard(Blizzard::Right, wrapped, &mut new_map);
                            } else {
                                upsert_blizzard(Blizzard::Right, next_spot, &mut new_map);
                            }
                        }
                        Blizzard::Up => {
                            let next_spot = (location.0, location.1 - 1);
                            if map.get(&next_spot) == Some(&Spot::Wall) {
                                let wrapped = (location.0, height - 1);
                                upsert_blizzard(Blizzard::Up, wrapped, &mut new_map);
                            } else {
                                upsert_blizzard(Blizzard::Up, next_spot, &mut new_map);
                            }
                        }
                        Blizzard::Down => {
                            let next_spot = (location.0, location.1 + 1);
                            if map.get(&next_spot) == Some(&Spot::Wall) {
                                let wrapped = (location.0, 1);
                                upsert_blizzard(Blizzard::Down, wrapped, &mut new_map);
                            } else {
                                upsert_blizzard(Blizzard::Down, next_spot, &mut new_map);
                            }
                        }
                    }
                }
            }
            Spot::Wall => {
                new_map.insert(location, Spot::Wall);
            }
        }
    }

    new_map
}

fn upsert_blizzard(
    blizzard: Blizzard,
    location: (isize, isize),
    map: &mut HashMap<(isize, isize), Spot>,
) {
    match map.get(&location) {
        Some(Spot::MovingBlizzard(blizzards)) => {
            let mut new_blizzards = blizzards.clone();
            new_blizzards.push(blizzard);
            map.insert(location, Spot::MovingBlizzard(new_blizzards));
        }
        _ => {
            map.insert(location, Spot::MovingBlizzard(vec![blizzard]));
        }
    }
}

fn manhattan_distance(a: (isize, isize), b: (isize, isize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn find_start(map: &HashMap<(isize, isize), Spot>) -> (isize, isize) {
    *map.iter()
        .filter(|(_, spot)| spot == &&Spot::Open)
        .min_by(|a, b| a.0 .1.cmp(&b.0 .1).then(a.0 .0.cmp(&b.0 .0)))
        .unwrap()
        .0
}

fn find_end(map: &HashMap<(isize, isize), Spot>) -> (isize, isize) {
    *map.iter()
        .filter(|(_, spot)| spot == &&Spot::Open)
        .max_by(|a, b| a.0 .1.cmp(&b.0 .1).then(a.0 .0.cmp(&b.0 .0)))
        .unwrap()
        .0
}

fn map_line_parser(input: &str) -> IResult<&str, Vec<char>> {
    let (input, output) = many1(nom::character::complete::satisfy(|c| {
        c == '.' || c == '#' || c == '<' || c == '>' || c == 'v' || c == '^'
    }))(input)?;
    Ok((input, output))
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, lines) =
        separated_list1(nom::character::complete::newline, map_line_parser)(input)?;

    let mut map = HashMap::new();
    for (y, row) in lines.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            let spot = match c {
                '#' => Spot::Wall,
                '.' => Spot::Open,
                '<' => Spot::MovingBlizzard(vec![Blizzard::Left]),
                '>' => Spot::MovingBlizzard(vec![Blizzard::Right]),
                '^' => Spot::MovingBlizzard(vec![Blizzard::Up]),
                'v' => Spot::MovingBlizzard(vec![Blizzard::Down]),
                _ => panic!("invalid spot"),
            };

            map.insert((x as isize, y as isize), spot);
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
