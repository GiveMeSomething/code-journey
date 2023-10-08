from typing import List, Tuple


def read_rucksack_from_file() -> List[Tuple[str, str]]:
    file = open("day_three/rucksacks.txt", "r", encoding="utf-8")

    rucksacks = []
    for line in file.readlines():
        divider = int(len(line) / 2)
        first_compartment = line[:divider]
        second_compartment = line[divider:]
        rucksacks.append((first_compartment, second_compartment))

    return rucksacks


def calculate_duplicate_char(rucksacks: List[Tuple[str, str]]) -> int:
    sum = 0
    for rucksack in rucksacks:
        first_compartment, second_compartment = rucksack
        first_char_map = {}
        second_char_map = {}

        print(first_compartment)
        print(second_compartment)

        dup_char = ""

        for i in range(len(first_compartment)):
            if second_char_map.get(first_compartment[i]) == None:
                first_char_map[first_compartment[i]] = False
            else:
                dup_char = first_compartment[i]
                break

            if first_char_map.get(second_compartment[i]) == None:
                second_char_map[second_compartment[i]] = False
            else:
                dup_char = second_compartment[i]
                break

        print(f"dup char {dup_char} value {char_value(dup_char)}")
        sum += char_value(dup_char)

    return sum


def calculate_duplicate_lines(rucksacks: List[Tuple[str, str]]) -> int:
    sum = 0
    for i in range(0, len(rucksacks), 3):
        char_map = {}
        found = False
        for j in range(3):
            currentRucksack = (rucksacks[i + j][0] + rucksacks[i + j][1]).strip()
            print(currentRucksack)
            for char in currentRucksack:
                # Only add new character if it's the first line in three line
                if char_map.get(char) == None and j == 0:
                    char_map[char] = 1
                # The list of possible characters will be reduced after iterated
                # Only check the char that is duplicate from the last iteration
                elif char_map.get(char) != None and char_map[char] == j:
                    char_map[char] = j + 1

                if char_map.get(char) == 3:
                    sum += char_value(char)
                    print(char)
                    found = True
                    break

            if found:
                break

        print("\n")

    return sum


def char_value(input: str) -> int:
    char_code = ord(input)

    # Lower case letter
    if char_code >= 97:
        return char_code - 96

    # Upper case letter
    return char_code - 64 + 26
