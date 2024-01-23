package two

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type Game struct {
	GameNo   int
	GameSets []GameSet
}

type GameSet struct {
	Red   int
	Green int
	Blue  int
}

func ReadGameFromFile() []Game {
	file, err := os.Open("two/two.txt")
	if err != nil {
		panic("Cannot read day 2 input file")
	}

	var result []Game
	reader := bufio.NewReader(file)
	for {
		currentLine, _, err := reader.ReadLine()
		if err != nil {
			break
		}

		parts := strings.Split(string(currentLine), ":")
		if len(parts) != 2 {
			panic("Invalid input line")
		}

		gameNumber := extractGameNo(parts[0])

		var gameSets []GameSet
		for _, rawGameSet := range strings.Split(strings.Trim(parts[1], " "), ";") {
			gameSets = append(gameSets, extractGameSet(rawGameSet))
		}

		result = append(result, Game{
			GameNo:   gameNumber,
			GameSets: gameSets,
		})
	}

	return result
}

func extractGameNo(input string) int {
	regex, err := regexp.Compile(`\d+`)
	if err != nil {
		panic(fmt.Errorf("cannot extract game number from string %s", input))
	}

	found := regex.FindString(input)
	gameNumber, err := strconv.Atoi(found)
	if err != nil {
		panic(fmt.Errorf("cannot parse string to number %s", found))
	}

	return gameNumber
}

// 6 red, 1 blue, 3 green
func extractGameSet(input string) GameSet {
	var red, green, blue int

	colors := strings.Split(input, ",")
	for _, color := range colors {
		parts := strings.Split(strings.Trim(color, " "), " ")
		value, err := strconv.Atoi(parts[0])
		if err != nil {
			fmt.Printf("Cannot parse %s with err %s\n", parts[0], err)
			continue
		}

		switch parts[1] {
		case "red":
			red += value
		case "green":
			green += value
		case "blue":
			blue += value
		}
	}

	return GameSet{
		Red:   red,
		Green: green,
		Blue:  blue,
	}
}

func SumPossibleGameId(games []Game) int {
	const redLimit = 12
	const greenLimit = 13
	const blueLimit = 14

	var sum int = 0

outer:
	for _, game := range games {
		for _, gameSet := range game.GameSets {
			if gameSet.Red > redLimit || gameSet.Blue > blueLimit || gameSet.Green > greenLimit {
				continue outer
			}
		}
		sum += game.GameNo
	}

	return sum
}

func MinimumPossibleGame(games []Game) int {
	var sum int

	for _, game := range games {
		var minRed, minGreen, minBlue int
		for _, gameSet := range game.GameSets {
			if gameSet.Red > minRed {
				minRed = gameSet.Red
			}

			if gameSet.Green > minGreen {
				minGreen = gameSet.Green
			}

			if gameSet.Blue > minBlue {
				minBlue = gameSet.Blue
			}
		}
		sum += minRed * minGreen * minBlue
	}

	return sum
}
