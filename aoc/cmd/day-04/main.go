package main

import (
	"aoc/pkg/input"
	"aoc/pkg/structs"
	"fmt"
	"strconv"
	"strings"
)

func main() {
	inputs := input.GetInputs(4, 2022)
	part1Answer := Part1(inputs)
	fmt.Println(fmt.Sprintf("Part 1: %d", part1Answer))
	part2Answer := Part2(inputs)
	fmt.Println(fmt.Sprintf("Part 2: %d", part2Answer))
}

type Section struct {
	Start int64
	End   int64
}

func FromStr(a string) Section {
	values := strings.Split(a, "-")
	startStr := values[0]
	endStr := values[1]

	start, _ := strconv.ParseInt(startStr, 10, 32)
	end, _ := strconv.ParseInt(endStr, 10, 32)

	return Section{
		Start: start,
		End:   end,
	}
}

type WorkPair struct {
	Section1 Section
	Section2 Section
}

func (w *WorkPair) overlapsCompletely() bool {

	if w.Section1.Start <= w.Section2.Start && w.Section2.End <= w.Section1.End {
		return true
	}
	if w.Section2.Start <= w.Section1.Start && w.Section1.End <= w.Section2.End {
		return true
	}

	return false
}

func (w *WorkPair) overlaps() bool {

	if w.Section1.Start <= w.Section2.Start && w.Section2.Start <= w.Section1.End {
		return true
	}
	if w.Section2.Start <= w.Section1.Start && w.Section1.Start <= w.Section2.End {
		return true
	}

	return false
}

func Part1(inputs []structs.Group) int32 {

	contents := inputs[0].Contents
	rounds := strings.Split(contents, "\n")
	var count int32 = 0

	for _, round := range rounds {
		if round == "" {
			break
		}
		sections := strings.Split(round, ",")
		section1 := FromStr(sections[0])
		section2 := FromStr(sections[1])

		pair := WorkPair{
			Section1: section1,
			Section2: section2,
		}
		if pair.overlapsCompletely() {
			count += 1
		}
	}
	return count
}

func Part2(inputs []structs.Group) int32 {

	contents := inputs[0].Contents
	rounds := strings.Split(contents, "\n")
	var count int32 = 0

	for _, round := range rounds {
		if round == "" {
			break
		}
		sections := strings.Split(round, ",")
		section1 := FromStr(sections[0])
		section2 := FromStr(sections[1])

		pair := WorkPair{
			Section1: section1,
			Section2: section2,
		}
		if pair.overlaps() {
			count += 1
		}
	}
	return count
}
