from src.solutions import one

if __name__ == "__main__":
    increases = one.count_increases()
    print("Day 1 Part 1: %d" % increases)

    increase_windows = one.count_increases_window()
    print("Day 1 Part 2: %d" % increase_windows)
    