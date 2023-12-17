from __future__ import annotations
from typing import Set, Optional
import math
from enum import Enum
from itertools import groupby, takewhile
from functools import cmp_to_key
from heapq import heapify, heappush, heappop


def read_file(file_name: str) -> tuple[int, int, dict[tuple[int, int], int]]:
    data_file = open(file_name, "r")
    lines = data_file.readlines()
    data_file.close()

    grid = {}
    for y, line in enumerate(lines):
        for x, char in enumerate(list(line.strip())):
            grid[(x, y)] = int(char)

    return (len(lines[0].strip()), len(lines), grid)


def next_position_options(
    path: list[tuple[tuple[int, int], str]], grid: dict[tuple[int, int], int]
) -> list[tuple[tuple[int, int], str, int]]:
    ((current_x, current_y), _) = path[-1]

    last_paths = list(reversed(path[-4:]))

    all_adjacent = [
        (
            (current_x + 1, current_y),
            ">",
            len(list(takewhile(lambda segment: segment[1] == ">", last_paths))),
        ),
        (
            (current_x - 1, current_y),
            "<",
            len(list(takewhile(lambda segment: segment[1] == "<", last_paths))),
        ),
        (
            (current_x, current_y - 1),
            "^",
            len(list(takewhile(lambda segment: segment[1] == "^", last_paths))),
        ),
        (
            (current_x, current_y + 1),
            "v",
            len(list(takewhile(lambda segment: segment[1] == "v", last_paths))),
        ),
    ]
    valid_positions = [
        (location, direction, direction_count)
        for (location, direction, direction_count) in all_adjacent
        if not any(location == segment[0] for segment in path)
        and location in grid
        and (direction_count < 3)
    ]
    return valid_positions


def part_1() -> None:
    (max_x, max_y, grid) = read_file("../data/2023/17.txt")

    starting_space = (0, 0)
    ending_space = (max_x - 1, max_y - 1)

    grid_search: list[tuple[int, list[tuple[tuple[int, int], str]]]] = [
        (0, [(starting_space, ".")])
    ]
    heapify(grid_search)

    best_to_spot: dict[tuple[tuple[int, int], str, int], int] = {}
    best_to_spot[(starting_space, ".", 0)] = 0

    score = 0

    while len(grid_search):
        (heat_cost, path) = heappop(grid_search)
        (current_location, current_direction) = path[-1]
        if current_location == ending_space:
            score = heat_cost
            # print(path)
            break

        options = next_position_options(path, grid)
        for option, direction, count_in_direction in options:
            heat_for_option = heat_cost + grid[option]

            if (
                option,
                direction,
                count_in_direction,
            ) in best_to_spot and heat_for_option >= best_to_spot[
                (
                    option,
                    direction,
                    count_in_direction,
                )
            ]:
                continue

            best_to_spot[
                (
                    option,
                    direction,
                    count_in_direction,
                )
            ] = heat_for_option

            heappush(
                grid_search,
                (
                    heat_for_option,
                    path + [(option, direction)],
                ),
            )

    print(score)
    # print(best_to_spot[ending_space])


def next_position_options_2(
    path: list[tuple[tuple[int, int], str]], grid: dict[tuple[int, int], int]
) -> list[tuple[tuple[int, int], str, int]]:
    ((current_x, current_y), current_direction) = path[-1]

    last_paths = list(reversed(path))

    all_adjacent = [
        (
            (current_x + 1, current_y),
            ">",
            1 + len(list(takewhile(lambda segment: segment[1] == ">", last_paths))),
        ),
        (
            (current_x - 1, current_y),
            "<",
            1 + len(list(takewhile(lambda segment: segment[1] == "<", last_paths))),
        ),
        (
            (current_x, current_y - 1),
            "^",
            1 + len(list(takewhile(lambda segment: segment[1] == "^", last_paths))),
        ),
        (
            (current_x, current_y + 1),
            "v",
            1 + len(list(takewhile(lambda segment: segment[1] == "v", last_paths))),
        ),
    ]

    count_in_current_direction = len(
        list(takewhile(lambda segment: segment[1] == current_direction, last_paths))
    )

    valid_positions = [
        (location, direction, direction_count)
        for (location, direction, direction_count) in all_adjacent
        if not any(location == segment[0] for segment in path)
        and location in grid
        and (
            (
                len(path) < 3
                and (direction == current_direction or current_direction == ".")
            )
            or (direction == current_direction and direction_count <= 10)
            or (count_in_current_direction >= 4 and direction != current_direction)
        )
    ]
    return valid_positions


def part_2() -> None:
    (max_x, max_y, grid) = read_file("../data/2023/17.txt")

    starting_space = (0, 0)
    ending_space = (max_x - 1, max_y - 1)

    grid_search: list[tuple[int, list[tuple[tuple[int, int], str]]]] = [
        (0, [(starting_space, ".")])
    ]
    heapify(grid_search)

    best_to_spot: dict[tuple[tuple[int, int], str, int], int] = {}
    best_to_spot[(starting_space, ".", 0)] = 0

    score = 0

    while len(grid_search):
        (heat_cost, path) = heappop(grid_search)
        (current_location, current_direction) = path[-1]

        count_in_current_direction = len(
            list(
                takewhile(
                    lambda segment: segment[1] == current_direction, reversed(path)
                )
            )
        )
        if current_location == ending_space:
            if count_in_current_direction >= 4:
                score = heat_cost
                # print(path)
                break
            else:
                continue

        options = next_position_options_2(path, grid)
        for option, direction, count_in_direction in options:
            heat_for_option = heat_cost + grid[option]

            if (
                option,
                direction,
                count_in_direction,
            ) in best_to_spot and heat_for_option >= best_to_spot[
                (
                    option,
                    direction,
                    count_in_direction,
                )
            ]:
                continue

            best_to_spot[
                (
                    option,
                    direction,
                    count_in_direction,
                )
            ] = heat_for_option

            heappush(
                grid_search,
                (
                    heat_for_option,
                    path + [(option, direction)],
                ),
            )

    print(score)


part_1()
part_2()
