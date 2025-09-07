from typing import List


def read_diagnostic_report() -> List[str]:
    result = []

    input_path = "src/solutions/three"
    with open(input_path, "r") as input_file:
        for line in input_file:
            result.append(line)
    return result


def calculate_bits(inputs: List[str]) -> List[int]:
    bit_map = []
    for i in range(0, len(inputs[0])):
        bit_map.append(0)

    for input in inputs:
        for i, bit in enumerate(input):
            bit_map[i] += 1 if bit == "1" else -1

    return bit_map


def bit_arr_to_i(inputs: List[int]) -> int:
    string_value = "".join(inputs)
    return int(string_value, 2)


def bit_arr_to_gamma(inputs: List[int]) -> int:
    result = ""
    for input in inputs:
        result += "1" if input > 0 else "0"
    return int(result, 2)


def bit_arr_to_epsilon(inputs: List[int]) -> int:
    result = ""
    for input in inputs:
        result += "0" if input > 0 else "1"

    return int(result, 2)


def run_part_1():
    inputs = read_diagnostic_report()
    bits = calculate_bits(inputs)

    gamma = bit_arr_to_gamma(bits)
    epsilon = bit_arr_to_epsilon(bits)

    result = gamma * epsilon
    print(f"Day 3 Part 1: {result}")
