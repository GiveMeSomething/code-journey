import numpy as np
import pandas as pd


def open_training_dataset(input_path):
    return pd.read_csv(input_path)


class CustomDataset:
    data = pd.DataFrame(None)
    x_matrix = []
    y_vector = []

    # Denormalize parameters for each columns
    # (min, max)[]
    denormalize_params_x = []
    denormalize_params_y = (0, 0)

    def __init__(self, input_path):
        data = open_training_dataset(input_path)
        cols = self.data.shape[1]

        # Normalize x_matrix using min/max
        x_matrix = data.iloc[:, 0 : cols - 1].to_numpy(dtype=np.float32)
        ones = np.full((data.shape[0], 1), 1, dtype=np.float32)
        x_matrix = np.hstack((ones, x_matrix))

        y_vector = data.iloc[:, -1].to_numpy()

        self.data = data
        self.x_matrix = x_matrix
        self.y_vector = y_vector

    def normalize_x(self):
        # Do normalize feature for all feature
        for i in range(self.x_matrix.shape[1]):
            self.denormalize_params_x.append((0, 1))
            # Skip x0 = 1 columns
            if i == 0:
                continue

            # Use min/max normalization
            current_col = self.data.iloc[:, i].to_numpy(dtype=np.float32)
            col_min = np.min(current_col)
            col_max = np.max(current_col)
            min_max_diff = col_max - col_min
            self.denormalize_params_x[i] = (col_min, col_max)
            for j in range(len(current_col)):
                if min_max_diff == 0:
                    self.x_matrix[j][i] = 1
                self.x_matrix[j][i] = (self.x_matrix[j][i] - col_min) / min_max_diff

    def normalize_y(self):
        y_min = np.min(self.y_vector)
        y_max = np.max(self.x_matrix)
        min_max_diff = y_max - y_min
        self.denormalize_params_y = (y_min, y_max)
        for i in range(len(self.y_vector)):
            self.y_vector[i] = (self.y_vector[i] - y_min) / min_max_diff

    # Denormalize a row of x values
    def denormalize_x(self, x_vector):
        result = []
        for i in range(len(x_vector)):
            min, max = self.denormalize_params_x[i]
            result[i] = x_vector[i] * (max - min) + min
        return result

    def denormalize_y(self, y):
        min, max = self.denormalize_params_y
        return y * (max - min) + min
