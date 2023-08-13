namespace AdventOfCode.Days;

public class Day03 : DayBase
{
    protected override string Part01(string input)
    {
        var score = input.Split("\n").Select(x =>
        {
            var len = x.Length;
            var c1 = x[..(len / 2)].ToCharArray();
            var c2 = x[(len / 2)..].ToCharArray();
            var dup = c1.First(y => c2.Contains(y));
            return (dup >= 'a') ? dup - 'a' + 1 : dup - 'A' + 27;
        }).Sum();
        return score.ToString();
    }

    protected override string Part02(string input)
    {
        var score = input.Split("\n")
            .Chunk(3)
            .Select(group =>
            {
                var packs = group.Select(x => x.ToCharArray()).ToArray();
                var dup = packs[0].First(y => packs[1].Contains(y) && packs[2].Contains(y));
                return (dup >= 'a') ? dup - 'a' + 1 : dup - 'A' + 27;
            }).Sum();
        return score.ToString();
    }
}