from __future__ import annotations
from typing import Set, Optional
import math


class Range:
    def __init__(self, starting_value: int, end_value: int) -> None:
        if end_value < starting_value:
            raise Exception(
                f"Cannot create a range where the end [{end_value}] is before the start [{starting_value}]"
            )
        self.starting_value = starting_value
        self.ending_value = end_value

    def __repr__(self) -> str:
        return f"({self.starting_value}, {self.ending_value})"

    def __str__(self) -> str:
        return f"({self.starting_value}, {self.ending_value})"

    def apply_offset(self, offset: int) -> Range:
        self.starting_value += offset
        self.ending_value += offset
        return self

    def overlap(self, other: Range) -> Optional[Range]:
        (smaller, bigger) = (
            (self, other)
            if self.starting_value < other.starting_value
            else (other, self)
        )

        if smaller.ending_value < bigger.starting_value:
            return None

        return Range(
            bigger.starting_value, min(smaller.ending_value, bigger.ending_value)
        )

    def split_into_overlap_sections(
        self, other: Range
    ) -> tuple[Range | None, Range | None, Range | None]:
        overlap = self.overlap(other)
        if overlap is None:
            if self.starting_value < other.starting_value:
                return (self, None, None)
            else:
                return (None, None, self)

        has_before = self.starting_value < other.starting_value
        before = (
            Range(self.starting_value, other.starting_value) if has_before else None
        )

        has_after = self.ending_value > other.ending_value
        after = Range(other.ending_value + 1, self.ending_value) if has_after else None

        # print(f"split {self} with {other} into {before} + {overlap} + {after}")

        return (before, overlap, after)


class RangeMapping:
    def __init__(
        self, source_start: int, destination_start: int, value_range: int
    ) -> None:
        self.source_range = Range(source_start, source_start + value_range - 1)
        self.destination_start = destination_start
        self.range = value_range

    def __repr__(self) -> str:
        return f"({self.source_range}, {self.destination_start} - {self.range})"

    def __str__(self) -> str:
        return f"({self.source_range}, {self.destination_start} - {self.range})"

    def map_range(self, values: Range) -> tuple[list[Range], list[Range]]:
        mapped_values: list[Range] = []
        unmapped_values: list[Range] = []
        offset = self.destination_start - self.source_range.starting_value

        (before_self, overlaps, after_self) = values.split_into_overlap_sections(
            self.source_range
        )

        mapped_overlaps = [] if overlaps is None else [overlaps.apply_offset(offset)]
        before_remaining = [] if before_self is None else [before_self]
        after_remaining = [] if after_self is None else [after_self]

        return (mapped_overlaps, before_remaining + after_remaining)


class Mapping:
    def __init__(
        self, source_start: int, destination_start: int, value_range: int
    ) -> None:
        self.source_start = source_start
        self.destination_start = destination_start
        self.range = value_range

    def map(self, value: int) -> Optional[int]:
        offset = value - self.source_start
        if offset < 0 or offset >= self.range:  # 10 2 -> 10 11
            return None

        return self.destination_start + offset


def mapValue(value: int, mappings: list[Mapping]) -> int:
    for mapping in mappings:
        mapped = mapping.map(value)
        if mapped is not None:
            return mapped

    return value


def map_ranges(
    value_ranges: list[Range], mapping: RangeMapping
) -> tuple[list[Range], list[Range]]:
    mapped = []
    unmapped = []

    for value_range in value_ranges:
        (range_mapped, range_unmapped) = mapping.map_range(value_range)
        mapped.extend(range_mapped)
        unmapped.extend(range_unmapped)

    # print(f"found {mapped} in {mapping}")

    return (mapped, unmapped)


def applyMappingsToRanges(
    value_ranges: list[Range], mappings: list[RangeMapping]
) -> list[Range]:
    results = []
    to_evaluate = value_ranges
    for mapping in mappings:
        if len(to_evaluate) == 0:
            break
        # print(f"input: {to_evaluate}, {mapping}")
        (mapping_results, remaining) = map_ranges(to_evaluate, mapping)
        # print(f"map_ranges result ({mapping_results}, {remaining})")
        to_evaluate = remaining
        results.extend(mapping_results)

    results.extend(to_evaluate)
    return results


def read_file(file_name: str) -> tuple[list[int], list[list[Mapping]]]:
    data_file = open(file_name, "r")
    lines = data_file.readlines()
    data_file.close()

    starting_seeds = [int(x) for x in lines[0].strip().split(": ")[1].split(" ")]

    all_mappings: list[list[Mapping]] = []

    current_mapping: list[Mapping] = []
    for line in lines[2:]:
        if line == "\n":
            all_mappings.append(current_mapping)
            current_mapping = []
            continue

        if "map:" in line:
            continue

        [destination_start, source_start, value_range] = [
            int(x) for x in line.strip().split(" ")
        ]

        current_mapping.append(Mapping(source_start, destination_start, value_range))

    all_mappings.append(current_mapping)
    return (starting_seeds, all_mappings)


def read_file_2(file_name: str) -> tuple[list[int], list[list[RangeMapping]]]:
    data_file = open(file_name, "r")
    lines = data_file.readlines()
    data_file.close()

    starting_seeds = [int(x) for x in lines[0].strip().split(": ")[1].split(" ")]

    all_mappings: list[list[RangeMapping]] = []

    current_mapping: list[RangeMapping] = []
    for line in lines[2:]:
        if line == "\n":
            all_mappings.append(current_mapping)
            current_mapping = []
            continue

        if "map:" in line:
            continue

        [destination_start, source_start, value_range] = [
            int(x) for x in line.strip().split(" ")
        ]

        current_mapping.append(
            RangeMapping(source_start, destination_start, value_range)
        )

    all_mappings.append(current_mapping)
    return (starting_seeds, all_mappings)


def part_1() -> None:
    (starting_seeds, mappings) = read_file("../data/2023/05.txt")

    lowest_final_mapping: Optional[int] = None
    for seed in starting_seeds:
        result = seed
        for mapping in mappings:
            result = mapValue(result, mapping)

        if lowest_final_mapping is None or result < lowest_final_mapping:
            lowest_final_mapping = result

    print(lowest_final_mapping)


def part_2() -> None:
    (starting_seeds, mappings) = read_file_2("../data/2023/05.txt")

    # seed_groups = [[82, 1]]
    seed_groups = zip(*(iter(starting_seeds),) * 2)

    lowest_final_mapping: Optional[int] = None
    for seed_group in seed_groups:
        starting_value = seed_group[0]
        seed_range = seed_group[1]
        ending_value = starting_value + seed_range - 1
        values = [Range(starting_value, ending_value)]
        for mapping_set in mappings:
            values = applyMappingsToRanges(values, mapping_set)
            # print(values)

        lowest_seed_from_range = min([x.starting_value for x in values])
        if (
            lowest_final_mapping is None
            or lowest_seed_from_range < lowest_final_mapping
        ):
            lowest_final_mapping = lowest_seed_from_range

    print(lowest_final_mapping)


part_1()
part_2()
