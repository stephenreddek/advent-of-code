from collections import defaultdict
from dataclasses import dataclass
from enum import Enum


@dataclass
class Input:
    rules: list[tuple[int, int]]
    printed_pages: list[list[int]]


def read_input(lines: list[str]) -> Input:
    result = Input(rules=[], printed_pages=[])
    finished_rules = False
    for line in lines:
        if line.strip() == "":
            finished_rules = True
            continue

        if finished_rules:
            # input printed pages
            pages = [int(x) for x in line.strip().split(",")]
            result.printed_pages.append(pages)

        else:
            # input rules
            [first, second] = line.strip().split("|")
            result.rules.append((int(first), int(second)))

    return result


def rule_passes(rule: tuple[int, int], page: list[int]) -> bool:
    lower = rule[0]
    upper = rule[1]
    index_of_lower = page.index(lower) if lower in page else None
    index_of_upper = page.index(upper) if upper in page else None

    if index_of_lower is not None and index_of_upper is not None:
        return index_of_lower < index_of_upper

    return True


def part_1() -> None:
    data_file = open("../data/2024/05.txt", "r")
    lines = data_file.readlines()
    data_file.close()

    input_data = read_input(lines)

    result = 0
    for page in input_data.printed_pages:
        passes = all(rule_passes(rule, page) for rule in input_data.rules)
        if passes:
            middle_index = len(page) // 2
            middle = page[middle_index]
            result += middle

    print(result)


def is_acceptable_position(
    new_order: list[int], rules: list[tuple[int, int]], position: int, value: int
) -> bool:
    for rule in rules:
        if rule[0] == value:
            # the other value cannot be earlier in the order
            for index in range(0, position):
                if new_order[index] == rule[1]:
                    return False

        elif rule[1] == value:
            # the other value cannot come after
            for index in range(position, len(new_order)):
                if new_order[index] == rule[0]:
                    return False

    return True


def order_pages(rules: list[tuple[int, int]], pages: list[int]) -> list[int]:
    new_order = []
    for page in pages:
        if not new_order:
            new_order.append(page)
            continue

        for potential_index in range(0, len(new_order) + 1):
            if is_acceptable_position(new_order, rules, potential_index, page):
                new_order.insert(potential_index, page)
                break

        else:
            raise Exception("Could not find acceptable location")

    return new_order


def part_2() -> None:
    data_file = open("../data/2024/05.txt", "r")
    lines = data_file.readlines()
    data_file.close()

    input_data = read_input(lines)

    result = 0
    for page in input_data.printed_pages:
        passes = all(rule_passes(rule, page) for rule in input_data.rules)
        if passes:
            continue

        ordered = order_pages(input_data.rules, page)
        middle_index = len(ordered) // 2
        middle = ordered[middle_index]
        result += middle

    print(result)


part_1()
part_2()
