namespace AdventOfCode.Days;

public abstract class DayBase {
    protected object _part1Buffer;

    public abstract string Name { get; }

    public void Process(){
        Console.WriteLine($"Processing day {this.Name}");
        var input  = LoadInput();
        var output = Part01(input);
        System.Console.WriteLine(output);
        
        output = Part02(input);
        System.Console.WriteLine(output);

    }

    protected string LoadInput() {
        var curr = Directory.GetCurrentDirectory();
        var inputPath = Path.Join(curr, "inputs");
        var currDayInput = Path.Join(inputPath, $"{this.GetType().Name}.txt");
        var input = File.ReadAllText(currDayInput);
        return input;
    }

    protected abstract string Part01(string input);
    protected abstract string Part02(string input);
}