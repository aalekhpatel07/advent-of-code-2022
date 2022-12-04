package internal

import (
	"errors"
	"golang.org/x/exp/constraints"
)

var EmptyError = errors.New("Iterable has no items.")

func Min[I constraints.Ordered](items []I) (I, error) {
	if len(items) == 0 {
		return nil, EmptyError
	}

	if len(items) == 1 {
		return items[0], nil
	}

	smallest := items[0]

	for idx, item := range items {
		if idx == 0 {
			continue
		}
		if item < smallest {
			smallest = item
		}
	}
	return smallest, nil
}

func Max[I constraints.Ordered](items []I) (I, error) {
	if len(items) == 0 {
		return nil, EmptyError
	}

	if len(items) == 1 {
		return items[0], nil
	}

	largest := items[0]

	for idx, item := range items {
		if idx == 0 {
			continue
		}
		if item > largest {
			largest = item
		}
	}
	return largest, nil
}
