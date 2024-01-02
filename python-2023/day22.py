from __future__ import annotations
from typing import Set, Optional, Callable
import math
from enum import Enum
from itertools import groupby, takewhile
from functools import cmp_to_key
from heapq import heapify, heappush, heappop
import random


class Brick:
    def __init__(
        self, index: int, a: tuple[int, int, int], b: tuple[int, int, int]
    ) -> None:
        self.index = index
        # self.name = "ABCDEFG"[index]
        self.a = a
        self.b = b
        self.min_x = min(self.a[0], self.b[0])
        self.max_x = max(self.a[0], self.b[0])
        self.min_y = min(self.a[1], self.b[1])
        self.max_y = max(self.a[1], self.b[1])
        self.min_z = min(self.a[2], self.b[2])
        self.max_z = max(self.a[2], self.b[2])

    def __lt__(self, other: Brick) -> bool:
        return self.min_z < other.min_z

    def drop(self, amount: int) -> None:
        self.a = (self.a[0], self.a[1], self.a[2] - amount)
        self.b = (self.b[0], self.b[1], self.b[2] - amount)
        self.min_z = min(self.a[2], self.b[2])
        self.max_z = max(self.a[2], self.b[2])

    def contains_space(self, space: tuple[int, int, int]) -> bool:
        return (
            (self.min_x <= space[0] <= self.max_x)
            and (self.min_y <= space[1] <= self.max_y)
            and (self.min_z <= space[2] <= self.max_z)
        )

    def contains_spaces(self, spaces: list[tuple[int, int, int]]) -> bool:
        return any(self.contains_space(space) for space in spaces)

    def spaces(self) -> list[tuple[int, int, int]]:
        spaces = []
        for x in range(self.min_x, self.max_x + 1):
            for y in range(self.min_y, self.max_y + 1):
                for z in range(self.min_z, self.max_z + 1):
                    spaces.append((x, y, z))

        return spaces

    def top_spaces(self) -> list[tuple[int, int, int]]:
        spaces = []
        for x in range(self.min_x, self.max_x + 1):
            for y in range(self.min_y, self.max_y + 1):
                spaces.append((x, y, self.max_z))

        return spaces

    def spaces_below(self) -> list[tuple[int, int, int]]:
        spaces = []
        for x in range(self.min_x, self.max_x + 1):
            for y in range(self.min_y, self.max_y + 1):
                spaces.append((x, y, self.min_z - 1))

        return spaces

    def spaces_above(self) -> list[tuple[int, int, int]]:
        spaces = []
        for x in range(self.min_x, self.max_x + 1):
            for y in range(self.min_y, self.max_y + 1):
                spaces.append((x, y, self.max_z + 1))

        return spaces

    def height(self) -> int:
        return self.min_z


def read_file(
    file_name: str,
) -> dict[int, Brick]:
    data_file = open(file_name, "r")
    lines = data_file.readlines()
    data_file.close()

    bricks: dict[int, Brick] = {}
    for index, line in enumerate(lines):
        [start, end] = line.strip().split("~")
        start_parts = start.split(",")
        end_parts = end.split(",")
        bricks[index] = Brick(
            index,
            (int(start_parts[0]), int(start_parts[1]), int(start_parts[2])),
            (int(end_parts[0]), int(end_parts[1]), int(end_parts[2])),
        )

    return bricks


def brick_indexes_by_height(bricks: dict[int, Brick]) -> list[tuple[int, int]]:
    return sorted([(x.min_z, x_index) for x_index, x in bricks.items()])


def fall_amount(brick: Brick, terrain: dict[tuple[int, int], int]) -> int:
    spaces_below = brick.spaces_below()
    distances = []
    for space in spaces_below:
        x_y = (space[0], space[1])
        ground_height = terrain[x_y] if x_y in terrain else 0
        distances.append(space[2] - ground_height)

    return min(distances)


def dependents(brick: Brick, bricks: dict[int, Brick]) -> list[Brick]:
    spaces_above = brick.spaces_above()
    return [x for x in bricks.values() if x.contains_spaces(spaces_above)]


def count_supports(brick: Brick, bricks: dict[int, Brick]) -> int:
    spaces_below = brick.spaces_below()
    return sum(1 for x in bricks.values() if x.contains_spaces(spaces_below))


def part_1() -> None:
    bricks = read_file("../data/2023/22.txt")

    bricks_low_to_high = brick_indexes_by_height(bricks)
    terrain: dict[tuple[int, int], int] = {}
    for _, brick_index in bricks_low_to_high:
        brick = bricks[brick_index]
        amount = fall_amount(brick, terrain)
        brick.drop(amount)
        for space in brick.top_spaces():
            x_y = (space[0], space[1])
            terrain[x_y] = space[2]

    removables = set()
    for brick in bricks.values():
        brick_dependents = dependents(brick, bricks)
        if len(brick_dependents) == 0:
            removables.add(brick.index)
            # print(brick.name)

        is_removable = all(
            count_supports(dependent_brick, bricks) > 1
            for dependent_brick in brick_dependents
        )
        if is_removable:
            removables.add(brick.index)
            # print(brick.name)

    score = len(removables)
    print(score)


def bricks_that_would_fall_if_removed(
    brick: Brick, bricks: dict[int, Brick]
) -> Set[int]:
    brick_dependents = dependents(brick, bricks)
    if len(brick_dependents) == 0:
        return set()

    removed_bricks = set()
    bricks_without_removed = bricks.copy()
    bricks_without_removed.pop(brick.index)

    bricks_to_evaluate = brick_dependents
    heapify(bricks_to_evaluate)

    while len(bricks_to_evaluate) > 0:
        dependent_brick = heappop(bricks_to_evaluate)
        if dependent_brick.index in removed_bricks:
            continue
        if count_supports(dependent_brick, bricks_without_removed) == 0:
            # print(f"{dependent_brick.name} would fall if {brick.name} fell")
            removed_bricks.add(dependent_brick.index)
            bricks_without_removed.pop(dependent_brick.index)
            for x in dependents(dependent_brick, bricks_without_removed):
                heappush(bricks_to_evaluate, x)

    return removed_bricks


def part_2() -> None:
    bricks = read_file("../data/2023/22.txt")

    bricks_low_to_high = brick_indexes_by_height(bricks)
    terrain: dict[tuple[int, int], int] = {}
    for _, brick_index in bricks_low_to_high:
        brick = bricks[brick_index]
        amount = fall_amount(brick, terrain)
        brick.drop(amount)
        for space in brick.top_spaces():
            x_y = (space[0], space[1])
            terrain[x_y] = space[2]

    bricks_that_would_fall = 0
    for brick in bricks.values():
        would_fall: Set[int] = bricks_that_would_fall_if_removed(brick, bricks)
        bricks_that_would_fall += len(would_fall)

    score = bricks_that_would_fall
    print(score)


part_1()
part_2()
