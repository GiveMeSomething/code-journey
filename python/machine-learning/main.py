def gradient_descent(dataset_x: list[int], dataset_y: list[int], limit: int):
    theta_0 = 0
    theta_1 = 0
    learning_rate = 0.001

    theta_0_history = []
    theta_1_history = []

    for i in range(limit):
        theta_0 = theta_0 - learning_rate * sum(
            map(lambda x, y: (theta_0 + theta_1 * x - y) * x, dataset_x, dataset_y)
        )
        theta_1 = theta_1 - learning_rate * sum(
            map(lambda x, y: (theta_0 + theta_1 * x - y) * x, dataset_x, dataset_y)
        )
        theta_0_history.append(theta_0)
        theta_1_history.append(theta_1)
        print(f"iter {i}, theta_0 = {theta_0}, theta_1 = {theta_1}")


if __name__ == "__main__":
    dataset_x = [1, 2, 3, 4, 5, 6, 7, 8]
    dataset_y = [2, 3, 4, 5, 6, 7, 8, 9]
    gradient_descent(dataset_x, dataset_y, 50)
