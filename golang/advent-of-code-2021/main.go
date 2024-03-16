package main

import (
	"aoc21/one"
	"fmt"
)

func main() {
	execOne()
}

func execOne() {
	sweeps := one.ReadSweepFromFile()

	increase_sweep := one.CountIncreaseSweep(&sweeps)
	fmt.Printf("The number of increase sweep %d\n", increase_sweep)

	increase_sweep_range := one.CountIncreaseSweepRange(&sweeps)
	fmt.Printf("The number of increase sweep range %d\n", increase_sweep_range)
}
