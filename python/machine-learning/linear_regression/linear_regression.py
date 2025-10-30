import numpy as np
from numpy.typing import NDArray

# LEARNING RATE
ALPHA = 0.01

# To stop when the change between 2 iterations is insignificant
DIFF_THRESHOLD = 0.00001


def prepend_x0(x_vector):
    return np.concatenate((np.array([1]), x_vector))


def linear_regression(x_matrix: NDArray[np.int32], y_vector: NDArray[np.int32]):
    features_count = x_matrix.shape[1]

    # Init all the theta to 0 at first
    theta_vector = np.array([0] * features_count, np.float64)

    iter = 0
    while True:
        # for a in range(5):
        iter += 1
        new_theta_vector = np.array(theta_vector, np.float64)

        for i in range(features_count):
            new_theta_vector[i] = new_theta_vector[i] - ALPHA * sum(
                map(
                    lambda y, x_vector: (x_vector @ new_theta_vector - y) * x_vector[i],
                    y_vector,
                    x_matrix,
                )
            )

        print(iter, new_theta_vector)
        # We break the cycle when the different accross all theta is below DIFF_THRESHOLD
        if (
            np.array(list(map(lambda x: abs(x), new_theta_vector - theta_vector)))
            < DIFF_THRESHOLD
        ).all():
            theta_vector = new_theta_vector
            break

        # Update theta, start new iteration
        theta_vector = new_theta_vector.copy()

    return theta_vector
