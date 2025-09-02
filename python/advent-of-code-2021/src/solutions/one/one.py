from utils.util_number import try_atoi


def read_depth_log() -> list[int]:
    result = []
    input_path = "src/solutions/one/input.txt"
    with open(input_path, "r") as input_file:
        for line in input_file:
            depth_log, err = try_atoi(line)
            if err is not None:
                raise ValueError(err)
            result.append(depth_log)
    return result


def count_increases(inputs: list[int]) -> int:
    increases = 0
    for i in range(0, len(inputs)-1):
        increases += 1 if inputs[i+1] - inputs[i] > 0 else 0
    return increases


def count_increase_windows(inputs: list[int]) -> int:
    increases = 0
    if len(inputs) <= 3:
        return 0

    for i in range(3, len(inputs)):
        increases += 1 if inputs[i] > inputs[i-3] else 0
    return increases


def run_part_1():
    depth_logs = read_depth_log()
    increases = count_increases(depth_logs)
    print(f"Day 1 Part 1: {increases}")


def run_part_2():
    depth_logs = read_depth_log()
    increases = count_increase_windows(depth_logs)
    print(f"Day 1 Part 2: {increases}")