package aoc

import (
	"bufio"
	"fmt"
	"io/ioutil"
	"os"
)

func Filename(day, challenge int) string {
	return fmt.Sprintf("../inputs/day%02d-%d.txt", day, challenge)
}

func ReadFile(day, challenge int) string {
	filename := Filename(day, challenge)
	bytes, err := ioutil.ReadFile(filename)
	if err != nil {
		panic(err)
	}
	return string(bytes)
}

func AsInts(day, challenge int) ([]int64, error) {
	filename := Filename(day, challenge)
	file, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	result := []int64{}
	scan := bufio.NewScanner(file)
	for scan.Scan() {
		var line int64
		_, err := fmt.Sscanf(scan.Text(), "%d", &line)
		if err != nil {
			return nil, err
		}
		result = append(result, line)
	}
	return result, nil
}
