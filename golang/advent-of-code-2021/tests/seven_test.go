package tests

import (
	"aoc21/seven"
	"testing"
)

func TestMinCrabMove(t *testing.T) {
	inputs := []int{16, 1, 2, 0, 4, 2, 7, 1, 2, 14}
	expected := 37
	result := seven.MinCrabMove(inputs)

	if result != expected {
		t.Fatalf("Expected %d, but receive %d\n", expected, result)
	}
}

func TestMinCrabExtraMove(t *testing.T) {
	inputs := []int{16, 1, 2, 0, 4, 2, 7, 1, 2, 14}
	expected := 168
	result := seven.MinCrabExtraMove(inputs)

	if result != expected {
		t.Fatalf("Expected %d, but receive %d\n", expected, result)
	}
}
