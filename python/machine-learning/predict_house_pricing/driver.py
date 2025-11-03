import numpy as np
import pandas as pd

from linear_regression.linear_regression import predict_with_model, train

LEARNING_RATE = 1 * 10**-10
STOP_THRESHOLD = 1 * 10**-4

TRAINING_DATA_PATH = "predict_house_pricing/house_pricing_train.csv"
TESTING_DATA_PATH = "predict_house_pricing/house_pricing_test.csv"


def prepare_data(filepath):
    df = pd.read_csv(filepath)

    print(f"Data from {filepath}")
    print(df.head())

    # Adding x0 column with x0 = 1
    df["x0"] = 1
    print("Data with x0")
    print(df.head())

    # Convert to features matrix with custom order
    feature_set = df[["x0", "area", "bedrooms", "bathrooms"]].to_numpy()

    # Convert to target set
    target_set = df["price"].to_numpy()

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


def start_predict_house_pricing():
    # model = train_model()
    model = np.array([11342.18997688, 1149.02664596, 29338.20368426, 15967.78216034])
    test_model(model)
