package six

import (
	"bufio"
	"os"
	"strconv"
	"strings"
)

func ReadFishIntervalsFromFile() []int {
	file, err := os.Open("six/six.txt")
	if err != nil {
		panic("Cannot find/open input file for day 6")
	}

	reader := bufio.NewReader(file)
	result := make([]int, 0)
	for {
		line, _, err := reader.ReadLine()
		if err != nil {
			break
		}

		for _, part := range strings.Split(string(line), ",") {
			value, err := strconv.Atoi(part)
			if err != nil {
				continue
			}
			result = append(result, value)
		}
	}

	return result
}

func CountFish(intervals *[]int, remainingDays int) int {
	sum := 0
	for _, interval := range *intervals {
		sum += countFishRecur(interval, remainingDays)
	}

	return sum
}

func countFishRecur(interval int, remainingDays int) int {
	sum := 1

	if remainingDays < interval+1 {
		return 1
	}

	n := (remainingDays-interval-1)/7 + 1
	for i := 0; i < n; i++ {
		sum += countFishRecur(8, remainingDays-interval-1-i*7)
	}

	return sum
}
