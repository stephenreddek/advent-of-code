from __future__ import annotations
from typing import Set, Optional, Callable
import math
from enum import Enum
from itertools import groupby, takewhile
from functools import cmp_to_key
from heapq import heapify, heappush, heappop


class Part:
    def __init__(self, x: int, m: int, a: int, s: int) -> None:
        # print(f"x: {x} m: {m} a: {a} s: {s}")
        self.components = {}
        self.components["x"] = x
        self.components["m"] = m
        self.components["a"] = a
        self.components["s"] = s


class PartRange:
    def __init__(
        self,
        x: tuple[int, int],
        m: tuple[int, int],
        a: tuple[int, int],
        s: tuple[int, int],
    ) -> None:
        self.components = {}
        self.components["x"] = x
        self.components["m"] = m
        self.components["a"] = a
        self.components["s"] = s

    def __repr__(self) -> str:
        (min_x, max_x) = self.components["x"]
        (min_m, max_m) = self.components["m"]
        (min_a, max_a) = self.components["a"]
        (min_s, max_s) = self.components["s"]
        return f"(x: ({min_x}, {max_x}), m: ({min_m}, {max_m}), a: ({min_a}, {max_a}), s: ({min_s}, {max_s}))"

    def __str__(self) -> str:
        (min_x, max_x) = self.components["x"]
        (min_m, max_m) = self.components["m"]
        (min_a, max_a) = self.components["a"]
        (min_s, max_s) = self.components["s"]
        return f"(x: ({min_x}, {max_x}), m: ({min_m}, {max_m}), a: ({min_a}, {max_a}), s: ({min_s}, {max_s}))"

    def contains(
        self, component: str, amount: int, include_lower: bool, include_upper: bool
    ) -> bool:
        (lower, upper) = self.components[component]
        passes_lower = amount >= lower if include_lower else amount > lower
        passes_upper = amount <= upper if include_upper else amount < upper
        return passes_lower and passes_upper

    def split(
        self, component: str, amount: int, include_in_lower: bool
    ) -> tuple[PartRange | None, PartRange | None]:
        (lower, upper) = self.components[component]

        lesser = PartRange(
            self.components["x"],
            self.components["m"],
            self.components["a"],
            self.components["s"],
        )

        lesser.components[component] = (
            (lower, amount) if include_in_lower else (lower, amount - 1)
        )

        greater = PartRange(
            self.components["x"],
            self.components["m"],
            self.components["a"],
            self.components["s"],
        )

        greater.components[component] = (
            (amount + 1, upper) if include_in_lower else (amount, upper)
        )

        # print(
        #     f" split {self} at {component} {amount} {include_in_lower} into {lesser} {greater}"
        # )

        return (
            lesser
            if lesser.components[component][0] <= lesser.components[component][1]
            else None,
            greater
            if greater.components[component][0] <= greater.components[component][1]
            else None,
        )

    def combinations(self) -> int:
        (min_x, max_x) = self.components["x"]
        (min_m, max_m) = self.components["m"]
        (min_a, max_a) = self.components["a"]
        (min_s, max_s) = self.components["s"]

        return (
            (max_x - min_x + 1)
            * (max_m - min_m + 1)
            * (max_a - min_a + 1)
            * (max_s - min_s + 1)
        )


class AlwaysRule:
    def __init__(self, result: str) -> None:
        self.result = result

    def __repr__(self) -> str:
        return f"always {self.result}"

    def __str__(self) -> str:
        return f"always {self.result}"

    def test(self, part: Part) -> tuple[bool, str]:
        return (True, self.result)


class LessThanRule:
    def __init__(self, result: str, component: str, amount: int) -> None:
        self.result = result
        self.component = component
        self.amount = amount

    def __repr__(self) -> str:
        return f"{self.component} < {self.amount} -> {self.result}"

    def __str__(self) -> str:
        return f"{self.component} < {self.amount} -> {self.result}"

    def test(self, part: Part) -> tuple[bool, str]:
        return (part.components[self.component] < self.amount, self.result)


class GreaterThanRule:
    def __init__(self, result: str, component: str, amount: int) -> None:
        self.result = result
        self.component = component
        self.amount = amount

    def __repr__(self) -> str:
        return f"{self.component} > {self.amount} -> {self.result}"

    def __str__(self) -> str:
        return f"{self.component} > {self.amount} -> {self.result}"

    def test(self, part: Part) -> tuple[bool, str]:
        return (part.components[self.component] > self.amount, self.result)


def parse_rule(rule_raw: str) -> AlwaysRule | LessThanRule | GreaterThanRule:
    if ">" in rule_raw:
        [component, rest] = rule_raw.split(">")
        [amount_raw, result] = rest.split(":")
        # print(f"{component} > {amount_raw} -> {result}")
        return GreaterThanRule(result, component, int(amount_raw))

    elif "<" in rule_raw:
        [component, rest] = rule_raw.split("<")
        [amount_raw, result] = rest.split(":")
        # print(f"{component} < {amount_raw} -> {result}")
        return LessThanRule(result, component, int(amount_raw))

    else:
        # print(f"always {rule_raw}")
        return AlwaysRule(rule_raw)


def read_file(
    file_name: str,
) -> tuple[
    list[tuple[str, list[AlwaysRule | GreaterThanRule | LessThanRule]]], list[Part]
]:
    data_file = open(file_name, "r")
    lines = data_file.readlines()
    data_file.close()

    workflows = []
    parts = []
    reading_workflows = True
    for line in lines:
        if line == "\n":
            reading_workflows = False
            continue

        if reading_workflows:
            [name, rules_raw] = line.strip().split("{")
            rules = rules_raw.split("}")[0].split(",")
            workflows.append((name, [parse_rule(r) for r in rules]))

        else:
            # {x=787,m=2655,a=1222,s=2876}
            split = line.strip().split("=")
            x = split[1].split(",")[0]
            m = split[2].split(",")[0]
            a = split[3].split(",")[0]
            s = split[4].split("}")[0]
            parts.append(Part(int(x), int(m), int(a), int(s)))

    return (workflows, parts)


def part_1() -> None:
    (workflows, parts) = read_file("../data/2023/19.txt")
    starting_workflow = "in"

    indexed_workflows: dict[str, list[AlwaysRule | GreaterThanRule | LessThanRule]] = {}
    for name, rules in workflows:
        indexed_workflows[name] = rules

    accepted = []
    rejected = []
    for part in parts:
        result = starting_workflow
        while True:
            if result == "A":
                accepted.append(part)
                break
            elif result == "R":
                rejected.append(part)
                break
            else:
                rules = indexed_workflows[result]
                for rule in rules:
                    (passes, rule_result) = rule.test(part)
                    if passes:
                        result = rule_result
                        break

    score = sum(
        part.components["x"]
        + part.components["m"]
        + part.components["a"]
        + part.components["s"]
        for part in accepted
    )
    print(score)


def part_2() -> None:
    (workflows, parts) = read_file("../data/2023/19.txt")

    starting_workflow = "in"

    indexed_workflows: dict[str, list[AlwaysRule | GreaterThanRule | LessThanRule]] = {}
    for name, rules in workflows:
        indexed_workflows[name] = rules

    # x: (1, 4000), m: (1, 4000), a: (1, 4000), s: (1, 4000)
    # for each rule, split testing on the condition

    accepted = []
    rejected = []
    to_evaluate = [
        (starting_workflow, PartRange((1, 4000), (1, 4000), (1, 4000), (1, 4000)))
    ]
    while len(to_evaluate):
        (state, part_range) = to_evaluate.pop()
        if state == "A":
            accepted.append(part_range)
        elif state == "R":
            rejected.append(part_range)
        else:
            rules = indexed_workflows[state]
            for rule in rules:
                if isinstance(rule, AlwaysRule):
                    to_evaluate.append((rule.result, part_range))
                    break
                elif isinstance(rule, GreaterThanRule):
                    if part_range.contains(rule.component, rule.amount, True, False):
                        (lesser, greater) = part_range.split(
                            rule.component, rule.amount, True
                        )
                        if lesser is not None:
                            to_evaluate.append((state, lesser))

                        if greater is not None:
                            to_evaluate.append((rule.result, greater))
                        break

                elif isinstance(rule, LessThanRule):
                    if part_range.contains(rule.component, rule.amount, False, True):
                        # print(rule)
                        (lesser, greater) = part_range.split(
                            rule.component, rule.amount, False
                        )
                        if lesser is not None:
                            to_evaluate.append((rule.result, lesser))

                        if greater is not None:
                            to_evaluate.append((state, greater))
                        break

                else:
                    raise Exception("Failed to process rule")

    score = sum(part_range.combinations() for part_range in accepted)
    print(score)


part_1()
part_2()
