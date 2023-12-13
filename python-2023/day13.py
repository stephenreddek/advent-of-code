from __future__ import annotations
from typing import Set, Optional
import math
from enum import Enum
from itertools import groupby
from functools import cmp_to_key


def read_file(file_name: str) -> list[tuple[int, int, dict[tuple[int, int], str]]]:
    data_file = open(file_name, "r")
    lines = data_file.readlines()
    data_file.close()

    all_map_lines: list[list[str]] = []
    current_map_lines: list[str] = []
    for line in lines:
        if line == "\n":
            all_map_lines.append(current_map_lines)
            current_map_lines = []
        else:
            current_map_lines.append(line.strip())
    all_map_lines.append(current_map_lines)

    maps: list[tuple[int, int, dict[tuple[int, int], str]]] = []
    for map_lines in all_map_lines:
        current_map = {}
        for y, line in enumerate(map_lines):
            for x, char in enumerate(list(line)):
                current_map[(x, y)] = char

        maps.append((len(map_lines[0]), len(map_lines), current_map))

    return maps


def differences_in_horizontal_reflection(
    y: int, max_x: int, max_y: int, current_map: dict[tuple[int, int], str]
) -> int:
    lines_before = range(y - 1, -1, -1)
    lines_after = range(y, max_y)
    comparison_lines = zip(lines_before, lines_after)
    differences = 0
    for a, b in comparison_lines:
        for x in range(0, max_x):
            if current_map[(x, a)] != current_map[(x, b)]:
                differences += 1

    return differences


def find_horizontal_mirror(
    max_x: int, max_y: int, current_map: dict[tuple[int, int], str], smudge_factor: int
) -> Optional[int]:
    for y in range(1, max_y):
        if (
            differences_in_horizontal_reflection(y, max_x, max_y, current_map)
            == smudge_factor
        ):
            return y

    return None


def differences_in_vertical_reflection(
    x: int, max_x: int, max_y: int, current_map: dict[tuple[int, int], str]
) -> int:
    columns_before = range(x - 1, -1, -1)
    columns_after = range(x, max_x)
    comparison_columns = zip(columns_before, columns_after)
    differences = 0
    for a, b in comparison_columns:
        for y in range(0, max_y):
            if current_map[(a, y)] != current_map[(b, y)]:
                differences += 1

    return differences


def find_vertical_mirror(
    max_x: int, max_y: int, current_map: dict[tuple[int, int], str], smudge_factor: int
) -> Optional[int]:
    for x in range(1, max_x):
        if (
            differences_in_vertical_reflection(x, max_x, max_y, current_map)
            == smudge_factor
        ):
            return x

    return None


def part_1() -> None:
    maps = read_file("../data/2023/13.txt")

    horizontal_mirrors: list[int] = []
    vertical_mirrors: list[int] = []
    for max_x, max_y, current_map in maps:
        horizontal_mirror = find_horizontal_mirror(max_x, max_y, current_map, 0)
        if horizontal_mirror is not None:
            horizontal_mirrors.append(horizontal_mirror)

        else:
            vertical_mirror = find_vertical_mirror(max_x, max_y, current_map, 0)
            if vertical_mirror is not None:
                vertical_mirrors.append(vertical_mirror)
            else:
                raise Exception(f"Unable to either vertical or horizontal mirror")

    score = sum(vertical_mirrors) + (sum(horizontal_mirrors) * 100)
    print(score)


def part_2() -> None:
    maps = read_file("../data/2023/13.txt")

    horizontal_mirrors: list[int] = []
    vertical_mirrors: list[int] = []
    for max_x, max_y, current_map in maps:
        horizontal_mirror = find_horizontal_mirror(max_x, max_y, current_map, 1)
        if horizontal_mirror is not None:
            horizontal_mirrors.append(horizontal_mirror)

        else:
            vertical_mirror = find_vertical_mirror(max_x, max_y, current_map, 1)
            if vertical_mirror is not None:
                vertical_mirrors.append(vertical_mirror)
            else:
                raise Exception(f"Unable to either vertical or horizontal mirror")

    score = sum(vertical_mirrors) + (sum(horizontal_mirrors) * 100)
    print(score)


part_1()
part_2()
