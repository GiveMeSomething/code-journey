import numpy as np
from numpy.typing import NDArray


# Cost Function = J(θ) = 1/2 * sum((y - h(x))^2)
# To minimize cost function, we need J(θ) to approach 0
def cost_function(
    x_matrix: NDArray[np.int32],
    y_vector: NDArray[np.int32],
    theta_vector: NDArray[np.int32],
) -> float:
    # Just happen that dot product of x and theta vector = x_vector * theta vector tranposed
    square_err = sum(
        map(
            lambda x_vector, y: (y - x_vector @ theta_vector) ** 2,
            x_matrix,
            y_vector,
        )
    )
    return 1 / 2 * square_err
