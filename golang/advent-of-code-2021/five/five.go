package five

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
)

func ReadVentFromFile() []VentLine {
	file, err := os.Open("five/five.txt")
	if err != nil {
		panic("Cannot find/open input file for day 5")
	}

	reader := bufio.NewReader(file)
	result := make([]VentLine, 0)
	for {
		currentLine, _, err := reader.ReadLine()
		if err != nil {
			break
		}
		ventRegex, err := regexp.Compile(`([0-9]*),([0-9]*) -> ([0-9]*),([0-9]*)`)
		if err != nil {
			panic("Invalid line extract regex")
		}
		result = append(result, extractVentLine(string(currentLine), ventRegex))
	}
	return result
}

func CountIntersections(ventLines *[]VentLine) int {
	intersections := 0
	pointMap := make(map[string]int)

	for _, ventLine := range *ventLines {
		if ventLine.IsDiagonal {
			continue
		}

		if ventLine.From.X == ventLine.To.X {
			for i := min(ventLine.From.Y, ventLine.To.Y); i <= max(ventLine.From.Y, ventLine.To.Y); i++ {
				key := fmt.Sprint(ventLine.From.X) + "," + fmt.Sprint(i)
				if _, ok := pointMap[key]; ok {
					pointMap[key] += 1
					continue
				}

				pointMap[key] = 1
			}
			continue
		}

		// For vertical vent line
		for i := min(ventLine.From.X, ventLine.To.X); i <= max(ventLine.From.X, ventLine.To.X); i++ {
			key := fmt.Sprint(i) + "," + fmt.Sprint(ventLine.From.Y)
			if _, ok := pointMap[key]; ok {
				pointMap[key] += 1
				continue
			}

			pointMap[key] = 1
		}
	}

	for _, value := range pointMap {
		if value >= 2 {
			intersections += 1
		}
	}

	return intersections
}

func extractVentLine(s string, regex *regexp.Regexp) VentLine {
	groups := regex.FindStringSubmatch(s)
	// ventLine := new(VentLine).Init(MustParseInt(groups[1]), MustParseInt(groups[2]), MustParseInt(groups[3]), MustParseInt(groups[4]))
	ventLine := NewVentLine(MustParseInt(groups[1]), MustParseInt(groups[2]), MustParseInt(groups[3]), MustParseInt(groups[4]))
	return *ventLine
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
