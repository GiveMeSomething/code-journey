package seven

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

func ReadCrabFromFile() ([]int, error) {
	file, err := os.Open("seven/seven.txt")
	if err != nil {
		return nil, fmt.Errorf("cannot find/open input file for day 7 with err %s", err)
	}

	reader := bufio.NewReader(file)
	line, _, err := reader.ReadLine()
	if err != nil {
		return nil, fmt.Errorf("cannot read input for day 7 with error %s", err)
	}

	result := make([]int, 0)
	for _, value := range strings.Split(string(line), ",") {
		if value == "" {
			continue
		}

		numberValue, err := strconv.Atoi(value)
		if err != nil {
			continue
		}

		result = append(result, numberValue)
	}

	return result, nil
}

func MinCrabMove(crabPositions []int) int {
	sort.Slice(crabPositions, func(i, j int) bool {
		return crabPositions[i] < crabPositions[j]
	})

	n := len(crabPositions)
	median := crabPositions[n/2]
	sum := 0

	for _, position := range crabPositions {
		sum += abs(position - median)
	}

	return sum
}

func MinCrabExtraMove(crabPositions []int) int {
	sum := 0
	for _, position := range crabPositions {
		sum += position
	}

	mean := float64(sum) / float64(len(crabPositions))
	ceilMean := int(math.Ceil(mean))
	floorMean := int(math.Floor(mean))
	result := 0
	for _, position := range crabPositions {
		result += extraMove(abs(position - ceilMean))
	}

	sum = 0
	for _, position := range crabPositions {
		sum += extraMove(abs(position - floorMean))
	}

	if sum < result {
		result = sum
	}

	return result
}

func abs(input int) int {
	if input < 0 {
		return -input
	}
	return input
}

func extraMove(input int) int {
	return input * (input + 1) / 2
}
