package main

import (
	"aoc21/five"
	"aoc21/four"
	"aoc21/one"
	"aoc21/six"
	"aoc21/three"
	"aoc21/two"
	"fmt"
	"math"
)

func main() {
	// execOne()
	// execTwo()
	// execThree()
	// execFour()
	// execFive()
	execSix()
}

func execOne() {
	sweeps := one.ReadSweepFromFile()

	increase_sweep := one.CountIncreaseSweep(&sweeps)
	fmt.Printf("The number of increase sweep %d\n", increase_sweep)

	increase_sweep_range := one.CountIncreaseSweepRange(&sweeps)
	fmt.Printf("The number of increase sweep range %d\n", increase_sweep_range)
}

func execTwo() {
	commands := two.ReadCommandFromFile()

	horizontal, vertical := two.SimulateCommands(&commands)
	fmt.Printf("Multiple of horiontal position and vertical position is %d\n", horizontal*vertical)

	horizontalWithAim, verticalWithAim := two.SimulateCommandsWithAim(&commands)
	fmt.Printf("With aim: Multiple of horiontal position and vertical position is %d\n", horizontalWithAim*verticalWithAim)
}

func execThree() {
	bits := three.ReadBitsFromFile()

	powerConsumption := three.CalculatePowerConsumption(bits)
	fmt.Printf("Power consumption %d\n", powerConsumption)

	oxygenRating := three.CalculateOxygenRating(bits)
	co2Rating := three.CalculateCO2Rating(bits)
	fmt.Printf("Life rating %d\n", oxygenRating*co2Rating)
}

func execFour() {
	bingoNumbers, bingos := four.ReadBingoFromFile()

	// Part 1
	minWinStep := math.MaxInt
	point := 0
	for _, bingo := range bingos {
		bingoWinStep, bingoPoint := four.CheckBingo(bingoNumbers, bingo)
		if bingoWinStep < minWinStep {
			minWinStep = bingoWinStep
			point = bingoPoint
		}
	}
	fmt.Println("Fatest win at", minWinStep, "with point", point)

	// Part 2
	maxWinStep := 0
	for _, bingo := range bingos {
		bingoWinStep, bingoPoint := four.CheckBingo(bingoNumbers, bingo)
		if bingoWinStep > minWinStep {
			minWinStep = bingoWinStep
			point = bingoPoint
		}
	}
	fmt.Println("Slowest win at", maxWinStep, "with point", point)
}

func execFive() {
	ventLines := five.ReadVentFromFile()
	intersections := five.CountIntersections(&ventLines)
	fmt.Printf("Number of intersections %d\n", intersections)

	intersectionDiagonal := five.CountIntersectionsWithDiagonal(&ventLines)
	fmt.Printf("Number of intersections with diagonal %d\n", intersectionDiagonal)
}

func execSix() {
	intervals := six.ReadFishIntervalsFromFile()

	days := 2
	count := six.CountFish(&intervals, days)

	fmt.Printf("Fishes after %d day(s): %d\n", days, count)
}
