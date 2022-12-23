use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map_res, opt},
    multi::separated_list0,
    multi::{many1, separated_list1},
    IResult,
};
use std::{cmp::Ordering, hash::Hash};
use std::{collections::BinaryHeap, time::SystemTime};
use std::{
    collections::{HashMap, HashSet},
    ptr,
};
use std::{fmt, ops};

type Input = (HashMap<(isize, isize), Spot>, Vec<Direction>);

type CubeLayout = HashMap<CubeFace, CubeFaceBoundary>;

#[derive(Copy, Clone, Debug)]
struct Position {
    x: isize,
    y: isize,
    facing: Orientation,
}

#[derive(Copy, Clone, Debug)]
struct CubePosition {
    x: isize,
    y: isize,
    // face: CubeFace,
    facing: Orientation,
}

#[derive(Copy, Clone, Debug)]
struct CubeFacePosition {
    x: isize,
    y: isize,
    face: CubeFace,
    facing: Orientation,
}

#[derive(Copy, Clone, Debug)]
struct CubeFaceBoundary {
    top_left: (isize, isize),
    bottom_right: (isize, isize),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum CubeFace {
    A,
    B,
    C,
    D,
    E,
    F,
}

struct Cube {
    map: HashMap<(isize, isize), Spot>,
    face_boundaries: Vec<CubeFaceBoundary>,
    size: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Move(isize),
    TurnLeft,
    TurnRight,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Spot {
    Open,
    Wall,
}

pub fn part1() -> isize {
    // let input = include_str!("../../data/2022/22-example.txt");
    let input = include_str!("../../data/2022/22.txt");

    match input_parser(input) {
        Ok((remaining_input, (map, directions))) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let &(starting_x, starting_y) = map
                .iter()
                .min_by(|&(a, _), &(b, _)| a.1.cmp(&b.1).then(a.0.cmp(&b.0)))
                .unwrap()
                .0;

            let mut position = Position {
                x: starting_x,
                y: starting_y,
                facing: Orientation::Right,
            };

            for direction in directions {
                position = follow_direction(position, direction, &map);
            }

            position.score()
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

pub fn part2() -> isize {
    let input = include_str!("../../data/2022/22.txt");
    let size = 50;
    // let input = include_str!("../../data/2022/22-example.txt");
    // let size = 4;

    match input_parser(input) {
        Ok((remaining_input, (map, directions))) if remaining_input.is_empty() => {
            println!("parsed entire input");

            let &(starting_x, starting_y) = map
                .iter()
                .min_by(|&(a, _), &(b, _)| a.1.cmp(&b.1).then(a.0.cmp(&b.0)))
                .unwrap()
                .0;

            let mut position = CubePosition {
                x: starting_x,
                y: starting_y,
                facing: Orientation::Right,
            };

            let cube_layout = map_to_cube(&map, size);

            // println!("{:?}", position);
            for direction in directions {
                position =
                    follow_direction_with_cube(position, direction, &map, &cube_layout, size);
                // println!("{:?}", position);
                print_position(&map, position);
                println!();
                println!();
            }

            position.score()
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

fn print_position(map: &HashMap<(isize, isize), Spot>, position: CubePosition) {
    for y in 0..16 {
        for x in 0..16 {
            if position.x == x && position.y == y {
                match position.facing {
                    Orientation::Up => print!("^"),
                    Orientation::Down => print!("v"),
                    Orientation::Left => print!("<"),
                    Orientation::Right => print!(">"),
                }
                continue;
            }
            match map.get(&(x, y)) {
                None => {
                    print!(" ");
                }
                Some(Spot::Wall) => {
                    print!("#");
                }
                Some(Spot::Open) => {
                    print!(".");
                }
            }
        }
        println!();
    }
}

fn follow_direction(
    position: Position,
    direction: Direction,
    map: &HashMap<(isize, isize), Spot>,
) -> Position {
    match direction {
        Direction::Move(n) => move_along_line(position, n, map),
        Direction::TurnLeft => position.rotate_left(),
        Direction::TurnRight => position.rotate_right(),
    }
}

fn follow_direction_with_cube(
    position: CubePosition,
    direction: Direction,
    map: &HashMap<(isize, isize), Spot>,
    cube_layout: &CubeLayout,
    size: isize,
) -> CubePosition {
    match direction {
        Direction::Move(n) => {
            println!("moving {n}");
            let mut result = position;
            for _ in 0..n {
                result = match result.facing {
                    Orientation::Up => result.move_up(map, cube_layout, size),
                    Orientation::Down => result.move_down(map, cube_layout, size),
                    Orientation::Left => result.move_left(map, cube_layout, size),
                    Orientation::Right => result.move_right(map, cube_layout, size),
                };
            }

            result
        }
        Direction::TurnLeft => position.rotate_left(),
        Direction::TurnRight => position.rotate_right(),
    }
}

fn move_along_line(position: Position, n: isize, map: &HashMap<(isize, isize), Spot>) -> Position {
    if n <= 0 {
        return position;
    }

    let (next_coordinate, next_spot) = match position.facing {
        Orientation::Up => {
            let next = (position.x, position.y - 1);
            if let Some(spot) = map.get(&next) {
                (next, spot)
            } else {
                //wrap to bottom (max y)
                let (&coordinate, spot) = map
                    .iter()
                    .filter(|&((x, _y), _)| *x == position.x)
                    .max_by(|(a, _), (b, _)| a.1.cmp(&b.1))
                    .unwrap();

                (coordinate, spot)
            }
        }
        Orientation::Down => {
            let next = (position.x, position.y + 1);
            if let Some(spot) = map.get(&next) {
                (next, spot)
            } else {
                //wrap to top (min y)
                let (&coordinate, spot) = map
                    .iter()
                    .filter(|&((x, _y), _)| *x == position.x)
                    .min_by(|(a, _), (b, _)| a.1.cmp(&b.1))
                    .unwrap();

                (coordinate, spot)
            }
        }
        Orientation::Left => {
            let next = (position.x - 1, position.y);
            if let Some(spot) = map.get(&next) {
                (next, spot)
            } else {
                //wrap to top (min y)
                let (&coordinate, spot) = map
                    .iter()
                    .filter(|&((_x, y), _)| *y == position.y)
                    .max_by(|(a, _), (b, _)| a.0.cmp(&b.0))
                    .unwrap();

                (coordinate, spot)
            }
        }
        Orientation::Right => {
            let next = (position.x + 1, position.y);
            if let Some(spot) = map.get(&next) {
                (next, spot)
            } else {
                //wrap to top (min y)
                let (&coordinate, spot) = map
                    .iter()
                    .filter(|&((_x, y), _)| *y == position.y)
                    .min_by(|(a, _), (b, _)| a.0.cmp(&b.0))
                    .unwrap();

                (coordinate, spot)
            }
        }
    };

    match next_spot {
        Spot::Open => move_along_line(position.set_coordinate(next_coordinate), n - 1, map),
        Spot::Wall => position,
    }
}

fn turn_left_parser(input: &str) -> IResult<&str, Direction> {
    let (input, _) = tag("L")(input)?;

    Ok((input, Direction::TurnLeft))
}

fn turn_right_parser(input: &str) -> IResult<&str, Direction> {
    let (input, _) = tag("R")(input)?;

    Ok((input, Direction::TurnRight))
}

fn move_parser(input: &str) -> IResult<&str, Direction> {
    let (input, num) = map_res(nom::character::complete::digit1, |s: &str| {
        s.parse::<isize>()
    })(input)?;

    Ok((input, Direction::Move(num)))
}

fn direction_parser(input: &str) -> IResult<&str, Direction> {
    let (input, direction) =
        nom::branch::alt((move_parser, turn_left_parser, turn_right_parser))(input)?;

    Ok((input, direction))
}

fn map_line_parser(input: &str) -> IResult<&str, Vec<char>> {
    let (input, output) = many1(nom::character::complete::satisfy(|c| {
        c == ' ' || c == '.' || c == '#'
    }))(input)?;
    Ok((input, output))
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    let (input, lines) =
        separated_list1(nom::character::complete::newline, map_line_parser)(input)?;
    let (input, _) = nom::character::complete::newline(input)?;
    let (input, _) = nom::character::complete::newline(input)?;
    let (input, directions) = many1(direction_parser)(input)?;

    let mut map = HashMap::new();
    for (y, row) in lines.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            match c {
                '.' => {
                    map.insert((x as isize, y as isize), Spot::Open);
                }
                '#' => {
                    map.insert((x as isize, y as isize), Spot::Wall);
                }
                _ => {}
            }
        }
    }

    Ok((input, (map, directions)))
}

fn map_to_cube(map: &HashMap<(isize, isize), Spot>, size: isize) -> CubeLayout {
    //we don't know the layout for sure, so see where they are
    //read left to right, top to bottom
    let mut faces = HashMap::new();
    let mut face_names = vec![
        CubeFace::F,
        CubeFace::E,
        CubeFace::D,
        CubeFace::C,
        CubeFace::B,
        CubeFace::A,
    ];
    for y in 0..6 {
        for x in 0..6 {
            let starting_position = (size * x, size * y);
            if map.contains_key(&starting_position) {
                //this is one of the faces
                faces.insert(
                    face_names.pop().unwrap(),
                    CubeFaceBoundary {
                        top_left: starting_position,
                        bottom_right: (x * size + size - 1, y * size + size - 1),
                    },
                );
            }
        }
    }

    // for (name, boundary) in faces.iter() {
    //     println!("{:?}: {:?}", name, boundary);
    // }

    faces
}

impl Position {
    fn score(&self) -> isize {
        let facing_score = match self.facing {
            Orientation::Right => 0,
            Orientation::Down => 1,
            Orientation::Left => 2,
            Orientation::Up => 3,
        };

        (self.x + 1) * 4 + (self.y + 1) * 1000 + facing_score
    }

    fn rotate_left(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            facing: self.facing.rotate_left(),
        }
    }

    fn rotate_right(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            facing: self.facing.rotate_right(),
        }
    }

    fn set_coordinate(&self, (x, y): (isize, isize)) -> Self {
        Self {
            x,
            y,
            facing: self.facing,
        }
    }
}

impl CubePosition {
    fn rotate_left(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            facing: self.facing.rotate_left(),
        }
    }

    fn rotate_right(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            facing: self.facing.rotate_right(),
        }
    }

    fn score(&self) -> isize {
        let facing_score = match self.facing {
            Orientation::Right => 0,
            Orientation::Down => 1,
            Orientation::Left => 2,
            Orientation::Up => 3,
        };

        (self.x + 1) * 4 + (self.y + 1) * 1000 + facing_score
    }

    fn move_up(
        &self,
        map: &HashMap<(isize, isize), Spot>,
        cube_layout: &CubeLayout,
        size: isize,
    ) -> CubePosition {
        let next = (self.x, self.y - 1);
        match map.get(&next) {
            Some(Spot::Open) => Self {
                x: next.0,
                y: next.1,
                facing: self.facing,
            },
            Some(Spot::Wall) => *self,
            None => {
                let relative_in_current_cube =
                    CubeFacePosition::from_global_position(self, cube_layout);

                let relative_in_new_cube =
                    relative_in_current_cube.real_move_in_direction(Orientation::Up, size);

                let global_position = relative_in_new_cube.to_global_position(cube_layout);

                let spot = map.get(&(global_position.x, global_position.y));
                if spot.is_none() || spot == Some(&Spot::Wall) {
                    *self
                } else {
                    global_position
                }
            }
        }
    }

    fn move_down(
        &self,
        map: &HashMap<(isize, isize), Spot>,
        cube_layout: &CubeLayout,
        size: isize,
    ) -> CubePosition {
        let next = (self.x, self.y + 1);
        match map.get(&next) {
            Some(Spot::Open) => Self {
                x: next.0,
                y: next.1,
                facing: self.facing,
            },
            Some(Spot::Wall) => *self,
            None => {
                let relative_in_current_cube =
                    CubeFacePosition::from_global_position(self, cube_layout);

                let relative_in_new_cube =
                    relative_in_current_cube.real_move_in_direction(Orientation::Down, size);

                let global_position = relative_in_new_cube.to_global_position(cube_layout);

                let spot = map.get(&(global_position.x, global_position.y));
                if spot.is_none() || spot == Some(&Spot::Wall) {
                    *self
                } else {
                    global_position
                }
            }
        }
    }

    fn move_left(
        &self,
        map: &HashMap<(isize, isize), Spot>,
        cube_layout: &CubeLayout,
        size: isize,
    ) -> CubePosition {
        let next = (self.x - 1, self.y);
        match map.get(&next) {
            Some(Spot::Open) => Self {
                x: next.0,
                y: next.1,
                facing: self.facing,
            },
            Some(Spot::Wall) => *self,
            None => {
                let relative_in_current_cube =
                    CubeFacePosition::from_global_position(self, cube_layout);

                let relative_in_new_cube =
                    relative_in_current_cube.real_move_in_direction(Orientation::Left, size);

                let global_position = relative_in_new_cube.to_global_position(cube_layout);

                let spot = map.get(&(global_position.x, global_position.y));
                if spot.is_none() || spot == Some(&Spot::Wall) {
                    *self
                } else {
                    global_position
                }
            }
        }
    }

    fn move_right(
        &self,
        map: &HashMap<(isize, isize), Spot>,
        cube_layout: &CubeLayout,
        size: isize,
    ) -> CubePosition {
        let next = (self.x + 1, self.y);
        match map.get(&next) {
            Some(Spot::Open) => Self {
                x: next.0,
                y: next.1,
                facing: self.facing,
            },
            Some(Spot::Wall) => *self,
            None => {
                let relative_in_current_cube =
                    CubeFacePosition::from_global_position(self, cube_layout);

                let relative_in_new_cube =
                    relative_in_current_cube.real_move_in_direction(Orientation::Right, size);

                let global_position = relative_in_new_cube.to_global_position(cube_layout);

                let spot = map.get(&(global_position.x, global_position.y));
                if spot.is_none() || spot == Some(&Spot::Wall) {
                    *self
                } else {
                    global_position
                }
            }
        }
    }

    fn in_boundary(&self, boundary: &CubeFaceBoundary) -> bool {
        self.x >= boundary.top_left.0
            && self.x <= boundary.bottom_right.0
            && self.y >= boundary.top_left.1
            && self.y <= boundary.bottom_right.1
    }
}

impl CubeFacePosition {
    fn from_global_position(position: &CubePosition, cube_layout: &CubeLayout) -> CubeFacePosition {
        // println!("translating x: {}, y: {}", position.x, position.y);
        // let cube_face_a = cube_layout.get(&CubeFace::A).unwrap();
        // println!("boundary: {:?}", cube_face_a);

        let (cube_face, boundary) = cube_layout
            .iter()
            .find(|&(_cube_face, boundary)| position.in_boundary(boundary))
            .unwrap();

        Self {
            x: position.x - boundary.top_left.0,
            y: position.y - boundary.top_left.1,
            face: *cube_face,
            facing: position.facing,
        }
    }

    fn example_move_in_direction(&self, direction: Orientation, size: isize) -> Self {
        match (self.face, direction) {
            (CubeFace::A, Orientation::Up) => Self {
                x: size - self.x - 1,
                y: 0,
                face: CubeFace::B,
                facing: Orientation::Down,
            },
            (CubeFace::A, Orientation::Down) => Self {
                x: self.x,
                y: 0,
                face: CubeFace::D,
                facing: Orientation::Down,
            },
            (CubeFace::A, Orientation::Left) => Self {
                x: self.y,
                y: 0,
                face: CubeFace::C,
                facing: Orientation::Down,
            },
            (CubeFace::A, Orientation::Right) => Self {
                x: size - 1,
                y: size - self.y - 1,
                face: CubeFace::F,
                facing: Orientation::Left,
            },
            (CubeFace::B, Orientation::Up) => Self {
                x: size - self.x - 1,
                y: 0,
                face: CubeFace::A,
                facing: Orientation::Down,
            },
            (CubeFace::B, Orientation::Down) => Self {
                x: size - self.x - 1,
                y: size - 1,
                face: CubeFace::E,
                facing: Orientation::Up,
            },
            (CubeFace::B, Orientation::Left) => Self {
                x: size - self.y - 1,
                y: size - 1,
                face: CubeFace::F,
                facing: Orientation::Up,
            },
            (CubeFace::B, Orientation::Right) => Self {
                x: 0,
                y: self.y,
                face: CubeFace::C,
                facing: Orientation::Right,
            },
            (CubeFace::C, Orientation::Up) => Self {
                x: 0,
                y: self.x,
                face: CubeFace::A,
                facing: Orientation::Right,
            },
            (CubeFace::C, Orientation::Down) => Self {
                x: 0,
                y: self.x,
                face: CubeFace::E,
                facing: Orientation::Right,
            },
            (CubeFace::C, Orientation::Left) => Self {
                x: size - 1,
                y: self.y,
                face: CubeFace::B,
                facing: Orientation::Left,
            },
            (CubeFace::C, Orientation::Right) => Self {
                x: 0,
                y: self.y,
                face: CubeFace::D,
                facing: Orientation::Right,
            },
            (CubeFace::D, Orientation::Up) => Self {
                x: self.x,
                y: size - 1,
                face: CubeFace::A,
                facing: Orientation::Up,
            },
            (CubeFace::D, Orientation::Down) => Self {
                x: self.x,
                y: 0,
                face: CubeFace::E,
                facing: Orientation::Down,
            },
            (CubeFace::D, Orientation::Left) => Self {
                x: size - 1,
                y: self.y,
                face: CubeFace::C,
                facing: Orientation::Left,
            },
            (CubeFace::D, Orientation::Right) => Self {
                x: size - self.y - 1,
                y: 0,
                face: CubeFace::F,
                facing: Orientation::Up,
            },
            (CubeFace::E, Orientation::Up) => Self {
                x: self.x,
                y: size - 1,
                face: CubeFace::D,
                facing: Orientation::Up,
            },
            (CubeFace::E, Orientation::Down) => Self {
                x: size - self.x - 1,
                y: size - 1,
                face: CubeFace::B,
                facing: Orientation::Up,
            },
            (CubeFace::E, Orientation::Left) => Self {
                x: size - self.y - 1,
                y: size - 1,
                face: CubeFace::C,
                facing: Orientation::Up,
            },
            (CubeFace::E, Orientation::Right) => Self {
                x: 0,
                y: self.y,
                face: CubeFace::F,
                facing: Orientation::Right,
            },
            (CubeFace::F, Orientation::Up) => Self {
                x: size - 1,
                y: size - self.x - 1,
                face: CubeFace::D,
                facing: Orientation::Left,
            },
            (CubeFace::F, Orientation::Down) => Self {
                x: 0,
                y: size - self.x - 1,
                face: CubeFace::B,
                facing: Orientation::Right,
            },
            (CubeFace::F, Orientation::Left) => Self {
                x: size - 1,
                y: self.y,
                face: CubeFace::E,
                facing: Orientation::Left,
            },
            (CubeFace::F, Orientation::Right) => Self {
                x: size - 1,
                y: size - self.y - 1,
                face: CubeFace::E,
                facing: Orientation::Left,
            },
        }
    }

    fn real_move_in_direction(&self, direction: Orientation, size: isize) -> Self {
        match (self.face, direction) {
            (CubeFace::A, Orientation::Up) => Self {
                x: 0,
                y: self.x,
                face: CubeFace::F,
                facing: Orientation::Right,
            },
            (CubeFace::A, Orientation::Down) => Self {
                x: self.x,
                y: 0,
                face: CubeFace::C,
                facing: Orientation::Down,
            },
            (CubeFace::A, Orientation::Left) => Self {
                x: 0,
                y: size - self.y - 1,
                face: CubeFace::D,
                facing: Orientation::Right,
            },
            (CubeFace::A, Orientation::Right) => Self {
                x: 0,
                y: self.y,
                face: CubeFace::B,
                facing: Orientation::Right,
            },
            (CubeFace::B, Orientation::Up) => Self {
                x: self.x,
                y: size - 1,
                face: CubeFace::F,
                facing: Orientation::Up,
            },
            (CubeFace::B, Orientation::Down) => Self {
                x: size - 1,
                y: self.x,
                face: CubeFace::C,
                facing: Orientation::Left,
            },
            (CubeFace::B, Orientation::Left) => Self {
                x: size - 1,
                y: self.y,
                face: CubeFace::A,
                facing: Orientation::Left,
            },
            (CubeFace::B, Orientation::Right) => Self {
                x: size - 1,
                y: size - self.y - 1,
                face: CubeFace::E,
                facing: Orientation::Left,
            },
            (CubeFace::C, Orientation::Up) => Self {
                x: self.x,
                y: size - 1,
                face: CubeFace::A,
                facing: Orientation::Up,
            },
            (CubeFace::C, Orientation::Down) => Self {
                x: self.x,
                y: 0,
                face: CubeFace::E,
                facing: Orientation::Down,
            },
            (CubeFace::C, Orientation::Left) => Self {
                x: self.y,
                y: 0,
                face: CubeFace::D,
                facing: Orientation::Down,
            },
            (CubeFace::C, Orientation::Right) => Self {
                x: self.y,
                y: size - 1,
                face: CubeFace::B,
                facing: Orientation::Up,
            },
            (CubeFace::D, Orientation::Up) => Self {
                x: 0,
                y: self.x,
                face: CubeFace::C,
                facing: Orientation::Right,
            },
            (CubeFace::D, Orientation::Down) => Self {
                x: self.x,
                y: 0,
                face: CubeFace::F,
                facing: Orientation::Down,
            },
            (CubeFace::D, Orientation::Left) => Self {
                x: 0,
                y: size - self.y - 1,
                face: CubeFace::A,
                facing: Orientation::Right,
            },
            (CubeFace::D, Orientation::Right) => Self {
                x: 0,
                y: self.y,
                face: CubeFace::E,
                facing: Orientation::Right,
            },
            (CubeFace::E, Orientation::Up) => Self {
                x: self.x,
                y: size - 1,
                face: CubeFace::C,
                facing: Orientation::Up,
            },
            (CubeFace::E, Orientation::Down) => Self {
                x: size - 1,
                y: self.x,
                face: CubeFace::F,
                facing: Orientation::Left,
            },
            (CubeFace::E, Orientation::Left) => Self {
                x: size - 1,
                y: self.y,
                face: CubeFace::D,
                facing: Orientation::Left,
            },
            (CubeFace::E, Orientation::Right) => Self {
                x: size - 1,
                y: size - self.y - 1,
                face: CubeFace::B,
                facing: Orientation::Left,
            },
            (CubeFace::F, Orientation::Up) => Self {
                x: self.x,
                y: size - 1,
                face: CubeFace::D,
                facing: Orientation::Up,
            },
            (CubeFace::F, Orientation::Down) => Self {
                x: self.x,
                y: 0,
                face: CubeFace::B,
                facing: Orientation::Down,
            },
            (CubeFace::F, Orientation::Left) => Self {
                x: self.y,
                y: 0,
                face: CubeFace::A,
                facing: Orientation::Down,
            },
            (CubeFace::F, Orientation::Right) => Self {
                x: self.y,
                y: size - 1,
                face: CubeFace::E,
                facing: Orientation::Up,
            },
        }
    }

    fn to_global_position(self, cube_layout: &CubeLayout) -> CubePosition {
        let cube_boundary = cube_layout.get(&self.face).unwrap();
        CubePosition {
            x: cube_boundary.top_left.0 + self.x,
            y: cube_boundary.top_left.1 + self.y,
            facing: self.facing,
        }
    }
}

impl Orientation {
    fn rotate_right(&self) -> Self {
        match self {
            Orientation::Up => Orientation::Right,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
            Orientation::Right => Orientation::Down,
        }
    }

    fn rotate_left(&self) -> Self {
        match self {
            Orientation::Up => Orientation::Left,
            Orientation::Down => Orientation::Right,
            Orientation::Left => Orientation::Down,
            Orientation::Right => Orientation::Up,
        }
    }
}

#[cfg(test)]
mod day22_tests {
    use crate::day22::{self, CubeFacePosition};

    use super::{input_parser, CubePosition};

    #[test]
    fn test_1() {
        let input = include_str!("../../data/2022/22-example.txt");
        let size = 4;

        let (_, (map, _directions)) = input_parser(input).unwrap();

        let position = CubePosition {
            x: 11,
            y: 5,
            facing: day22::Orientation::Right,
        };

        let cube_layout = day22::map_to_cube(&map, size);

        let relative_in_current_cube =
            CubeFacePosition::from_global_position(&position, &cube_layout);

        assert_eq!(relative_in_current_cube.face, day22::CubeFace::D);

        let result = position.move_right(&map, &cube_layout, size);

        assert_eq!(result.x, 14);
        assert_eq!(result.y, 8);
        assert_eq!(result.facing, day22::Orientation::Up);
    }

    #[test]
    fn test_2() {
        let input = include_str!("../../data/2022/22-example.txt");
        let size = 4;

        let (_, (map, _directions)) = input_parser(input).unwrap();

        let position = CubePosition {
            x: 10,
            y: 11,
            facing: day22::Orientation::Down,
        };

        let cube_layout = day22::map_to_cube(&map, size);

        let relative_in_current_cube =
            CubeFacePosition::from_global_position(&position, &cube_layout);

        assert_eq!(relative_in_current_cube.face, day22::CubeFace::E);

        let result = position.move_down(&map, &cube_layout, size);

        assert_eq!(result.x, 1);
        assert_eq!(result.y, 7);
        assert_eq!(result.facing, day22::Orientation::Up);
    }
}
