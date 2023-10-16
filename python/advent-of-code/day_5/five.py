import re
from typing import List


def read_cargo_from_file() -> str:
    file = open("day_5/cargo.txt")

    # mode 0 for processing cargo setup
    # mode 1 for processing cargo instructions
    process_mode = 0

    stack_list = []

    for line in file.readlines():
        if line.strip() == "":
            # Change process mode
            process_mode = 1
            continue

        if process_mode == 0:
            line_to_cargos(line, stack_list)

        if process_mode == 1:
            # process_cargo_single(line, stack_list)
            process_cargo_batch(line, stack_list)

    result = ""
    for stack in stack_list:
        result += stack[-1]

    return result


# Process cargo line, add data to list
def line_to_cargos(line: str, stack_list: List[List[int]]):
    for i in range(1, len(line), 4):
        if line[i].strip() == "":
            continue

        if line[i].isnumeric():
            # Avoid processing cargo number row
            break

        list_position = int((i - 1) / 4)
        stack_exist = list_position < len(stack_list)
        if stack_exist:
            stack_list[list_position].insert(0, line[i])
        else:
            for j in range(list_position - len(stack_list) + 1):
                stack_list.append([])
            stack_list[list_position].insert(0, line[i])


# Move one cargo at a time
def process_cargo_single(order: str, stack_list: List[List[int]], /):
    result = re.search("move (\d+) from (\d+) to (\d+)", order)
    time = int(result.group(1))
    stack_from = int(result.group(2)) - 1
    stack_to = int(result.group(3)) - 1

    for i in range(time):
        cargo = stack_list[stack_from].pop()
        stack_list[stack_to].append(cargo)


# Move multiple cargo at a time, retain their order
def process_cargo_batch(order: str, stack_list: List[List[int]], /):
    result = re.search("move (\d+) from (\d+) to (\d+)", order)
    time = int(result.group(1))
    stack_from = int(result.group(2)) - 1
    stack_to = int(result.group(3)) - 1

    cargos = stack_list[stack_from][len(stack_list[stack_from]) - time :]
    stack_list[stack_to].extend(cargos)
    for i in range(time):
        stack_list[stack_from].pop()
