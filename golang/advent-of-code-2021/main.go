package main

import (
	"aoc21/four"
	"aoc21/one"
	"aoc21/three"
	"aoc21/two"
	"fmt"
)

func main() {
	// execOne()
	// execTwo()
	// execThree()
	execFour()
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
	fmt.Println(bingoNumbers)
	fmt.Printf("%+v\n", bingos)
}
