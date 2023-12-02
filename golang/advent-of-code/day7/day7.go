package day7

import (
	"bufio"
	"os"
	"strconv"
	"strings"
)

func ParseDirTree() map[string]int {
	file, err := os.Open("day7/files.txt")
	if err != nil {
		panic("Cannot open day 7 input file")
	}

	reader := bufio.NewReader(file)

	fileMap := make(map[string]int)
	directoryStack := []string{"/"}

	for {
		line, _, err := reader.ReadLine()
		if err != nil {
			break
		}

		tokens := strings.Split(string(line), " ")
		if tokens[0] == "$" {
			// Parse commands
			if tokens[1] == "cd" {
				switch tokens[2] {
				case "/":
					directoryStack = []string{"/"}
				case "..":
					directoryStack = directoryStack[:len(directoryStack)-1]
				default:
					directoryStack = append(directoryStack, tokens[2])
				}
			}
		} else {
			// Parse commands' result
			if tokens[1] == "dir" {
				continue
			}

			fileSize, err := strconv.Atoi(tokens[1])
			if err != nil {
				continue
			}

			filePath := strings.Join(directoryStack, "/")
			fileMap[filePath] = fileSize
		}
	}
}
