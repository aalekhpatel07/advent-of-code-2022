package year_2022

import (
	"aoc/pkg/structs"
	"fmt"
	"strconv"
	"strings"
)

type Day04 struct {
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

func (d *Day04) Part1(inputs []structs.Group) string {

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
	return fmt.Sprintf("%d", count)
}

func (d *Day04) Part2(inputs []structs.Group) string {

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
	return fmt.Sprintf("%d", count)
}
