use itertools::Itertools;
use nom::{bytes::complete::tag, combinator::map_res, multi::separated_list1, IResult};
use std::collections::{HashMap, HashSet};

type BeaconLocation = (i32, i32, i32);

#[derive(Clone)]
struct ScannerData {
    id: i32,
    beacons: Vec<BeaconLocation>,
}

type Translation = (i32, i32, i32);

pub fn part1() -> usize {
    let input = include_str!("../../data/2021/19.txt");

    match input_parser(input) {
        Ok((remaining_input, scanners)) if remaining_input.is_empty() => {
            println!("parsed entire input");
            let mut all_beacons: HashSet<(i32, i32, i32)> =
                scanners[0].beacons.clone().into_iter().collect();

            let mut scanners_to_solve = scanners.iter().skip(1).collect_vec();

            while !scanners_to_solve.is_empty() {
                let mut next_scanners_to_solve = Vec::new();

                for scanner in scanners_to_solve {
                    if let Some((rotated_scanner, translation)) = overlaps(&all_beacons, scanner) {
                        all_beacons.extend(rotated_scanner.beacons.iter().map(|&(x, y, z)| {
                            (x + translation.0, y + translation.1, z + translation.2)
                        }));
                        println!("joined {} into the fold", scanner.id);
                    } else {
                        next_scanners_to_solve.push(scanner);
                        println!("going to try {} again later", scanner.id);
                    }
                }

                scanners_to_solve = next_scanners_to_solve;
            }

            all_beacons.len()
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

pub fn part2() -> i32 {
    let input = include_str!("../../data/2021/19.txt");

    match input_parser(input) {
        Ok((remaining_input, scanners)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut all_beacons: HashSet<(i32, i32, i32)> =
                scanners[0].beacons.clone().into_iter().collect();

            let mut scanner_positions: Vec<(i32, i32, i32)> = vec![(0, 0, 0)];

            let mut scanners_to_solve = scanners.iter().skip(1).collect_vec();

            while !scanners_to_solve.is_empty() {
                let mut next_scanners_to_solve = Vec::new();

                for scanner in scanners_to_solve {
                    if let Some((rotated_scanner, translation)) = overlaps(&all_beacons, scanner) {
                        all_beacons.extend(rotated_scanner.beacons.iter().map(|&(x, y, z)| {
                            (x + translation.0, y + translation.1, z + translation.2)
                        }));
                        scanner_positions.push(translation);
                        println!("joined {} into the fold", scanner.id);
                    } else {
                        next_scanners_to_solve.push(scanner);
                        println!("going to try {} again later", scanner.id);
                    }
                }

                scanners_to_solve = next_scanners_to_solve;
            }

            scanner_positions
                .iter()
                .combinations(2)
                .map(|pairs| manhattan_distance(*pairs[0], *pairs[1]))
                .max()
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

fn manhattan_distance(a: (i32, i32, i32), b: (i32, i32, i32)) -> i32 {
    (b.0 - a.0).abs() + (b.1 - a.1).abs() + (b.2 - a.2).abs()
}

fn overlaps(
    known_beacons: &HashSet<(i32, i32, i32)>,
    scanner2: &ScannerData,
) -> Option<(ScannerData, Translation)> {
    for scanner_rotation in rotate_scanner(scanner2).iter() {
        let mut point_distance: HashMap<(i32, i32, i32), usize> = HashMap::new();

        for beacon_from_1 in known_beacons.iter() {
            for beacon_from_2 in scanner_rotation.beacons.iter() {
                let distance_between = distance_between(beacon_from_1, beacon_from_2);
                upsert(&mut point_distance, distance_between, 1);
            }
        }

        let maybe_translation =
            point_distance
                .iter()
                .find_map(|(distance, count)| if *count >= 12 { Some(*distance) } else { None });

        if let Some(translation) = maybe_translation {
            return Some((scanner_rotation.clone(), translation));
        }
    }

    None
}

fn rotate_scanner(scanner: &ScannerData) -> Vec<ScannerData> {
    let mut data: Vec<ScannerData> = Vec::new();
    let mut scanner_orientation: ScannerData = scanner.clone();

    for _ in 0..3 {
        //each orientation
        for _ in 0..2 {
            //each top/bottom
            for _ in 0..4 {
                //each side
                data.push(scanner_orientation.clone());

                spin_clockwise(&mut scanner_orientation);
            }

            flip(&mut scanner_orientation);
        }

        twist(&mut scanner_orientation);
    }

    data
}

fn spin_clockwise(scanner: &mut ScannerData) {
    //keep the "z" axis the same, but rotate X and Y
    // y -> x, x -> -y, -y -> -x, -x -> y
    for position in scanner.beacons.iter_mut() {
        let (x, y, _z) = position.to_owned();
        position.0 = y;
        position.1 = -x;
    }
}

fn flip(scanner: &mut ScannerData) {
    //Flipping Z will also invert either x or y. we are choosing to invert x.
    // z -> -z
    for position in scanner.beacons.iter_mut() {
        let (x, _y, z) = position.to_owned();
        position.0 = -x;
        position.2 = -z;
    }
}

fn twist(scanner: &mut ScannerData) {
    //rotate clockwise and then tip onto positive x (x becomes -z)
    //z-> x, x -> -z, y-> y, -z -> -x, -x -> z
    spin_clockwise(scanner);
    for position in scanner.beacons.iter_mut() {
        let (x, y, z) = position.to_owned();
        position.0 = z;
        position.1 = y;
        position.2 = -x;
    }
}

fn distance_between(first: &BeaconLocation, second: &BeaconLocation) -> (i32, i32, i32) {
    // (((first.0 - second.0) ^ 2) as f32
    //     + ((first.1 - second.1) ^ 2) as f32
    //     + ((first.2 - second.2) ^ 2) as f32)
    //     .sqrt()
    //     .floor() as usize
    // ((first.0 - second.0).abs() + (first.1 - second.1).abs() + (first.2 - second.2).abs()) as usize
    (
        (first.0 - second.0),
        (first.1 - second.1),
        (first.2 - second.2),
    )
}

fn upsert(map: &mut HashMap<(i32, i32, i32), usize>, key: (i32, i32, i32), val: usize) {
    match map.get(&key) {
        Some(&current_val) => {
            map.insert(key, current_val + val);
        }
        None => {
            map.insert(key, val);
        }
    }
}

// fn rotations(beacons: &[BeaconLocation]) -> Vec<Vec<BeaconLocation>> {}

fn parse_number(input: &str) -> IResult<&str, i32> {
    let (input, maybe_negative) = nom::combinator::opt(tag("-"))(input)?;
    let (input, val) =
        map_res(nom::character::complete::digit1, |s: &str| s.parse::<i32>())(input)?;

    Ok((input, if maybe_negative.is_some() { -val } else { val }))
}

fn beacon_location_parser(input: &str) -> IResult<&str, BeaconLocation> {
    let (input, x) = parse_number(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = parse_number(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, z) = parse_number(input)?;

    Ok((input, (x, y, z)))
}

fn scanner_parser(input: &str) -> IResult<&str, ScannerData> {
    let (input, _) = tag("--- scanner ")(input)?;
    let (input, id) = parse_number(input)?;
    let (input, _) = tag(" ---")(input)?;
    let (input, _) = nom::character::complete::newline(input)?;
    let (input, beacons) =
        separated_list1(nom::character::complete::newline, beacon_location_parser)(input)?;

    let (input, _) = nom::character::complete::newline(input)?;

    Ok((input, ScannerData { id, beacons }))
}

fn input_parser(input: &str) -> IResult<&str, Vec<ScannerData>> {
    let (input, scanners) =
        separated_list1(nom::character::complete::newline, scanner_parser)(input)?;

    Ok((input, scanners))
}
