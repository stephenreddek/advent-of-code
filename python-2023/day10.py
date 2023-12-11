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

    grid: dict[tuple[int, int], str] = {}
    for y, line in enumerate(lines):
        for x, map_char in enumerate(list(line.strip())):
            grid[(x, y)] = map_char

    return (len(lines[0]), len(lines), grid)


def find_connections_of_start(
    spot: tuple[int, int], grid: dict[tuple[int, int], str]
) -> list[tuple[int, int]]:
    (x, y) = spot
    left = (x - 1, y)
    right = (x + 1, y)
    top = (x, y - 1)
    bottom = (x, y + 1)

    connects_on_left = ["F", "-", "L"]
    connects_on_right = ["7", "-", "J"]
    connects_on_top = ["F", "|", "t7"]
    connects_on_bottom = ["L", "|", "J"]

    results = []

    results.extend([left] if left in grid and grid[left] in connects_on_left else [])
    results.extend(
        [right] if right in grid and grid[right] in connects_on_right else []
    )
    results.extend([top] if top in grid and grid[top] in connects_on_top else [])
    results.extend(
        [bottom] if bottom in grid and grid[bottom] in connects_on_bottom else []
    )

    return results


def find_connections(
    spot: tuple[int, int], grid: dict[tuple[int, int], str]
) -> list[tuple[int, int]]:
    character = grid[spot]
    (x, y) = spot
    if character == "S":
        return find_connections_of_start(spot, grid)
    elif character == "F":
        return [(x, y + 1), (x + 1, y)]
    elif character == "7":
        return [(x - 1, y), (x, y + 1)]
    elif character == "J":
        return [(x - 1, y), (x, y - 1)]
    elif character == "L":
        return [(x, y - 1), (x + 1, y)]
    elif character == "-":
        return [(x - 1, y), (x + 1, y)]
    elif character == "|":
        return [(x, y - 1), (x, y + 1)]
    else:
        return []


def part_1() -> None:
    (len_x, len_y, grid) = read_file("../data/2023/10.txt")

    distances: dict[tuple[int, int], int] = {}

    starting_spot: tuple[int, int] = next(iter({k for k in grid if grid[k] == "S"}))

    distances[starting_spot] = 0
    to_evaluate: list[tuple[int, int]] = [starting_spot]
    while len(to_evaluate) > 0:
        spot = to_evaluate.pop(0)
        spot_distance = distances[spot]
        connections = find_connections(spot, grid)
        for connection in connections:
            if connection in distances.keys():
                continue

            distances[connection] = spot_distance + 1
            to_evaluate.append(connection)

    max_distance = max(distances.values())
    print(max_distance)


def count_lines_to_top(
    spot: tuple[int, int], grid: dict[tuple[int, int], str], loop: set[tuple[int, int]]
) -> int:
    intersections = 0
    line_start = None

    for y in range(spot[1], -1, -1):
        if (spot[0], y) in loop:
            char = grid[(spot[0], y)]
            if char == "-":
                intersections += 1
            elif char in ["J", "L"]:
                line_start = char
            elif char in ["F", "7"]:
                if (char == "F" and line_start == "J") or (
                    char == "7" and line_start == "L"
                ):
                    intersections += 1

                line_start = None
            else:  # |
                continue

    return intersections


def print_grid(
    len_x: int,
    len_y: int,
    grid: dict[tuple[int, int], str],
    loop: set[tuple[int, int]],
    inside: set[tuple[int, int]],
) -> None:
    for y in range(0, len_y):
        for x in range(0, len_x):
            if (x, y) in inside:
                print("I", end="")
            elif (x, y) in loop:
                print(grid[(x, y)], end="")
            else:
                print("O", end="")

        print()


def find_starting(spot: tuple[int, int], grid: dict[tuple[int, int], str]) -> str:
    (x, y) = spot
    left = (x - 1, y)
    right = (x + 1, y)
    top = (x, y - 1)
    bottom = (x, y + 1)

    connects_on_left = left in grid and grid[left] in ["F", "-", "L"]
    connects_on_right = right in grid and grid[right] in ["7", "-", "J"]
    connects_on_top = top in grid and grid[top] in ["F", "|", "t7"]
    connects_on_bottom = bottom in grid and grid[bottom] in ["L", "|", "J"]

    if connects_on_bottom and connects_on_top:
        return "|"
    if connects_on_bottom and connects_on_left:
        return "7"
    if connects_on_bottom and connects_on_right:
        return "F"
    if connects_on_top and connects_on_left:
        return "J"
    if connects_on_top and connects_on_right:
        return "L"
    if connects_on_left and connects_on_right:
        return "-"
    else:
        raise Exception(f"Unable to find starting icon")


def part_2() -> None:
    (len_x, len_y, grid) = read_file("../data/2023/10.txt")

    starting_spot: tuple[int, int] = next(iter({k for k in grid if grid[k] == "S"}))

    loop_coordinates = set({starting_spot})
    to_evaluate: list[tuple[int, int]] = [starting_spot]
    while len(to_evaluate) > 0:
        spot = to_evaluate.pop(0)
        connections = find_connections(spot, grid)
        for connection in connections:
            if connection in loop_coordinates:
                continue

            loop_coordinates.add(connection)
            to_evaluate.append(connection)

    starting_replacement = find_starting(starting_spot, grid)
    grid[starting_spot] = starting_replacement

    inside = set()
    for coordinate in grid.keys():
        if coordinate in loop_coordinates:
            continue

        # in order to be in the loop, the count of crosses of the line to a border must be odd
        if count_lines_to_top(coordinate, grid, loop_coordinates) % 2 != 0:
            inside.add(coordinate)

    print(len(inside))
    # print_grid(len_x, len_y, grid, loop_coordinates, inside)


part_1()
part_2()
