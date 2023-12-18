from __future__ import annotations
from typing import Set, Optional
import math
from enum import Enum
from itertools import groupby, takewhile
from functools import cmp_to_key
from heapq import heapify, heappush, heappop


def read_file(file_name: str) -> list[tuple[str, int, str]]:
    data_file = open(file_name, "r")
    lines = data_file.readlines()
    data_file.close()

    result = []

    for line in lines:
        [direction, amount, color_section] = line.strip().split()
        color = color_section[2:-1]
        result.append((direction, int(amount), color))

    return result


def follow_direction(
    position: tuple[int, int], direction: str, amount: int
) -> tuple[int, int]:
    if direction == "U":
        return (position[0], position[1] - amount)
    elif direction == "D":
        return (position[0], position[1] + amount)
    elif direction == "R":
        return (position[0] + amount, position[1])
    elif direction == "L":
        return (position[0] - amount, position[1])
    else:
        raise Exception(f"Invalid direction [{direction}]")


def find_corners_of_container(
    grid: Set[tuple[int, int]]
) -> tuple[tuple[int, int], tuple[int, int]]:
    min_x = 0
    min_y = 0
    max_x = 0
    max_y = 0
    for x, y in grid:
        if x > max_x:
            max_x = x
        elif x < min_x:
            min_x = x

        if y > max_y:
            max_y = y
        elif y < min_y:
            min_y = y

    return ((min_x - 1, min_y - 1), (max_x + 1, max_y + 1))


def size_of_unflooded(grid: Set[tuple[int, int]]) -> int:
    ((min_x, min_y), (max_x, max_y)) = find_corners_of_container(grid)
    flooded: Set[tuple[int, int]] = set()
    to_flood = [(min_x, min_y)]

    while len(to_flood):
        spot = to_flood.pop()
        if spot[0] > max_x or spot[1] > max_y:
            continue

        if spot[0] < min_x or spot[1] < min_y:
            continue

        if spot in grid or spot in flooded:
            continue

        flooded.add(spot)
        to_flood.append((spot[0] + 1, spot[1]))
        to_flood.append((spot[0], spot[1] + 1))
        to_flood.append((spot[0] - 1, spot[1]))
        to_flood.append((spot[0], spot[1] - 1))

    total_area = (max_x - min_x + 1) * (max_y - min_y + 1)

    return total_area - len(flooded)


def part_1() -> None:
    instructions = read_file("../data/2023/18.txt")

    grid: Set[tuple[int, int]] = set()

    position = (0, 0)
    grid.add(position)
    for direction, amount, _ in instructions:
        for _ in range(0, amount):
            position = follow_direction(position, direction, 1)
            grid.add(position)

    score = size_of_unflooded(grid)
    print(score)


def color_to_instruction(color: str) -> tuple[str, int]:
    amount_hex = color[:-1]
    direction_hex = color[-1]
    direction = ""
    if direction_hex == "0":
        direction = "R"
    elif direction_hex == "1":
        direction = "D"
    elif direction_hex == "2":
        direction = "L"
    elif direction_hex == "3":
        direction = "U"
    else:
        raise Exception(f"Invalid direction [{direction_hex}]")

    return (direction, int(amount_hex, 16))


def part_2() -> None:
    raw_instructions = read_file("../data/2023/18.txt")
    instructions = [color_to_instruction(color) for _, _, color in raw_instructions]

    vertices: list[tuple[tuple[int, int], tuple[int, int]]] = []

    position = (0, 0)

    for direction, amount in instructions:
        next_position = follow_direction(position, direction, amount)
        vertices.append((position, next_position))
        position = next_position

    print(vertices)

    # trapezoid formula
    internal_area = abs(
        sum(((y2 - y1) * (x1 + x2)) for ((x1, y1), (x2, y2)) in vertices) // 2
    )
    wall_area = sum(wall_size for (_, wall_size) in instructions) // 2
    score = internal_area + wall_area + 1
    print(score)


part_1()
part_2()
