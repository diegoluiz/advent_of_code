package main

import (
	"advent_of_code/2022/day_01"
	"advent_of_code/2022/day_02"
	"advent_of_code/2022/day_03"
	"advent_of_code/2022/day_04"
	h "advent_of_code/2022/helpers"
)

func main() {
	defer h.End(h.Start())

	day_01.Run()
	day_02.Run()
	day_03.Run()
	day_04.Run()
}
