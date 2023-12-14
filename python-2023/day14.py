from __future__ import annotations
from typing import Set, Optional
import math
from enum import Enum
from itertools import groupby
from functools import cmp_to_key


def read_file(file_name: str) -> tuple[int, int, dict[tuple[int, int], str]]:
    data_file = open(file_name, "r")
    lines = data_file.readlines()
    data_file.close()

    grid = {}
    for y, line in enumerate(lines):
        for x, char in enumerate(list(line.strip())):
            grid[(x, y)] = char

    return (len(lines[0].strip()), len(lines), grid)


def load_for_rock(max_y: int, rock: tuple[int, int]) -> int:
    return max_y - rock[1]


def load_on_north_beam(max_x: int, max_y: int, grid: dict[tuple[int, int], str]) -> int:
    total = 0
    for y in range(0, max_y):
        for x in range(0, max_x):
            if grid[(x, y)] == "O":
                total += load_for_rock(max_y, (x, y))

    return total


def tilt_north_step(
    max_x: int, max_y: int, grid: dict[tuple[int, int], str]
) -> tuple[bool, dict[tuple[int, int], str]]:
    changed = False
    for y in range(0, max_y):
        for x in range(0, max_x):
            if grid[(x, y)] == "O":
                north_spot = (x, y - 1)
                if north_spot in grid and grid[north_spot] == ".":
                    changed = True
                    grid[north_spot] = "O"
                    grid[(x, y)] = "."

    return (changed, grid)


def tilt_north(
    max_x: int, max_y: int, grid: dict[tuple[int, int], str]
) -> dict[tuple[int, int], str]:
    result: dict[tuple[int, int], str]
    while True:
        (changed, after_step) = tilt_north_step(max_x, max_y, grid)
        result = after_step
        if not changed:
            break

    return result


def tilt_south_step(
    max_x: int, max_y: int, grid: dict[tuple[int, int], str]
) -> tuple[bool, dict[tuple[int, int], str]]:
    changed = False
    for y in range(max_y - 1, -1, -1):
        for x in range(0, max_x):
            if grid[(x, y)] == "O":
                south_spot = (x, y + 1)
                if south_spot in grid and grid[south_spot] == ".":
                    changed = True
                    grid[south_spot] = "O"
                    grid[(x, y)] = "."

    return (changed, grid)


def tilt_south(
    max_x: int, max_y: int, grid: dict[tuple[int, int], str]
) -> dict[tuple[int, int], str]:
    result: dict[tuple[int, int], str]
    while True:
        (changed, after_step) = tilt_south_step(max_x, max_y, grid)
        result = after_step
        if not changed:
            break

    return result


def tilt_east_step(
    max_x: int, max_y: int, grid: dict[tuple[int, int], str]
) -> tuple[bool, dict[tuple[int, int], str]]:
    changed = False
    for y in range(0, max_y):
        for x in range(max_x - 1, -1, -1):
            if grid[(x, y)] == "O":
                east_spot = (x + 1, y)
                if east_spot in grid and grid[east_spot] == ".":
                    changed = True
                    grid[east_spot] = "O"
                    grid[(x, y)] = "."

    return (changed, grid)


def tilt_east(
    max_x: int, max_y: int, grid: dict[tuple[int, int], str]
) -> dict[tuple[int, int], str]:
    result: dict[tuple[int, int], str]
    while True:
        (changed, after_step) = tilt_east_step(max_x, max_y, grid)
        result = after_step
        if not changed:
            break

    return result


def tilt_west_step(
    max_x: int, max_y: int, grid: dict[tuple[int, int], str]
) -> tuple[bool, dict[tuple[int, int], str]]:
    changed = False
    for y in range(0, max_y):
        for x in range(0, max_x):
            if grid[(x, y)] == "O":
                west_spot = (x - 1, y)
                if west_spot in grid and grid[west_spot] == ".":
                    changed = True
                    grid[west_spot] = "O"
                    grid[(x, y)] = "."

    return (changed, grid)


def tilt_west(
    max_x: int, max_y: int, grid: dict[tuple[int, int], str]
) -> dict[tuple[int, int], str]:
    result: dict[tuple[int, int], str]
    while True:
        (changed, after_step) = tilt_west_step(max_x, max_y, grid)
        result = after_step
        if not changed:
            break

    return result


def cycle(
    max_x: int, max_y: int, grid: dict[tuple[int, int], str]
) -> dict[tuple[int, int], str]:
    after_north = tilt_north(max_x, max_y, grid)
    after_west = tilt_west(max_x, max_y, after_north)
    after_south = tilt_south(max_x, max_y, after_west)
    after_east = tilt_east(max_x, max_y, after_south)

    return after_east


def part_1() -> None:
    (max_x, max_y, grid) = read_file("../data/2023/14.txt")
    result = tilt_north(max_x, max_y, grid)

    score = load_on_north_beam(max_x, max_y, result)
    print(score)


def in_cycle(
    current_i: int, scores: dict[int, int], reverse_scores: dict[int, int]
) -> tuple[bool, int]:
    if current_i < 120:
        return (False, 0)

    finds: Set[int] = set()
    for i in range(current_i - 20, current_i + 1):
        score = scores[i]
        first_find = reverse_scores[score]
        finds.add(first_find)

    return (len(finds) < 20, len(finds))


def part_2() -> None:
    (max_x, max_y, grid) = read_file("../data/2023/14.txt")

    result = grid
    scores = {}
    reverse_scores: dict[int, int] = {}
    cycle_length = 0
    cycle_start = 0
    for i in range(0, 1000000000):
        result = cycle(max_x, max_y, result)
        round_score = load_on_north_beam(max_x, max_y, result)

        if not round_score in reverse_scores:
            reverse_scores[round_score] = i

        scores[i] = round_score

        (is_in_cycle, found_cycle_length) = in_cycle(i, scores, reverse_scores)
        if is_in_cycle:
            print(f"in a cycle as of {i} of length {found_cycle_length}")
            cycle_length = found_cycle_length
            cycle_start = i
            break

    offset = (1000000000 - cycle_start - 1) % cycle_length
    for i in range(0, offset):
        result = cycle(max_x, max_y, result)

    score = load_on_north_beam(max_x, max_y, result)
    print(score)


part_1()
part_2()
