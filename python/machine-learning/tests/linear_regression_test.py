import numpy as np
from linear_regression.cost_function import cost_function


def test_cost_function():
    x_matrix = np.array([[1, 2, 3], [1, 5, 6]])

    # Target f(x) = 2 * x0 + 2 * x1 + 2 * x2
    # 2 * 1 + 2 * 2 + 2 * 3 = 12
    # 2 * 1 + 2 * 5 + 2 * 6 = 24
    y_vector = np.array([12, 24])

    # h(x) = 1 * x0 + 1 * x1 + 1 * x2 = 1 + x1 + x2
    theta_vector = np.array([1, 1, 1])

    # Cost Function = J(θ) = 1/2 * sum((y - h(x))^2)
    # 1/2 * (12 - 6) ^ 2 = 18
    # 1/2 * (24 - 12) ^ 2 = 72
    expected_cost = 90

    result_cost = cost_function(x_matrix, y_vector, theta_vector)
    assert expected_cost == result_cost
