package main

import (
	"fmt"
	"log"

	"adventofcode/example"
)

func main() {
	execExample()
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
