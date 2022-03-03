use itertools::Itertools;
use nom::{
    bytes::complete::tag, combinator::map_res, multi::separated_list1, sequence::separated_pair,
    IResult,
};
use std::ops;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Clone, Debug, PartialEq)]
struct Range {
    min: i32,
    max: i32,
}

#[derive(Clone, Debug, PartialEq)]
struct Cuboid {
    x: Range,
    y: Range,
    z: Range,
}

struct Instruction {
    state: bool,
    cuboid: Cuboid,
}

struct ReactorState {
    cubes: Vec<Cuboid>,
}

impl ReactorState {
    fn new() -> Self {
        ReactorState { cubes: Vec::new() }
    }

    fn add(&mut self, cuboid: Cuboid) {
        //add and reduce
        // by reduce, I mean to split up the cuboid to only add new non-intersecting cubes

        let mut resulting_cubes: Vec<Cuboid> = Vec::with_capacity(self.cubes.len());
        resulting_cubes.push(cuboid);

        let mut to_analyze = self.cubes.clone();
        // to_analyze.push(cuboid);
        while !to_analyze.is_empty() {
            let mut to_analyze_next = Vec::with_capacity(to_analyze.len());
            for cube in to_analyze {
                let mut intersects = false;
                for already_accepted_cube in resulting_cubes.iter() {
                    if already_accepted_cube.envelops(&cube) {
                        //if it is enveloped by one already in the set, then ignore it
                        intersects = true;
                        break;
                    }

                    //if it intersects with one already in the set, then break it up along the intersection and analyze it later
                    if cube.overlaps(already_accepted_cube) {
                        let pieces = cube.split_into_pieces_at_overlap(already_accepted_cube);
                        to_analyze_next.extend(pieces);
                        intersects = true;
                        break;
                    }
                }
                //if it does not intersect, then add it
                if !intersects {
                    resulting_cubes.push(cube);
                }
            }

            to_analyze = to_analyze_next;
        }

        self.cubes = resulting_cubes;
    }

    fn subtract(&mut self, cuboid_to_remove: &Cuboid) {
        //remove cuboids that are encompassed in the off cube.
        //will need to split up cubes that are partially enclosed

        let mut resulting_cubes = Vec::new();

        //consider all cubes we have. only add ones that are not in the "cuboid to remove"
        let mut to_analyze = self.cubes.clone();
        while !to_analyze.is_empty() {
            let mut to_analyze_next = Vec::with_capacity(to_analyze.len());
            for cube in to_analyze {
                if cuboid_to_remove.envelops(&cube) {
                    //if it is enveloped by one already in the set, then ignore it
                } else if cuboid_to_remove.overlaps(&cube) {
                    let pieces = cube.split_into_pieces_at_overlap(cuboid_to_remove);
                    to_analyze_next.extend(pieces);
                } else {
                    //if it does not intersect, then add it
                    resulting_cubes.push(cube);
                }
            }

            to_analyze = to_analyze_next;
        }

        self.cubes = resulting_cubes;
    }

    fn count_on(&self) -> usize {
        self.cubes.iter().fold(0, |acc, cube| acc + cube.size())
    }
}

impl Cuboid {
    fn split_x(&self, x: i32) -> (Cuboid, Cuboid) {
        let (lesser_x, greater_x) = self.x.split(x);
        let lesser = Cuboid {
            x: lesser_x,
            y: self.y.clone(),
            z: self.z.clone(),
        };
        let greater = Cuboid {
            x: greater_x,
            y: self.y.clone(),
            z: self.z.clone(),
        };

        (lesser, greater)
    }

    fn split_y(&self, y: i32) -> (Cuboid, Cuboid) {
        let (lesser_y, greater_y) = self.y.split(y);
        let lesser = Cuboid {
            x: self.x.clone(),
            y: lesser_y,
            z: self.z.clone(),
        };
        let greater = Cuboid {
            x: self.x.clone(),
            y: greater_y,
            z: self.z.clone(),
        };

        (lesser, greater)
    }

    fn split_z(&self, z: i32) -> (Cuboid, Cuboid) {
        let (lesser_z, greater_z) = self.z.split(z);
        let lesser = Cuboid {
            x: self.x.clone(),
            y: self.y.clone(),
            z: lesser_z,
        };
        let greater = Cuboid {
            x: self.x.clone(),
            y: self.y.clone(),
            z: greater_z,
        };

        (lesser, greater)
    }

    fn overlaps(&self, cuboid2: &Cuboid) -> bool {
        !(self.x.max < cuboid2.x.min
            || self.x.min > cuboid2.x.max
            || self.y.max < cuboid2.y.min
            || self.y.min > cuboid2.y.max
            || self.z.max < cuboid2.z.min
            || self.z.min > cuboid2.z.max)
    }

    fn split_into_pieces_at_overlap(&self, already_accepted_cube: &Cuboid) -> Vec<Cuboid> {
        let mut pieces = Vec::new();
        if let Some(x_intersection) = self.overlaps_x(already_accepted_cube) {
            let to_split = {
                if self.x.is_min_size() {
                    self.to_owned()
                } else {
                    let (lesser, greater) = self.split_x(x_intersection);
                    let (to_split, done_splitting) = if lesser.overlaps(already_accepted_cube) {
                        (lesser, greater)
                    } else {
                        (greater, lesser)
                    };

                    pieces.push(done_splitting);
                    to_split
                }
            };

            if let Some(y_intersection) = to_split.overlaps_y(already_accepted_cube) {
                let to_split = {
                    if to_split.y.is_min_size() {
                        to_split
                    } else {
                        let (lesser, greater) = to_split.split_y(y_intersection);
                        let (to_split, done_splitting) = if lesser.overlaps(already_accepted_cube) {
                            (lesser, greater)
                        } else {
                            (greater, lesser)
                        };

                        pieces.push(done_splitting);
                        to_split
                    }
                };

                if let Some(z_intersection) = to_split.overlaps_z(already_accepted_cube) {
                    if to_split.z.is_min_size() {
                        pieces.push(to_split);
                    } else {
                        let (lesser, greater) = to_split.split_z(z_intersection);
                        pieces.push(greater);
                        pieces.push(lesser);
                        // if lesser.overlaps(already_accepted_cube) {

                        // } else {
                        //     pieces.push(lesser);
                        //     pieces.push(greater);
                        // }
                    }

                    return pieces;
                }
            }
        }

        panic!("unable to split properly");
    }

    fn overlaps_x(&self, the_cube_we_dont_want: &Cuboid) -> Option<i32> {
        if self.x.max >= the_cube_we_dont_want.x.min && self.x.min <= the_cube_we_dont_want.x.max {
            if self.x.min >= the_cube_we_dont_want.x.min {
                Some(the_cube_we_dont_want.x.max.min(self.x.max))
            } else {
                Some(the_cube_we_dont_want.x.min.max(self.x.min))
            }
        } else {
            None
        }
    }

    fn overlaps_y(&self, the_cube_we_dont_want: &Cuboid) -> Option<i32> {
        if self.y.max >= the_cube_we_dont_want.y.min && self.y.min <= the_cube_we_dont_want.y.max {
            if self.y.min >= the_cube_we_dont_want.y.min {
                Some(the_cube_we_dont_want.y.max.min(self.y.max))
            } else {
                Some(the_cube_we_dont_want.y.min.max(self.y.min))
            }
        } else {
            None
        }
    }

    fn overlaps_z(&self, the_cube_we_dont_want: &Cuboid) -> Option<i32> {
        if self.z.max >= the_cube_we_dont_want.z.min && self.z.min <= the_cube_we_dont_want.z.max {
            if self.z.min >= the_cube_we_dont_want.z.min {
                Some(the_cube_we_dont_want.z.max.min(self.z.max))
            } else {
                Some(the_cube_we_dont_want.z.min.max(self.z.min))
            }
        } else {
            None
        }
    }

    fn envelops(&self, cuboid2: &Cuboid) -> bool {
        self.x.max >= cuboid2.x.max
            && self.x.min <= cuboid2.x.min
            && self.y.max >= cuboid2.y.max
            && self.y.min <= cuboid2.y.min
            && self.z.max >= cuboid2.z.max
            && self.z.min <= cuboid2.z.min
    }

    fn size(&self) -> usize {
        ((self.x.max + 1 - self.x.min).abs() as usize)
            * ((self.y.max + 1 - self.y.min).abs() as usize)
            * ((self.z.max + 1 - self.z.min).abs() as usize)
    }
}

impl Range {
    fn new(min: i32, max: i32) -> Self {
        if min > max {
            panic!("trying to create invalid Range {} {}", min, max);
        }
        Range { min, max }
    }

    fn split(&self, split: i32) -> (Range, Range) {
        if split < self.max {
            (Range::new(self.min, split), Range::new(split + 1, self.max))
        } else if split > self.min {
            (Range::new(self.min, split - 1), Range::new(split, self.max))
        } else {
            panic!("trying to split the unsplitable");
        }
    }

    fn is_min_size(&self) -> bool {
        self.min == self.max
    }
}

pub fn part1() -> usize {
    let input = include_str!("../../data/2021/22-test.txt");

    match input_parser(input) {
        Ok((remaining_input, instructions)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let mut core: HashSet<(i32, i32, i32)> = HashSet::new();

            for instruction in instructions {
                if is_valid_for_part1(&instruction.cuboid) {
                    for x in instruction.cuboid.x.min..(instruction.cuboid.x.max + 1) {
                        for y in instruction.cuboid.y.min..(instruction.cuboid.y.max + 1) {
                            for z in instruction.cuboid.z.min..(instruction.cuboid.z.max + 1) {
                                if instruction.state {
                                    core.insert((x, y, z));
                                } else {
                                    core.remove(&(x, y, z));
                                }
                            }
                        }
                    }
                }
            }

            core.len()
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
    let input = include_str!("../../data/2021/22.txt");

    match input_parser(input) {
        Ok((remaining_input, instructions)) if remaining_input.is_empty() => {
            println!("parsed entire input");
            let mut reactor_state = ReactorState::new();

            for instruction in instructions {
                // if !is_valid_for_part1(&instruction.cuboid) {
                //     continue;
                // }

                println!(
                    "current state: cubes: {}, cubes_on: {}",
                    reactor_state.cubes.len(),
                    reactor_state.count_on()
                );
                // println!("current state: cubes: {}", reactor_state.cubes.len(),);

                if instruction.state {
                    println!("adding cube {:?}", instruction.cuboid);
                    reactor_state.add(instruction.cuboid);
                } else {
                    println!("removing cube {:?}", instruction.cuboid);
                    reactor_state.subtract(&instruction.cuboid);
                }

                println!(
                    "current state: cubes: {}, cubes_on: {}",
                    reactor_state.cubes.len(),
                    reactor_state.count_on()
                );
                // println!("current state: cubes: {}", reactor_state.cubes.len(),);

                println!();
            }

            reactor_state.count_on()
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
fn is_valid_for_part1(cuboid: &Cuboid) -> bool {
    cuboid.x.min >= -50
        && cuboid.x.max <= 50
        && cuboid.y.min >= -50
        && cuboid.y.max <= 50
        && cuboid.z.min >= -50
        && cuboid.z.max <= 50
}

fn parse_number(input: &str) -> IResult<&str, i32> {
    let (input, maybe_negative) = nom::combinator::opt(tag("-"))(input)?;
    let (input, val) =
        map_res(nom::character::complete::digit1, |s: &str| s.parse::<i32>())(input)?;

    Ok((input, if maybe_negative.is_some() { -val } else { val }))
}

fn range_parser(input: &str) -> IResult<&str, Range> {
    let (input, min) = parse_number(input)?;
    let (input, _) = tag("..")(input)?;
    let (input, max) = parse_number(input)?;

    Ok((input, Range { min, max }))
}

fn on_parser(input: &str) -> IResult<&str, bool> {
    let (input, _) = tag("on")(input)?;
    Ok((input, true))
}

fn off_parser(input: &str) -> IResult<&str, bool> {
    let (input, _) = tag("off")(input)?;
    Ok((input, false))
}

fn instruction_parser(input: &str) -> IResult<&str, Instruction> {
    let (input, state) = nom::branch::alt((on_parser, off_parser))(input)?;

    let (input, _) = tag(" x=")(input)?;
    let (input, x_range) = range_parser(input)?;

    let (input, _) = tag(",y=")(input)?;
    let (input, y_range) = range_parser(input)?;

    let (input, _) = tag(",z=")(input)?;
    let (input, z_range) = range_parser(input)?;

    Ok((
        input,
        Instruction {
            state,
            cuboid: Cuboid {
                x: x_range,
                y: y_range,
                z: z_range,
            },
        },
    ))
}

fn input_parser(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, numbers) =
        separated_list1(nom::character::complete::newline, instruction_parser)(input)?;

    let (input, _) = nom::character::complete::newline(input)?;

    Ok((input, numbers))
}

#[cfg(test)]
mod day22_tests {
    use crate::day22::Cuboid;
    use crate::day22::Range;
    use crate::day22::ReactorState;

    #[test]
    fn adding_and_removing_results_in_empty() {
        let mut state = ReactorState::new();

        let test_cuboid = Cuboid {
            x: Range::new(-10, 20),
            y: Range::new(-20, 40),
            z: Range::new(-10, 10),
        };

        state.add(test_cuboid.to_owned());
        state.subtract(&test_cuboid);

        assert_eq!(state.count_on(), 0);
    }

    #[test]
    fn adding_disjoint_cubes_results_in_correct_count() {
        let mut state = ReactorState::new();

        let unit_cuboid = Cuboid {
            x: Range::new(1, 1),
            y: Range::new(1, 1),
            z: Range::new(1, 1),
        };
        state.add(unit_cuboid);

        assert_eq!(state.count_on(), 1);

        let unit_cuboid = Cuboid {
            x: Range::new(2, 2),
            y: Range::new(2, 2),
            z: Range::new(2, 2),
        };
        state.add(unit_cuboid);

        assert_eq!(state.count_on(), 2);
    }

    #[test]
    fn overlapping_cubes_should_return_overlapped() {
        let first = Cuboid {
            x: Range::new(1, 2),
            y: Range::new(1, 2),
            z: Range::new(1, 2),
        };
        let second = Cuboid {
            x: Range::new(2, 2),
            y: Range::new(2, 2),
            z: Range::new(2, 2),
        };

        assert!(first.overlaps(&second));
        assert!(second.overlaps(&first));
    }

    #[test]
    fn adding_overlapping_cubes_results_in_correct_count() {
        let mut state = ReactorState::new();

        let unit_cuboid = Cuboid {
            x: Range::new(1, 2),
            y: Range::new(1, 2),
            z: Range::new(1, 2),
        };
        state.add(unit_cuboid);

        assert_eq!(state.count_on(), 8);

        let unit_cuboid = Cuboid {
            x: Range::new(2, 2),
            y: Range::new(2, 2),
            z: Range::new(2, 2),
        };
        state.add(unit_cuboid);

        assert_eq!(state.count_on(), 8);
    }

    #[test]
    fn removing_cube_that_covers_partial_results_in_correct_count() {
        let mut state = ReactorState::new();

        let cuboid = Cuboid {
            x: Range::new(1, 1),
            y: Range::new(1, 1),
            z: Range::new(1, 2),
        };
        state.add(cuboid);

        assert_eq!(state.count_on(), 2);

        let cuboid = Cuboid {
            x: Range::new(1, 1),
            y: Range::new(1, 1),
            z: Range::new(2, 2),
        };
        state.subtract(&cuboid);

        assert_eq!(state.count_on(), 1);
    }

    #[test]
    fn should_be_able_to_add() {
        let mut state = ReactorState::new();

        let cuboid = Cuboid {
            x: Range::new(-1, 1),
            y: Range::new(0, 1),
            z: Range::new(1, 1),
        };
        state.add(cuboid);

        // assert_eq!(state.count_on(), 2);

        let cuboid = Cuboid {
            x: Range::new(-1, 2),
            y: Range::new(1, 1),
            z: Range::new(1, 1),
        };
        state.add(cuboid);

        assert_eq!(state.count_on(), 7);
    }

    #[test]
    fn should_be_able_to_add_real_examples() {
        let mut state = ReactorState::new();

        let cuboid = Cuboid {
            x: Range::new(-20, 26),
            y: Range::new(-36, 17),
            z: Range::new(-47, 7),
        };
        state.add(cuboid);

        // assert_eq!(state.count_on(), 2);

        let cuboid = Cuboid {
            x: Range::new(-20, 33),
            y: Range::new(-21, 23),
            z: Range::new(-26, 28),
        };
        state.add(cuboid);

        assert_eq!(state.count_on(), 210918);
    }

    #[test]
    fn range_split_should_work() {
        assert_eq!(
            Range::new(-2, 2).split(-1),
            (Range::new(-2, -1), Range::new(0, 2))
        );
        assert_eq!(
            Range::new(-2, 2).split(-2),
            (Range::new(-2, -2), Range::new(-1, 2))
        );
    }
}
