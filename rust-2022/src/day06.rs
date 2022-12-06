use std::collections::VecDeque;

use itertools::Itertools;
use nom::{
    bytes::complete::tag, combinator::map_res, multi::separated_list0, multi::separated_list1,
    IResult,
};

#[derive(Debug, PartialEq)]
struct MoveInstruction {
    from: usize,
    to: usize,
    count: usize,
}

pub fn part1() -> usize {
    let input = include_str!("../../data/2022/06.txt");
    length_before_packet_start(input)
}

pub fn part2() -> usize {
    let input = include_str!("../../data/2022/06.txt");
    length_before_message_start(input)
}

pub fn length_before_packet_start(stream: &str) -> usize {
    let mut potential_packet = VecDeque::new();
    for (index, c) in stream.chars().enumerate() {
        if index < 3 {
            potential_packet.push_front(c);
            continue;
        }

        if index > 3 {
            potential_packet.pop_back();
        }

        potential_packet.push_front(c);

        if is_unique(&potential_packet, 4) {
            return index + 1;
        }
    }

    panic!("Unable to find a unique sequence")
}

pub fn length_before_message_start(stream: &str) -> usize {
    let mut potential_packet = VecDeque::new();
    for (index, c) in stream.chars().enumerate() {
        if index < 14 {
            potential_packet.push_front(c);
            continue;
        }

        if index > 14 {
            potential_packet.pop_back();
        }

        potential_packet.push_front(c);

        if is_unique(&potential_packet, 14) {
            return index + 1;
        }
    }

    panic!("Unable to find a unique sequence")
}

fn is_unique(packet: &VecDeque<char>, len: usize) -> bool {
    packet.iter().unique().count() == len
}

#[cfg(test)]
mod day06_tests {
    use crate::day06::length_before_packet_start;

    #[test]
    fn example_0() {
        assert_eq!(length_before_packet_start("mjqa"), 4);
    }

    #[test]
    fn example_1() {
        assert_eq!(
            length_before_packet_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            7
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            length_before_packet_start("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            5
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            length_before_packet_start("nppdvjthqldpwncqszvftbrmjlhg"),
            6
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            length_before_packet_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            10
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(
            length_before_packet_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            11
        );
    }
}
