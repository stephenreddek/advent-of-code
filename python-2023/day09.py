from __future__ import annotations
from typing import Set, Optional
import math
from enum import Enum
from itertools import groupby
from functools import cmp_to_key


def read_file(file_name: str) -> list[list[int]]:
    data_file = open(file_name, "r")
    lines = data_file.readlines()
    data_file.close()

    return [[int(x) for x in line.strip().split(" ")] for line in lines]


def extrapolate_next(numbers: list[int]) -> int:
    if all(x == 0 for x in numbers):
        return 0

    differences = [x - numbers[index] for index, x in enumerate(numbers[1:])]

    return numbers[-1] + extrapolate_next(differences)


def extrapolate_prev(numbers: list[int]) -> int:
    if all(x == 0 for x in numbers):
        return 0

    differences = [x - numbers[index] for index, x in enumerate(numbers[1:])]

    return numbers[0] - extrapolate_prev(differences)


def part_1() -> None:
    lists = read_file("../data/2023/09.txt")

    result = sum([extrapolate_next(line) for line in lists])
    print(result)


def part_2() -> None:
    lists = read_file("../data/2023/09.txt")

    result = sum([extrapolate_prev(line) for line in lists])
    print(result)


part_1()
part_2()
