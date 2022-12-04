package main

import (
	"aoc/internal"
	"aoc/pkg/advent"
	"aoc/pkg/structs"
	"fmt"
	"log"
	"sort"
	"strconv"
	"strings"
)

func main() {
	inputs, err := advent.GetInputs(1, 2022)
	if err != nil {
		log.Fatal(err)
	}
	part1Answer := Part1(inputs)
	fmt.Println(fmt.Sprintf("Part 1: %d", part1Answer))
	part2Answer := Part2(inputs)
	fmt.Println(fmt.Sprintf("Part 2: %d", part2Answer))
}

func Part1(inputs []structs.Group) int64 {
	sums := make([]int64, len(inputs))

	for idx, group := range inputs {
		sums[idx] = sumContents(group.Contents)
	}
	result, _ := internal.Max(sums)
	return result
}

func Part2(inputs []structs.Group) int64 {
	sums := make([]int64, len(inputs))

	for idx, group := range inputs {
		sums[idx] = sumContents(group.Contents)
	}
	sort.Slice(sums, func(i, j int) bool {
		return sums[i] > sums[j]
	})
	return sums[0] + sums[1] + sums[2]
}

func sumContents(contents string) int64 {
	stringlyNumbers := strings.Split(contents, "\n")
	var sum int64 = 0
	for _, number := range stringlyNumbers {
		val, _ := strconv.ParseInt(number, 10, 0)
		sum += val
	}
	return sum
}
