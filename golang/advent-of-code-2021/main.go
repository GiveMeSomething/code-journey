package main

import (
	"aoc21/one"
	"aoc21/two"
	"fmt"
)

func main() {
	// execOne()
	execTwo()
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
