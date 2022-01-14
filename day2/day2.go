package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Opcode string

const (
	OpForward Opcode = "forward"
	OpDown    Opcode = "down"
	OpUp      Opcode = "up"
)

type Operation struct {
	OpCode  Opcode
	Operand int
}

func readInput(filePath string) ([]Operation, error) {
	file, err := os.Open(filePath)

	if err != nil {
		return nil, err
	}

	scanner := bufio.NewScanner(file)

	result := []Operation{}

	var (
		opBuffer []string
		op       Operation
	)

	for scanner.Scan() {
		opBuffer = strings.Split(scanner.Text(), " ")
		if len(opBuffer) != 2 {
			continue
		}

		op = Operation{}
		switch opBuffer[0] {
		case string(OpUp):
			op.OpCode = OpUp
		case string(OpDown):
			op.OpCode = OpDown
		case string(OpForward):
			op.OpCode = OpForward
		default:
			continue
		}

		op.Operand, err = strconv.Atoi(opBuffer[1])
		if err != nil {
			return nil, fmt.Errorf("cannot convert operand: %w", err)
		}

		result = append(result, op)
	}

	return result, nil
}

func part1(input []Operation) int {
	var (
		x int
		y int
	)

	for _, op := range input {
		switch op.OpCode {
		case OpForward:
			x += op.Operand
		case OpDown:
			y += op.Operand
		case OpUp:
			y -= op.Operand
		}
	}

	return x * y
}

func part2(input []Operation) int {
	var (
		x   int
		y   int
		aim int
	)

	for _, op := range input {
		switch op.OpCode {
		case OpForward:
			x += op.Operand
			y += op.Operand * aim
		case OpDown:
			aim += op.Operand
		case OpUp:
			aim -= op.Operand
		}
	}

	return x * y
}

func main() {
	input, err := readInput("../inputs/day2.txt")
	if err != nil {
		panic(err)
	}

	fmt.Printf("Part 1: %d\n", part1(input))
	fmt.Printf("Part 2: %d\n", part2(input))
}
