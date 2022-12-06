package year_2022

import (
	"fmt"

	"golang.org/x/exp/slices"

	"aoc/pkg/structs"
)

type Day06 struct {
}

func allDistinct(arr []int32) bool {
	if len(arr) <= 1 {
		return true
	}

	sorted := slices.Clone(arr)
	slices.Sort(sorted)

	for idx, _ := range sorted {
		if idx == 0 {
			continue
		}
		if sorted[idx] == sorted[idx - 1] {
			return false
		}
	}

	return true
}


func (d *Day06) Part1(inputs []structs.Group) string {

	var window []int32
	counter := 0

	for _, char := range inputs[0].Contents {
		if len(window) >= 4 {
			if allDistinct(window) {
				return fmt.Sprintf("%d", counter)
			}
			window = window[1:]
		}
		window = append(window, char)
		counter += 1
	}

	return fmt.Sprintf("%d", counter)
}


func (d *Day06) Part2(inputs []structs.Group) string {

	var window []int32
	counter := 0

	for _, char := range inputs[0].Contents {
		if len(window) >= 14 {
			if allDistinct(window) {
				return fmt.Sprintf("%d", counter)
			}
			window = window[1:]
		}
		window = append(window, char)
		counter += 1
	}

	return fmt.Sprintf("%d", counter)
}