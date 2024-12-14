from collections import defaultdict
import copy
from dataclasses import dataclass
from enum import Enum
import itertools


@dataclass
class Input:
    plant_map: dict[tuple[int, int], str]
    width: int
    height: int


class Side(Enum):
    TOP = "top"
    RIGHT = "right"
    BOTTOM = "bottom"
    LEFT = "left"


def read_input() -> Input:
    data_file = open("../data/2024/12.txt", "r")
    lines = data_file.readlines()
    data_file.close()

    result = Input(plant_map={}, width=len(lines[0].strip()), height=len(lines))
    for y, line in enumerate(lines):
        for x, char in enumerate(line.strip()):
            result.plant_map[(x, y)] = char

    return result


def find_neighbors(point: tuple[int, int]) -> list[tuple[tuple[int, int], Side]]:
    return [
        ((point[0] - 1, point[1]), Side.LEFT),
        ((point[0] + 1, point[1]), Side.RIGHT),
        ((point[0], point[1] - 1), Side.TOP),
        ((point[0], point[1] + 1), Side.BOTTOM),
    ]


def find_all_perimeters_by_point(input: Input) -> dict[tuple[int, int], int]:
    result = defaultdict(int)

    for point in input.plant_map:
        for neighbor, side in find_neighbors(point):
            if neighbor not in input.plant_map:
                result[point] += 1
            elif input.plant_map[neighbor] != input.plant_map[point]:
                result[point] += 1

    return result


def group_points_into_neighbor_sets_by_crop(input: Input) -> list[set[tuple[int, int]]]:
    sets_by_point: dict[tuple[int, int], set[tuple[int, int]]] = {}
    for point in input.plant_map:
        sets_by_point[point] = {point}

    for point in input.plant_map:
        for neighbor, side in find_neighbors(point):
            if (
                neighbor in input.plant_map
                and input.plant_map[neighbor] == input.plant_map[point]
            ):
                sets_by_point[point].update(sets_by_point[neighbor])
                for other_point in sets_by_point[neighbor]:
                    sets_by_point[other_point] = sets_by_point[point]

    results = []
    for point in input.plant_map:
        if sets_by_point[point] not in results:
            results.append(sets_by_point[point])
    return results


def find_all_sides_by_point(
    points: set[tuple[int, int]],
) -> dict[tuple[int, int], set[Side]]:
    result = defaultdict(set)

    for point in points:
        for neighbor, side in find_neighbors(point):
            if neighbor not in points:
                result[point].add(side)

    return result


def neighbors_that_could_share_side(
    point: tuple[int, int], side: Side
) -> list[tuple[int, int]]:
    if side == Side.TOP or side == Side.BOTTOM:
        return [
            (point[0] - 1, point[1]),
            (point[0] + 1, point[1]),
        ]
    elif side == Side.RIGHT or side == Side.LEFT:
        return [
            (point[0], point[1] - 1),
            (point[0], point[1] + 1),
        ]
    else:
        raise ValueError("Invalid side")


def group_into_contiguous_sets_with_matching_sides(
    points: set[tuple[int, int]],
    side: Side,
) -> list[set[tuple[int, int]]]:
    sets_by_point: dict[tuple[int, int], set[tuple[int, int]]] = {}
    sides_by_points = find_all_sides_by_point(points)

    for point in points:
        if side in sides_by_points[point]:
            sets_by_point[point] = {point}

    for point in sets_by_point.keys():
        for neighbor in neighbors_that_could_share_side(point, side):
            if neighbor in sets_by_point:
                sets_by_point[point].update(sets_by_point[neighbor])
                for other_point in sets_by_point[neighbor]:
                    sets_by_point[other_point] = sets_by_point[point]

    results = []
    for point in points:
        if point in sets_by_point and sets_by_point[point] not in results:
            results.append(sets_by_point[point])
    return results


def count_of_sides(points: set[tuple[int, int]]) -> int:
    result = 0
    result += len(group_into_contiguous_sets_with_matching_sides(points, Side.TOP))
    result += len(group_into_contiguous_sets_with_matching_sides(points, Side.LEFT))
    result += len(group_into_contiguous_sets_with_matching_sides(points, Side.RIGHT))
    result += len(group_into_contiguous_sets_with_matching_sides(points, Side.BOTTOM))

    return result


def part_1() -> None:
    input_data = read_input()

    result = 0
    sets_of_crops = group_points_into_neighbor_sets_by_crop(input_data)
    all_perimeters = find_all_perimeters_by_point(input_data)
    for crop in sets_of_crops:
        crop_area = len(crop)
        crop_perimeter = sum([all_perimeters[point] for point in crop])
        # crop_type = input_data.plant_map[next(iter(crop))]
        # print(f"{crop_type}: {crop_area} {crop_perimeter}")
        fence_pricing = crop_area * crop_perimeter
        result += fence_pricing

    print(result)


def part_2() -> None:
    input_data = read_input()
    result = 0
    sets_of_crops = group_points_into_neighbor_sets_by_crop(input_data)

    for crop in sets_of_crops:
        crop_area = len(crop)
        crop_sides = count_of_sides(crop)

        fence_pricing = crop_area * crop_sides
        result += fence_pricing

    print(result)


part_1()
part_2()
