package five

import (
	"bufio"
	"os"
	"regexp"
	"strconv"
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

func extractVentLine(s string, regex *regexp.Regexp) VentLine {
	groups := regex.FindStringSubmatch(s)
	// ventLine := new(VentLine).Init(MustParseInt(groups[1]), MustParseInt(groups[2]), MustParseInt(groups[3]), MustParseInt(groups[4]))
	ventLine := newVentLine(MustParseInt(groups[1]), MustParseInt(groups[2]), MustParseInt(groups[3]), MustParseInt(groups[4]))
	return *ventLine
}

func newVentLine(fromX, fromY, toX, toY int) *VentLine {
	isDiagonal := fromX != toX && fromY != toY
	from := Point{
		X: fromX,
		Y: fromY,
	}
	to := Point{
		X: toX,
		Y: toY,
	}
	return &VentLine{
		From:       from,
		To:         to,
		IsDiagonal: isDiagonal,
	}
}

// Function to mask error by returning 0
// Only use when you are sure that there are only valid number string
func MustParseInt(s string) int {
	value, err := strconv.Atoi(s)
	if err != nil {
		panic(err)
	}
	return value
}
