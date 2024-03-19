package three

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func ReadBitsFromFile() []string {
	file, err := os.Open("three/three.txt")
	if err != nil {
		panic("Cannot find/open input file for day 3")
	}

	reader := bufio.NewReader(file)
	result := make([]string, 0)
	for {
		currentLine, _, err := reader.ReadLine()
		if err != nil {
			break
		}
		result = append(result, string(currentLine))
	}
	return result
}

func CalculatePowerConsumption(bits []string) int {
	bitCounter := make([]int, len(bits[0]))
	for _, bitLine := range bits {
		for i, character := range bitLine {
			if character == '0' {
				bitCounter[i] -= 1
			} else {
				bitCounter[i] += 1
			}
		}
	}

	gamma := ""
	epsilon := ""
	for _, count := range bitCounter {
		if count >= 0 {
			gamma += "1"
			epsilon += "0"
		} else {
			gamma += "0"
			epsilon += "1"
		}
	}

	gammaValue, err := strconv.ParseInt(gamma, 2, 0)
	if err != nil {
		panic(fmt.Sprintf("Cannot parse %s into int with err %v", gamma, err))
	}

	epsilonValue, err := strconv.ParseInt(epsilon, 2, 0)
	if err != nil {
		panic(fmt.Sprintf("Cannot parse %s into int with err %v", gamma, err))
	}

	return int(gammaValue) * int(epsilonValue)
}

func CalculateOxygenRating(bits []string) int {
	index := 0

	for {
		if len(bits) <= 1 {
			break
		}

		count := 0
		for _, bitLine := range bits {
			if rune(bitLine[index]) == '0' {
				count--
			} else {
				count++
			}
		}

		if count >= 0 {
			bits = filterBits(bits, '1', index)
		} else {
			bits = filterBits(bits, '0', index)
		}

		index++
	}

	rating, err := strconv.ParseInt(bits[0], 2, 0)
	if err != nil {
		panic(fmt.Sprintf("Cannot parse %s into int with err %v", bits[0], err))
	}

	return int(rating)
}

func CalculateCO2Rating(bits []string) int {
	index := 0

	for {
		if len(bits) <= 1 {
			break
		}

		count := 0
		for _, bitLine := range bits {
			if rune(bitLine[index]) == '0' {
				count--
			} else {
				count++
			}
		}

		if count >= 0 {
			bits = filterBits(bits, '0', index)
		} else {
			bits = filterBits(bits, '1', index)
		}

		index++
	}

	rating, err := strconv.ParseInt(bits[0], 2, 0)
	if err != nil {
		panic(fmt.Sprintf("Cannot parse %s into int with err %v", bits[0], err))
	}

	return int(rating)
}

func filterBits(bits []string, filter rune, filterIndex int) []string {
	result := make([]string, 0)
	for _, bitLine := range bits {
		if bitLine[filterIndex] == byte(filter) {
			result = append(result, bitLine)
		}
	}

	return result
}
