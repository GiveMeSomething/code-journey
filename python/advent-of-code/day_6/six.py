def read_message_from_file():
    file = open("day_6/buffer.txt", "r", encoding="utf-8")

    message = file.readline()
    return message


def get_marker_pos(message: str) -> int:
    char_set = set()
    for i in range(3, len(message)):
        char_set.clear()
        for j in range(i - 3, i + 1):
            char_set.add(message[j])
        if len(char_set) == 4:
            return i + 1

    return 0
