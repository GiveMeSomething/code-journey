from typing import List

from .step import Step
from .position import Position

def read_command_logs() -> List[str]:
    result = []
    input_path = "src/solutions/two/input.txt"
    with open(input_path, "r") as input_file:
        for line in input_file:
            result.append(line)
    return result

def run_part_1():
    inputs = read_command_logs()
    position = Position()
    for input in inputs:
        current_step = Step.from_string(input)
        position.move(current_step)
    
    print(f"Day 2 Part 1: {position.x * position.y}")