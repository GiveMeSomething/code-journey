package six

import (
	"fmt"
	"testing"
)

func TestCountFish(t *testing.T) {
	remainingDays := []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 18, 80, 256}
	intervals := []int{3, 4, 3, 1, 2}
	expectResults := []int{5, 6, 7, 9, 10, 10, 10, 10, 11, 12, 26, 5934, 26984457539}

	for i, day := range remainingDays {
		result := CountFish(&intervals, day)
		fmt.Printf("Testing with %d days \n", day)

		if result != expectResults[i] {
			t.Fatalf("Failed. Receive %d, expect %d\n", result, expectResults[i])
		}
	}
}
