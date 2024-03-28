package five

import "strconv"

func MustParseInt(s string) int {
	value, err := strconv.Atoi(s)
	if err != nil {
		panic(err)
	}
	return value
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

func abs(a int) int {
	if a < 0 {
		return a * -1
	}
	return a
}
