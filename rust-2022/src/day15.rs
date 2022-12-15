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

type Input = Vec<SensorPlacement>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct SensorPlacement {
    location: (isize, isize),
    closest_beacon: (isize, isize),
}

pub fn part1() -> isize {
    // let input = include_str!("../../data/2022/15-example.txt");
    // let row_to_check: isize = 10;
    let input = include_str!("../../data/2022/15.txt");
    let row_to_check: isize = 2000000;

    match input_parser(input) {
        Ok((remaining_input, sensors)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            // let mut sum: isize = 0;

            // for (index, sensor) in sensors
            //     .iter()
            //     .sorted_by(|a, b| a.location.0.cmp(&b.location.0))
            //     .enumerate()
            // {
            //     let count = count_on_line(sensor, row_to_check) as isize;

            //     if count == 0 {
            //         continue;
            //     }

            //     println!("on the line {}", count);

            //     sum += count;

            //     //if the beacon is on the line, it doesn't count
            //     if sensor.closest_beacon.1 == row_to_check {
            //         println!("B on the line {:?}", sensor.closest_beacon);
            //         sum -= 1;
            //     }

            //     for other_sensor in sensors.iter().skip(index + 1) {
            //         if other_sensor.location == sensor.location {
            //             continue;
            //         }

            //         // let radius = manhattan_distance(sensor.location, sensor.closest_beacon);
            //         // let other_radius =
            //         //     manhattan_distance(other_sensor.location, other_sensor.closest_beacon);
            //         // let distance_between =
            //         //     manhattan_distance(sensor.location, other_sensor.location);

            //         let amount_overlapping =
            //             amount_overlapping_on_line(sensor, other_sensor, row_to_check);
            //         sum -= amount_overlapping as isize;

            //         if amount_overlapping > 0 {
            //             println!("overlapping {}", amount_overlapping);
            //         }
            //     }

            //     //if double covered, subtract how much
            //     // sum -= count_on_line(&sensor, row_to_check);
            // }

            // sum as usize

            let mut projections = vec![];
            let mut beacons: HashSet<(isize, isize)> = HashSet::new();
            for sensor in sensors.iter() {
                if let Some(projection) = project_on_line(sensor, row_to_check) {
                    println!("projected {:?} on line {:?}", sensor, projection);
                    add_projection(&mut projections, projection);
                    println!("after adding {:?}", projections);
                }

                beacons.insert(sensor.closest_beacon);
            }

            let mut sum = 0;
            for projection in projections {
                sum += 1 + projection.1 - projection.0;
            }

            //remove spots taken by beacons
            let beacons_on_line = beacons.iter().filter(|b| b.1 == row_to_check).count();

            sum - beacons_on_line as isize
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
    // let input = include_str!("../../data/2022/15-example.txt");
    // let min_y = 0;
    // let max_y = 20;
    let input = include_str!("../../data/2022/15.txt");
    let min_y = 0;
    let max_y = 4000000;

    match input_parser(input) {
        Ok((remaining_input, sensors)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            for row_to_check in min_y..(max_y + 1) {
                let mut projections = vec![];
                let mut beacons: HashSet<(isize, isize)> = HashSet::new();
                for sensor in sensors.iter() {
                    if let Some(projection) = project_on_line(sensor, row_to_check) {
                        // println!("projected {:?} on line {:?}", sensor, projection);
                        add_projection(&mut projections, projection);
                        // println!("after adding {:?}", projections);
                    }

                    beacons.insert(sensor.closest_beacon);
                }

                let mut last_projections_len = projections.len();
                projections.sort_by(|a, b| a.0.cmp(&b.0));
                let mut joined_projections = projections.to_owned();
                loop {
                    joined_projections = join_sorted_projections(&joined_projections);
                    if joined_projections.len() == last_projections_len {
                        break;
                    }

                    last_projections_len = joined_projections.len();
                }

                if joined_projections.len() == 1 {
                    println!("no space in row {}", row_to_check);
                } else {
                    println!("chunks: {:?}", joined_projections);
                    let y = row_to_check;
                    let x = joined_projections[0].1 + 1;

                    if x >= 0 || x <= 4000000 {
                        println!("found: {:?}", (x, y));
                        return x * 4000000 + y;
                    }
                }

                // let mut sum = 0;
                // for projection in projections {
                //     sum += 1 + projection.1 - projection.0;
                // }

                //remove spots taken by beacons
                // let beacons_on_line = beacons.iter().filter(|b| b.1 == row_to_check).count();

                // println!("spots unavailable {}", sum - beacons_on_line as isize);
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

fn add_coordinates_inside_range(
    start: (isize, isize),
    range: usize,
    coordinates: &mut HashSet<(isize, isize)>,
) {
    for x_change in 0..(range + 1) {
        for y_change in 0..(range - x_change + 1) {
            coordinates.insert((start.0 + (x_change as isize), start.1 + (y_change as isize)));
            coordinates.insert((start.0 - (x_change as isize), start.1 + (y_change as isize)));
            coordinates.insert((start.0 + (x_change as isize), start.1 - (y_change as isize)));
            coordinates.insert((start.0 - (x_change as isize), start.1 - (y_change as isize)));
        }
    }
}

fn count_on_line(sensor: &SensorPlacement, y_axis: isize) -> usize {
    let radius = manhattan_distance(sensor.location, sensor.closest_beacon);

    let closest_point_on_line = (sensor.location.0, y_axis);

    let line_distance_from_sensor = manhattan_distance(sensor.location, closest_point_on_line);

    if line_distance_from_sensor > radius {
        0
    } else {
        1 + (radius - line_distance_from_sensor) * 2
    }
}

fn amount_overlapping_on_line(a: &SensorPlacement, b: &SensorPlacement, y_axis: isize) -> usize {
    let radius_a = manhattan_distance(a.location, a.closest_beacon);
    let radius_b = manhattan_distance(b.location, b.closest_beacon);
    // let distance_between = manhattan_distance(a.location, b.location);

    let middle_point_on_line_a = (a.location.0, y_axis);
    let middle_point_on_line_b = (b.location.0, y_axis);

    let line_distance_from_a = manhattan_distance(a.location, middle_point_on_line_a);
    let line_distance_from_b = manhattan_distance(b.location, middle_point_on_line_b);

    if line_distance_from_a > radius_a || line_distance_from_b > radius_b {
        return 0;
    }

    let width_at_line_a = 1 + (radius_a - line_distance_from_a); //2
    let width_at_line_b = 1 + (radius_b - line_distance_from_b); //2

    let overlap = width_at_line_a as isize + width_at_line_b as isize
        - (a.location.0.abs_diff(b.location.0) + 1) as isize;

    if overlap < 0 {
        0
    } else {
        overlap as usize
    }
}

fn project_on_line(sensor: &SensorPlacement, y_axis: isize) -> Option<(isize, isize)> {
    let radius = manhattan_distance(sensor.location, sensor.closest_beacon);

    let closest_point_on_line = (sensor.location.0, y_axis);

    let line_distance_from_sensor = manhattan_distance(sensor.location, closest_point_on_line);

    if line_distance_from_sensor > radius {
        None
    } else {
        let radius_at_intersection = radius as isize - line_distance_from_sensor as isize;
        Some((
            sensor.location.0 - radius_at_intersection,
            sensor.location.0 + radius_at_intersection,
        ))
    }
}

fn add_projection(projections: &mut Vec<(isize, isize)>, to_add: (isize, isize)) {
    let mut to_visit = vec![to_add];

    while let Some(projection) = to_visit.pop() {
        let mut overlap_found = false;
        for accepted in projections.iter() {
            //if overlaps, split and add part to to_visit then continue
            if let Some(splits) = split_at_overlap(*accepted, projection) {
                for split in splits {
                    to_visit.push(split);
                }

                overlap_found = true;
            }

            if overlap_found {
                break;
            }
        }

        if !overlap_found {
            projections.push(projection);
        }
    }
}

fn split_at_overlap(
    accepted: (isize, isize),
    to_add: (isize, isize),
) -> Option<Vec<(isize, isize)>> {
    let no_overlap = accepted.1 < to_add.0 || accepted.0 > to_add.1;
    if no_overlap {
        //
        return None;
    }

    if accepted.0 <= to_add.0 {
        if accepted.1 >= to_add.1 {
            //wholly covered
            Some(vec![])
        } else {
            Some(vec![(accepted.1 + 1, to_add.1)])
        }
    } else {
        //to_add is farther to the left of accepted
        if to_add.1 >= accepted.1 {
            //wholly covered
            if accepted.1 == to_add.1 {
                Some(vec![(to_add.0, accepted.0 - 1)])
            } else {
                Some(vec![(to_add.0, accepted.0 - 1), (accepted.1 + 1, to_add.1)])
            }
        } else {
            Some(vec![(to_add.0, accepted.0 - 1)])
        }
    }
}

fn join_sorted_projections(projections: &[(isize, isize)]) -> Vec<(isize, isize)> {
    let mut results = vec![];
    for pair in projections.chunks(2) {
        if pair.len() == 1 {
            results.push(pair[0]);
        } else {
            let left = pair[0];
            let right = pair[1];
            if left.1 + 1 == right.0 {
                results.push((left.0, right.1));
            } else {
                results.push(left);
                results.push(right);
            }
        }
    }

    results
}

fn manhattan_distance(a: (isize, isize), b: (isize, isize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn coordinate_parser(input: &str) -> IResult<&str, (isize, isize)> {
    let (input, _) = tag("x=")(input)?;
    let (input, x_negative_sign) = opt(nom::character::complete::char('-'))(input)?;
    let (input, x) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<isize>()
    })(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y_negative_sign) = opt(nom::character::complete::char('-'))(input)?;
    let (input, y) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<isize>()
    })(input)?;

    Ok((
        input,
        (
            if x_negative_sign.is_some() { -x } else { x },
            if y_negative_sign.is_some() { -y } else { y },
        ),
    ))
}

fn sensor_placement_parser(input: &str) -> IResult<&str, SensorPlacement> {
    let (input, _) = tag("Sensor at ")(input)?;
    let (input, location) = coordinate_parser(input)?;
    let (input, _) = tag(": closest beacon is at ")(input)?;
    let (input, closest_beacon) = coordinate_parser(input)?;

    Ok((
        input,
        SensorPlacement {
            location,
            closest_beacon,
        },
    ))
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, formations) =
        separated_list1(nom::character::complete::newline, sensor_placement_parser)(input)?;

    Ok((input, formations))
}

#[cfg(test)]
mod day15_tests {
    use crate::day15::add_coordinates_inside_range;
    use crate::day15::{
        amount_overlapping_on_line, count_on_line, join_sorted_projections, split_at_overlap,
        SensorPlacement,
    };
    use std::collections::HashSet;

    #[test]
    fn test_coordinates_inside_range() {
        let mut set = HashSet::new();
        add_coordinates_inside_range((0, 0), 2, &mut set);
        assert_eq!(set.len(), 13);
    }

    #[test]
    fn test_count_on_line_when_on_tip() {
        assert_eq!(
            count_on_line(
                &SensorPlacement {
                    location: (0, 0),
                    closest_beacon: (1, 1),
                },
                2
            ),
            1
        );
    }

    #[test]
    fn test_count_on_line_when_half_way() {
        assert_eq!(
            count_on_line(
                &SensorPlacement {
                    location: (0, 0),
                    closest_beacon: (1, 1),
                },
                1
            ),
            3
        );
    }

    #[test]
    fn test_count_on_line_when_on_center() {
        assert_eq!(
            count_on_line(
                &SensorPlacement {
                    location: (0, 0),
                    closest_beacon: (1, 1),
                },
                0
            ),
            5
        );
    }

    #[test]
    fn test_amount_overlapping_on_line_1() {
        assert_eq!(
            amount_overlapping_on_line(
                &SensorPlacement {
                    location: (0, 0),
                    closest_beacon: (1, 0),
                },
                &SensorPlacement {
                    location: (2, 1),
                    closest_beacon: (1, 0),
                },
                0
            ),
            1
        );
    }

    #[test]
    fn test_amount_overlapping_on_line_2() {
        assert_eq!(
            amount_overlapping_on_line(
                &SensorPlacement {
                    location: (8, 7),
                    closest_beacon: (2, 10),
                },
                &SensorPlacement {
                    location: (2, 0),
                    closest_beacon: (2, 10),
                },
                10
            ),
            1
        );
    }

    #[test]
    fn test_amount_overlapping_on_line_3() {
        assert_eq!(
            amount_overlapping_on_line(
                &SensorPlacement {
                    location: (8, 7),
                    closest_beacon: (2, 10),
                },
                &SensorPlacement {
                    location: (0, 11),
                    closest_beacon: (2, 10),
                },
                10
            ),
            1
        );
    }

    #[test]
    fn test_split_at_overlap() {
        assert_eq!(split_at_overlap((0, 4), (4, 5)), Some(vec![(5, 5)]));
        assert_eq!(split_at_overlap((0, 4), (3, 5)), Some(vec![(5, 5)]));
        assert_eq!(split_at_overlap((0, 5), (3, 5)), Some(vec![]));
        assert_eq!(split_at_overlap((3, 5), (0, 5)), Some(vec![(0, 2)]));
        assert_eq!(
            split_at_overlap((12, 12), (2, 14)),
            Some(vec![(2, 11), (13, 14)])
        );
    }

    #[test]
    fn test_join_sorted_projections() {
        assert_eq!(join_sorted_projections(&[(0, 3), (4, 5)]), vec![(0, 5)]);
        assert_eq!(
            join_sorted_projections(&[(-8, 10), (11, 14), (15, 26)]),
            vec![(-8, 14), (15, 26)]
        );
    }
}
