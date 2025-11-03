import pandas as pd

from linear_regression.linear_regression import train

LEARNING_RATE = 5 * 10**-6
STOP_THRESHOLD = 1 * 10**-6

TRAINING_DATA_PATH = "predict_function/function_train.csv"
TESTING_DATA_PATH = "predict_function/function_test.csv"


def prepare_training_data():
    df = pd.read_csv(TRAINING_DATA_PATH)

    print(f"Data from {TRAINING_DATA_PATH}")
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
    feature_set, target_set = prepare_training_data()
    model = train(
        x_matrix=feature_set,
        y_vector=target_set,
        learning_rate=LEARNING_RATE,
        stop_threshold=STOP_THRESHOLD,
    )
    print(f"Result model {model}")


def start_predict_function():
    model = train_model()
