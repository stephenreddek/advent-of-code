from __future__ import annotations
from typing import Set, Optional, Callable
import math
from enum import Enum
from itertools import groupby, takewhile
from functools import cmp_to_key
from heapq import heapify, heappush, heappop


def read_file(
    file_name: str,
) -> tuple[int, int, tuple[int, int], dict[tuple[int, int], str]]:
    data_file = open(file_name, "r")
    lines = data_file.readlines()
    data_file.close()

    starting_spot = (0, 0)

    grid: dict[tuple[int, int], str] = {}
    for y, line in enumerate(lines):
        for x, char in enumerate(list(line.strip())):
            grid[(x, y)] = char
            if char == "S":
                starting_spot = (x, y)

    return (len(lines[0].strip()), len(lines), starting_spot, grid)


def translate_grid_spot(
    spot: tuple[int, int], max_x: int, max_y: int
) -> tuple[int, int]:
    return (spot[0] % max_x, spot[1] % max_y)


def possible_steps(
    spot: tuple[int, int], grid: dict[tuple[int, int], str], max_x: int, max_y: int
) -> list[tuple[int, int]]:
    all_spots = [
        (spot[0] + 1, spot[1]),
        (spot[0] - 1, spot[1]),
        (spot[0], spot[1] + 1),
        (spot[0], spot[1] - 1),
    ]

    return [x for x in all_spots if grid[translate_grid_spot(x, max_x, max_y)] != "#"]


def part_1() -> None:
    (max_x, max_y, starting_spot, grid) = read_file("../data/2023/21.txt")

    evens: Set[tuple[int, int]] = set()
    last_evens: Set[tuple[int, int]] = set()
    odds: Set[tuple[int, int]] = set()
    last_odds: Set[tuple[int, int]] = set()

    last_evens.add(starting_spot)

    max_steps = 64
    for step in range(1, max_steps + 1):
        is_even = step % 2 == 0

        if is_even:
            last_evens = set()
            last_steps = last_odds
        else:
            last_odds = set()
            last_steps = last_evens

        for spot in last_steps:
            for possible_spot in possible_steps(spot, grid, max_x, max_y):
                if (
                    possible_spot in grid
                    and possible_spot not in evens
                    and possible_spot not in odds
                ):
                    if is_even:
                        evens.add(possible_spot)
                        last_evens.add(possible_spot)
                    else:
                        odds.add(possible_spot)
                        last_odds.add(possible_spot)

    score = len(evens) if max_steps % 2 == 0 else len(odds)
    print(score)


def part_2() -> None:
    (max_x, max_y, starting_spot, grid) = read_file("../data/2023/21.txt")

    # for a given starting spot and a number of steps, how many are filled?
    # map_state_lookup: dict[tuple[tuple[int, int], int], tuple[int, int]]

    evens: Set[tuple[int, int]] = set()
    last_evens: Set[tuple[int, int]] = set()
    odds: Set[tuple[int, int]] = set()
    last_odds: Set[tuple[int, int]] = set()

    last_evens.add(starting_spot)

    max_steps = 26501365
    for step in range(1, max_steps + 1):
        is_even = step % 2 == 0

        if is_even:
            last_evens = set()
            last_steps = last_odds
        else:
            last_odds = set()
            last_steps = last_evens

        for spot in last_steps:
            for possible_spot in possible_steps(spot, grid, max_x, max_y):
                if possible_spot not in evens and possible_spot not in odds:
                    if is_even:
                        evens.add(possible_spot)
                        last_evens.add(possible_spot)
                    else:
                        odds.add(possible_spot)
                        last_odds.add(possible_spot)

    score = len(evens) if max_steps % 2 == 0 else len(odds)
    print(score)


part_1()
part_2()
