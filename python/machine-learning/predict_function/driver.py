import numpy as np
import pandas as pd

from linear_regression.linear_regression import predict_with_model, train

LEARNING_RATE = 5 * 10**-6
STOP_THRESHOLD = 1 * 10**-6

TRAINING_DATA_PATH = "predict_function/function_train.csv"
TESTING_DATA_PATH = "predict_function/function_test.csv"


def prepare_data(filepath):
    df = pd.read_csv(filepath)

    print(f"Data from {filepath}")
    print(df.head())

    # Adding x0 column with x0 = 1
    df["x0"] = 1
    print("Data with x0")
    print(df.head())

    # Convert to features matrix with custom order
    feature_set = df[["x0", "x1", "x2", "x3"]].to_numpy()

    # Convert to target set
    target_set = df["y"].to_numpy()

    return (feature_set, target_set)


def train_model():
    feature_set, target_set = prepare_data(TRAINING_DATA_PATH)
    model = train(
        x_matrix=feature_set,
        y_vector=target_set,
        learning_rate=LEARNING_RATE,
        stop_threshold=STOP_THRESHOLD,
    )
    print(f"Result model {model}")


def test_model(model):
    feature_set, target_set = prepare_data(TESTING_DATA_PATH)

    squared_err = sum(
        map(
            lambda feature_vector, target: (
                predict_with_model(model, feature_vector) - target
            )
            ** 2,
            feature_set,
            target_set,
        )
    )
    mean = squared_err / feature_set.shape[0]
    print("Mean square error (MSE). Smaller is better. Nearer to 0 is better")
    print(mean)


def start_predict_function():
    model = np.array([99.19445803, 1.0053856, 5.02496687, 10.0407719])
    # model = train_model()
    test_model(model)
