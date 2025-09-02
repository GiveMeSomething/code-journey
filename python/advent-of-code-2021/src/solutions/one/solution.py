def read_depth_log() -> list[int]:
    result = []
    input_path = "src/solutions/one/input.txt"
    with open(input_path, "r") as input_file:
        for depth_log in input_file:
