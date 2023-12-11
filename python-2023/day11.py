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

    columns_without_galaxies = []
    for x in range(0, len(lines[0].strip())):
        contains_galaxy = False
        for line in lines:
            if line[x] == "#":
                contains_galaxy = True
                break

        if not contains_galaxy:
            columns_without_galaxies.append(x)

    expanded_lines = []
    for line in lines:
        expanded_line = expand_line(line, columns_without_galaxies)
        if "#" not in line:
            expanded_lines.append(expanded_line)
        expanded_lines.append(expanded_line)

    grid: dict[tuple[int, int], str] = {}
    for y, line in enumerate(expanded_lines):
        for x, map_char in enumerate(list(line.strip())):
            grid[(x, y)] = map_char

    return (len(expanded_lines[0].strip()), len(expanded_lines), grid)


def expand_line(line: str, columns_to_double: list[int]) -> str:
    result = []
    for x, char in enumerate(list(line)):
        result.append(char)
        if x in columns_to_double:
            result.append(char)

    return "".join(result)


def manhattan_distance(a: tuple[int, int], b: tuple[int, int]) -> int:
    return abs(b[0] - a[0]) + abs(b[1] - a[1])


def part_1() -> None:
    (len_x, len_y, grid) = read_file("../data/2023/11.txt")

    galaxies = []
    for y in range(0, len_y):
        for x in range(0, len_x):
            if grid[(x, y)] == "#":
                galaxies.append((x, y))

    pairs = []
    for i in range(0, len(galaxies) - 1):
        for j in range(i + 1, len(galaxies)):
            pairs.append((galaxies[i], galaxies[j]))

    result = sum(manhattan_distance(a, b) for (a, b) in pairs)

    print(result)


def read_file_2(
    file_name: str,
) -> tuple[int, int, list[int], list[int], dict[tuple[int, int], str]]:
    data_file = open(file_name, "r")
    lines = data_file.readlines()
    data_file.close()

    columns_without_galaxies = []
    for x in range(0, len(lines[0].strip())):
        contains_galaxy = False
        for line in lines:
            if line[x] == "#":
                contains_galaxy = True
                break

        if not contains_galaxy:
            columns_without_galaxies.append(x)

    lines_without_galaxies = []
    grid: dict[tuple[int, int], str] = {}
    for y, line in enumerate(lines):
        if "#" not in line:
            lines_without_galaxies.append(y)

        for x, map_char in enumerate(list(line.strip())):
            grid[(x, y)] = map_char

    return (
        len(lines[0].strip()),
        len(lines),
        columns_without_galaxies,
        lines_without_galaxies,
        grid,
    )


def modified_manhattan_distance(
    a: tuple[int, int],
    b: tuple[int, int],
    expanded_columns: list[int],
    expanded_rows: list[int],
) -> int:
    multiplier = 1000000

    crosses_expanded_columns = 0
    for x in range(min(a[0], b[0]) + 1, max(a[0], b[0])):
        if x in expanded_columns:
            crosses_expanded_columns += 1

    crosses_expanded_rows = 0
    for y in range(min(a[1], b[1]) + 1, max(a[1], b[1])):
        if y in expanded_rows:
            crosses_expanded_rows += 1

    return (
        abs(b[0] - a[0])
        + abs(b[1] - a[1])
        + ((crosses_expanded_columns + crosses_expanded_rows) * (multiplier - 1))
    )


def part_2() -> None:
    (len_x, len_y, expanded_columns, expanded_rows, grid) = read_file_2(
        "../data/2023/11.txt"
    )

    galaxies = []
    for y in range(0, len_y):
        for x in range(0, len_x):
            if grid[(x, y)] == "#":
                galaxies.append((x, y))

    pairs = []
    for i in range(0, len(galaxies) - 1):
        for j in range(i + 1, len(galaxies)):
            pairs.append((galaxies[i], galaxies[j]))

    result = sum(
        modified_manhattan_distance(a, b, expanded_columns, expanded_rows)
        for (a, b) in pairs
    )

    print(result)


part_1()
part_2()
