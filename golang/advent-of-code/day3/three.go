package day3

import (
	"bufio"
	"fmt"
	"os"
)

func ReadRucksackFromFile() []string {
	file, err := os.Open("day3/rucksack.txt")
	if err != nil {
		fmt.Println("Cannot find/open rucksack.txt")
		return []string{}
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

func CalculatePrioritySum(inputs []string) int32 {
	var sum int32
	for _, input := range inputs {
		pivot := len(input) / 2
		firstCompartment := input[:pivot]
		secondCompartment := input[pivot:]

		sum += getPriorityCharacterValue(firstCompartment, secondCompartment)
	}

	return sum
}

func CalculateGroupPrioritySum(inputs []string) int32 {
	var sum int32
	for i := 0; i < len(inputs); i += 3 {
		sum += getGroupCharacterValue(inputs[i], inputs[i+1], inputs[i+2])
	}
	return sum
}

func getPriorityCharacterValue(first, second string) int32 {
	charMap := make(map[rune]bool)
	for _, char := range first {
		charMap[char] = true
	}

	for _, char := range second {
		_, ok := charMap[char]
		if ok {
			return getCharacterValue(char)
		}
	}

	return 0
}

func getGroupCharacterValue(first, second, third string) int32 {
	var charMap = make(map[rune]int32)
	for _, char := range first {
		charMap[char] = 1
	}

	for _, char := range second {
		_, ok := charMap[char]
		if ok {
			charMap[char] = 2
		}
	}

	for _, char := range third {
		value, ok := charMap[char]
		if ok && value == 2 {
			return getCharacterValue(char)
		}
	}

	return 0
}

func getCharacterValue(character rune) int32 {
	if character >= 65 && character <= 90 {
		return character - 38
	} else {
		return character - 96
	}
}
