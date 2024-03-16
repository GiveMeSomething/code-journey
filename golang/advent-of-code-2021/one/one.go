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
