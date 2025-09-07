from solutions.three.three import bit_arr_to_epsilon, bit_arr_to_gamma, calculate_bits


INPUTS = [
    "00100",
    "11110",
    "10110",
    "10111",
    "10101",
    "01111",
    "00111",
    "11100",
    "10000",
    "11001",
    "00010",
    "01010",
]


def test_calculate_bits():
    expecteds = [1, 0, 1, 1, 0]
    result = calculate_bits(INPUTS)

    for i, expected in enumerate(expecteds):
        assert (result[i] > 0) if expected == 1 else (result[i] < 0)


def test_calculate_gamma():
    expected = 22

    bits = calculate_bits(INPUTS)
    gamma = bit_arr_to_gamma(bits)

    assert gamma == expected


def test_calcualte_epsilon():
    expected = 9

    bits = calculate_bits(INPUTS)
    epsilon = bit_arr_to_epsilon(bits)

    assert epsilon == expected
