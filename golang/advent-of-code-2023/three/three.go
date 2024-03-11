package three

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
)

type NumberRange struct {
	start int
	end   int
	value int
}

type Gear struct {
	row int
	col int
}

func ReadPartFromFile() []string {
	file, err := os.Open("three/three.txt")
	if err != nil {
		panic("Cannot find/open day 3 input file")
	}

	reader := bufio.NewReader(file)

	numberMap := make(map[int][]NumberRange)

	row := -1
	for {
		row += 1

		currentLine, _, err := reader.ReadLine()
		if err != nil {
			break
		}

		var currentNumber = 0

		col := -1
		for _, character := range currentLine {
			col += 1

			if rune(character) == '.' {
				if currentNumber > 0 {
					numberMap[row] = append(numberMap[row], NumberRange{
						start: col - int(math.Log10(float64(currentNumber))),
						end:   col,
						value: currentNumber,
					})
				}
				currentNumber = 0
				continue
			}

			charValue, err := strconv.Atoi(string(character))
			if err != nil {
				// Character is a special character
				if currentNumber > 0 {
					numberMap[row] = append(numberMap[row], NumberRange{
						start: col - int(math.Log10(float64(currentNumber))),
						end:   col,
						value: currentNumber,
					})
				}
				currentNumber = 0
				continue
			}

			// Character is a number
			currentNumber = currentNumber*10 + charValue
		}
	}

	fmt.Println(numberMap)

	return []string{}
}
