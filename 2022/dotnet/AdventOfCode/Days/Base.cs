namespace AdventOfCode.Days;

public abstract class DayBase {
    protected object? _part1Buffer;

    public string? Name { get; private set; }

    public void Process(){
        Name = GetType().Name;

        Console.WriteLine($"Processing day {Name}");
        var input  = LoadInput();
        var output = Part01(input);
        Console.WriteLine(output);
        
        output = Part02(input);
        Console.WriteLine(output);

    }

    protected string LoadInput() {
        var curr = Directory.GetCurrentDirectory();
        var inputPath = Path.Join(curr, "inputs");
        var currDayInput = Path.Join(inputPath, $"{Name}.txt");
        var input = File.ReadAllText(currDayInput);
        return input;
    }

    protected abstract string Part01(string input);
    protected abstract string Part02(string input);
}