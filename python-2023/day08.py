from __future__ import annotations
from typing import Set, Optional
import math
from enum import Enum
from itertools import groupby
from functools import cmp_to_key


def read_file(file_name: str) -> tuple[list[str], dict[str, tuple[str, str]]]:
    data_file = open(file_name, "r")
    lines = data_file.readlines()
    data_file.close()

    directions = list(lines[0].strip())
    nodes: dict[str, tuple[str, str]] = {}
    for line in lines[2:]:
        [node, connections] = line.strip().split(" = ")
        (left_raw, right_raw) = connections.split(", ")
        left = left_raw[1:]
        right = right_raw[:-1]
        nodes[node] = (left, right)

    return (directions, nodes)


def follow_direction(
    location: str, direction: str, nodes: dict[str, tuple[str, str]]
) -> str:
    (left, right) = nodes[location]
    return left if direction == "L" else right


def part_1() -> None:
    (directions, nodes) = read_file("../data/2023/08.txt")

    location = "AAA"
    steps_taken = 0
    while location != "ZZZ":
        direction_index = steps_taken % len(directions)
        direction = directions[direction_index]
        location = follow_direction(location, direction, nodes)
        steps_taken += 1

    print(steps_taken)


def is_finished(locations: list[str]) -> bool:
    return all(is_final_location(x) for x in locations)


def is_final_location(location: str) -> bool:
    return location[-1] == "Z"


def least_common_multiple(values: list[int]) -> int:
    lcm = 1
    for i in values:
        lcm = lcm * i // math.gcd(lcm, i)
    return lcm


def part_2() -> None:
    (directions, nodes) = read_file("../data/2023/08.txt")

    locations = [x for x in nodes.keys() if x[-1] == "A"]
    # while not is_finished(locations):
    #     direction_index = steps_taken % len(directions)
    #     direction = directions[direction_index]
    #     locations = [
    #         follow_direction(location, direction, nodes) for location in locations
    #     ]
    #     steps_taken += 1

    solutions = []
    for starting_location in locations:
        steps_taken = 0
        location = starting_location
        while not is_final_location(location):
            direction_index = steps_taken % len(directions)
            direction = directions[direction_index]
            location = follow_direction(location, direction, nodes)
            steps_taken += 1

        solutions.append(steps_taken)

    print(solutions)

    print(least_common_multiple(solutions))


part_1()
part_2()
