from solutions.one import one


def test_count_increases():
    input = [
        199,
        200,
        208,
        210,
        200,
        207,
        240,
        269,
        260,
        263,
    ]
    expected = 7
    result = one.count_increases(input)

    assert result == expected

def test_count_increase_windows():
    input = [
        199,
        200,
        208,
        210,
        200,
        207,
        240,
        269,
        260,
        263,
    ]
    expected = 5
    result = one.count_increase_windows(input)

    assert result == expected