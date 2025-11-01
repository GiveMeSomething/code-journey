from linear_regression.linear_regression import train, predict_with_model
from utils.read_csv import CustomDataset

TRAINING_DATA_PATH = "linear_regression/training.csv"
TESTING_DATA_PATH = "linear_regression/testing.csv"


def train_model(dataset: CustomDataset):
    model_theta_vector = train(dataset.x_matrix, dataset.y_vector)

    print(f"Training model: {model_theta_vector}")

    return model_theta_vector


def read_data():
    training_dataset = CustomDataset(TRAINING_DATA_PATH)
    print(
        f"Training dataset: x_matrix = {training_dataset.x_matrix} y_vector = {training_dataset.y_vector}"
    )

    testing_dataset = CustomDataset(TESTING_DATA_PATH)
    print(
        f"Testing dataset: x_matrix = {testing_dataset.x_matrix} y_vector = {testing_dataset.y_vector}"
    )
    return (training_dataset, testing_dataset)


def test_model(model, dataset: CustomDataset, denormalize_params):
    prediction = list(
        map(lambda x_vector: predict_with_model(model, x_vector), dataset.x_matrix)
    )
    prediction = list(
        map(lambda i: dataset.denormalize_y(i, denormalize_params), prediction)
    )
    print(f"Prediction = {prediction}")

    y_vector = list(
        map(lambda i: dataset.denormalize_y(i, denormalize_params), dataset.y_vector)
    )
    print(f"Target = {y_vector}")

    mse = (
        sum(
            map(
                lambda hypothesis, target: abs(hypothesis - target) ** 2,
                prediction,
                y_vector,
            )
        )
        / dataset.x_matrix.shape[0]
    )

    print(f"Model mean square error: {mse}")


def start_linear_regression_driver():
    training_dataset, testing_dataset = read_data()
    # training_dataset.normalize_x()
    # training_dataset.normalize_y()

    model = train_model(training_dataset)

    # testing_dataset.normalize_x()
    # testing_dataset.normalize_y()
    test_model(model, testing_dataset, training_dataset.denormalize_params_y)
