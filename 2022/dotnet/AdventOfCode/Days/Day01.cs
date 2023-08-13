namespace AdventOfCode.Days;

public class Day01 : DayBase
{
    public override string Name => "Day 01";

    protected override string Part01(string input)
    {
        var elvensCals = input.Split("\n\n").Select(x => {
            return x.Split("\n").Select(y=> int.Parse(y)).ToList().Sum();
        }).ToList();
        _part1Buffer = elvensCals;
        return elvensCals.Max().ToString();
    }

    protected override string Part02(string input)
    {
        var elvensCals = ((List<int>)_part1Buffer).OrderDescending().Take(3);
        return elvensCals.Sum().ToString();
    }
}