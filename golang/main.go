package main

import (
	"fmt"

	"github.com/tobikris/adventofcode-2021/aoc"
)

func main() {
	days := []func(day int){aoc.Day01}
	for i, day := range days {
		fmt.Printf("day%02d:\n", i+1)
		day(i + 1)
	}
}
