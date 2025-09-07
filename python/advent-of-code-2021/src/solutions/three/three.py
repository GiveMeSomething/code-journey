from typing import List


def read_diagnostic_report() -> List[str]:
    result = []

    input_path = "src/solutions/three/input.txt"
    with open(input_path, "r") as input_file:
        for line in input_file:
            result.append(line.strip())
    return result


def calculate_bits(inputs: List[str]) -> List[int]:
    bit_map = []
    for i in range(0, len(inputs[0])):
        bit_map.append(0)

    for input in inputs:
        for i, bit in enumerate(input):
            if bit == "1":
                bit_map[i] += 1
            if bit == "0":
                bit_map[i] -= 1
    return bit_map


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


def calculate_oxygen_rating(inputs: List[str]):
    current_list = inputs[:]
    current_index = 0
    while len(current_list) > 1:
        bits = calculate_bits(current_list)
        selected_bit = "1" if bits[current_index] >= 0 else "0"
        current_list = [
            selected
            for selected in current_list
            if selected[current_index] == selected_bit
        ]
        current_index += 1
    return int(current_list[0], 2)


def calculate_co2_rating(inputs: List[str]):
    current_list = inputs[:]
    current_index = 0
    while len(current_list) > 1:
        bits = calculate_bits(current_list)
        selected_bit = "0" if bits[current_index] >= 0 else "1"
        current_list = [
            selected
            for selected in current_list
            if selected[current_index] == selected_bit
        ]
        current_index += 1
    return int(current_list[0], 2)


def run_part_1():
    inputs = read_diagnostic_report()
    bits = calculate_bits(inputs)

    gamma = bit_arr_to_gamma(bits)
    epsilon = bit_arr_to_epsilon(bits)

    result = gamma * epsilon
    print(f"Day 3 Part 1: {result}")


def run_part_2():
    inputs = read_diagnostic_report()

    oxygen_rating = calculate_oxygen_rating(inputs)
    co2_rating = calculate_co2_rating(inputs)

    life_support_rating = oxygen_rating * co2_rating
    print(f"Day 3 Part 2: {life_support_rating}")
