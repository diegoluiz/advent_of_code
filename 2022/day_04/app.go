package day_04

import (
	h "advent_of_code/2022/helpers"
	"fmt"
	"os"
	"strconv"
	"strings"
)

var day = "04"

func Run() {
	defer h.EndDay(h.StartDay(day))
	lines, _ := os.ReadFile("./day_" + day + "/data/input.txt")
	input := string(lines)

	assignment_pairs := strings.Split(input, "\n")

	count_p1 := 0
	count_p2 := 0

	for _, assignment_pair := range assignment_pairs {
		pairs := strings.Split(assignment_pair, ",")
		range_1_str := strings.Split(pairs[0], "-")
		range_2_str := strings.Split(pairs[1], "-")

		range_1_s, _ := strconv.Atoi(range_1_str[0])
		range_1_e, _ := strconv.Atoi(range_1_str[1])

		range_2_s, _ := strconv.Atoi(range_2_str[0])
		range_2_e, _ := strconv.Atoi(range_2_str[1])

		if range_1_s >= range_2_s && range_1_e <= range_2_e {
			fmt.Printf("%s - %s\n", range_1_str, range_2_str)
			count_p1 += 1
			count_p2 += 1
		} else if range_2_s >= range_1_s && range_2_e <= range_1_e {
			count_p1 += 1
			count_p2 += 1
		} else if range_1_s >= range_2_s && range_1_s <= range_2_e {
			count_p2 += 1
		} else if range_1_e >= range_2_s && range_1_s <= range_2_e {
			count_p2 += 1
		} else {
			print("")
		}
	}

	fmt.Printf(" - p1: [%v]\n - p2: [%v]\n", count_p1, count_p2)
}
