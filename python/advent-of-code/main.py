from day_1.one import read_item_from_file, max_item, top_three_item
from day_2.two import read_inputs_from_file, cal_points, cal_point_v2
from day_3.three import (
    read_rucksack_from_file,
    calculate_duplicate_char,
    calculate_duplicate_lines,
)
from day_4.four import (
    read_pairs_from_file,
    count_overlap_pairs,
    count_true_overlap_pairs,
)
from day_5.five import read_cargo_from_file
from day_6.six import read_message_from_file, get_marker_pos, get_message_pos


def day_one():
    items = read_item_from_file()
    max_val = max_item(items)
    top_three = top_three_item(items)

    print(f"The max item is: {max_val}")
    print(f"The top three item sum is: {top_three}\n")


def day_two():
    inputs = read_inputs_from_file()
    points = cal_points(inputs)
    points_v2 = cal_point_v2(inputs)
    print(f"Final point after playing rocks, papers, scissors is: {points}")
    print(f"Final point after changing strategy is: {points_v2}\n")


def day_three():
    rucksacks = read_rucksack_from_file()
    priority_sum = calculate_duplicate_char(rucksacks)
    badge_sum = calculate_duplicate_lines(rucksacks)

    print(f"Sum of priorities of all rucksacks is: {priority_sum}")
    print(f"Sum of badge sum of all rucksacks is: {badge_sum}\n")


def day_four():
    pairs = read_pairs_from_file()
    overlapped_pairs = count_overlap_pairs(pairs)
    true_overlapped_pairs = count_true_overlap_pairs(pairs)
    print(f"Number of totally overlapped pairs are: {overlapped_pairs}")
    print(f"Number of partially overlapped pairs are: {true_overlapped_pairs}\n")


def day_five():
    last_cargos = read_cargo_from_file()
    print(last_cargos)


def day_six():
    message = read_message_from_file()

    marker_start = get_marker_pos(message)
    print(marker_start)

    message_start = get_message_pos(message)
    print(message_start)


# day_one()
# day_two()
# day_three()
# day_four()
# day_five()
day_six()
