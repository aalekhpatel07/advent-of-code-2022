# Advent of Code 2022

This Go project contains my solutions for Advent of Code 2022.

**Disclaimer**: I'm trying out Go for the first time so this project may not be as idiomatic.

## Usage

Build the project with Go: 
```sh
cd aoc && go build
```
 
Run the `aoc` binary with your Advent of Code session ID provided as an environment variable.

**Note**: The binary depends on an AOC provided session ID which can be found in the browser's `session` cookie after logging in.

## Examples

#### Compute the solution for Day 1, Part 1, Year 2022:
```sh
AOC_SESSION_ID="" ./aoc --day 1 --part 1 --year 2022
```

#### Compute **and submit** the solution for Day 2, Part 2, Year 2022:
```sh
AOC_SESSION_ID="" ./aoc --day 2 --part 2 --year 2022 --submit
```
