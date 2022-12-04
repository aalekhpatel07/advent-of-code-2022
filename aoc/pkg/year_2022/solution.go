package year_2022

import (
	"fmt"
	"log"

	"aoc/pkg/advent"
	"aoc/pkg/structs"
)

type Solution interface {
	Part1([]structs.Group) string
	Part2([]structs.Group) string
}

func Solve(day int, year int, part int, inputs []structs.Group, solution Solution, submit bool) string {
	answer := ""
	if part == 1 {
		answer = solution.Part1(inputs)
	} else if part == 2 {
		answer = solution.Part2(inputs)
	} else {
		log.Fatal("Part must be one of 1 or 2.")
	}
	if submit {
		_, err := advent.PostAnswer(day, year, part, answer)
		if err != nil {
			fmt.Println(fmt.Sprintf("Error! (Day: %d, Year: %d, Part: %d) Answer: %s", day, year, part, answer))
			fmt.Println(err)
		} else {
			fmt.Println(fmt.Sprintf("Success! (Day: %d, Year: %d, Part: %d) Answer: %s", day, year, part, answer))
		}
	} else {
		fmt.Println(fmt.Sprintf("(Day: %d, Year: %d, Part: %d) Answer: %s", day, year, part, answer))
	}
	return answer
}
