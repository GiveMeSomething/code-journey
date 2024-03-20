package four

import (
	"bufio"
	"os"
	"strconv"
	"strings"
)

func ReadBingoFromFile() ([]int, [][][]int) {
	file, err := os.Open("four/four.txt")
	if err != nil {
		panic("Cannot find/open input file for day 4")
	}

	var bingoNumbers []int
	bingos := make([][][]int, 0)

	reader := bufio.NewReader(file)
	isFirstLine := true

	currentBingo := make([][]int, 0)
	for {
		currentLine, _, err := reader.ReadLine()
		if err != nil {
			break
		}

		if string(currentLine) == "" {
			if len(currentBingo) == 0 {
				continue
			}

			bingos = append(bingos, currentBingo)
			currentBingo = make([][]int, 0)
			continue
		}

		if isFirstLine {
			bingoNumbers = extractNumberLine(string(currentLine), ",")
			isFirstLine = false
			continue
		}

		currentBingo = append(currentBingo, extractNumberLine(string(currentLine), " "))
	}

	return bingoNumbers, bingos
}

func extractNumberLine(s string, separator string) []int {
	parts := strings.Split(s, separator)
	result := make([]int, 0)
	for _, part := range parts {
		value, err := strconv.Atoi(part)
		if err != nil {
			continue
		}
		result = append(result, value)
	}
	return result
}
