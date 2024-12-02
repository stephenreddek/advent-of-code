from collections import defaultdict


def part_1() -> None:
    data_file = open("../data/2024/01.txt", "r")
    lines = data_file.readlines()
    data_file.close()

    parts = [line.split("   ") for line in lines]
    left_values = sorted([int(line[0]) for line in parts])
    right_values = sorted([int(line[1]) for line in parts])

    distance_sum: int = 0
    for i, left in enumerate(left_values):
        right = right_values[i]
        distance_sum += abs(left - right)

    print(distance_sum)


def part_2() -> None:
    data_file = open("../data/2024/01.txt", "r")
    lines = data_file.readlines()
    data_file.close()

    parts = [line.split("   ") for line in lines]
    left_values = sorted([int(line[0]) for line in parts])
    right_values = sorted([int(line[1]) for line in parts])

    right_counts: dict[int, int] = defaultdict(int)
    for value in right_values:
        right_counts[value] += 1

    similarity_score = 0
    for left in left_values:
        similarity_score += left * right_counts[left]

    print(similarity_score)


part_1()
part_2()
