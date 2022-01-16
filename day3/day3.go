package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	input, err := readInput("../inputs/day3.txt")
	if err != nil {
		panic(err)
	}

	fmt.Printf("Part 1: %d\n", part1(input))
	fmt.Printf("Part 2: %d\n", part2(input))
}

func readInput(filePath string) ([]string, error) {
	file, err := os.Open(filePath)

	if err != nil {
		return nil, err
	}

	scanner := bufio.NewScanner(file)
	result := []string{}
	for scanner.Scan() {
		result = append(result, scanner.Text())
	}

	return result, nil
}

func mostCommonBits(input []string) ([]uint, []uint) {
	zeros := make([]uint, len(input[0]))
	ones := make([]uint, len(input[0]))

	for _, inputNumber := range input {
		for i := range inputNumber {

			switch inputNumber[i] {
			case '0':
				zeros[i]++
			default:
				ones[i]++
			}
		}
	}
	return ones, zeros
}

func part1(input []string) int {
	ones, zeros := mostCommonBits(input)

	mbs := len(ones) - 1
	gamma := 0
	maxNumber := 0
	for bytePos := mbs; bytePos >= 0; bytePos-- {
		if ones[bytePos] > zeros[bytePos] {
			gamma = gamma + 1<<(mbs-bytePos)
		}
		maxNumber = maxNumber + 1<<bytePos
	}

	return (maxNumber - gamma) * gamma
}

func part2(input []string) int {
	ones, zeros := mostCommonBits(input)

	oxygenGeneratorRating := input

	var tmp []string

	for i := 0; i < len(input[0]) && len(oxygenGeneratorRating) > 1; i++ {
		tmp = []string{}
		for _, row := range oxygenGeneratorRating {
			if (ones[i] >= zeros[i]) && row[i] == '1' || ones[i] < zeros[i] && row[i] == '0' {
				tmp = append(tmp, row)
			}
		}
		oxygenGeneratorRating = tmp
		ones, zeros = mostCommonBits(oxygenGeneratorRating)
	}

	ones, zeros = mostCommonBits(input)
	co2ScrubberRating := input
	for i := 0; i < len(input[0]) && len(co2ScrubberRating) > 1; i++ {
		tmp = []string{}
		for _, row := range co2ScrubberRating {
			if (ones[i] < zeros[i] && row[i] == '1') || ones[i] >= zeros[i] && row[i] == '0' {
				tmp = append(tmp, row)
			}
		}
		co2ScrubberRating = tmp
		ones, zeros = mostCommonBits(co2ScrubberRating)
	}

	i1, _ := strconv.ParseInt(oxygenGeneratorRating[0], 2, 64)
	i2, _ := strconv.ParseInt(co2ScrubberRating[0], 2, 64)
	return int(i1) * int(i2)
}
