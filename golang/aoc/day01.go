package aoc

import (
	"fmt"
)

func Day01(day int) {
	fmt.Printf("...1: %s\n", challenge1(day))
	fmt.Printf("...2: %s\n", challenge2(day))
}

func challenge1(day int) string {
	integers, err := AsInts(day, 1)
	if err != nil {
		panic(err)
	}
	increases := countDepthIncreasesWin(integers, 2)
	return fmt.Sprintf("Increasing depth: %d times", increases)
}

func challenge2(day int) string {
	integers, err := AsInts(day, 1)
	if err != nil {
		panic(err)
	}
	increases := countDepthIncreasesWin(integers, 4)
	return fmt.Sprintf("Increasing depth in windows of 3: %d times", increases)
}

func countDepthIncreasesWin(measurements []int64, window int) int {
	count := 0
	for i := 0; i < len(measurements)-window+1; i++ {
		if measurements[i] < measurements[i+window-1] {
			count++
		}
	}
	return count
}
