package day4

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func ReadAssignmentFromFile() []string {
	file, err := os.Open("day4/assignment.txt")
	if err != nil {
		fmt.Println("Cannot find/open assignment.txt")
	}

	reader := bufio.NewReader(file)
	var result = []string{}

	for {
		line, _, err := reader.ReadLine()
		if err != nil {
			break
		}

		result = append(result, string(line))
	}
	return result
}

func CounOverlapPair(inputs []string) int32 {
	var counter int32 = 0
	for _, input := range inputs {
		var extremePoints = []int32{}

		for _, pointRange := range strings.Split(input, ",") {
			for _, point := range strings.Split(pointRange, "-") {
				value, err := strconv.Atoi(point)
				if err != nil {
					panic(err)
				}
				extremePoints = append(extremePoints, int32(value))
			}
		}

		if isFullOverlap(
			extremePoints[0],
			extremePoints[1],
			extremePoints[2],
			extremePoints[3],
		) {
			counter++
		}
	}

	return counter
}

func CountPartialOverlapPair(inputs []string) int32 {
	var counter int32 = 0
	for _, input := range inputs {
		var extremePoints = []int32{}

		for _, pointRange := range strings.Split(input, ",") {
			for _, point := range strings.Split(pointRange, "-") {
				value, err := strconv.Atoi(point)
				if err != nil {
					panic(err)
				}
				extremePoints = append(extremePoints, int32(value))
			}
		}

		if isPartialOverlap(
			extremePoints[0],
			extremePoints[1],
			extremePoints[2],
			extremePoints[3],
		) {
			counter++
		}
	}

	return counter
}

func isFullOverlap(start1, end1, start2, end2 int32) bool {
	return (start1 <= start2 && end1 >= end2) ||
		(start2 <= start1 && end2 >= end1)
}

func isPartialOverlap(start1, end1, start2, end2 int32) bool {
	return (start1 <= start2 && end1 >= start2) ||
		(start1 <= end2 && end1 >= end2) ||
		(start2 <= start1 && end2 >= start1) ||
		(start2 <= end1 && end2 >= end1)
}
