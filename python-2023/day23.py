from __future__ import annotations
from typing import Set, Optional, Callable
import math
from enum import Enum
from itertools import groupby, takewhile
from functools import cmp_to_key
from heapq import heapify, heappush, heappop
import random


def read_file(
    file_name: str,
) -> tuple[int, int, dict[tuple[int, int], str]]:
    data_file = open(file_name, "r")
    lines = data_file.readlines()
    data_file.close()

    grid: dict[tuple[int, int], str] = {}
    for y, line in enumerate(lines):
        for x, char in enumerate(list(line.strip())):
            grid[(x, y)] = char

    return (len(lines[0]), len(lines), grid)


def generate_next_steps_with_slopes(
    grid: dict[tuple[int, int], str],
    position: tuple[int, int],
    history: Set[tuple[int, int]],
) -> list[tuple[int, int]]:
    (current_x, current_y) = position

    all_adjacent = [
        (current_x + 1, current_y),
        (current_x - 1, current_y),
        (current_x, current_y - 1),
        (current_x, current_y + 1),
    ]

    current_tile = grid[position]
    if current_tile != ".":
        # on a slope
        if current_tile == ">":
            all_adjacent = [(current_x + 1, current_y)]

        elif current_tile == "<":
            all_adjacent = [(current_x - 1, current_y)]
        elif current_tile == "v":
            all_adjacent = [(current_x, current_y + 1)]
        elif current_tile == "^":
            all_adjacent = [(current_x, current_y - 1)]
        else:
            raise Exception(f"Invalid tile [{current_tile}]")

    valid_positions = [
        location
        for location in all_adjacent
        if location in grid and location not in history and grid[location] != "#"
    ]
    return valid_positions


def generate_next_steps(
    grid: dict[tuple[int, int], str],
    position: tuple[int, int],
    history: Set[tuple[int, int]],
) -> list[tuple[int, int]]:
    (current_x, current_y) = position

    all_adjacent = [
        (current_x + 1, current_y),
        (current_x - 1, current_y),
        (current_x, current_y - 1),
        (current_x, current_y + 1),
    ]

    valid_positions = [
        location
        for location in all_adjacent
        if location in grid and location not in history and grid[location] != "#"
    ]
    return valid_positions


def part_1() -> None:
    (max_x, max_y, grid) = read_file("../data/2023/23.txt")

    starting_spot = (0, 0)
    ending_spot = (0, 0)
    for x in range(0, max_x):
        if grid[(x, 0)] == ".":
            starting_spot = (x, 0)
            break

    for x in range(0, max_x):
        if grid[(x, max_y - 1)] == ".":
            ending_spot = (x, max_y - 1)
            break

    possible_hikes = []
    building_hikes = [(0, starting_spot, set([starting_spot]))]
    while len(building_hikes) > 0:
        (steps, spot, history) = building_hikes.pop()
        possible_next_steps = generate_next_steps_with_slopes(grid, spot, history)
        if len(possible_next_steps) == 0:
            if ending_spot in history:
                possible_hikes.append(steps)
        else:
            for next_step in possible_next_steps:
                next_history = history.copy()
                next_history.add(next_step)
                building_hikes.append((steps + 1, next_step, next_history))

    score = max(possible_hikes)
    print(score)


def manhattan_distance(point: tuple[int, int], end: tuple[int, int]) -> int:
    return abs(point[0] - end[0]) + abs(point[1] - end[1])


def part_2() -> None:
    (max_x, max_y, grid) = read_file("../data/2023/23.txt")

    starting_spot = (0, 0)
    ending_spot = (0, 0)
    for x in range(0, max_x):
        if grid[(x, 0)] == ".":
            starting_spot = (x, 0)
            break

    for x in range(0, max_x):
        if grid[(x, max_y - 1)] == ".":
            ending_spot = (x, max_y - 1)
            break

    possible_hikes = []
    building_hikes = [
        (
            -manhattan_distance(starting_spot, ending_spot),
            0,
            starting_spot,
            set([starting_spot]),
        )
    ]
    heapify(building_hikes)
    cache: dict[tuple[int, int], int] = {}
    while len(building_hikes) > 0:
        (_, steps, spot, history) = heappop(building_hikes)
        # (steps, spot, history) = building_hikes.pop()
        possible_next_steps = generate_next_steps(grid, spot, history)
        if len(possible_next_steps) == 0:
            if ending_spot in history:
                print(f"-----possible: {steps}")
                possible_hikes.append(steps)
        else:
            for next_step in possible_next_steps:
                next_history = history.copy()
                next_history.add(next_step)
                # if next_step not in cache or cache[next_step] < steps + 1:
                heappush(
                    building_hikes,
                    (
                        -manhattan_distance(next_step, ending_spot),
                        steps - 1,
                        next_step,
                        next_history,
                    ),
                )
                # building_hikes.append((steps + 1, next_step, next_history))
                # cache[next_step] = steps + 1

    score = -min(possible_hikes)
    # print(possible_hikes)
    score = max(possible_hikes)
    print(score)


# part_1()
part_2()  # 5102 is too low
