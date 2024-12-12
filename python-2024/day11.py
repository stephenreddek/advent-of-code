from collections import defaultdict
import copy
from dataclasses import dataclass
from enum import Enum
import itertools


@dataclass
class Input:
    values: list[int]


def read_input() -> Input:
    data_file = open("../data/2024/11.txt", "r")
    lines = data_file.readlines()
    data_file.close()

    result = Input(values=[int(val) for val in lines[0].strip().split(" ")])

    return result


def apply_rules(stones: dict[int, int]) -> dict[int, int]:
    new_stones = defaultdict(int)

    for stone, count in stones.items():
        if stone == 0:
            new_stones[1] += count
            continue

        str_version = str(stone)
        if len(str_version) % 2 == 0:
            # even length
            left, right = (
                str_version[: len(str_version) // 2],
                str_version[len(str_version) // 2 :],
            )
            new_stones[int(left)] += count
            new_stones[int(right)] += count
            continue

        new_stones[stone * 2024] += count

    return new_stones


def part_1() -> None:
    input_data = read_input()

    stones: dict[int, int] = defaultdict(int)
    for stone in input_data.values:
        stones[stone] += 1

    result = stones
    for _ in range(25):
        result = apply_rules(result)

    count_stones = sum(result.values())
    print(count_stones)


def part_2() -> None:
    input_data = read_input()

    stones: dict[int, int] = defaultdict(int)
    for stone in input_data.values:
        stones[stone] += 1

    result = stones
    for _ in range(75):
        result = apply_rules(result)

    count_stones = sum(result.values())
    print(count_stones)


part_1()
part_2()
