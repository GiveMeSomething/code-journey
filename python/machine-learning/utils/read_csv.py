import numpy as np
import pandas as pd


def open_training_dataset(input_path):
    return pd.read_csv(input_path)


class CustomDataset:
    x_matrix = []
    y_vector = []

    def __init__(self, input_path):
        data = open_training_dataset(input_path)
        cols = data.shape[1]

        # Assume that there're n columns in the dataset
        # First n-1 columns will be training features
        # Last column (n-th) will be training targets

        # Normalize x_matrix using min/max
        self.x_matrix = data.iloc[:, 0 : cols - 1].to_numpy(dtype=np.float32)

        # for i in range(data.shape[1] - 1):
        #     current_col = data.iloc[:, i].to_numpy(dtype=np.float32)
        #     col_min = np.min(current_col)
        #     col_max = np.max(current_col)
        #     for j in range(len(current_col)):
        #         if col_max - col_min == 0:
        #             self.x_matrix[j][i] = 0
        #             continue
        #         self.x_matrix[j][i] = (self.x_matrix[j][i] - col_min) / (
        #             col_max - col_min
        #         )

        self.x_matrix = np.hstack(
            (np.ones((data.shape[0], 1), dtype=np.float32), self.x_matrix)
        )

        # Normalize y vector using min/max
        self.y_vector = data.iloc[:, -1].to_numpy()
        # y_min = np.min(self.y_vector)
        # y_max = np.max(self.y_vector)
        # for i in range(self.y_vector.shape[0]):
        #     if y_max - y_min == 0:
        #         self.y_vector[i] = 0
        #         continue
        #     self.y_vector[i] = (self.y_vector[i] - y_min) / (y_max - y_min)
