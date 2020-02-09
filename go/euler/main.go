package main

import (
	"fmt"
	"time"
)

func main() {
	i, err := getProblemNumber()
	if err != nil {
		fmt.Println("You have entered an invalid problem number!")
		return
	}
	fmt.Println("You have entered", i)

	implementations := [2](func() string){implementation1, implementation2}
	solutions := [1][2](func() string){implementations}

	for _, implementation := range solutions[i] {
		printOutputWithTimeLogging(implementation)
	}
}

func getProblemNumber() (int, error) {
	var problemNumber int
	fmt.Print("Enter problem number: ")
	_, err := fmt.Scan(&problemNumber)
	return problemNumber, err
}

func printOutputWithTimeLogging(fn func() string) {
	start := time.Now()
	duration := formatDuration(time.Now().Sub(start))
	fmt.Printf("Result: %s. Executed in %s\n", fn(), duration)
}

func formatDuration(d time.Duration) string {
	if d.Seconds() > 1 {
		return fmt.Sprintf("%fs", d.Seconds())
	}
	if d/time.Millisecond > 1 {
		return fmt.Sprintf("%dms", d/time.Millisecond)
	}
	if d/time.Microsecond > 1 {
		return fmt.Sprintf("%dμs", d/time.Microsecond)
	}
	return fmt.Sprintf("%dns", d.Nanoseconds())
}

func implementation1() string {
	var result = 0
	for n := 1; n < 1000; n++ {
		if n%3 == 0 || n%5 == 0 {
			result += n
		}
	}

	return fmt.Sprintf("%d", result)
}

func sumOfNaturals(n int) int {
	return n * (n + 1) / 2
}

func implementation2() string {
	result := sumOfNaturals(1000/3) + sumOfNaturals(1000/5) - sumOfNaturals(1000/15)
	return fmt.Sprintf("%d", result)
}
