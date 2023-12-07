from __future__ import annotations
from typing import Set, Optional
import math
from enum import Enum
from itertools import groupby
from functools import cmp_to_key


class HandType(Enum):
    HIGH_CARD = 1
    ONE_PAIR = 2
    TWO_PAIR = 3
    THREE_OF_A_KIND = 4
    FULL_HOUSE = 5
    FOUR_OF_A_KIND = 6
    FIVE_OF_A_KIND = 7


def read_file(file_name: str) -> list[tuple[str, int]]:
    data_file = open(file_name, "r")
    lines = data_file.readlines()
    data_file.close()

    hands = []
    for line in lines:
        [cards, bid_str] = line.strip().split(" ")
        hands.append((cards, int(bid_str)))

    return hands


# <0 means a < b
def compare_hands(a: tuple[str, int], b: tuple[str, int]) -> int:
    type_a: HandType = type_of_hand(a[0])
    type_b: HandType = type_of_hand(b[0])

    if type_a.value == type_b.value:
        return compare_equal_type_hands(a[0], b[0])

    return -1 if type_a.value < type_b.value else 1


def compare_hands_2(a: tuple[str, int], b: tuple[str, int]) -> int:
    type_a: HandType = type_of_hand_2(a[0])
    type_b: HandType = type_of_hand_2(b[0])

    if type_a.value == type_b.value:
        return compare_equal_type_hands(a[0], b[0])

    return -1 if type_a.value < type_b.value else 1


def type_of_hand_2(hand: str) -> HandType:
    jokers = hand.count("J")
    hand_without_jokers = hand.replace("J", "")
    grouped = [
        (key, len(list(group)))
        for key, group in groupby(sorted(list(hand_without_jokers)))
    ]

    sorted_groups = sorted(grouped, key=lambda x: -x[1])

    groups_count = len(sorted_groups)

    if groups_count == 0 or groups_count == 1:
        ## all one or all jokers
        return HandType.FIVE_OF_A_KIND
    elif groups_count == 2:
        if sorted_groups[0][1] + jokers == 4:
            return HandType.FOUR_OF_A_KIND
        else:
            return HandType.FULL_HOUSE
    elif groups_count == 3:
        if sorted_groups[0][1] + jokers >= 3:
            return HandType.THREE_OF_A_KIND
        else:
            return HandType.TWO_PAIR
    elif groups_count == 4:
        return HandType.ONE_PAIR
    else:
        return HandType.HIGH_CARD


def compare_cards(a: str, b: str) -> int:
    card_orders = [
        "A",
        "K",
        "Q",
        "T",
        "9",
        "8",
        "7",
        "6",
        "5",
        "4",
        "3",
        "2",
        "J",
    ]  # part 2 order

    a_index = card_orders.index(a)
    b_index = card_orders.index(b)

    if a_index == b_index:
        return 0

    return -1 if a_index > b_index else 1


def compare_equal_type_hands(a: str, b: str) -> int:
    for card_a, card_b in zip(list(a), list(b)):
        comp = compare_cards(card_a, card_b)
        if comp != 0:
            return comp

    return 0


def type_of_hand(hand: str) -> HandType:
    grouped = [(key, len(list(group))) for key, group in groupby(sorted(list(hand)))]

    sorted_groups = sorted(grouped, key=lambda x: -x[1])

    groups_count = len(sorted_groups)

    # for key, count in sorted_groups:
    #     print(f"k: {key}")
    #     print(f"g: {count}")

    if groups_count == 1:
        return HandType.FIVE_OF_A_KIND
    elif groups_count == 2:
        if sorted_groups[0][1] == 4:
            return HandType.FOUR_OF_A_KIND
        else:
            return HandType.FULL_HOUSE
    elif groups_count == 3:
        if sorted_groups[0][1] == 3:
            return HandType.THREE_OF_A_KIND
        else:
            return HandType.TWO_PAIR
    elif groups_count == 4:
        return HandType.ONE_PAIR
    else:
        return HandType.HIGH_CARD


def part_1() -> None:
    hands = read_file("../data/2023/07.txt")
    for hand, bid in hands:
        type_of_hand(hand)

    sorted_hands: list[tuple[str, int]] = sorted(
        hands, key=cmp_to_key(lambda a, b: compare_hands(a, b))
    )
    score = sum([bid * (index + 1) for index, (hand, bid) in enumerate(sorted_hands)])
    print(score)  ## broken because of card order now


def part_2() -> None:
    hands = read_file("../data/2023/07.txt")
    for hand, bid in hands:
        type_of_hand(hand)

    # print(hands)
    sorted_hands: list[tuple[str, int]] = sorted(
        hands, key=cmp_to_key(lambda a, b: compare_hands_2(a, b))
    )
    score = sum([bid * (index + 1) for index, (hand, bid) in enumerate(sorted_hands)])
    # print(sorted_hands)
    print(score)


part_1()
part_2()
