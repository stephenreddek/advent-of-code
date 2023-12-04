import math


def read_in_grid(filename: str) -> tuple[dict[tuple[int, int], str], int, int]:
    data_file = open(filename, "r")
    lines = data_file.readlines()
    data_file.close()

    grid = {}
    for y, line in enumerate(lines):
        trimmed = line.strip()
        for x in range(0, len(trimmed)):
            grid[(x, y)] = trimmed[x]

    return (grid, len(lines[0].strip()), len(lines))


def is_adjacent_to_symbol(
    coordinate: tuple[int, int], grid: dict[tuple[int, int], str]
) -> bool:
    for x_offset in range(-1, 2):
        for y_offset in range(-1, 2):
            c = (coordinate[0] + x_offset, coordinate[1] + y_offset)
            if c in grid:
                char = grid[c]
                if char.isdigit() or char == ".":
                    continue
                else:
                    return True

    return False


def coordinates_for_number(
    end_coordinate: tuple[int, int], value: int
) -> list[tuple[int, int]]:
    length_of_number = math.floor(
        math.log10(value)
    )  # really the length is + 1 of that, but then we'd be subtracting 1 in the next step
    starting_x: int = end_coordinate[0] - length_of_number
    return [(x, end_coordinate[1]) for x in range(starting_x, end_coordinate[0] + 1)]


def is_number_touching_symbol(
    end_coordinate: tuple[int, int], value: int, grid: dict[tuple[int, int], str]
) -> bool:
    # print(value)
    length_of_number = int(math.log10(value))
    starting_x: int = end_coordinate[0] - length_of_number
    for x in range(starting_x, end_coordinate[0] + 1):
        if is_adjacent_to_symbol((x, end_coordinate[1]), grid):
            return True

    return False


def part_1() -> None:
    (grid, count_x, count_y) = read_in_grid("../data/2023/03.txt")

    parts_sum = 0
    for y in range(0, count_y):
        current_value = 0
        for x in range(0, count_x):
            char: str = grid[(x, y)]
            if char.isdigit():
                current_value = current_value * 10 + int(char)
            elif char == ".":
                # non number or symbol
                if current_value > 0 and is_number_touching_symbol(
                    (x - 1, y), current_value, grid
                ):
                    # is a part
                    parts_sum += current_value
                current_value = 0
            elif current_value > 0:
                # symbol
                # is a part
                parts_sum += current_value
                current_value = 0

        if current_value > 0:
            # unresolved number
            if is_number_touching_symbol((x, y), current_value, grid):
                # is a part
                parts_sum += current_value

    print(parts_sum)


def radial_distance(a: tuple[int, int], b: tuple[int, int]) -> int:
    return max(abs(a[0] - b[0]), abs(a[1] - b[1]))


def is_adjacent_to_gear(
    gear: tuple[int, int], part_number: tuple[int, list[tuple[int, int]]]
) -> bool:
    for coordinate_of_number in part_number[1]:
        if radial_distance(gear, coordinate_of_number) < 2:
            return True

    return False


def connections_for_gear(
    gear: tuple[int, int], part_numbers: list[tuple[int, list[tuple[int, int]]]]
) -> list[int]:
    connections = []
    for part_number in part_numbers:
        if is_adjacent_to_gear(gear, part_number):
            connections.append(part_number[0])

    return connections


def part_2() -> None:
    (grid, count_x, count_y) = read_in_grid("../data/2023/03.txt")

    part_numbers: list[tuple[int, list[tuple[int, int]]]] = []
    for y in range(0, count_y):
        current_value = 0
        for x in range(0, count_x):
            char: str = grid[(x, y)]
            if char.isdigit():
                current_value = current_value * 10 + int(char)
            elif char == ".":
                if current_value > 0:
                    part_numbers.append(
                        (
                            current_value,
                            coordinates_for_number((x - 1, y), current_value),
                        )
                    )
                current_value = 0
            elif current_value > 0:
                part_numbers.append(
                    (current_value, coordinates_for_number((x - 1, y), current_value))
                )
                current_value = 0

        if current_value > 0:
            # unresolved number
            part_numbers.append(
                (current_value, coordinates_for_number((x, y), current_value))
            )

    gears: list[tuple[int, int]] = []
    for y in range(0, count_y):
        for x in range(0, count_x):
            char = grid[(x, y)]
            if char == "*":
                gears.append((x, y))

    sum_of_connected_gears = 0
    for gear in gears:
        connections = connections_for_gear(gear, part_numbers)
        if len(connections) == 2:
            sum_of_connected_gears += connections[0] * connections[1]

    print(sum_of_connected_gears)


part_1()
part_2()
