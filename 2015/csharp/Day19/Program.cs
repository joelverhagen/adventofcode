using System;
using AdventOfCode.Day19.Part1;

namespace AdventOfCode.Day19
{
    public class Program
    {
        public void Run()
        {
            Console.WriteLine($"Part 1 answer: {new Part1.Solver().GetAnswer(@"Day19\input.txt")}");

            Console.WriteLine($"Part 2 answer: {new Part2.Solver().GetAnswer(@"Day19\input.txt")}");
        }
    }
}
