from utils.util_number import try_atoi


def test_try_atoi():
    inputs = ["1", "1.0", "a", "A", "1e"]
    expecteds = [1, 0, 0, 0, 0]

    for i, input  in enumerate(inputs):
        expected = expecteds[i]
        result, err = try_atoi(input)
        assert result == expected
        if expected == 0:
            assert err is not None