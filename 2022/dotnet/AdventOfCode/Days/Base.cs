using System.Diagnostics;

namespace AdventOfCode.Days;

public abstract class DayBase
{
    protected object? _part1Buffer;

    public string? Name { get; private set; }

    public void Process()
    {
        Name = GetType().Name;

        var sw = new Stopwatch();
        sw.Start();

        var input = LoadInput();
        var p1output = Part01(input);

        var p2output = Part02(input);

        Console.WriteLine($"Processed day {Name} in {sw.Elapsed.TotalNanoseconds} nanosec");
        Console.WriteLine($" - {p1output}");
        Console.WriteLine($" - {p2output}");
    }

    protected string LoadInput()
    {
        var curr = Directory.GetCurrentDirectory();
        var inputPath = Path.Join(curr, "inputs");
        var currDayInput = Path.Join(inputPath, $"{Name}.txt");
        var input = File.ReadAllText(currDayInput);
        return input;
    }

    protected abstract string Part01(string input);
    protected abstract string Part02(string input);
}