from __future__ import annotations
from typing import Set, Optional
import math
from enum import Enum
from itertools import groupby
from functools import cmp_to_key


def read_file(file_name: str) -> list[str]:
    data_file = open(file_name, "r")
    line = data_file.readline()
    data_file.close()

    return line.strip().split(",")


def hash_command(command: str) -> int:
    chars = list(command)
    hash_value = 0
    for char in chars:
        ascii_code = ord(char)
        hash_value += ascii_code
        hash_value *= 17
        hash_value = hash_value % 256

    return hash_value


def part_1() -> None:
    sequence = read_file("../data/2023/15.txt")
    score = sum(hash_command(x) for x in sequence)
    print(score)


def box_focus_power(box_number: int, lenses: list[tuple[str, int]]) -> int:
    if len(lenses) == 0:
        return 0

    return sum(
        (box_number + 1) * (lens_index + 1) * (lens[1])
        for lens_index, lens in enumerate(lenses)
    )


def part_2() -> None:
    sequence = read_file("../data/2023/15.txt")

    boxes: dict[int, list[tuple[str, int]]] = {}
    for box_number in range(0, 256):
        boxes[box_number] = []

    for command in sequence:
        if "-" in command:
            label = command.split("-")[0]
            box_number = hash_command(label)
            current_lenses = boxes[box_number]
            without_label = [x for x in current_lenses if x[0] != label]
            boxes[box_number] = without_label
        else:
            [label, lens_size] = command.split("=")
            box_number = hash_command(label)
            current_lenses = boxes[box_number]
            if any(x[0] == label for x in current_lenses):
                boxes[box_number] = [
                    (label, int(lens_size))
                    if existing_label == label
                    else (existing_label, existing_lens_size)
                    for (existing_label, existing_lens_size) in current_lenses
                ]
            else:
                current_lenses.append((label, int(lens_size)))

    score = sum(
        box_focus_power(box_number, lenses) for box_number, lenses in boxes.items()
    )
    print(score)


part_1()
part_2()
