package main

import (
	"fmt"
	"log"

	"adventofcode/day1"
	"adventofcode/day2"
	"adventofcode/day3"
	"adventofcode/day4"
	"adventofcode/day5"
	"adventofcode/day6"
	"adventofcode/example"
)

func main() {
	// execExample()
	// execDayOne()
	// execDay2()
	// execDay3()
	// execDay4()
	// execDay5()
	execDay6()
}

//lint:ignore U1000 Ignore unused function temporarily for debugging
func execDayOne() {
	items := day1.ReadItemFromFile()
	maxSum := day1.GetMaxSum(items)
	topThreeSum := day1.GetTopThreeSum(items)

	fmt.Printf("The max sum is: %v\n", maxSum)
	fmt.Printf("The sum of the top three item is: %v\n", topThreeSum)
}

//lint:ignore U1000 Ignore unused function temporarily for debugging
func execDay2() {
	inputs := day2.ReadEncryptFromFile()
	winningPoint := day2.CalculatePoint(inputs)
	winningPoint2 := day2.CalculatePointPart2(inputs)

	fmt.Printf("You have win %v points after %v match\n", winningPoint, len(inputs))
	fmt.Printf("Part II: You have win %v points after %v match\n", winningPoint2, len(inputs))
}

//lint:ignore U1000 Ignore unused function temporarily for debugging
func execDay3() {
	rucksacks := day3.ReadRucksackFromFile()
	prioritySum := day3.CalculatePrioritySum(rucksacks)
	groupSum := day3.CalculateGroupPrioritySum(rucksacks)

	fmt.Printf("Priority sum of all rucksacks is %v\n", prioritySum)
	fmt.Printf("Priority sum of all group is %v\n", groupSum)
}

//lint:ignore U1000 Ignore unused function temporarily for debugging
func execDay4() {
	assignments := day4.ReadAssignmentFromFile()
	fullOverlapPair := day4.CounOverlapPair(assignments)
	partialOverlapPair := day4.CountPartialOverlapPair(assignments)

	fmt.Printf("Number of fully overlapping pair is %v\n", fullOverlapPair)
	fmt.Printf("Number of partially overlapping pair is %v\n", partialOverlapPair)
}

//lint:ignore U1000 Ignore unused function temporarily for debugging
func execDay5() {
	cargos, instructions := day5.ReadInputFromFile()

	message1 := day5.PeekTopCrate(day5.MakeCargosCopy(cargos), instructions)
	message2 := day5.PeekTopCrate9001(day5.MakeCargosCopy(cargos), instructions)

	fmt.Println(cargos)
	fmt.Println(instructions)

	fmt.Printf("Part 1 message: %v\n", message1)
	fmt.Printf("Part 2 message: %v\n", message2)
	fmt.Println()
}

//lint:ignore U1000 Ignore unused function temporarily for debugging
func execDay6() {
	message := day6.ReadMessageFromFile()

	startOfPacket := day6.FindStartOfPacket(message)
	startOfMessage := day6.FindStartOfMessage(message)

	fmt.Printf("Start of packet at index %v\n", startOfPacket)
	fmt.Printf("Start of message at index %v\n", startOfMessage)
}

//lint:ignore U1000 Ignore unused function temporarily for debugging
func execExample() {
	messages, err := example.Hellos([]string{"Give", "Me", "Something"})
	if err != nil {
		log.Fatal(err)
	}

	for _, message := range messages {
		fmt.Println(message)
		fmt.Println()
	}
}
