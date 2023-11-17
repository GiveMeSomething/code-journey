package day2

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

var pointMap map[string]int32 = map[string]int32{
	"A": 1,
	"X": 1,
	"B": 2,
	"Y": 2,
	"C": 3,
	"Z": 3,
}

func ReadEncryptFromFile() []string {
	var result []string = []string{}

	file, err := os.Open("day2/encrypt.txt")
	if err != nil {
		fmt.Println("Cannot find the required file")
		return result
	}

	reader := bufio.NewReader(file)

	for {
		currentLine, _, err := reader.ReadLine()
		if err != nil {
			fmt.Println("Cannot read current line. Possibly reach EOF")
			break
		}

		result = append(result, string(currentLine))
	}

	return result
}

func CalculatePoint(inputs []string) int32 {
	var sum int32
	for _, v := range inputs {
		input := strings.Split(v, " ")
		sum += calculatePointSingle(input[0], input[1])
	}

	return sum
}

func CalculatePointPart2(inputs []string) int32 {
	var sum int32
	for _, v := range inputs {
		input := strings.Split(v, " ")
		sum += calculatePointSingle2(input[0], input[1])
	}
	return sum
}

func calculatePointSingle(opponent string, you string) int32 {
	opponentPoint, ok := pointMap[opponent]
	if !ok {
		return 0
	}

	yourPoint, ok := pointMap[you]
	if !ok {
		return 0
	}

	if (opponentPoint == 3 && yourPoint == 1) || (opponentPoint == yourPoint-1) {
		return yourPoint + 6
	}

	if opponentPoint == yourPoint {
		return yourPoint + 3
	}

	return yourPoint
}

func calculatePointSingle2(opponent string, outcome string) int32 {
	opponentPoint, ok := pointMap[opponent]
	if !ok {
		return 0
	}

	if outcome == "Z" {
		if opponentPoint == 3 {
			return 6 + 1
		}

		return opponentPoint + 1 + 6
	} else if outcome == "Y" {
		return opponentPoint + 3
	} else {
		if opponentPoint == 1 {
			return 3
		}

		return opponentPoint - 1
	}
}
