def read_depth_input():
    result = []

    input_path = "./src/one/one.txt"

    try:
        with open(input_path, "r") as input_file:
            for line in input_file:
                result.append(int(line))
    except FileNotFoundError:
        print("[ONE] Input not found at %s: FileNotFound" % input_path)
    except ValueError as e:
        print("[ONE] Invalid input: %s" % e)
    except Exception as e:
        print("[ONE] Failed to open file at %s: %s" % (input_path, e))

    return result

def count_increases():
    increases = 0

    depths = read_depth_input()
    last = depths[0]

    for depth in depths[1:]:
        if depth - last > 0:
            increases += 1
        last = depth
    
    return increases

def count_increases_window():
    increases = 0

    depths = read_depth_input()
    last_window = depths[0] + depths[1] + depths[2]

    for i in range(3, len(depths)):
        current_window = last_window + depths[i] - depths[i - 3]
        if current_window > last_window:
            increases += 1
        last_window = current_window
    
    return increases