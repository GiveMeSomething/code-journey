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

func getCharacterValue(character rune) int32 {
	if character >= 65 && character <= 90 {
		return character - 38
	} else {
		return character - 96
	}
}
