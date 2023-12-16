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


def apply_energy(
    space: tuple[int, int], direction: str, grid: dict[tuple[int, int], str]
) -> list[tuple[tuple[int, int], str]]:
    next_space = space
    if direction == ">":
        next_space = (space[0] + 1, space[1])
    elif direction == "<":
        next_space = (space[0] - 1, space[1])
    elif direction == "^":
        next_space = (space[0], space[1] - 1)
    elif direction == "v":
        next_space = (space[0], space[1] + 1)
    else:
        raise Exception(f"Invalid energy direction [{direction}]")

    if next_space not in grid:
        return []

    at_space = grid[next_space]

    if at_space == ".":
        return [(next_space, direction)]
    elif at_space == "-":
        if direction == ">" or direction == "<":
            return [(next_space, direction)]
        else:
            return [(next_space, "<"), (next_space, ">")]
    elif at_space == "|":
        if direction == "v" or direction == "^":
            return [(next_space, direction)]
        else:
            return [(next_space, "v"), (next_space, "^")]
    elif at_space == "/":
        if direction == ">":
            return [(next_space, "^")]
        elif direction == "<":
            return [(next_space, "v")]
        elif direction == "^":
            return [(next_space, ">")]
        else:  # direction == "v"
            return [(next_space, "<")]
    elif at_space == "\\":
        if direction == ">":
            return [(next_space, "v")]
        elif direction == "<":
            return [(next_space, "^")]
        elif direction == "^":
            return [(next_space, "<")]
        else:  # direction == "v"
            return [(next_space, ">")]
    else:
        raise Exception(f"Invalid space [{at_space}]")


def energized_spaces_for_starting_tile(
    space: tuple[int, int], direction: str, grid: dict[tuple[int, int], str]
) -> int:
    energized_spaces: dict[tuple[int, int], list[str]] = {}
    energized_spaces[space] = [direction]

    has_updated = True
    while has_updated:
        last_energized = energized_spaces.copy()
        has_updated = False
        for space, energy_directions in last_energized.items():
            for direction in energy_directions:
                new_energy_spaces = apply_energy(space, direction, grid)
                for new_location, new_direction in new_energy_spaces:
                    if new_location not in energized_spaces:
                        has_updated = True
                        energized_spaces[new_location] = [new_direction]

                    elif new_direction not in energized_spaces[new_location]:
                        has_updated = True
                        energized_spaces[new_location].append(new_direction)

    energized_spaces.pop(space)
    return len(energized_spaces)


def part_1() -> None:
    (max_x, max_y, grid) = read_file("../data/2023/16.txt")

    score = energized_spaces_for_starting_tile((-1, 0), ">", grid)
    print(score)


def part_2() -> None:
    (max_x, max_y, grid) = read_file("../data/2023/16.txt")

    possible_starting_positions = []
    for x in range(0, max_x):
        possible_starting_positions.append(((x, -1), "v"))
        possible_starting_positions.append(((x, max_y), "^"))

    for y in range(0, max_y):
        possible_starting_positions.append(((-1, y), ">"))
        possible_starting_positions.append(((max_x, y), "<"))

    score = max(
        energized_spaces_for_starting_tile(space, direction, grid)
        for space, direction in possible_starting_positions
    )
    print(score)


part_1()
part_2()
