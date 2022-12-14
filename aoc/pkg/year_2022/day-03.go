package year_2022

import (
	"fmt"
	"strings"

	"aoc/pkg/structs"
)

type Day03 struct {
}

func (d *Day03) Part1(inputs []structs.Group) string {

	contents := inputs[0].Contents
	rounds := strings.Split(contents, "\n")
	var sum int32 = 0

	for _, round := range rounds {
		if round == "" {
			break
		}
		firstCompartment := round[0 : len(round)/2]
		secondCompartment := round[len(round)/2:]

		words := make([]string, 2)
		words[0] = firstCompartment
		words[1] = secondCompartment
		unique := findUnique2(words[0], words[1])
		currentSum := computeScoreForChar(unique)
		sum += currentSum
	}
	return fmt.Sprintf("%d", sum)
}

func (d *Day03) Part2(inputs []structs.Group) string {

	contents := inputs[0].Contents
	rounds := strings.Split(contents, "\n")
	var sum int32 = 0

	for i := 0; i < len(rounds)/3; i++ {
		first := rounds[3*i]
		if first == "" {
			break
		}
		second := rounds[3*i+1]
		third := rounds[3*i+2]

		unique := findUnique3(first, second, third)
		currentSum := computeScoreForChar(unique)
		sum += currentSum
	}

	return fmt.Sprintf("%d", sum)
}

func computeScoreForChar(char int32) int32 {
	if 97 <= char && char <= 122 {
		// lowercase.
		return char - 96
	} else {
		// uppercase.
		return char - 64 + 26
	}
}

func findUnique2(word1 string, word2 string) int32 {

	for _, char := range word1 {
		if strings.Contains(word2, fmt.Sprintf("%c", char)) {
			return char
		}
	}
	return 0
}

func findUnique3(word1 string, word2 string, word3 string) int32 {

	commonChars := make([]int32, 0)

	for _, char := range word1 {
		if strings.Contains(word2, fmt.Sprintf("%c", char)) {
			commonChars = append(commonChars, char)
		}
	}

	for _, char := range commonChars {
		if strings.Contains(word3, fmt.Sprintf("%c", char)) {
			return char
		}
	}
	return 0
}
