package day_0x

import (
	h "advent_of_code/2022/helpers"
	"fmt"
	"os"
)

var day = "0x"

func Run() {
	defer h.EndDay(h.StartDay(day))
	lines, _ := os.ReadFile("./day_" + day + "/data/input.txt")
	input := string(lines)

	print(input)
	points_p1 := 0
	points_p2 := 0

	fmt.Printf(" - p1: [%v]\n - p2: [%v]\n", points_p1, points_p2)
}
