from day_one.one import read_item_from_file, max_item


def day_one():
    items = read_item_from_file()
    max_val = max_item(items)

    print(f"The max item is: {max_val}")


day_one()
