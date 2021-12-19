use bitreader::BitReader;
use hex;
use itertools::Itertools;
use nom::{
    bytes::complete::tag, combinator::map_res, multi::separated_list1, sequence::separated_pair,
    IResult,
};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, PartialEq)]
struct Packet {
    version: u64,
    type_id: u64,
    contents: PacketContents,
}

#[derive(Debug, PartialEq)]
enum PacketContents {
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    Literal(u64),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    Equal(Vec<Packet>),
}

pub fn part1() -> u64 {
    let input = include_str!("../../data/2021/16.txt");

    match input_parser(input) {
        Ok((remaining_input, packet)) if remaining_input.is_empty() => {
            println!("parsed entire input");
            sum_packet_version(&packet)
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

pub fn part2() -> u64 {
    let input = include_str!("../../data/2021/16.txt");

    match input_parser(input) {
        Ok((remaining_input, packet)) if remaining_input.is_empty() => {
            println!("parsed entire input");

            evaluate_packet(&packet)
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

fn evaluate_packet(packet: &Packet) -> u64 {
    match &packet.contents {
        PacketContents::Literal(value) => *value,
        PacketContents::Sum(operands) => operands
            .iter()
            .fold(0, |acc, packet| acc + evaluate_packet(packet)),
        PacketContents::Product(operands) => operands
            .iter()
            .fold(1, |acc, packet| acc * evaluate_packet(packet)),
        PacketContents::Minimum(operands) => operands
            .iter()
            .map(|packet| evaluate_packet(packet))
            .min()
            .unwrap(),
        PacketContents::Maximum(operands) => operands
            .iter()
            .map(|packet| evaluate_packet(packet))
            .max()
            .unwrap(),
        PacketContents::GreaterThan(operands) => {
            let operand_1_value = evaluate_packet(&operands[0]);
            let operand_2_value = evaluate_packet(&operands[1]);
            if operand_1_value > operand_2_value {
                1
            } else {
                0
            }
        }
        PacketContents::LessThan(operands) => {
            let operand_1_value = evaluate_packet(&operands[0]);
            let operand_2_value = evaluate_packet(&operands[1]);
            if operand_1_value < operand_2_value {
                1
            } else {
                0
            }
        }
        PacketContents::Equal(operands) => {
            let operand_1_value = evaluate_packet(&operands[0]);
            let operand_2_value = evaluate_packet(&operands[1]);
            if operand_1_value == operand_2_value {
                1
            } else {
                0
            }
        }
    }
}

fn sum_packet_version(packet: &Packet) -> u64 {
    sum_packet_version_help(0, packet)
}

fn sum_packet_version_help(acc: u64, packet: &Packet) -> u64 {
    match &packet.contents {
        PacketContents::Literal(_) => acc + packet.version,
        PacketContents::Sum(operands) => acc + packet.version + sum_operands_version(operands),
        PacketContents::Product(operands) => acc + packet.version + sum_operands_version(operands),
        PacketContents::Minimum(operands) => acc + packet.version + sum_operands_version(operands),
        PacketContents::Maximum(operands) => acc + packet.version + sum_operands_version(operands),
        PacketContents::GreaterThan(operands) => {
            acc + packet.version + sum_operands_version(operands)
        }
        PacketContents::LessThan(operands) => acc + packet.version + sum_operands_version(operands),
        PacketContents::Equal(operands) => acc + packet.version + sum_operands_version(operands),
    }
}

fn sum_operands_version(packets: &Vec<Packet>) -> u64 {
    packets.iter().fold(0, |packets_acc, operand| {
        packets_acc + sum_packet_version(operand)
    })
}

fn literal_group_parser(reader: &mut BitReader) -> (bool, u64) {
    let has_more_groups = reader.read_bool().unwrap();
    let group_value = reader.read_u64(4).unwrap();

    (has_more_groups, group_value)
}

fn literal_parser(reader: &mut BitReader) -> PacketContents {
    let mut literal: u64 = 0;
    loop {
        let (has_more_groups, group_value) = literal_group_parser(reader);
        literal = (literal << 4) + group_value;

        if !has_more_groups {
            return PacketContents::Literal(literal);
        }
    }
}

fn operand_parser(reader: &mut BitReader) -> Vec<Packet> {
    let length_type_id = reader.read_u64(1).unwrap();
    match length_type_id {
        0 => {
            let sub_packet_length: u64 = reader.read_u64(15).unwrap();
            let current_position: u64 = reader.position();
            let ending_position = current_position + sub_packet_length;
            let mut sub_packets = Vec::new();
            loop {
                sub_packets.push(packet_parser(reader));
                if reader.position() == ending_position {
                    break;
                }
            }
            sub_packets
        }
        1 => {
            let sub_packet_count = reader.read_u64(11).unwrap();
            let mut sub_packets = Vec::with_capacity(sub_packet_count.try_into().unwrap());
            for _ in 0..sub_packet_count {
                sub_packets.push(packet_parser(reader));
            }
            sub_packets
        }
        _ => {
            panic!("Invalid length typ id {:?}", length_type_id);
        }
    }
}

fn packet_parser(reader: &mut BitReader) -> Packet {
    let version = reader.read_u64(3).unwrap();
    let id = reader.read_u64(3).unwrap();

    let packet_contents = match id {
        0 => PacketContents::Sum(operand_parser(reader)),
        1 => PacketContents::Product(operand_parser(reader)),
        2 => PacketContents::Minimum(operand_parser(reader)),
        3 => PacketContents::Maximum(operand_parser(reader)),
        4 => literal_parser(reader),
        5 => PacketContents::GreaterThan(operand_parser(reader)),
        6 => PacketContents::LessThan(operand_parser(reader)),
        7 => PacketContents::Equal(operand_parser(reader)),
        _ => panic!("invalid type id"),
    };

    Packet {
        version: version,
        type_id: id,
        contents: packet_contents,
    }
}

fn packet_parser_from_string(hex_string: &str) -> Result<Packet, String> {
    let bytes = hex::decode(hex_string).unwrap();
    let mut reader = BitReader::new(&bytes);

    Ok(packet_parser(&mut reader))
}

fn input_parser(input: &str) -> IResult<&str, Packet> {
    let (input, packet) = map_res(
        nom::character::complete::hex_digit1,
        packet_parser_from_string,
    )(input)?;

    let (input, _) = nom::character::complete::newline(input)?;

    Ok((input, packet))
}

#[cfg(test)]
mod day16_tests {
    use crate::day16::evaluate_packet;
    use crate::day16::input_parser;
    use crate::day16::Packet;
    use crate::day16::PacketContents;

    #[test]
    fn can_parse_literal_value() {
        let test_string = "D2FE28\n"; //"110100101111111000101000";
        let result = input_parser(test_string);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().1,
            Packet {
                version: 6,
                type_id: 4,
                contents: PacketContents::Literal(2021)
            }
        );
    }

    #[test]
    fn can_parse_operator_with_two_literals() {
        let test_string = "38006F45291200\n";
        let result = input_parser(test_string);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().1,
            Packet {
                version: 1,
                type_id: 6,
                contents: PacketContents::LessThan(vec![
                    Packet {
                        version: 6,
                        type_id: 4,
                        contents: PacketContents::Literal(10)
                    },
                    Packet {
                        version: 2,
                        type_id: 4,
                        contents: PacketContents::Literal(20)
                    }
                ]),
            }
        );
    }

    #[test]
    fn can_parse_operator_with_three_literals() {
        let test_string = "EE00D40C823060\n";
        let result = input_parser(test_string);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().1,
            Packet {
                version: 7,
                type_id: 3,
                contents: PacketContents::Maximum(vec![
                    Packet {
                        version: 2,
                        type_id: 4,
                        contents: PacketContents::Literal(1)
                    },
                    Packet {
                        version: 4,
                        type_id: 4,
                        contents: PacketContents::Literal(2)
                    },
                    Packet {
                        version: 1,
                        type_id: 4,
                        contents: PacketContents::Literal(3)
                    }
                ]),
            }
        );
    }

    #[test]
    fn evaluating_C200B40A82() {
        let test_string = "C200B40A82\n";
        let result = input_parser(test_string);
        assert!(result.is_ok());
        let evaluation = evaluate_packet(&result.unwrap().1);
        assert_eq!(evaluation, 3);
    }

    #[test]
    fn evaluating_04005AC33890() {
        let test_string = "04005AC33890\n";
        let result = input_parser(test_string);
        assert!(result.is_ok());
        let evaluation = evaluate_packet(&result.unwrap().1);
        assert_eq!(evaluation, 54);
    }

    #[test]
    fn evaluating_880086C3E88112() {
        let test_string = "880086C3E88112\n";
        let result = input_parser(test_string);
        assert!(result.is_ok());
        let evaluation = evaluate_packet(&result.unwrap().1);
        assert_eq!(evaluation, 7);
    }

    #[test]
    fn evaluating_CE00C43D881120() {
        let test_string = "CE00C43D881120\n";
        let result = input_parser(test_string);
        assert!(result.is_ok());
        let evaluation = evaluate_packet(&result.unwrap().1);
        assert_eq!(evaluation, 9);
    }

    #[test]
    fn evaluating_D8005AC2A8F0() {
        let test_string = "D8005AC2A8F0\n";
        let result = input_parser(test_string);
        assert!(result.is_ok());
        let evaluation = evaluate_packet(&result.unwrap().1);
        assert_eq!(evaluation, 1);
    }

    #[test]
    fn evaluating_F600BC2D8F() {
        let test_string = "F600BC2D8F\n";
        let result = input_parser(test_string);
        assert!(result.is_ok());
        let evaluation = evaluate_packet(&result.unwrap().1);
        assert_eq!(evaluation, 0);
    }

    #[test]
    fn evaluating_9C005AC2F8F0() {
        let test_string = "9C005AC2F8F0\n";
        let result = input_parser(test_string);
        assert!(result.is_ok());
        let evaluation = evaluate_packet(&result.unwrap().1);
        assert_eq!(evaluation, 0);
    }

    #[test]
    fn evaluating_9C0141080250320F1802104A08() {
        let test_string = "9C0141080250320F1802104A08\n";
        let result = input_parser(test_string);
        assert!(result.is_ok());
        let evaluation = evaluate_packet(&result.unwrap().1);
        assert_eq!(evaluation, 1);
    }
}
