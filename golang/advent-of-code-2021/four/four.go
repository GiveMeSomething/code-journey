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
			bingos = append(bingos, currentBingo)
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

func CheckBingo(numbers []int, bingo [][]int) (int, int) {
	// Init checker
	checker := make([][]int, 5)
	for i := range checker {
		checker[i] = make([]int, 5)
	}

	for count, number := range numbers {
		for i, line := range bingo {
			for j, value := range line {
				if value == number {
					checker[i][j] = 1
					if checkRow(checker, i) || checkCol(checker, j) {
						return count, sumUncheckedNumber(bingo, checker) * value
					}
				}
			}
		}
	}

	return -1, -1
}

func checkRow(bingo [][]int, row int) bool {
	for i := 0; i < 5; i++ {
		if bingo[row][i] == 0 {
			return false
		}
	}
	return true
}

func checkCol(bingo [][]int, col int) bool {
	for i := 0; i < 5; i++ {
		if bingo[i][col] == 0 {
			return false
		}
	}
	return true
}

func sumUncheckedNumber(bingo [][]int, checker [][]int) int {
	sum := 0
	for i, line := range bingo {
		for j, value := range line {
			if checker[i][j] == 1 {
				continue
			}
			sum += value
		}
	}
	return sum
}
