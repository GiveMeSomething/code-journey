import numpy as np
from linear_regression.linear_regression import train, predict_with_model
from utils.read_csv import CustomDataset

TRAINING_DATA_PATH = "linear_regression/training.csv"
TESTING_DATA_PATH = "linear_regression/testing.csv"


def train_model():
    dataset = CustomDataset(TRAINING_DATA_PATH)
    model_theta_vector = train(dataset.x_matrix, dataset.y_vector, 1_000_000)

    print(f"Training model: {model_theta_vector}")

    return model_theta_vector


def test_read_data():
    training_dataset = CustomDataset(TRAINING_DATA_PATH)
    print(
        f"Training dataset: x_matrix = {training_dataset.x_matrix} y_vector = {training_dataset.y_vector}"
    )

    testing_dataset = CustomDataset(TESTING_DATA_PATH)
    print(
        f"Testing dataset: x_matrix = {testing_dataset.x_matrix} y_vector = {testing_dataset.y_vector}"
    )


def test_model(model):
    dataset = CustomDataset(TESTING_DATA_PATH)

    prediction = list(
        map(lambda x_vector: predict_with_model(model, x_vector), dataset.x_matrix)
    )

    print(prediction)

    for i in range(dataset.x_matrix.shape[0]):
        accuracy = 1 - (abs(prediction[i] - dataset.y_vector[i]) / dataset.y_vector[i])
        print(
            f"Dataset {dataset.x_matrix[i]} Target {dataset.y_vector[i]} Prediction {prediction[i]} Accuracy {accuracy * 100}"
        )
    accuracy = (
        sum(
            map(
                lambda hypothesis, target: (1 - abs(hypothesis - target) / target)
                * 100,
                prediction,
                dataset.y_vector,
            )
        )
        / dataset.x_matrix.shape[0]
    )

    print(f"Model accuracy: {accuracy}%")


def start_linear_regression_driver():
    test_read_data()
    model = train_model()
    test_model(model)
