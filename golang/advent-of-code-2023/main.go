package main

import (
	"aoc23/one"
	"aoc23/two"
	"fmt"
)

func main() {
	// exec_one()
	exec_two()
}

func exec_one() {
	inputs := one.ReadInputFromFile()

	calibrationSum := one.SumCalibrationValue(inputs)
	textCalibrationSum := one.SumCalibrationTextValue(inputs)

	fmt.Printf("Calibration sum: %d\n", calibrationSum)
	fmt.Printf("Calibration sum with text value %d\n", textCalibrationSum)
}

func exec_two() {
	games := two.ReadGameFromFile()

	gameIdSum := two.SumPossibleGameId(games)
	minPossibleGame := two.MinimumPossibleGame(games)

	fmt.Printf("Sum of all possible game id is %d\n", gameIdSum)
	fmt.Printf("Product of minimum possible game is %d\n", minPossibleGame)
}