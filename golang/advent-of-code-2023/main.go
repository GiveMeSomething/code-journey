package main

import (
	"aoc23/one"
	"fmt"
)

func main() {
	exec_one()
}

func exec_one() {
	inputs := one.ReadInputFromFile()

	calibrationSum := one.SumCalibrationValue(inputs)
	textCalibrationSum := one.SumCalibrationTextValue(inputs)

	fmt.Printf("Calibration sum: %d\n", calibrationSum)
	fmt.Printf("Calibration sum with text value %d\n", textCalibrationSum)
}
