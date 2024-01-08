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

func SumCalibrationTextValue(inputs []string) int {
	regex, err := regexp.Compile(`one|two|three|four|five|six|seven|eight|nine|[0-9]`)
	if err != nil {
		panic("Invalid regex")
	}

	revRegex, err := regexp.Compile(`eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|[0-9]`)
	if err != nil {
		panic("Invalid reverse regex")
	}

	var sum int = 0
	for _, input := range inputs {
		var found bool = false
		firstMatch := string(regex.Find([]byte(input)))
		firstMatchValue, err := strconv.Atoi(firstMatch)
		if err != nil {
			if firstMatchValue, found = textNumberMap[firstMatch]; !found {
				panic("Invalid regex match. Please check your regex again")
			}
		}

		secondMatch := string(revRegex.Find([]byte(Reverse(input))))
		secondMatchValue, err := strconv.Atoi(secondMatch)
		if err != nil {
			if secondMatchValue, found = textNumberMap[Reverse(secondMatch)]; !found {
				panic("Invalid regex match. Please check your reverse regex again")
			}
		}
		sum += firstMatchValue*10 + secondMatchValue
	}

	return sum
}

func Reverse(s string) string {
	runes := []rune(s)
	for i, j := 0, len(s)-1; i < j; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}
	return string(runes)
}
