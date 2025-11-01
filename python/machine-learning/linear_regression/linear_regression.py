import numpy as np
from numpy.typing import NDArray

# LEARNING RATE
ALPHA = 1 * 10**-3

# To stop when the change between 2 iterations is insignificant
THRESHOLD = 0.01


def prepend_x0(x_vector):
    return np.concatenate((np.array([1]), x_vector))


def train(x_matrix: NDArray[np.int32], y_vector: NDArray[np.int32], iter_limit=-1):
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

        # We break the cycle when the Mean Square Error (MSE) is below a certain threshold
        mse = (
            sum(
                map(
                    lambda x_vector, y: (y - x_vector @ new_theta_vector) ** 2,
                    x_matrix,
                    y_vector,
                )
            )
            / x_matrix.shape[0]
        )
        if mse < THRESHOLD:
            break

        if iter_limit > 0 and iter == iter_limit:
            break

        # Update theta, start new iteration
        theta_vector = new_theta_vector.copy()

    return theta_vector


def predict_with_model(theta_vector, x_vector):
    return theta_vector @ x_vector
