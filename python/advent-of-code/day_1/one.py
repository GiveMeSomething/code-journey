def read_item_from_file():
    itemFile = open("day_one/items.txt", "r", encoding="utf-8")
    items = []

    sum = 0
    for item in itemFile.readlines():
        current_line = item.strip()

        # add as new item when the encounter empty line
        if current_line == "":
            items.append(sum)
            sum = 0
            continue

        # add to current sum
        sum += int(current_line)

    return items


def max_item(items: list) -> int:
    max_val = 0
    for item in items:
        max_val = max(max_val, item)

    return max_val


def top_three_item(items: list) -> int:
    # sort the items list in descending order
    items.sort(reverse=True)

    return items[0] + items[1] + items[2]
