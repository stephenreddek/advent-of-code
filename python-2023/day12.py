from __future__ import annotations
from typing import Set, Optional
import math
from enum import Enum
from itertools import groupby
from functools import cmp_to_key


def read_line(line: str) -> tuple[str, list[int]]:
    [record, values] = line.strip().split(" ")
    return (record, [int(x) for x in values.split(",")])


def read_file(file_name: str) -> list[tuple[str, list[int]]]:
    data_file = open(file_name, "r")
    lines = data_file.readlines()
    data_file.close()

    return [read_line(line) for line in lines]


def get_sizes(record: str) -> list[int]:
    return [len(x) for x in record.split(".") if x != ""]


def fits_sizes(record: str, sizes: list[int]) -> int:
    return get_sizes(record) == sizes


def split_record(record: str) -> tuple[str, str]:
    if "?" not in record:
        return (record, "")

    unknown_index = record.index("?")

    confirmed = record[:unknown_index].rstrip(
        "#"
    )  # stripping off the end since we don't know if it's finished being completed by replacements
    rest = record[len(confirmed) :]
    return (confirmed, rest)


def partially_fits_sizes(record: str, sizes: list[int]) -> int:
    if "?" not in record:
        return fits_sizes(record, sizes)

    (partial, _) = split_record(record)
    partial_damaged = get_sizes(partial)
    partial_sizes = sizes[: len(partial_damaged)]
    return partial_damaged == partial_sizes


def make_record_key(record: str) -> str:
    (confirmed, rest) = split_record(record)
    return (
        f"{','.join(str(x) for x in get_sizes(confirmed))}!{rest}!{record.count('#')}"
    )


def number_of_possibilities(
    cache: dict[str, int], record: str, sizes: list[int]
) -> int:
    record_key = make_record_key(record)
    if record_key in cache:
        return cache[record_key]

    if "?" not in record:
        return 1 if fits_sizes(record, sizes) else 0

    with_damaged = record.replace("?", "#", 1)
    with_operational = record.replace("?", ".", 1)

    result = (
        number_of_possibilities(cache, with_damaged, sizes)
        if partially_fits_sizes(with_damaged, sizes)
        else 0
    ) + (
        number_of_possibilities(cache, with_operational, sizes)
        if partially_fits_sizes(with_operational, sizes)
        else 0
    )

    cache[record_key] = result

    return result


def part_1() -> None:
    lines = read_file("../data/2023/12.txt")

    result = sum(
        number_of_possibilities({}, record, sizes) for (record, sizes) in lines
    )

    print(result)


def expand_line(line: tuple[str, list[int]]) -> tuple[str, list[int]]:
    expanded_record = line[0]
    expanded_sizes = line[1][:]
    for _ in range(4):
        expanded_record += "?" + line[0]
        expanded_sizes.extend(line[1])

    return (expanded_record, expanded_sizes)


def expand_lines(lines: list[tuple[str, list[int]]]) -> list[tuple[str, list[int]]]:
    return [expand_line(x) for x in lines]


def part_2() -> None:
    lines = expand_lines(read_file("../data/2023/12.txt"))

    found_possibilities = 0
    total_lines = len(lines)
    for i, (record, sizes) in enumerate(lines):
        this_line = number_of_possibilities({}, record, sizes)
        found_possibilities += this_line
        print(f"completed line: {i + 1} / {total_lines}")

    result = found_possibilities

    print(result)


# print(f"test [?.]{partially_fits_sizes('?.', [1])}")
# print(f"test [#.]{partially_fits_sizes('#.', [1])}")
# print(f"test [..]{partially_fits_sizes('..', [1])}")
# print(f"test [.#.]{partially_fits_sizes('.#.', [1])}")
# print(f"test [.#?.]{partially_fits_sizes('.#?.', [1])}")
# print(
#     f"test [.#.#?#?#?#?#?#?] [1,3,1,6]{partially_fits_sizes('.#.#?#?#?#?#?#?', [1,3,1,6])}"
# )


part_1()
part_2()
