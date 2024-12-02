from collections import defaultdict
from enum import Enum


class Direction(Enum):
    Increasing = "increasing"
    Decreasing = "decreasing"


def is_safe(values: list[int], allow_skips: bool) -> bool:
    direction = None
    last = None
    have_used_skip = False
    for value in values:
        if last is None:
            last = value
            continue

        if value > last:
            if direction == Direction.Decreasing:
                if have_used_skip or not allow_skips:
                    return False
                else:
                    have_used_skip = True
                    # don't set last
                    continue

            if value - last > 3:
                if have_used_skip or not allow_skips:
                    return False
                else:
                    have_used_skip = True
                    # don't set last
                    continue

            direction = Direction.Increasing
        elif value < last:
            if direction == Direction.Increasing:
                if have_used_skip or not allow_skips:
                    return False
                else:
                    have_used_skip = True
                    # don't set last
                    continue

            if last - value > 3:
                if have_used_skip or not allow_skips:
                    return False
                else:
                    have_used_skip = True
                    # don't set last
                    continue

            direction = Direction.Decreasing
        else:
            if have_used_skip or not allow_skips:
                return False
            else:
                have_used_skip = True
                # don't set last
                continue

        last = value

    return True


def part_1() -> None:
    data_file = open("../data/2024/02.txt", "r")
    lines = data_file.readlines()
    data_file.close()

    count_safe = 0
    for line in lines:
        values = [int(x) for x in line.split(" ")]

        if is_safe(values, False):
            count_safe += 1

    print(count_safe)


def part_2() -> None:
    data_file = open("../data/2024/02.txt", "r")
    lines = data_file.readlines()
    data_file.close()

    count_safe = 0
    for line in lines:
        values = [int(x) for x in line.split(" ")]

        if is_safe(values, False):
            count_safe += 1
            continue

        for i in range(0, len(values)):
            copy = values[:]
            del copy[i]
            if is_safe(copy, False):
                count_safe += 1
                break

    print(count_safe)


part_1()
part_2()
