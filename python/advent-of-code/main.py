from day_one.one import read_item_from_file, max_item, top_three_item


def day_one():
    items = read_item_from_file()
    max_val = max_item(items)
    top_three = top_three_item(items)

    print(f"The max item is: {max_val}")
    print(f"The top three item sum is: {top_three}")


day_one()
