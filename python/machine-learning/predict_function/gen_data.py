import pandas as pd
import numpy as np

# Set a seed for reproducibility
np.random.seed(42)

# Define the number of examples
num_examples = 50

# --- Generate input variables (X) ---
# Generate random integers for x1, x2, and x3.
# The ranges are chosen arbitrarily for demonstration.

# x1: e.g., integers from 1 to 50
x1 = np.random.randint(1, 51, num_examples)

# x2: e.g., integers from 5 to 25
x2 = np.random.randint(5, 26, num_examples)

# x3: e.g., integers from 1 to 10
x3 = np.random.randint(1, 11, num_examples)

# --- Calculate the target variable (y) based on the function ---
# Function: y = 100 + x1 + 5 * x2 + 10 * x3
y = 100 + x1 + 5 * x2 + 10 * x3

# --- Create a DataFrame ---
data = {"x1": x1, "x2": x2, "x3": x3, "y": y}
df = pd.DataFrame(data)

# First num_examples - 10 is the train data
train_data = df.iloc[0 : num_examples - 10]

# Last 10 examples is the test data
test_data = df.iloc[-10:]

# --- Save to CSV file ---
train_file_name = "predict_function/function_train.csv"
train_data.to_csv(train_file_name, index=False)

print(
    f"✅ Successfully generated {num_examples - 10} examples and saved them to '{train_file_name}'."
)

test_file_name = "predict_function/function_test.csv"
test_data.to_csv(test_file_name, index=False)

print(f"✅ Successfully generated {10} examples and saved them to '{test_file_name}'.")
