namespace AdventOfCode.Days;

public class Day02 : DayBase
{
    private enum Hand {
        Rock = 1,
        Paper = 2,
        Scissors = 3,
    }

    private int CalculateGameScore(Hand other, Hand mine) {
        if (other == mine) return 3;

        if (other == Hand.Rock) {
            if (mine == Hand.Paper) return 6;
            else return 0;
        }

        if (other == Hand.Paper) {
            if (mine == Hand.Scissors) return 6;
            else return 0;
        }

        if (other == Hand.Scissors) {
            if (mine == Hand.Rock) return 6;
            else return 0;
        }

        return 0;
    }

    protected override string Part01(string input)
    {
        var score = input.Split('\n').Select(x=> {
            var a = x.Split(" ");
            Hand other = 0, mine = 0;
            switch (a[0])
            {
                case "A":
                    other = Hand.Rock;
                    break;
                case "B":
                    other = Hand.Paper;
                    break;
                case "C":
                    other = Hand.Scissors;
                    break;
                default:
                    break;
            }
            switch (a[1])
            {
                case "X":
                    mine = Hand.Rock;
                    break;
                case "Y":
                    mine = Hand.Paper;
                    break;
                case "Z":
                    mine = Hand.Scissors;
                    break;
                default:
                    break;
            }

            return new Tuple<Hand, Hand>(other, mine);
        }).Select(x=> {
            var output = CalculateGameScore(x.Item1, x.Item2);
            return ((int)x.Item2) + output;

        }).Sum();
        return score.ToString();
    }

    protected override string Part02(string input)
    {
        var score = input.Split('\n').Select(x=> {
            var a = x.Split(" ");
            Hand other = 0, mine = 0;
            switch (a[0])
            {
                case "A":
                    other = Hand.Rock;
                    break;
                case "B":
                    other = Hand.Paper;
                    break;
                case "C":
                    other = Hand.Scissors;
                    break;
                default:
                    break;
            }
            switch (a[1])
            {
                case "X":
                    if (other == Hand.Rock) mine = Hand.Scissors;
                    if (other == Hand.Paper) mine = Hand.Rock;
                    if (other == Hand.Scissors) mine = Hand.Paper;
                    break;
                case "Y":
                    mine = other;
                    break;
                case "Z":
                    if (other == Hand.Rock) mine = Hand.Paper;
                    if (other == Hand.Paper) mine = Hand.Scissors;
                    if (other == Hand.Scissors) mine = Hand.Rock;
                    break;
                default:
                    break;
            }

            return new Tuple<Hand, Hand>(other, mine);
        }).Select(x=> {
            var output = CalculateGameScore(x.Item1, x.Item2);
            return ((int)x.Item2) + output;
        }).Sum();
        return score.ToString();
    }
}