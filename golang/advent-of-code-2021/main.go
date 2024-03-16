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

	fmt.Println(sweeps)
}
