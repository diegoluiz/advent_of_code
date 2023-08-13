package day_01

import (
	h "advent_of_code/2022/helpers"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

var day = "01"

func Run() {
	defer h.EndDay(h.StartDay(day))
	lines, _ := os.ReadFile("./day_" + day + "/data/input.txt")
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

	fmt.Printf(" - p1: [%v]\n - p2: [%v]\n", total_cals[l-1], (total_cals[l-1] + total_cals[l-2] + total_cals[l-3]))
}
