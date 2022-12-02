package template

import (
	h "advent_of_code/2022/helpers"
	"fmt"
	"os"
)

var day string = "xx"

func Run() {
	defer h.EndDay(h.StartDay(day))
	lines, _ := os.ReadFile("./" + day + "/data/input.txt")
	input := string(lines)

	p1 := len(input)
	p2 := len(input)
	fmt.Printf(" - p1: [%v]\n - p2: [%v]\n", p1, p2)
}
