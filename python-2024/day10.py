from collections import defaultdict
import copy
from dataclasses import dataclass
from enum import Enum
import itertools


@dataclass
class Input:
    topological_map: dict[tuple[int, int], int]
    trailheads: list[tuple[int, int]]


def read_input() -> Input:
    data_file = open("../data/2024/10.txt", "r")
    lines = data_file.readlines()
    data_file.close()

    result = Input(topological_map={}, trailheads=[])
    for y, line in enumerate(lines):
        for x, char in enumerate(line.strip()):
            result.topological_map[(x, y)] = int(char) if char != "." else -1
            if char == "0":
                result.trailheads.append((x, y))

    return result


def calculate_peak_paths(input_data: Input, trailhead: tuple[int, int]) -> int:
    been = set()
    to_visit: set[tuple[int, int]] = set()
    to_visit.add(trailhead)
    peaks = set()
    while to_visit:
        current = to_visit.pop()
        been.add(current)

        if input_data.topological_map[current] == 9:
            peaks.add(current)
            continue

        for direction in [(0, -1), (1, 0), (0, 1), (-1, 0)]:
            next_x = current[0] + direction[0]
            next_y = current[1] + direction[1]
            next_pos = (next_x, next_y)
            if next_pos in been:
                continue

            if (
                next_pos in input_data.topological_map
                and input_data.topological_map[next_pos]
                == input_data.topological_map[current] + 1
            ):
                to_visit.add(next_pos)

    return len(peaks)


def calculate_number_of_paths(input_data: Input, trailhead: tuple[int, int]) -> int:
    to_visit: list[tuple[int, int]] = []
    to_visit.append(trailhead)
    peaks = 0
    while to_visit:
        current = to_visit.pop()

        if input_data.topological_map[current] == 9:
            peaks += 1
            continue

        for direction in [(0, -1), (1, 0), (0, 1), (-1, 0)]:
            next_x = current[0] + direction[0]
            next_y = current[1] + direction[1]
            next_pos = (next_x, next_y)

            if (
                next_pos in input_data.topological_map
                and input_data.topological_map[next_pos]
                == input_data.topological_map[current] + 1
            ):
                to_visit.append(next_pos)

    return peaks


def part_1() -> None:
    input_data = read_input()

    result = 0
    for trailhead in input_data.trailheads:
        result += calculate_peak_paths(input_data, trailhead)

    print(result)


def part_2() -> None:
    input_data = read_input()
    result = 0
    for trailhead in input_data.trailheads:
        result += calculate_number_of_paths(input_data, trailhead)
    print(result)


part_1()
part_2()
