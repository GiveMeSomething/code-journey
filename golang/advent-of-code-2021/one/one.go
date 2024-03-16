package one

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func ReadSweepFromFile() []int {
	file, err := os.Open("one/one.txt")
	if err != nil {
		panic("Cannot find/open input file for day 1")
	}

	reader := bufio.NewReader(file)
	result := make([]int, 0)
	for {
		curentLine, _, err := reader.ReadLine()
		if err != nil {
			break
		}

		lineValue, err := strconv.Atoi(string(curentLine))
		if err != nil {
			fmt.Println("Cannot read current line as number")
			continue
		}

		result = append(result, lineValue)
	}

	return result
}

func CountIncreaseSweep(sweeps *[]int) int {
	count := 0
	previousSweep := (*sweeps)[0]

	for i, sweep := range *sweeps {
		if i == 0 {
			continue
		}

		if sweep > previousSweep {
			count++
		}
		previousSweep = sweep
	}
	return count
}

func CountIncreaseSweepRange(sweeps *[]int) int {
	count := 0
	previousSweepRange := (*sweeps)[0] + (*sweeps)[1] + (*sweeps)[2]

	for i := 1; i < len(*sweeps)-2; i++ {
		if (*sweeps)[i-1] < (*sweeps)[i+2] {
			count++
		}

		previousSweepRange = previousSweepRange - (*sweeps)[i-1] + (*sweeps)[i+2]
	}

	return count
}
