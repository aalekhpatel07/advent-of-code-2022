package internal

import (
	"errors"
)

var EmptyError = errors.New("Iterable has no items.")

func Min[I int64 | float64 | int](items []I) (I, error) {
	if len(items) == 0 {
		return 0, EmptyError
	}

	if len(items) == 1 {
		return items[0], nil
	}

	smallest := items[0]

	for idx := 1; idx < len(items); idx++ {
		if items[idx] < smallest {
			smallest = items[idx]
		}
	}
	return smallest, nil
}

func Max[I int64 | float64 | int](items []I) (I, error) {
	if len(items) == 0 {
		return 0, EmptyError
	}

	if len(items) == 1 {
		return items[0], nil
	}

	largest := items[0]

	for idx := 1; idx < len(items); idx++ {
		if items[idx] > largest {
			largest = items[idx]
		}
	}
	return largest, nil
}
