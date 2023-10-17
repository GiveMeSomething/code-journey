from utils.utils import read_data_from_file
from typing import List, Dict


def parse_command_to_dir() -> Dict[str, int]:
    commands = read_data_from_file("day_7/commands.txt")

    # Keep track of directory size
    dir_size_map = {"/": 0}

    # Keep track of nested directory
    current_path = "/"

    # Parse commands based on their token
    for command in commands:
        tokens = command.strip().split()

        # $ indicate a command
        if tokens[0] == "$":
            match tokens[1]:
                case "cd":
                    match tokens[2]:
                        case "/":
                            # Command to return to root
                            current_path = "/"
                        case "..":
                            # Command to go back one directory
                            current_path = current_path[: current_path.rindex("/")]
                        case _:
                            # Else, it is command to go to a specific directory
                            current_path = current_path + "/" + tokens[2]
                case "ls":
                    # Ignore ls command, only care about their resultx
                    pass
        else:
            # If not begin with $, it indicate that this is a command's result
            match tokens[0]:
                case "dir":
                    # If there are directories, add them to the map (if non-existent)
                    next_path = current_path + "/" + tokens[1]
                    if dir_size_map.get(next_path) == None:
                        dir_size_map[next_path] = 0
                    pass
                case _:
                    # Add the file size for all directory currently in the stack
                    temp = current_path
                    while temp != "":
                        dir_size_map[temp] += int(tokens[0])
                        temp = temp[: temp.rindex("/")]

    # Print dir_size_map for debugging
    # print(dir_size_map)

    return dir_size_map


def calculate_dir_size(dir_map: Dict[str, int]) -> int:
    # Caluate the sum of all directory that their size at most 100.000
    sum = 0
    for value in dir_map.values():
        if value < 100000:
            sum += value

    return sum


def calculate_minimum_deletion(dir_map: Dict[str, int]) -> int:
    # Find all valid directories that can be delete to achieve target unused dir

    # Find out how much we need
    require_space = dir_map["/"] - 40_000_000

    # print(f"We need {require_space} more")

    valid_dir = []
    for value in dir_map.values():
        if value >= require_space:
            valid_dir.append(value)

    return min(valid_dir)
