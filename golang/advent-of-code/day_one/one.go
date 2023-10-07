package day_one

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

func ReadItemFromFile() []int {
	file, err := os.Open("day_one/items.txt")
	check(err)

	reader := bufio.NewReader(file)

	var items []int
	sum := 0
	for {
		currentLine, _, err := reader.ReadLine()
		if err != nil {
			break
		}

		item := string(currentLine)
		if item == "" {
			items = append(items, sum)
			sum = 0
			continue
		}

		itemValue, err := strconv.Atoi(item)
		if err != nil {
			fmt.Printf("Cannot parse string: %v\n", item)
		}
		sum += itemValue
	}

	return items
}

func GetMaxSum(items []int) int {
	max := 0

	for _, item := range items {
		if item > max {
			max = item
		}
	}

	return max
}

func GetTopThreeSum(items []int) int {
	// This sort in increasing order
	// sort.Ints(items)

	// This sort in decreasing order
	sort.Slice(items, func(i, j int) bool {
		return items[i] > items[j]
	})

	return getSum(items[:3]...)
}

func getSum(nums ...int) int {
	sum := 0
	for _, value := range nums {
		sum += value
	}
	return sum
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
