package main

import (
	"fmt"
	"log"

	"adventofcode/day1"
	"adventofcode/day2"
	"adventofcode/day3"
	"adventofcode/example"
)

func main() {
	// execExample()
	// execDayOne()
	// execDay2()
	execDay3()
}

func execDayOne() {
	items := day1.ReadItemFromFile()
	maxSum := day1.GetMaxSum(items)
	topThreeSum := day1.GetTopThreeSum(items)

	fmt.Printf("The max sum is: %v\n", maxSum)
	fmt.Printf("The sum of the top three item is: %v\n", topThreeSum)
}

func execDay2() {
	inputs := day2.ReadEncryptFromFile()
	winningPoint := day2.CalculatePoint(inputs)
	winningPoint2 := day2.CalculatePointPart2(inputs)

	fmt.Printf("You have win %v points after %v match\n", winningPoint, len(inputs))
	fmt.Printf("Part II: You have win %v points after %v match\n", winningPoint2, len(inputs))
}

func execDay3() {
	rucksacks := day3.ReadRucksackFromFile()
	prioritySum := day3.CalculatePrioritySum(rucksacks)
	groupSum := day3.CalculateGroupPrioritySum(rucksacks)

	fmt.Printf("Priority sum of all rucksacks is %v\n", prioritySum)
	fmt.Printf("Priority sum of all group is %v\n", groupSum)
}

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
