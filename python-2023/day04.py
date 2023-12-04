from typing import Set
import math


def read_file(file_name: str) -> list[tuple[Set[int], Set[int]]]:
    data_file = open(file_name, "r")
    lines = data_file.readlines()
    data_file.close()

    cards = []

    for line in lines:
        all_numbers = line.split(": ")[1]
        [winning_numbers, own_numbers] = all_numbers.split(" | ")

        parsed_winning_numbers = [int(x) for x in winning_numbers.split(" ") if x != ""]
        parsed_own_numbers = [int(x) for x in own_numbers.split(" ") if x != ""]

        cards.append((set(parsed_winning_numbers), set(parsed_own_numbers)))

    return cards


def part_1() -> None:
    cards = read_file("../data/2023/04.txt")

    score = 0
    for winning_numbers, own_numbers in cards:
        matching = len(winning_numbers.intersection(own_numbers))
        if matching == 0:
            continue

        card_score = math.pow(2, matching - 1)

        score += int(card_score)

    print(score)


def part_2() -> None:
    cards = read_file("../data/2023/04.txt")

    copies: dict[int, int] = {}
    for x in range(0, len(cards)):
        copies[x] = 1

    for index, (winning_numbers, own_numbers) in enumerate(cards):
        matching = len(winning_numbers.intersection(own_numbers))
        if matching == 0:
            continue

        for card in range(index + 1, index + matching + 1):
            copies[card] += copies[index]

    print(sum(copies.values()))


part_1()
part_2()
