import os
from typing import List

def calibration_value_number():
  result = 0
  try:
    input_path = os.getcwd() + "/one/input.txt"
    with open(input_path) as file:
      for line in file:
        digits = _extract_digit(line)
        result += int(digits[0] + digits[len(digits) - 1])
  except FileNotFoundError:
    print("input file missing")

  print(result)


def _extract_digit(line: str) -> List[str]:
  return [digit for digit in line if digit.isdigit()]