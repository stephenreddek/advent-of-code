from collections import defaultdict
import copy
from dataclasses import dataclass
from enum import Enum
import itertools
import re
import heapq


@dataclass
class Machine:
    a_x: int
    a_y: int
    b_x: int
    b_y: int
    prize: tuple[int, int]


@dataclass
class Input:
    machines: list[Machine]


def read_input() -> Input:
    data_file = open("../data/2024/13.txt", "r")
    lines = data_file.readlines()
    data_file.close()

    result = Input(machines=[])
    for line_index in range(0, len(lines), 4):
        a_button_line = lines[line_index]
        b_button_line = lines[line_index + 1]
        prize_line = lines[line_index + 2]
        # then empty line

        a_button_match = re.match(r"Button A: X\+(\d+), Y\+(\d+)", a_button_line)
        b_button_match = re.match(r"Button B: X\+(\d+), Y\+(\d+)", b_button_line)
        prize_match = re.match(r"Prize: X=(\d+), Y=(\d+)", prize_line)

        if not a_button_match or not b_button_match or not prize_match:
            raise Exception("Invalid input data")

        result.machines.append(
            Machine(
                a_x=int(a_button_match.group(1)),
                a_y=int(a_button_match.group(2)),
                b_x=int(b_button_match.group(1)),
                b_y=int(b_button_match.group(2)),
                prize=(int(prize_match.group(1)), int(prize_match.group(2))),
            )
        )

    return result


A_BUTTON_COST = 3
B_BUTTON_COST = 1


def part_1() -> None:
    input_data = read_input()

    cost = 0
    for machine in input_data.machines:
        # a_mul * a_x + b_mul * b_x = prize_x
        # a_mul * a_y + b_mul * b_y = prize_y

        # a_mul = (prize_x - b_mul * b_x) / a_x
        # a_mul = (prize_y - b_mul * b_y) / a_y
        # ======
        # b_mul = (prize_x * a_y - prize_y * a_x) / (b_x * a_y - b_y * a_x)
        # a_mul = (prize_y - b_mul * b_y) / a_y

        b_mul = (machine.prize[0] * machine.a_y - machine.prize[1] * machine.a_x) / (
            machine.b_x * machine.a_y - machine.b_y * machine.a_x
        )
        a_mul = (machine.prize[1] - b_mul * machine.b_y) / machine.a_y

        if a_mul == int(a_mul) and b_mul == int(b_mul):
            cost += int(a_mul) * A_BUTTON_COST + int(b_mul) * B_BUTTON_COST

    print(cost)


def part_2() -> None:
    input_data = read_input()

    cost = 0

    difficulty_boost = 10000000000000
    for machine in input_data.machines:
        prize_x = machine.prize[0] + difficulty_boost
        prize_y = machine.prize[1] + difficulty_boost
        b_mul = (prize_x * machine.a_y - prize_y * machine.a_x) / (
            machine.b_x * machine.a_y - machine.b_y * machine.a_x
        )
        a_mul = (prize_y - b_mul * machine.b_y) / machine.a_y

        if a_mul == int(a_mul) and b_mul == int(b_mul):
            # print(f"Found: {a_mul=}, {b_mul=}")
            cost += int(a_mul) * A_BUTTON_COST + int(b_mul) * B_BUTTON_COST
        # else:
        # print(f"Failed: {a_mul=}, {b_mul=}")

    print(cost)


part_1()
part_2()
