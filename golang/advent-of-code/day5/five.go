package five

import (
	"bufio"
	"os"
	"regexp"
	"strconv"
)

type Cargo struct {
	value    rune
	position int
}

type Instruction struct {
	move int
	from int
	to   int
}

func ReadInputFromFile() ([][]string, []Instruction) {
	file, err := os.Open("day5/cargo.txt")
	checkError(err)

	reader := bufio.NewReader(file)

	var cargos [][]string = [][]string{}
	var instructions []Instruction = []Instruction{}

	var init bool = false

	// Step 0: Read cargo input, remember to ignore number line
	// Step 1: Read instructions
	currentStep := 0
	for {
		input, _, err := reader.ReadLine()
		if err != nil {
			// Probably reach EOF
			break
		}

		if !init {
			for i := 0; i < len(input)/4; i++ {
				cargos = append(cargos, []string{})
			}
			init = true
		}

		if len(input) == 0 {
			// Empty line => end of cargo input, switch to read instructions
			currentStep = 1
			continue
		}

		currentLine := string(input)
		switch currentStep {
		case 0:
			// Read cargos
			for _, cargo := range extractCargoLine(currentLine) {
				cargos[cargo.position] = append(cargos[cargo.position], string(currentLine))
			}
		case 1:
			// Read instructions
			instructions = append(instructions, extractInstructionLine(currentLine))
		}
	}

	return cargos, instructions
}

// Return cargo's value and stack position
func extractCargoLine(input string) []Cargo {
	var result []Cargo = []Cargo{}
	for i, char := range input {
		if char == ' ' || char == '[' || char == ']' {
			continue
		}
		position := (i - 1) / 4
		result = append(result, Cargo{value: char, position: position})
	}
	return result
}

func extractInstructionLine(input string) Instruction {
	instructionRegex, err := regexp.Compile("move (.+) from (.+) to (.+)")
	checkError(err)

	values := instructionRegex.FindStringSubmatch(input)
	if len(values) < 3 {
		panic("The number of values in an instruction should be greater or equal 3")
	}

	moveValue, err := strconv.Atoi(values[1])
	checkError(err)
	fromValue, err := strconv.Atoi(values[2])
	checkError(err)
	toValue, err := strconv.Atoi(values[3])
	checkError(err)

	return Instruction{
		move: moveValue,
		from: fromValue,
		to:   toValue,
	}
}

func checkError(err error) {
	if err != nil {
		panic(err)
	}
}
