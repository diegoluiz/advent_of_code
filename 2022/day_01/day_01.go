package day_01

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func Run() {

	fmt.Printf("day 01\n")
	lines, _ := os.ReadFile("./day_01/data/input.txt")
	input := string(lines)

	elves_foods := strings.Split(input, "\n\n")

	l := len(elves_foods)
	total_cals := make([]int, l)

	for idx, elve_foods := range elves_foods {
		curr := 0
		elve_food := strings.Split(elve_foods, "\n")
		for _, x := range elve_food {
			v, _ := strconv.Atoi(x)
			curr += v
		}

		total_cals[idx] = curr
	}

	sort.Ints(total_cals)

	fmt.Printf("p1: ")
	fmt.Println(total_cals[l-1])

	fmt.Printf("p2: ")
	fmt.Println(total_cals[l-1] + total_cals[l-2] + total_cals[l-3])
}
