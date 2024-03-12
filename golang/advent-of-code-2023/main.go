package main

import (
	"aoc23/one"
	"aoc23/three"
	"aoc23/two"
	"fmt"
)

func main() {
	// exec_one()
	// exec_two()

	exec_three()
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

func exec_three() {
	// Part 1
	numberRanges, parts := three.ReadPartFromFile()
	gearSum := three.CalculateGearSum(numberRanges, &parts)
	fmt.Printf("Sum of all gear in the matrix is %d\n", gearSum)

	// Part 2
	numberRanges, gears := three.ReadGearFromFile()
	trueGearSum := three.CalculatTrueGearSum(numberRanges, &gears)
	fmt.Printf("Sum of all true gear in the matrix is %d\n", trueGearSum)
}
