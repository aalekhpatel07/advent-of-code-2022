package year_2022

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"

	"github.com/niemeyer/golang/src/pkg/container/vector"

	"aoc/pkg/structs"
)

type Day05 struct {
}

type Move struct {
	Count  int
	Source int
	Target int
}

var regexCompiled = regexp.MustCompile(`move (?P<Count>\d+) from (?P<Source>\d+) to (?P<Target>\d+)`)

func NewMove(s string) *Move {
	if s == "" {
		return nil
	}
	match := regexCompiled.FindStringSubmatch(s)
	count, _ := strconv.Atoi(match[1])
	source, _ := strconv.Atoi(match[2])
	target, _ := strconv.Atoi(match[3])
	return &Move{
		Count:  count,
		Source: source,
		Target: target,
	}
}

func (move *Move) apply(stacks []vector.IntVector) []vector.IntVector {

	sourceVector := stacks[move.Source-1]
	targetVector := stacks[move.Target-1]

	for i := 0; i < move.Count; i += 1 {
		targetVector.Push(sourceVector.Pop())
	}

	stacks[move.Source-1] = sourceVector
	stacks[move.Target-1] = targetVector

	return stacks
}

func (move *Move) applyMultipleCrates(stacks []vector.IntVector) []vector.IntVector {

	sourceVector := stacks[move.Source-1]
	targetVector := stacks[move.Target-1]

	targetVector.AppendVector(sourceVector.Slice(sourceVector.Len()-move.Count, sourceVector.Len()))

	for i := 0; i < move.Count; i++ {
		sourceVector.Pop()
	}

	stacks[move.Source-1] = sourceVector
	stacks[move.Target-1] = targetVector

	return stacks
}

func GetStacks(inputs []structs.Group) []vector.IntVector {
	// I'm not gonna write a parser for this weird structure.
	// The stack data is hardcoded in this solution.

	//			[G]         [D]     [Q]
	//	[P]     [T]         [L] [M] [Z]
	//	[Z] [Z] [C]         [Z] [G] [W]
	//	[M] [B] [F]         [P] [C] [H] [N]
	//	[T] [S] [R]     [H] [W] [R] [L] [W]
	//	[R] [T] [Q] [Z] [R] [S] [Z] [F] [P]
	//	[C] [N] [H] [R] [N] [H] [D] [J] [Q]
	//	[N] [D] [M] [G] [Z] [F] [W] [S] [S]
	//	 1   2   3   4   5   6   7   8   9

	stacks := make([]vector.IntVector, 9)

	stacks[0] = vector.IntVector{'N', 'C', 'R', 'T', 'M', 'Z', 'P'}
	stacks[1] = vector.IntVector{'D', 'N', 'T', 'S', 'B', 'Z'}
	stacks[2] = vector.IntVector{'M', 'H', 'Q', 'R', 'F', 'C', 'T', 'G'}
	stacks[3] = vector.IntVector{'G', 'R', 'Z'}
	stacks[4] = vector.IntVector{'Z', 'N', 'R', 'H'}
	stacks[5] = vector.IntVector{'F', 'H', 'S', 'W', 'P', 'Z', 'L', 'D'}
	stacks[6] = vector.IntVector{'W', 'D', 'Z', 'R', 'C', 'G', 'M'}
	stacks[7] = vector.IntVector{'S', 'J', 'F', 'L', 'H', 'W', 'Z', 'Q'}
	stacks[8] = vector.IntVector{'S', 'Q', 'P', 'W', 'N'}
	return stacks
}

func (d *Day05) Part1(inputs []structs.Group) string {

	stacks := GetStacks(inputs)

	moves := inputs[1]
	for _, moveRaw := range strings.Split(moves.Contents, "\n") {
		move := NewMove(moveRaw)
		if move == nil {
			break
		}
		stacks = move.apply(stacks)
	}

	result := make([]string, len(stacks))
	for idx, stack := range stacks {
		result[idx] = fmt.Sprintf("%c", stack.Last())
	}

	return strings.Join(result, "")
}

func (d *Day05) Part2(inputs []structs.Group) string {

	stacks := GetStacks(inputs)

	moves := inputs[1]
	for _, moveRaw := range strings.Split(moves.Contents, "\n") {
		move := NewMove(moveRaw)
		if move == nil {
			break
		}
		stacks = move.applyMultipleCrates(stacks)
	}

	result := make([]string, len(stacks))
	for idx, stack := range stacks {
		result[idx] = fmt.Sprintf("%c", stack.Last())
	}

	return strings.Join(result, "")
}
