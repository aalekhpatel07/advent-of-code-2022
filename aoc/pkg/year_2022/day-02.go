package year_2022

import (
	"fmt"
	"strings"

	"aoc/pkg/structs"
)

type Day02 struct {
}

type Round struct {
	Opponent string
	Self     string
}

func (d *Day02) Part1(inputs []structs.Group) string {

	scoreMap := make(map[Round]int)

	scoreMap[Round{Opponent: "A", Self: "X"}] = 1 + 3
	scoreMap[Round{Opponent: "A", Self: "Y"}] = 2 + 6
	scoreMap[Round{Opponent: "A", Self: "Z"}] = 3 + 0
	scoreMap[Round{Opponent: "B", Self: "X"}] = 1 + 0
	scoreMap[Round{Opponent: "B", Self: "Y"}] = 2 + 3
	scoreMap[Round{Opponent: "B", Self: "Z"}] = 3 + 6
	scoreMap[Round{Opponent: "C", Self: "X"}] = 1 + 6
	scoreMap[Round{Opponent: "C", Self: "Y"}] = 2 + 0
	scoreMap[Round{Opponent: "C", Self: "Z"}] = 3 + 3

	scores := make([]int, len(inputs))

	for idx, group := range inputs {
		scores[idx] = calculateScore(group.Contents, scoreMap)
	}
	return fmt.Sprintf("%d", scores[0])
}

func (d *Day02) Part2(inputs []structs.Group) string {
	scoreMap := make(map[Round]int)

	scoreMap[Round{Opponent: "A", Self: "X"}] = 3 + 0
	scoreMap[Round{Opponent: "A", Self: "Y"}] = 1 + 3
	scoreMap[Round{Opponent: "A", Self: "Z"}] = 2 + 6
	scoreMap[Round{Opponent: "B", Self: "X"}] = 1 + 0
	scoreMap[Round{Opponent: "B", Self: "Y"}] = 2 + 3
	scoreMap[Round{Opponent: "B", Self: "Z"}] = 3 + 6
	scoreMap[Round{Opponent: "C", Self: "X"}] = 2 + 0
	scoreMap[Round{Opponent: "C", Self: "Y"}] = 3 + 3
	scoreMap[Round{Opponent: "C", Self: "Z"}] = 1 + 6

	scores := make([]int, len(inputs))

	for idx, group := range inputs {
		scores[idx] = calculateScore(group.Contents, scoreMap)
	}
	return fmt.Sprintf("%d", scores[0])
}

func calculateScore(contents string, scoreMap map[Round]int) int {
	rounds := strings.Split(contents, "\n")
	sum := 0
	for _, round := range rounds {
		if round == "" {
			break
		}
		plays := strings.Split(round, " ")
		opponent := plays[0]
		self := plays[1]
		sum += scoreMap[Round{Opponent: opponent, Self: self}]
	}
	return sum
}
