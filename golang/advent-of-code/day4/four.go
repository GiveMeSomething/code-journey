package day4

import (
	"bufio"
	"fmt"
	"os"
)

func ReadAssignmentFromFile() []string {
	file, err := os.Open("day4/assignment.txt")
	if err != nil {
		fmt.Println("Cannot find/open assignment.txt")
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
