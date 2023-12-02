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
			if tokens[0] == "dir" {
				continue
			}

			currentFileSize, err := strconv.Atoi(tokens[0])
			if err != nil {
				continue
			}

			// Update directory tree
			for i := range directoryStack {
				var filePath string
				if i == 0 {
					filePath = "/"
				} else {
					filePath = strings.Join(directoryStack[:i+1], "/")
				}

				fileSize, found := fileMap[filePath]
				if !found {
					fileMap[filePath] = currentFileSize
				} else {
					fileMap[filePath] = fileSize + currentFileSize
				}
			}
		}
	}

	return fileMap
}

func SumSmallFiles(fileMap map[string]int) int {
	sum := 0
	for _, fileSize := range fileMap {
		if fileSize <= 100000 {
			sum += fileSize
		}
	}
	return sum
}
