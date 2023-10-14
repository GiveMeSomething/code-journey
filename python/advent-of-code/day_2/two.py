from typing import Tuple, List


def read_inputs_from_file() -> Tuple[List[str], List[str]]:
    inputFile = open("day_two/inputs.txt", "r", encoding="utf-8")

    opponent_inputs = []
    user_inputs = []

    for input in inputFile.readlines():
        opponent_input, user_input = input.strip().split(" ", 1)
        opponent_inputs.append(opponent_input)
        user_inputs.append(user_input)

    return opponent_inputs, user_inputs


def cal_points(inputs: Tuple[List[str], List[str]]) -> int:
    oppo_inputs, user_inputs = inputs

    input_to_point = {"A": 1, "B": 2, "C": 3, "X": 1, "Y": 2, "Z": 3}

    point = 0
    for i in range(len(oppo_inputs)):
        oppo_value = input_to_point[oppo_inputs[i]]
        user_value = input_to_point[user_inputs[i]]

        win_value = 3 if oppo_value == 2 else (oppo_value + 1) % 3
        lose_value = 3 if oppo_value == 1 else oppo_value - 1

        if user_value == win_value:
            # Win condition
            point += 6 + user_value
        elif user_value == lose_value:
            # Lose condition
            point += 0 + user_value
        else:
            # Draw condition
            point += 3 + user_value

    return point


def cal_point_v2(inputs: Tuple[List[str], List[str]]) -> int:
    oppo_inputs, user_inputs = inputs

    input_to_point = {"A": 1, "B": 2, "C": 3, "X": 1, "Y": 2, "Z": 3}

    point = 0
    for i in range(len(oppo_inputs)):
        oppo_value = input_to_point[oppo_inputs[i]]
        user_input = user_inputs[i]

        if user_input == "X":
            # Need to lose condition
            # Add lose point
            point += 0
            # Add input to lose point
            point += 3 if oppo_value == 1 else oppo_value - 1
        elif user_input == "Y":
            # Need to draw condition
            # Add draw point + input to draw point
            point += 3 + oppo_value
        else:
            # Need to win condition
            # Add win point
            point += 6
            # Add input to win point
            point += 3 if oppo_value == 2 else (oppo_value + 1) % 3

    return point
