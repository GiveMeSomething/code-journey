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