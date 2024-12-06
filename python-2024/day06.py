from collections import defaultdict
import copy
from dataclasses import dataclass
from enum import Enum


@dataclass
class Input:
    grid: dict[tuple[int, int], str]
    starting_position: tuple[int, int]


class Direction(Enum):
    UP = "^"
    DOWN = "v"
    LEFT = "<"
    RIGHT = ">"


def read_input() -> Input:
    data_file = open("../data/2024/06.txt", "r")
    lines = data_file.readlines()
    data_file.close()

    result = Input(grid={}, starting_position=(0, 0))
    for y, line in enumerate(lines):
        for x, char in enumerate(line.strip()):
            result.grid[(x, y)] = char
            if char == "^":
                result.starting_position = (x, y)

    return result


def spin_right(direction: Direction) -> Direction:
    if direction == Direction.UP:
        return Direction.RIGHT
    elif direction == Direction.RIGHT:
        return Direction.DOWN
    elif direction == Direction.DOWN:
        return Direction.LEFT
    elif direction == Direction.LEFT:
        return Direction.UP

    raise Exception(f"Unknown direction: {direction}")


def position_in_front(
    position: tuple[int, int], direction: Direction
) -> tuple[int, int]:
    x, y = position
    if direction == Direction.UP:
        return (x, y - 1)
    elif direction == Direction.DOWN:
        return (x, y + 1)
    elif direction == Direction.LEFT:
        return (x - 1, y)
    elif direction == Direction.RIGHT:
        return (x + 1, y)

    raise Exception(f"Unknown direction: {direction}")


def step(
    position: tuple[int, int], direction: Direction, grid: dict[tuple[int, int], str]
) -> tuple[tuple[int, int], Direction]:
    next_desired = position_in_front(position, direction)
    if grid.get(next_desired) == "#":
        return (position, spin_right(direction))

    return (next_desired, direction)


def part_1() -> None:
    input_data = read_input()

    visited_locations = set()
    visited_locations.add(input_data.starting_position)
    current_position = input_data.starting_position
    current_direction = Direction.UP
    while True:
        current_position, current_direction = step(
            current_position, current_direction, input_data.grid
        )
        if current_position in input_data.grid:
            visited_locations.add(current_position)
        else:
            break

    print(len(visited_locations))


def results_in_loop(starting_position, grid):
    current_position = starting_position
    current_direction = Direction.UP
    seen = set()
    seen.add((current_direction, current_direction))
    while True:
        current_position, current_direction = step(
            current_position, current_direction, grid
        )
        if (current_position, current_direction) in seen:
            return True

        if current_position in grid:
            seen.add((current_position, current_direction))
        else:
            return False


def part_2() -> None:
    input_data = read_input()

    possible_obstructions = [
        position
        for position in input_data.grid.keys()
        if input_data.grid[position] == "."
    ]
    count_where_loops = 0
    for location in possible_obstructions:
        test_grid = copy.copy(input_data.grid)
        test_grid[location] = "#"
        if results_in_loop(input_data.starting_position, test_grid):
            count_where_loops += 1

    print(count_where_loops)


part_1()
part_2()
