package year_2022

import (
	"aoc/internal"
	"aoc/pkg/structs"
	"fmt"
	"sort"
	"strconv"
	"strings"
)

type Day01 struct{}

func (d *Day01) Part1(inputs []structs.Group) string {
	sums := make([]int64, len(inputs))

	for idx, group := range inputs {
		sums[idx] = sumContents(group.Contents)
	}
	result, _ := internal.Max(sums)
	return fmt.Sprintf("%d", result)
}

func (d *Day01) Part2(inputs []structs.Group) string {
	sums := make([]int64, len(inputs))

	for idx, group := range inputs {
		sums[idx] = sumContents(group.Contents)
	}
	sort.Slice(sums, func(i, j int) bool {
		return sums[i] > sums[j]
	})
	return fmt.Sprintf("%d", sums[0]+sums[1]+sums[2])
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
