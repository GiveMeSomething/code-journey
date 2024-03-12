package three

import (
	"bufio"
	"math"
	"os"
	"strconv"
)

type NumberRange struct {
	start   int
	end     int
	value   int
	checked bool
}

type Gear struct {
	row int
	col int
}

func ReadPartFromFile() (map[int][]*NumberRange, []Gear) {
	file, err := os.Open("three/three.txt")
	if err != nil {
		panic("Cannot find/open day 3 input file")
	}

	reader := bufio.NewReader(file)

	numberMap := make(map[int][]*NumberRange)
	gears := make([]Gear, 0)

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
					numberMap[row] = append(numberMap[row], &NumberRange{
						start:   col - int(math.Log10(float64(currentNumber))) - 1,
						end:     col,
						value:   currentNumber,
						checked: false,
					})
				}
				currentNumber = 0
				continue
			}

			charValue, err := strconv.Atoi(string(character))
			if err != nil {
				// Character is a special character
				if currentNumber > 0 {
					numberMap[row] = append(numberMap[row], &NumberRange{
						start:   col - int(math.Log10(float64(currentNumber))) - 1,
						end:     col,
						value:   currentNumber,
						checked: false,
					})
				}

				gears = append(gears, Gear{
					col: col,
					row: row,
				})
				currentNumber = 0
				continue
			}

			// Character is a number
			currentNumber = currentNumber*10 + charValue
		}
	}

	return numberMap, gears
}

func CalculateGearSum(numbers map[int][]*NumberRange, gears *[]Gear) int {
	var sum = 0

	// Iterate through all detected gears
	for _, gear := range *gears {
		// Iterate through all possible row arround a single gear
		for i := gear.row - 1; i <= gear.row+1; i++ {
			if i < 0 {
				continue
			}

			// Only continue if the row have some numbers
			if ranges, found := numbers[i]; found {
				// Iterate through all possible col around a single gear
				for j := gear.col - 1; j <= gear.col+1; j++ {
					if j < 0 {
						continue
					}

					// Check the current col with all matched ranges
					for _, numberRange := range ranges {
						if numberRange.checked {
							continue
						}

						if j >= numberRange.start && j <= numberRange.end {
							sum += numberRange.value
							numberRange.checked = true
						}
					}
				}
			}
		}
	}

	return sum
}
