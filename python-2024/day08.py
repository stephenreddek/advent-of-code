from collections import defaultdict
import copy
from dataclasses import dataclass
from enum import Enum
import itertools


@dataclass
class Input:
    grid: dict[tuple[int, int], str]
    antennas: dict[str, list[tuple[int, int]]]


def read_input() -> Input:
    data_file = open("../data/2024/08.txt", "r")
    lines = data_file.readlines()
    data_file.close()

    result = Input(grid={}, antennas=defaultdict(list))
    for y, line in enumerate(lines):
        for x, char in enumerate(line.strip()):
            result.grid[(x, y)] = char
            if char != "." and char != "#":
                result.antennas[char].append((x, y))

    return result


def part_1() -> None:
    input_data = read_input()

    result = set()
    for antenna_type, positions in input_data.antennas.items():
        antenna_pairs = itertools.combinations(positions, 2)
        for first, second in antenna_pairs:
            delta_x = second[0] - first[0]
            delta_y = second[1] - first[1]
            possible_antinodes = [
                (first[0] - delta_x, first[1] - delta_y),
                (second[0] + delta_x, second[1] + delta_y),
            ]
            # print(first, second)
            # print(possible_antinodes)
            antinodes = [
                possibility
                for possibility in possible_antinodes
                if possibility in input_data.grid
            ]

            result.update(set(antinodes))

    print(len(result))


def part_2() -> None:
    input_data = read_input()

    result = set()
    for antenna_type, positions in input_data.antennas.items():
        antenna_pairs = itertools.combinations(positions, 2)
        for first, second in antenna_pairs:
            result.add(first)
            result.add(second)

            delta_x = second[0] - first[0]
            delta_y = second[1] - first[1]

            last = (first[0], first[1])
            while True:
                to_check = (last[0] - delta_x, last[1] - delta_y)
                if to_check in input_data.grid:
                    result.add(to_check)
                    last = to_check
                else:
                    break

            last = (second[0], second[1])
            while True:
                to_check = (last[0] + delta_x, last[1] + delta_y)
                if to_check in input_data.grid:
                    result.add(to_check)
                    last = to_check
                else:
                    break

    print(len(result))


part_1()
part_2()
