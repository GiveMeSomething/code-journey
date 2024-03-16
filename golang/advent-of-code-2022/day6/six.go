package day6

import (
	"bufio"
	"os"
)

func ReadMessageFromFile() string {
	file, err := os.Open("day6/messages.txt")
	if err != nil {
		panic("Cannot open day 6 input file")
	}

	reader := bufio.NewReader(file)

	// There is only one line in input
	line, _, err := reader.ReadLine()
	if err != nil {
		panic("Cannot read line from day 6 input file")
	}

	return string(line)
}

func FindStartOfPacket(input string) int {
	return findUniqueSequence(input, 4)
}

func FindStartOfMessage(input string) int {
	return findUniqueSequence(input, 14)
}

func findUniqueSequence(input string, sequenceLength int) int {
	char_map := make(map[rune]int)

	for i := 0; i < sequenceLength; i++ {
		current_char := rune(input[i])
		count, found := char_map[current_char]
		if !found {
			char_map[current_char] = 1
		} else {
			char_map[current_char] = count + 1
		}
	}

	for i := sequenceLength; i < len(input); i++ {
		// Add current character to map
		current_char := rune(input[i])
		count, found := char_map[current_char]
		if !found {
			char_map[current_char] = 1
		} else {
			char_map[current_char] = count + 1
		}

		// Update count for out-of-window character
		previous_char := rune(input[i-sequenceLength])
		count = char_map[previous_char]
		if count == 1 {
			delete(char_map, previous_char)
		} else {
			char_map[previous_char] = count - 1
		}

		if len(char_map) == sequenceLength {
			// +1 to display the number of characters
			return i + 1
		}
	}

	return 0
}
