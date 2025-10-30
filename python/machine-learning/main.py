import numpy as np

from linear_regression.linear_regression import linear_regression


def gradient_descent(x_matrix, y_vector):
    theta_vector = np.array([0, 0], np.float64)
    for i in range(10):
        theta_vector[0] = theta_vector[0] - 0.01 * sum(
            map(
                lambda y, x_vector: (x_vector @ theta_vector - y) * x_vector[0],
                y_vector,
                x_matrix,
            )
        )
        theta_vector[1] -= 0.01 * sum(
            map(
                lambda y, x_vector: (x_vector @ theta_vector - y) * x_vector[1],
                y_vector,
                x_matrix,
            )
        )
        print(f"i = {i}, theta_0 = {theta_vector[0]}, theta_1 = {theta_vector[1]}")


if __name__ == "__main__":
    x_matrix = np.array([[1, 1], [1, 2], [1, 3]])
    y_vector = np.array([2, 3, 4])

    result = linear_regression(x_matrix, y_vector)
    print(f"Final result: {result}")
