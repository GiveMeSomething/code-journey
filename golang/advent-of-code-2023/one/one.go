package one

import (
	"bufio"
	"os"
	"regexp"
	"strconv"
)

var textNumberMap = map[string]int{
	"one":   1,
	"two":   2,
	"three": 3,
	"four":  4,
	"five":  5,
	"six":   6,
	"seven": 7,
	"eight": 8,
	"nine":  9,
}

func ReadInputFromFile() []string {
	file, err := os.Open("one/one.txt")

	if err != nil {
		panic(err)
	}

	var result []string
	reader := bufio.NewReader(file)
	for {
		currentLine, _, err := reader.ReadLine()
		if err != nil {
			break
		}

		result = append(result, string(currentLine))
	}

	return result
}

func SumCalibrationValue(inputs []string) int {
	regex, err := regexp.Compile(`[0-9]`)
	if err != nil {
		panic("Invalid regular expression")
	}

	var sum int = 0

	for _, input := range inputs {
		matched := regex.FindAllString(input, -1)
		value, err := strconv.Atoi(matched[0] + matched[len(matched)-1])
		if err != nil {
			panic(err)
		}
		sum += value
	}

	return sum
}
