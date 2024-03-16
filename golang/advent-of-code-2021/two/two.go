package two

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Command struct {
	Direction string
	Step      int
}

func ReadCommandFromFile() []Command {
	file, err := os.Open("two/two.txt")
	if err != nil {
		panic("Cannot find/open input file for day 2")
	}

	reader := bufio.NewReader(file)
	commands := make([]Command, 0)
	for {
		currentLine, _, err := reader.ReadLine()
		if err != nil {
			break
		}

		parts := strings.Split(string(currentLine), " ")
		step, err := strconv.Atoi(parts[1])
		if err != nil {
			fmt.Printf("Cannot convert %s to number\n", parts[1])
		}

		commands = append(commands, Command{Direction: parts[0], Step: step})
	}

	return commands
}

func SimulateCommands(commands *[]Command) (int, int) {
	horizontal := 0
	vertical := 0
	for _, command := range *commands {
		switch command.Direction {
		case "forward":
			horizontal += command.Step
		case "up":
			vertical -= command.Step
		case "down":
			vertical += command.Step
		}
	}

	return horizontal, vertical
}

func SimulateCommandsWithAim(commands *[]Command) (int, int) {
	horizontal, vertical, aim := 0, 0, 0
	for _, command := range *commands {
		switch command.Direction {
		case "forward":
			horizontal += command.Step
			vertical += aim * command.Step
		case "up":
			aim -= command.Step
		case "down":
			aim += command.Step
		}
	}

	return horizontal, vertical
}
