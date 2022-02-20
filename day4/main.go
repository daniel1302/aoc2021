package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type DataStream []int
type Board struct {
	data   []int
	called []int
	won    bool
}

func (b Board) Valid() bool {
	return len(b.data) >= 25
}

func (b *Board) Append(num int) {
	b.data = append(b.data, num)
	b.called = append(b.called, 0)
}

func (b *Board) Call(num int) {
	for i, boardVal := range b.data {
		if boardVal == num {
			b.called[i] = 1
		}
	}
}

func (b Board) Score(factor int) int {
	sum := 0

	for i, num := range b.data {
		if b.called[i] == 0 {
			sum += num
		}
	}

	return sum * factor
}

func (b *Board) Bingo() bool {
	for i := 0; i <= 4; i++ {
		if b.called[i*5]+b.called[(i*5)+1]+b.called[(i*5)+2]+b.called[(i*5)+3]+b.called[(i*5)+4] == 5 {
			b.won = true
			return true
		}
		if b.called[i]+b.called[i+5]+b.called[i+10]+b.called[i+15]+b.called[i+20] == 5 {
			b.won = true
			return true
		}
	}

	return false
}

func splitIntoIntegers(line, sep string) ([]int, error) {
	if len(line) < 1 {
		return []int{}, nil
	}

	res := []int{}
	for _, strNum := range strings.Split(line, sep) {
		if len(strNum) < 1 {
			continue
		}
		intNum, err := strconv.Atoi(strings.Trim(strNum, " \t"))
		if err != nil {
			return nil, fmt.Errorf("cannot read data from line \"%v\": %w", line, err)
		}

		res = append(res, intNum)
	}

	return res, nil
}

func readInput(filePath string) (DataStream, []Board, error) {
	var (
		ds  DataStream
		err error
	)
	file, err := os.Open(filePath)
	if err != nil {
		return nil, nil, err
	}

	scanner := bufio.NewScanner(file)
	if scanner.Scan() {
		ds, err = splitIntoIntegers(scanner.Text(), ",")
	}

	input := []int{}
	for scanner.Scan() {
		data, err := splitIntoIntegers(scanner.Text(), " ")
		if err != nil {
			return nil, nil, err
		}

		input = append(input, data...)
	}

	boards := []Board{}
	board := Board{}
	for _, num := range input {
		board.Append(num)

		if board.Valid() {
			boards = append(boards, board)
			board = Board{}
		}
	}

	return ds, boards, nil
}

func part1(input []int, boards []Board) int {
	for _, num := range input {
		for i, b := range boards {
			boards[i].Call(num)

			if b.Bingo() {
				return b.Score(num)
			}
		}
	}

	return -1
}

func part2(input []int, boards []Board) int {
	var lastScore int

	var num int

	for _, num = range input {
		for i, b := range boards {
			if b.won {
				continue
			}
			boards[i].Call(num)

			if boards[i].Bingo() {
				lastScore = b.Score(num)
			}
		}
	}

	return lastScore
}

func main() {
	a, b, _ := readInput("../inputs/day4.txt")
	boardsCopy := make([]Board, len(b))

	copy(boardsCopy, b)
	fmt.Printf("Part 1: %d\n", part1(a, b))
	fmt.Printf("Part 2: %d\n", part2(a, boardsCopy))
	// fmt.Printf("%#v\n%#v\n%#v\n", a, b, c)
}
