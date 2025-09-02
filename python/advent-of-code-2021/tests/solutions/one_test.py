from solutions.one.solution import count_increases


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
    result = count_increases(input)

    assert result == expected
