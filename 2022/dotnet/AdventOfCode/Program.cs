using AdventOfCode.Days;

namespace AdventOfCode;

public class Solution {
    public static void Main(String[] args) {
        var days = new List<DayBase> {
            new Day01(),
            new Day02(),
            new Day03(),
        };

        Console.WriteLine($"Starting {days.Count}");

        foreach (var day in days){
            day.Process();
        }
    }
}
