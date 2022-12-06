package cmd

import (
	"log"

	"github.com/spf13/cobra"

	"aoc/pkg/advent"
	"aoc/pkg/year_2022"
)

var rootCmd = &cobra.Command{
	Use:   "my-aoc",
	Short: "Compute and submit Advent Of Code solutions",
	Long: "my-aoc lets you submit the solution to any day of Advent Of Code by running your text input against the solutions provided in this project. Please" +
		" provide the AOC_SESSION_ID environment variable with your session ID when interacting with this binary.",
	Run: Run,
}

var Submit bool = false

func init() {
	rootCmd.PersistentFlags().IntP("day", "d", 1, "The day to run solutions for.")
	rootCmd.PersistentFlags().IntP("year", "y", 2022, "The year to run solutions for. Currently, only 2022 is supported.")
	rootCmd.PersistentFlags().IntP("part", "p", 1, "The part to run solution for.")
	rootCmd.PersistentFlags().BoolVarP(&Submit, "submit", "s", false, "If set, will submit your solution to AOC.")

	_ = rootCmd.MarkFlagRequired("verbose")
	_ = rootCmd.MarkFlagRequired("day")
	_ = rootCmd.MarkFlagRequired("year")
	_ = rootCmd.MarkFlagRequired("part")
}

func Run(cmd *cobra.Command, args []string) {
	flags := cmd.Flags()
	day, _ := flags.GetInt("day")
	year, _ := flags.GetInt("year")
	part, _ := flags.GetInt("part")

	if year != 2022 {
		log.Fatal("Currently only have solutions for 2022. Please use 2022 as year flag or skip that flag altogether.")
	}

	inputs, err := advent.GetInputs(day, year)
	if err != nil {
		log.Fatalf("Could not fetch inputs: %s", err.Error())
	}

	solutions := make(map[int]year_2022.Solution)

	solutions[1] = &year_2022.Day01{}
	solutions[2] = &year_2022.Day02{}
	solutions[3] = &year_2022.Day03{}
	solutions[4] = &year_2022.Day04{}
	solutions[5] = &year_2022.Day05{}
	solutions[6] = &year_2022.Day06{}
	//solutions[7] = &year_2022.Day07{}
	//solutions[8] = &year_2022.Day08{}
	//solutions[9] = &year_2022.Day09{}

	year_2022.Solve(day, year, part, inputs, solutions[day], Submit)
}

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		log.Fatal(err)
	}
}
