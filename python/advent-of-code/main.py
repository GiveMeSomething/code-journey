from day_one.one import read_item_from_file, max_item, top_three_item
from day_two.two import read_inputs_from_file, cal_points


def day_one():
    items = read_item_from_file()
    max_val = max_item(items)
    top_three = top_three_item(items)

    print(f"The max item is: {max_val}")
    print(f"The top three item sum is: {top_three}")


def day_two():
    inputs = read_inputs_from_file()
    points = cal_points(inputs)

    print(f"Final point after playing rocks, papers, scissors is: {points}")


# day_one()
day_two()
