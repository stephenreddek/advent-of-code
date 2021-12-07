use itertools::Itertools;

#[derive(Debug)]
enum Instruction {
    Up(i32),
    Down(i32),
    Forward(i32),
}

pub fn part1() -> i32 {
    let input = include_str!("../../data/2021/day02.txt");
    let instructions = parse_input(input);
    let (final_x, final_y) = instructions
        .iter()
        .fold((0, 0), |(x, y), current| match current {
            Instruction::Forward(distance) => (x + distance, y),
            Instruction::Up(distance) => (x, y - distance),
            Instruction::Down(distance) => (x, y + distance),
        });
    final_x * final_y
}

pub fn part2() -> i32 {
    let input = include_str!("../../data/2021/day02.txt");
    let instructions = parse_input(input);
    let (final_x, final_y, _) =
        instructions
            .iter()
            .fold((0, 0, 0), |(x, y, aim), current| match current {
                Instruction::Forward(distance) => (x + distance, y + (aim * distance), aim),
                Instruction::Up(distance) => (x, y, aim - distance),
                Instruction::Down(distance) => (x, y, aim + distance),
            });
    final_x * final_y
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter_map(|line| line.split_once(" "))
        .filter_map(|(direction, amount)| match (direction, amount.parse()) {
            ("forward", Ok(distance)) => Some(Instruction::Forward(distance)),
            ("up", Ok(distance)) => Some(Instruction::Up(distance)),
            ("down", Ok(distance)) => Some(Instruction::Down(distance)),
            _ => None,
        })
        .collect_vec()
}
