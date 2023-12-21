from __future__ import annotations
from typing import Set, Optional, Callable
import math
from enum import Enum
from itertools import groupby, takewhile
from functools import cmp_to_key
from heapq import heapify, heappush, heappop


class FlipFlop:
    def __init__(self, name: str, destinations: list[str]):
        self.name = name
        self.memory = False
        self.destinations = destinations
        self.inputs: list[str] = []

    def addInput(self, connection: str) -> None:
        self.inputs.append(connection)

    def trigger(self, input: str, value: bool) -> list[tuple[str, bool]]:
        if value:
            return []

        self.memory = not self.memory
        return [(x, self.memory) for x in self.destinations]


class Conjunction:
    def __init__(self, name: str, destinations: list[str]):
        self.name = name
        self.memory: dict[str, bool] = {}
        self.destinations = destinations
        self.inputs: list[str] = []

    def addInput(self, connection: str) -> None:
        self.memory[connection] = False
        self.inputs.append(connection)

    def trigger(self, input: str, value: bool) -> list[tuple[str, bool]]:
        self.memory[input] = value

        all_high = all(self.memory.values())
        output = not all_high

        return [(x, output) for x in self.destinations]


class Broadcaster:
    def __init__(self, name: str, destinations: list[str]):
        self.name = name
        self.destinations = destinations
        self.inputs: list[str] = []

    def addInput(self, connection: str) -> None:
        self.inputs.append(connection)

    def trigger(self, input: str, value: bool) -> list[tuple[str, bool]]:
        return [(x, value) for x in self.destinations]


def read_file(
    file_name: str,
) -> dict[str, FlipFlop | Conjunction | Broadcaster]:
    data_file = open(file_name, "r")
    lines = data_file.readlines()
    data_file.close()

    modules: dict[str, FlipFlop | Conjunction | Broadcaster] = {}
    for line in lines:
        [module_description, destination_description] = line.strip().split(" -> ")
        destinations = destination_description.split(", ")
        if module_description == "broadcaster":
            modules[module_description] = Broadcaster(module_description, destinations)
        elif module_description.startswith("%"):
            module_name = module_description[1:]
            modules[module_name] = FlipFlop(module_name, destinations)
        elif module_description.startswith("&"):
            module_name = module_description[1:]
            modules[module_name] = Conjunction(module_name, destinations)
        else:
            raise Exception(f"Invalid module description [{module_description}]")

    for module_name in modules.keys():
        module = modules[module_name]
        for destination in module.destinations:
            if destination == "output" or destination not in modules:
                continue

            destination_module = modules[destination]
            destination_module.addInput(module_name)

    modules["broadcaster"].addInput("button")

    return modules


def part_1() -> None:
    modules = read_file("../data/2023/20.txt")

    low_output = 0
    high_output = 0
    all_low = 0
    all_high = 0
    for _ in range(0, 1000):
        to_evaluate = [("broadcaster", False, "button")]
        all_low += 1
        while len(to_evaluate):
            (module_name, value, triggering_module) = to_evaluate.pop()

            if module_name not in modules:
                continue

            module = modules[module_name]
            outputs = module.trigger(triggering_module, value)
            for destination, value in outputs:
                # print(
                #     f"{module_name} -> { 'high' if value else 'low'} -> {destination}"
                # )
                if value:
                    all_high += 1
                else:
                    all_low += 1
                if destination == "output":
                    if value:
                        high_output += 1
                    else:
                        low_output += 1

                else:
                    to_evaluate.append((destination, value, module_name))

    print(f"low: {low_output}, high: {high_output}")
    print(f"all low: {all_low}, all high: {all_high}")
    score = all_low * all_high
    print(score)


def run_until_rx(modules: dict[str, FlipFlop | Conjunction | Broadcaster]) -> int:
    presses = 0
    while True:
        to_evaluate = [("broadcaster", False, "button")]
        presses += 1

        # print("state: ", end="")
        # for module_name, module in modules.items():
        #     if isinstance(module, Conjunction):
        #         print(f"{module_name}: ", end="")
        #         for slot_name, slot_value in module.memory.items():
        #             print(f"{slot_name}={slot_value}", end=" ")
        #         print(", ", end="")

        #     elif isinstance(module, FlipFlop):
        #         print(f"{module_name}: {module.memory}", end=", ")
        # print()

        # for module_name, module in modules.items():
        #     if isinstance(module, Conjunction):
        #         for slot_name, slot_value in module.memory.items():
        #             if slot_value:
        #                 print(f"{module_name} -> {slot_name}: {module.memory}")

        #     elif isinstance(module, FlipFlop):
        #         if module.memory:
        #             print(f"{module_name}: {module.memory}")

        if presses % 100000 == 0:
            print(presses)

        while len(to_evaluate):
            (module_name, value, triggering_module) = to_evaluate.pop()

            module = modules[module_name]
            outputs = module.trigger(triggering_module, value)
            for destination, value in outputs:
                if destination == "rx" and not value:
                    return presses
                elif destination in modules:
                    to_evaluate.append((destination, value, module_name))


# def build_dependency_tree(
#     modules: dict[str, FlipFlop | Conjunction | Broadcaster]
# ) -> dict[str, int]:
#     count_to_get_low = {}
#     count_to_get_low["button"] = 1
#     count_to_get_low["broadcaster"] = 1

#     all_destinations = set()
#     for module in modules.values():
#         for destination in module.destinations:
#             all_destinations.add(destination)
#     to_evaluate = list(all_destinations)

#     while len(to_evaluate):
#         module_name = to_evaluate.pop()
#         module = modules[module_name]
#         inputs = module.inputs
#         all_inputs_recorded = [x in count_to_get_low for x in inputs]
#         if not all_inputs_recorded:
#             to_evaluate.append(module_name)
#             continue

#         if isinstance(module, Conjunction):
#             # sends low when all high. all start low.
#             # will send low when all inputs sync up to send low
#             all_input_counts = [count_to_get_low[x] for x in inputs]
#             this_count = math.lcm(all_input_counts)
#             count_to_get_low[module_name] = this_count

#         elif isinstance(module, FlipFlop):
#             for input_module in inputs:

#         else:
#             raise Exception(f"invalid destination module type for {module_name}")


def part_2() -> None:
    modules = read_file("../data/2023/20.txt")

    presses = run_until_rx(modules)

    score = presses
    print(score)


part_1()
part_2()
