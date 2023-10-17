from utils.utils import read_data_from_file
from typing import List


def calculate_dir_size():
    commands = read_data_from_file("day_7/commands.txt")

    # Keep track of directory size
    dir_size_map = {}

    # Keep track of nested directory
    dir_stack = []

    # Parse commands based on their token
    for command in commands:
        tokens = command.strip().split(" ")
        print(tokens)

        # $ indicate a command
        if tokens[0] == "$":
            match tokens[1]:
                case "cd":
                    match tokens[2]:
                        # Command to return to root
                        case "/":
                            dir_stack.clear()
                        # Command to go back one directory
                        case "..":
                            dir_stack.pop()
                        # Else, it is command to go to a specific directory
                        case _:
                            dir_stack.append(tokens[2])
                case "ls":
                    # Ignore ls command, only care about their result
                    pass
        else:
            # If not begin with $, it indicate that this is a command's result
            match tokens[0]:
                case "dir":
                    # If there are directories, add them to the map
                    dir_size_map[tokens[1]] = 0
                    pass
                case _:
                    # Add the file size for all directory currently in the stack
                    for dir in dir_stack:
                        dir_size_map[dir] += int(tokens[0])

    # Print dir_size_map for debugging
    print(dir_size_map)

    # Caluate the sum of all directory that their size at most 100.000
    sum = 0
    for value in dir_size_map.values():
        if value <= 100000:
            sum += value

    return sum
