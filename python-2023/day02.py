def is_valid_reveal(reveal: str, cubes_in_game: dict[str, int]) -> bool:
    cubes = reveal.split(", ")
    for cube in cubes:
        [number_str, color] = cube.split(" ")
        number = int(number_str)
        if number > cubes_in_game[color]:
            return False

    return True


def is_valid_game(reveal_string: str, cubes_in_game: dict[str, int]) -> bool:
    reveals = reveal_string.split("; ")
    for reveal in reveals:
        if not is_valid_reveal(reveal, cubes_in_game):
            return False

    return True


def part_1() -> None:
    data_file = open("../data/2023/02.txt", "r")
    games = data_file.readlines()
    data_file.close()

    cubes_in_game = {"red": 12, "green": 13, "blue": 14}

    valid_game_id_sum: int = 0
    for game in games:
        [game_description, reveal_string] = game.strip().split(": ")
        if is_valid_game(reveal_string, cubes_in_game):
            game_id = game_description.split(" ")[1]
            valid_game_id_sum += int(game_id)

    print(valid_game_id_sum)


def part_2() -> None:
    data_file = open("../data/2023/02.txt", "r")
    games = data_file.readlines()
    data_file.close()

    game_power_sum: int = 0
    for game in games:
        cubes_in_game = {"red": 0, "green": 0, "blue": 0}
        [game_description, reveal_string] = game.strip().split(": ")
        reveals = reveal_string.split("; ")
        for reveal in reveals:
            cubes = reveal.split(", ")
            for cube in cubes:
                [number_str, color] = cube.split(" ")
                number = int(number_str)
                if number > cubes_in_game[color]:
                    cubes_in_game[color] = number

        game_power = (
            cubes_in_game["red"] * cubes_in_game["green"] * cubes_in_game["blue"]
        )
        game_power_sum += game_power

    print(game_power_sum)


part_1()
part_2()
