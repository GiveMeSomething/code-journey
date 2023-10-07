package main

import (
	"fmt"
	"log"

	"adventofcode/day_one"
	"adventofcode/example"
)

func main() {
	// execExample()
	execDayOne()
}

func execDayOne() {
	items := day_one.ReadItemFromFile()
	maxSum := day_one.GetMaxSum(items)
	topThreeSum := day_one.GetTopThreeSum(items)

	fmt.Printf("The max sum is: %v\n", maxSum)
	fmt.Printf("The sum of the top three item is: %v\n", topThreeSum)
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
