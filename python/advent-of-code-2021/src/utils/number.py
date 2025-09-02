from typing import Optional, Tuple


def try_atoi(s: str) -> Tuple[int, Optional[str]]:
    try:
        return (int(s, 10), None)
    except Exception:
        return (0, f"Invalid integer: {s}")
