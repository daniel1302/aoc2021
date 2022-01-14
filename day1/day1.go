package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	input, err := readInput("../inputs/day1.txt")
	if err != nil {
		panic(err)
	}

	fmt.Printf("Part 1: %d\n", part1(input))
	fmt.Printf("Part 2: %d\n", part2(input))
}

func readInput(filePath string) ([]int, error) {
	file, err := os.Open(filePath)

	if err != nil {
		return nil, err
	}

	scanner := bufio.NewScanner(file)

	result := []int{}
	num := 0
	for scanner.Scan() {
		if num, err = strconv.Atoi(scanner.Text()); err == nil {
			result = append(result, num)
		}
	}

	return result, nil
}

func part1(input []int) int {
	var (
		res int
	)
	for idx, val := range input {
		if idx > 0 && val > input[idx-1] {
			res++
		}
	}

	return res
}

func part2(input []int) int {
	var (
		series = []int{}
	)

	for idx, val := range input {
		if idx > 1 {
			series = append(series, (val + input[idx-1] + input[idx-2]))
		}
	}

	return part1(series)
}
