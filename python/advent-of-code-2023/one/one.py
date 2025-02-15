import os

from typing import Tuple


def calibration_value_digit():
    result = 0
    try:
        input_path = os.getcwd() + "/one/input.txt"
        with open(input_path) as file:
            for line in file:
                first_digit = _extract_first_digit(line)
                last_digit = _extract_last_digit(line)
                result += first_digit * 10 + last_digit
    except FileNotFoundError:
        print("input file missing")

    print(result)


def calibration_value_number_str():
    result = 0
    try:
        input_path = os.getcwd() + "/one/input.txt"
        with open(input_path) as file:
            for line in file:
                first_number, last_number = _extract_numbers(line)
                result += first_number * 10 + last_number
    except FileNotFoundError:
        print("input file missing")

    print(result)


def _extract_first_digit(line: str) -> int:
    for char in line:
        if char.isdigit():
            return int(char)
    return 0


def _extract_last_digit(line: str) -> int:
    return _extract_first_digit(line[::-1])


def _extract_numbers(line: str) -> Tuple[int, int]:
    number_str_dict = {
        "one": 1,
        "two": 2,
        "three": 3,
        "four": 4,
        "five": 5,
        "six": 6,
        "seven": 7,
        "eight": 8,
        "nine": 9,
        "ten": 10,
    }

    numbers = []

    for i in range(len(line)):
        if line[i].isdigit():
            numbers.append(int(line[i]))
            continue
        for key, value in number_str_dict.items():
            current_str = line[i : i + len(key)]
            if current_str == key:
                numbers.append(value)

    return numbers[0], numbers[len(numbers) - 1]
