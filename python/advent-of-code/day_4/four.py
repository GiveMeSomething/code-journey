from typing import List, Tuple


# Pair will be saved in the following DS List[Tuple(a, b)]
def read_pairs_from_file() -> List[Tuple[int, int]]:
    file = open("day_4/pairs.txt", "r", encoding="utf-8")

    pairs = []
    for line in file.readlines():
        line = line.strip()
        first_range, second_range = line.split(",")

        first_range_num = [int(value) for value in first_range.split("-")]
        second_range_num = [int(value) for value in second_range.split("-")]
        pairs.append(tuple(first_range_num))
        pairs.append(tuple(second_range_num))

    return pairs


def count_overlap_pairs(pairs: List[Tuple[int, int]]) -> int:
    count = 0
    for i in range(0, len(pairs), 2):
        first_pair = pairs[i]
        second_pair = pairs[i + 1]

        overlap = is_overlapped(first_pair, second_pair)

        if overlap:
            count += 1

    return count


def count_true_overlap_pairs(pairs: List[Tuple[int, int]]) -> int:
    count = 0
    for i in range(0, len(pairs), 2):
        first_pair = pairs[i]
        second_pair = pairs[i + 1]
        true_overlap = is_true_overlapped(first_pair, second_pair)
        if true_overlap:
            count += 1

    return count


# Two range are totally overlapped (cotain the other)
def is_overlapped(first: Tuple[int, int], second: Tuple[int, int]) -> bool:
    a, b = first
    c, d = second
    return (a <= c and b >= d) or (c <= a and d >= b)


# Two range are partially overlapped when one range contain the other start or end
def is_true_overlapped(first, second) -> bool:
    a, b = first
    c, d = second

    return (
        (a <= c and b >= c)
        or (c <= a and d >= b)
        or (a <= d and b >= d)
        or (c <= b and d >= b)
    )
