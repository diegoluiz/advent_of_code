package day_02

import (
	h "advent_of_code/2022/helpers"
	"fmt"
	"os"
	"strings"
)

var day string = "02"

type HandShape int

const (
	Rock     HandShape = 0
	Paper              = 1
	Scissors           = 2
)

type GameResult int

const (
	Win GameResult = iota
	Lose
	Tie
)

func ConvOp(x string) HandShape {
	switch x {
	case "A":
		return Rock
	case "B":
		return Paper
	case "C":
		return Scissors
	default:
		return 0
	}
}

func ConvMe(me string) HandShape {
	switch me {
	case "X":
		return Rock
	case "Y":
		return Paper
	case "Z":
		return Scissors
	default:
		return 0
	}
}

func ConvMeP2(me string, op HandShape) HandShape {
	switch me {
	case "Y":
		return op
	case "X":
		return HandShape((op + 2) % 3)
	case "Z":
		return HandShape((op + 1) % 3)
	default:
		return 0
	}
}

func Game(op HandShape, me HandShape) GameResult {
	if (op == Rock) && (me == Paper) {
		return Win
	}
	if (op == Rock) && (me == Scissors) {
		return Lose
	}

	if (op == Paper) && (me == Scissors) {
		return Win
	}
	if (op == Paper) && (me == Rock) {
		return Lose
	}

	if (op == Scissors) && (me == Rock) {
		return Win
	}
	if (op == Scissors) && (me == Paper) {
		return Lose
	}

	return Tie
}

func Run() {
	defer h.EndDay(h.StartDay(day))
	lines, _ := os.ReadFile("./day_" + day + "/data/input.txt")
	input := string(lines)

	games := strings.Split(input, "\n")

	points_p1 := 0
	for _, game := range games {
		x := strings.Split(game, " ")
		op := ConvOp(x[0])
		me := ConvMe(x[1])
		res := Game(op, me)
		switch res {
		case Win:
			points_p1 += 6
		case Tie:
			points_p1 += 3
		}
		points_p1 += int(me) + 1
	}

	points_p2 := 0

	for _, game := range games {
		x := strings.Split(game, " ")
		op := ConvOp(x[0])
		me := ConvMeP2(x[1], op)
		res := Game(op, me)
		switch res {
		case Win:
			points_p2 += 6
		case Tie:
			points_p2 += 3
		}
		points_p2 += int(me) + 1
	}

	fmt.Printf(" - p1: [%v]\n - p2: [%v]\n", points_p1, points_p2)
}
