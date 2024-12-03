from collections import defaultdict
from enum import Enum
import re
from typing import Any


def part_1() -> None:
    data_file = open("../data/2024/03.txt", "r")
    data = data_file.read()
    data_file.close()

    multiply_expressions: list[Any] = re.findall("mul\((\d{1,3}),(\d{1,3})\)", data)
    value = 0
    for a_raw, b_raw in multiply_expressions:
        a = int(a_raw)
        b = int(b_raw)
        value += a * b

    print(value)


def part_2() -> None:
    data_file = open("../data/2024/03.txt", "r")
    data = data_file.read()
    data_file.close()

    multiply_expressions: list[re.Match[str]] = re.finditer(
        "(mul\((\d{1,3}),(\d{1,3})\))|(don't\(\))|(do\(\))", data
    )
    value = 0
    enabled = True
    for match in multiply_expressions:
        groups = match.groups()
        if groups[0]:
            # mul
            if enabled:
                a = int(groups[1])
                b = int(groups[2])
                value += a * b
        elif groups[3]:
            # don't
            enabled = False

        elif groups[4]:
            # do
            enabled = True

    print(value)


part_1()
part_2()
