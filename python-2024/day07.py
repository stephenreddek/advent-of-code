from collections import defaultdict
import copy
from dataclasses import dataclass
from enum import Enum


@dataclass
class Input:
    test_value: int
    equation_numbers: list[int]


def read_input() -> list[Input]:
    data_file = open("../data/2024/07.txt", "r")
    lines = data_file.readlines()
    data_file.close()

    result = []
    for line in lines:
        [test_value_raw, equation_numbers_raw] = line.strip().split(": ")
        test_value = int(test_value_raw)
        equation_numbers = [int(number) for number in equation_numbers_raw.split(" ")]
        result.append(Input(test_value, equation_numbers))

    return result


def get_combinations(number_of_operators: int) -> list[str]:
    operators = ["+", "*"]
    if number_of_operators == 0:
        return []

    if number_of_operators == 1:
        return operators

    result = []
    for operator in operators:
        for combination in get_combinations(number_of_operators - 1):
            result.append(operator + combination)

    return result


def get_combinations_part2(number_of_operators: int) -> list[list[str]]:
    operators = ["+", "*", "||"]
    if number_of_operators == 0:
        return []

    if number_of_operators == 1:
        return [[operator] for operator in operators]

    result = []
    for operator in operators:
        for combination in get_combinations_part2(number_of_operators - 1):
            result.append([operator] + combination)

    return result


def evaluate_equation(equation: list[str | int]) -> int:
    result = int(equation[0])
    for i in range(1, len(equation), 2):
        operator = equation[i]
        number = equation[i + 1]
        if operator == "+":
            result += int(number)
        elif operator == "*":
            result *= int(number)
        elif operator == "||":
            result = int(str(result) + str(number))

    return result


def can_be_solved(test_value: int, equation_numbers: list[int]) -> bool:
    number_of_operators = len(equation_numbers) - 1
    combinations = get_combinations(number_of_operators)
    for combination in combinations:
        equation: list[str | int] = [equation_numbers[0]]
        for i in range(number_of_operators):
            equation.append(combination[i])
            equation.append(equation_numbers[i + 1])

        if evaluate_equation(equation) == test_value:
            return True

    return False


def can_be_solved_part2(test_value: int, equation_numbers: list[int]) -> bool:
    number_of_operators = len(equation_numbers) - 1
    combinations = get_combinations_part2(number_of_operators)
    for combination in combinations:
        equation: list[str | int] = [equation_numbers[0]]
        for i in range(number_of_operators):
            equation.append(combination[i])
            equation.append(equation_numbers[i + 1])

        if evaluate_equation(equation) == test_value:
            return True

    return False


def part_1() -> None:
    input_data = read_input()

    result = 0
    for equation in input_data:
        if can_be_solved(equation.test_value, equation.equation_numbers):
            result += equation.test_value

    print(result)


def part_2() -> None:
    input_data = read_input()

    result = 0
    for equation in input_data:
        if can_be_solved_part2(equation.test_value, equation.equation_numbers):
            result += equation.test_value

    print(result)


part_1()
part_2()
