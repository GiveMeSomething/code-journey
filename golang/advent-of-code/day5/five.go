package day5

import (
	"bufio"
	"os"
	"regexp"
	"strconv"
	"unicode"
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
			for i := 0; i <= len(input)/4; i++ {
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
				cargos[cargo.position] = prependString(cargos[cargo.position], string(cargo.value))
			}
		case 1:
			// Read instructions
			instructions = append(instructions, extractInstructionLine(currentLine))
		}
	}

	return cargos, instructions
}

func PeekTopCrate(cargos [][]string, instructions []Instruction) string {
	for _, instruction := range instructions {
		for i := 0; i < instruction.move; i++ {
			// Pop from, append to
			stackFrom := cargos[instruction.from]
			stackTo := cargos[instruction.to]

			value := stackFrom[len(stackFrom)-1]

			cargos[instruction.from] = stackFrom[:len(stackFrom)-1]
			cargos[instruction.to] = append(stackTo, value)
		}
	}

	result := ""
	for _, stack := range cargos {
		result += string(stack[len(stack)-1])
	}
	return result
}

func PeekTopCrate9001(cargos [][]string, instructions []Instruction) string {
	return ""
}

// This is to avoid reference error when using the same 2D array
func MakeCargosCopy(cargos [][]string) [][]string {
	duplicate := make([][]string, len(cargos))
	for i := range duplicate {
		duplicate[i] = make([]string, len(cargos[i]))
		copy(duplicate[i], cargos[i])
	}
	return duplicate
}

// Return cargo's value and stack position
func extractCargoLine(input string) []Cargo {
	var result []Cargo = []Cargo{}
	for i, char := range input {
		if char == ' ' || char == '[' || char == ']' || unicode.IsDigit(char) {
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
		from: fromValue - 1,
		to:   toValue - 1,
	}
}

func checkError(err error) {
	if err != nil {
		panic(err)
	}
}

func prependString(arr []string, input string) []string {
	arr = append(arr, "")
	copy(arr[1:], arr)
	arr[0] = input
	return arr
}
