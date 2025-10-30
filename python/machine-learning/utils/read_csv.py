import pandas as pd


def open_training_dataset(input_path):
    return pd.read_csv(input_path)


class Dataset:
    x_matrix = []
    y_vector = []

    def __init__(self, input_path):
        data = open_training_dataset(input_path)
        cols = data.shape()[1]

        # Assume that there're n columns in the dataset
        # First n-1 columns will be training features
        # Last column (n-th) will be training targets
        self.x_matrix = data.loc[0 : cols - 2].to_numpy()
        self.y_vector = data.loc[cols - 1].to_numpy()
