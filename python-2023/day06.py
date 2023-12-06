from __future__ import annotations
from typing import Set, Optional
import math


def read_file(file_name: str) -> zip[tuple[int, int]]:
    data_file = open(file_name, "r")
    lines = data_file.readlines()
    data_file.close()

    times = [
        int(x) for x in lines[0].strip().split(":")[1].strip().split(" ") if x != ""
    ]
    distances = [
        int(x) for x in lines[1].strip().split(":")[1].strip().split(" ") if x != ""
    ]

    return zip(times, distances)


def read_file2(file_name: str) -> tuple[int, int]:
    data_file = open(file_name, "r")
    lines = data_file.readlines()
    data_file.close()

    time = int(
        "".join(
            [x for x in lines[0].strip().split(":")[1].strip().split(" ") if x != ""]
        )
    )
    distance = int(
        "".join(
            [x for x in lines[1].strip().split(":")[1].strip().split(" ") if x != ""]
        )
    )

    return (time, distance)


def calculate_distance(race_time: int, hold_time: int) -> int:
    travel_time = race_time - hold_time
    speed = hold_time
    return speed * travel_time


def part_1() -> None:
    races_data = read_file("../data/2023/06.txt")

    score = 1
    for race_time, distance_record in races_data:
        count_of_wins = 0
        for hold_time in range(1, race_time):
            if calculate_distance(race_time, hold_time) > distance_record:
                count_of_wins += 1

        score *= count_of_wins

    print(score)


def part_2() -> None:
    (race_time, distance_record) = read_file2("../data/2023/06.txt")

    count_of_wins = 0
    for hold_time in range(1, race_time):
        if calculate_distance(race_time, hold_time) > distance_record:
            count_of_wins += 1

    print(count_of_wins)


part_1()
part_2()
