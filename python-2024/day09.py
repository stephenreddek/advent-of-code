from collections import defaultdict
import copy
from dataclasses import dataclass
from enum import Enum
import itertools


@dataclass
class FileBlock:
    id: int
    length: int = 1


@dataclass
class EmptyBlock:
    id: int = -1
    length: int = 1


@dataclass
class Input:
    disk: list[FileBlock | EmptyBlock]


def read_input() -> Input:
    data_file = open("../data/2024/09-example.txt", "r")
    lines = data_file.readlines()
    data_file.close()

    result = Input(disk=[])
    line = lines[0].strip()
    id_counter = 0
    for i in range(0, len(line), 2):
        block_length = int(line[i])
        empty_space = int(line[i + 1]) if i + 1 < len(line) else None

        for _ in range(0, block_length):
            result.disk.append(FileBlock(id=id_counter))

        if empty_space is not None:
            for _ in range(0, empty_space):
                result.disk.append(EmptyBlock())

        id_counter += 1

    return result


def read_input_2() -> Input:
    data_file = open("../data/2024/09.txt", "r")
    lines = data_file.readlines()
    data_file.close()

    result = Input(disk=[])
    line = lines[0].strip()
    id_counter = 0
    for i in range(0, len(line), 2):
        block_length = int(line[i])
        result.disk.append(FileBlock(id=id_counter, length=block_length))

        empty_space = int(line[i + 1]) if i + 1 < len(line) else None
        if empty_space:
            result.disk.append(EmptyBlock(length=empty_space))

        id_counter += 1

    return result


def calculate_checksum(disk: list[FileBlock | EmptyBlock]) -> int:
    checksum = 0
    for i, block in enumerate(disk):
        if isinstance(block, FileBlock):
            checksum += i * block.id

    return checksum


def compact(disk: list[FileBlock | EmptyBlock]) -> list[FileBlock | EmptyBlock]:
    result = copy.deepcopy(disk)
    for i in range(0, len(result)):
        if isinstance(result[i], EmptyBlock):
            # swap with first non-empty block from end
            for j in range(len(result) - 1, i, -1):
                if isinstance(result[j], FileBlock):
                    result[i], result[j] = result[j], result[i]
                    break

    return result


def part_1() -> None:
    input_data = read_input()

    compacted_disk = compact(input_data.disk)

    print(calculate_checksum(compacted_disk))


def defrag(
    disk: list[FileBlock | EmptyBlock], seen: set[int]
) -> list[FileBlock | EmptyBlock]:
    result = disk
    ids_to_check = [block.id for block in disk if isinstance(block, FileBlock)]
    for id_to_check in reversed(sorted(ids_to_check)):
        # visualize(result)
        to_move_index = next(
            i for i, block in enumerate(result) if block.id == id_to_check
        )
        for free_space_index in range(0, to_move_index):
            if (
                isinstance(result[free_space_index], EmptyBlock)
                and result[free_space_index].length >= result[to_move_index].length
            ):
                size_of_block_to_move = result[to_move_index].length
                free_space = result[free_space_index].length

                # move file in
                result[free_space_index] = FileBlock(
                    id=id_to_check, length=size_of_block_to_move
                )
                result[to_move_index] = EmptyBlock(length=size_of_block_to_move)
                if free_space > size_of_block_to_move:
                    result.insert(
                        free_space_index + 1,
                        EmptyBlock(length=free_space - size_of_block_to_move),
                    )
                # result[to_move_index], result[free_space_index] = (
                #     result[free_space_index],
                #     result[to_move_index],
                # )

                # remaining_space = free_space - size_of_block_to_move
                # if remaining_space > 0:
                #     result[to_move_index].length = size_of_block_to_move
                #     result.insert(free_space_index + 1, EmptyBlock(remaining_space))

                break

    return result


def visualize(disk: list[FileBlock | EmptyBlock]) -> None:
    for block in disk:
        if isinstance(block, FileBlock):
            for _ in range(0, block.length):
                print(block.id, end="")
        else:
            for _ in range(0, block.length):
                print(".", end="")
    print()


def expand_files(disk: list[FileBlock | EmptyBlock]) -> list[FileBlock | EmptyBlock]:
    result = []
    for block in disk:
        if isinstance(block, FileBlock):
            for _ in range(0, block.length):
                result.append(FileBlock(id=block.id))
        else:
            for _ in range(0, block.length):
                result.append(EmptyBlock())

    return result


def part_2() -> None:
    input_data = read_input_2()

    defragged_disk = defrag(input_data.disk, set())
    # visualize(defragged_disk)

    print(calculate_checksum(expand_files(defragged_disk)))


part_1()
part_2()
