use itertools::Itertools;

pub fn part1() -> usize {
    let input = include_str!("../../data/2021/day01.txt");
    let result = input.lines().map(|n| n.parse::<usize>().unwrap()).sum();
    result
}

pub fn part2() -> usize {
    let input = include_str!("../../data/2021/day01.txt");
    let result = input
        .lines()
        .map(|n| n.parse::<usize>().unwrap())
        .fold(1, |acc, e| acc * e);
    result
}
