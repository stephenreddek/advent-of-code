from collections import defaultdict
from dataclasses import dataclass
from enum import Enum


class Direction(Enum):
    N = "N"
    NE = "NE"
    E = "E"
    SE = "SE"
    S = "S"
    SW = "SW"
    W = "W"
    NW = "NW"


def positions_to_check(x, y, direction: Direction) -> list[tuple[int, int]]:
    match direction:
        case Direction.N:
            return [(x, y + 1), (x, y + 2), (x, y + 3)]

        case Direction.NE:
            return [(x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)]

        case Direction.E:
            return [(x + 1, y), (x + 2, y), (x + 3, y)]

        case Direction.SE:
            return [(x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)]

        case Direction.S:
            return [(x, y - 1), (x, y - 2), (x, y - 3)]

        case Direction.SW:
            return [(x - 1, y - 1), (x - 2, y - 2), (x - 3, y - 3)]

        case Direction.W:
            return [(x - 1, y), (x - 2, y), (x - 3, y)]

        case Direction.NW:
            return [(x - 1, y + 1), (x - 2, y + 2), (x - 3, y + 3)]


def count_start_of_xmas(x: int, y: int, grid: dict[tuple[int, int], str]) -> int:
    if not grid[(x, y)] == "X":
        return 0

    count = 0
    for _, direction in enumerate(Direction):
        [m_pos, a_pos, s_pos] = positions_to_check(x, y, direction)
        if grid.get(m_pos) == "M" and grid.get(a_pos) == "A" and grid.get(s_pos) == "S":
            count += 1

    return count


def part_1() -> None:
    data_file = open("../data/2024/04.txt", "r")
    lines = data_file.readlines()
    data_file.close()

    height = len(lines)
    width = len(lines[0].strip())

    grid = {}
    for y, line in enumerate(lines):
        for x, char in enumerate(line.strip()):
            grid[(x, y)] = char

    count_xmas = 0
    for y in range(0, height):
        for x in range(0, width):
            count_xmas += count_start_of_xmas(x, y, grid)

    print(count_xmas)


@dataclass
class Possibility:
    m_position: tuple[int, int]
    s_position: tuple[int, int]


def get_possibilities(x: int, y: int) -> list[tuple[Possibility, Possibility]]:
    return [
        (
            Possibility(m_position=(x - 1, y - 1), s_position=(x + 1, y + 1)),
            Possibility(m_position=(x + 1, y + 1), s_position=(x - 1, y - 1)),
        ),
        (
            Possibility(m_position=(x - 1, y + 1), s_position=(x + 1, y - 1)),
            Possibility(m_position=(x + 1, y - 1), s_position=(x - 1, y + 1)),
        ),
    ]


def is_x_mas(x: int, y: int, grid: dict[tuple[int, int], str]) -> bool:
    if not grid[(x, y)] == "A":
        return 0

    trend_up_side_possibilities = (
        Possibility(m_position=(x - 1, y - 1), s_position=(x + 1, y + 1)),
        Possibility(m_position=(x + 1, y + 1), s_position=(x - 1, y - 1)),
    )
    trend_down_side_possibilities = (
        Possibility(m_position=(x - 1, y + 1), s_position=(x + 1, y - 1)),
        Possibility(m_position=(x + 1, y - 1), s_position=(x - 1, y + 1)),
    )

    return any(
        grid.get(possibility.m_position) == "M"
        and grid.get(possibility.s_position) == "S"
        for possibility in trend_up_side_possibilities
    ) and any(
        grid.get(possibility.m_position) == "M"
        and grid.get(possibility.s_position) == "S"
        for possibility in trend_down_side_possibilities
    )


def part_2() -> None:
    data_file = open("../data/2024/04.txt", "r")
    lines = data_file.readlines()
    data_file.close()

    height = len(lines)
    width = len(lines[0].strip())

    grid = {}
    for y, line in enumerate(lines):
        for x, char in enumerate(line.strip()):
            grid[(x, y)] = char

    count_xmas = 0
    for y in range(0, height):
        for x in range(0, width):
            if is_x_mas(x, y, grid):
                count_xmas += 1

    print(count_xmas)


part_1()
part_2()
