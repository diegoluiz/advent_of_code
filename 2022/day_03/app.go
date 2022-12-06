package day_03

import (
	h "advent_of_code/2022/helpers"
	"fmt"
	"os"
	"strings"
	"unicode"
)

type M map[rune]bool

func find_duplicate(l M, r M) rune {
	for l_c := range l {
		if r[l_c] {
			return l_c
		}
	}
	return ' '
}

func find_group(r1 M, r2 M, r3 M) rune {
	for r1_l := range r1 {
		if r2[r1_l] && r3[r1_l] {
			return r1_l
		}
	}
	return ' '
}

func get_priority(letter rune) int {
	if unicode.IsUpper(letter) {
		score := int(letter) - 64 + 26
		return score
	} else {
		score := int(letter) - 96
		return score
	}
}

func get_set(str string) map[rune]bool {
	m := make(map[rune]bool)
	for _, v := range str {
		m[v] = true
	}
	return m
}

func Run() {
	defer h.EndDay(h.StartDay("03"))
	lines, _ := os.ReadFile("./day_03/data/input.txt")
	input := string(lines)

	rucksacks := strings.Split(input, "\n")

	p1_total := 0
	p2_total := 0

	var groups []M = make([]M, 3)

	for idx, rucksack := range rucksacks {
		mid := len(rucksack) / 2
		l := get_set(rucksack[:mid])
		r := get_set(rucksack[mid:])
		dup := find_duplicate(l, r)
		p1_score := get_priority(dup)
		p1_total += p1_score

		i := idx % 3
		groups[i] = get_set(rucksack)
		if i == 2 {
			group := find_group(groups[0], groups[1], groups[2])
			p2_score := get_priority(group)
			p2_total += p2_score
			groups = make([]M, 3)
		}
	}

	fmt.Printf(" - p1: [%v]\n - p2: [%v]\n", p1_total, p2_total)
}
