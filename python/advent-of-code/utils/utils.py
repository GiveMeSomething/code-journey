from typing import List


def read_data_from_file(root_relative_path: str, /) -> List[str]:
    try:
        file = open(root_relative_path, "r", encoding="utf-8")
        return file.readlines()
    except OSError:
        print("Failed to load file from root path. Falling back to project path")

    try:
        file = open(
            f"python/advent-of-code/{root_relative_path}", "r", encoding="utf-8"
        )
        return file.readlines()
    except OSError:
        print("Failed to load file from project root path. Abort")
        raise OSError()
